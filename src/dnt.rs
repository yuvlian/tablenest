use byteorder::{LittleEndian, ReadBytesExt};
use indexmap::IndexMap;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};

#[derive(Debug, Clone, Copy)]
pub enum ColumnType {
    String = 1,
    Int32 = 2,
    UInt32 = 3,
    Float32 = 4,
    Float32Alt = 5,
    Float64 = 6,
}

impl TryFrom<u8> for ColumnType {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ColumnType::String),
            2 => Ok(ColumnType::Int32),
            3 => Ok(ColumnType::UInt32),
            4 => Ok(ColumnType::Float32),
            5 => Ok(ColumnType::Float32Alt),
            6 => Ok(ColumnType::Float64),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid column type",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub column_type: ColumnType,
}

impl Column {
    #[inline]
    fn u32_column() -> Self {
        Self {
            column_type: ColumnType::UInt32,
        }
    }
}

#[derive(Debug)]
pub struct DntHeader {
    pub row_count: u32,
    pub columns: IndexMap<String, Column>,
}

impl DntHeader {
    pub fn read(reader: &mut BufReader<File>) -> io::Result<Self> {
        reader.seek(SeekFrom::Current(4))?;

        let column_count = reader.read_u16::<LittleEndian>()? as usize;
        let row_count = reader.read_u32::<LittleEndian>()?;

        let mut columns = IndexMap::with_capacity(column_count + 1);
        columns.insert(String::from("_RowID|3"), Column::u32_column());

        let mut buffer = Vec::new();

        for _ in 0..column_count {
            let name = read_string(reader, &mut buffer)?;
            let column_type = ColumnType::try_from(reader.read_u8()?)?;
            columns.insert(name, Column { column_type });
        }

        Ok(DntHeader { row_count, columns })
    }
}

fn read_string(reader: &mut BufReader<File>, buffer: &mut Vec<u8>) -> io::Result<String> {
    let length = reader.read_u16::<LittleEndian>()? as usize;
    buffer.clear();
    buffer.reserve(length);
    reader.take(length as u64).read_to_end(buffer)?;
    String::from_utf8(buffer.clone())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

pub struct RowData {
    pub row_id: u32,
    pub raw_data: Vec<u8>,
}
