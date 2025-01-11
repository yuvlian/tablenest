use crate::dnt::{Column, FLOAT32, FLOAT64, Header, INT32, LPNNTS, UINT8, UINT32, WriteCell, UINT16};
use crate::io_utils::{create_reader, create_writer};
use byteorder::WriteBytesExt;
use read_from::{ReadFrom};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::Instant;

pub fn convert_to_tsv(input_path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
    let start = Instant::now();

    let mut reader = create_reader(input_path)?;
    let mut writer = create_writer(output_path)?;
    let tab: u8 = 0x09;
    let new_line: u8 = 0x0a;

    // header
    let header = Header::read_from(&mut reader)?;
    if header.magic_number.0 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unexpected magic number: {}", header.magic_number.0),
        ));
    }

    // columns
    let column_count = header.column_count.0 as usize + 1;
    let mut columns: Vec<Column> = Vec::with_capacity(column_count);
    columns.push(Column {
        name: LPNNTS("_RowID|3".as_bytes().to_vec()),
        data_type: UINT8(2), // UINT32
    });
    writer.write("_RowID|3".as_bytes())?;
    for _ in 1..column_count {
        writer.write_u8(tab)?;
        let col = Column::read_from(&mut reader)?;
        col.name.write_to(&mut writer)?;
        columns.push(col);
    }
    writer.write_u8(new_line)?;

    // rows
    let row_count = header.row_count.0 as usize;
    for _ in 0..row_count {
        for i in 0..column_count {
            if i > 0 {
                writer.write_u8(tab)?;
            }
            match columns[i].data_type.0 {
                1 => {
                    let len = UINT16::read_from(&mut reader)?;
                    let mut handle = reader.by_ref().take(len.0 as u64);
                    io::copy(&mut handle, &mut writer)?;
                }
                2 => INT32::read_from(&mut reader)?.write_to(&mut writer)?,
                3 => UINT32::read_from(&mut reader)?.write_to(&mut writer)?,
                4 => FLOAT32::read_from(&mut reader)?.write_to(&mut writer)?,
                5 => FLOAT32::read_from(&mut reader)?.write_to(&mut writer)?,
                6 => FLOAT64::read_from(&mut reader)?.write_to(&mut writer)?,
                x => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid column type: {}", x),
                    ));
                }
            };
        }
        writer.write_u8(new_line)?;
    }

    println!("Conversion completed in {:?}", start.elapsed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_conversion() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir
            .path()
            .to_path_buf()
            .join("skillleveltable_rune.tsv");
        let test_path = PathBuf::from("./test/skillleveltable_rune.dnt");
        println!("Test output: {:?}", temp_path);
        convert_to_tsv(&test_path, &temp_path).unwrap();
        let digest = sha256::try_digest(temp_path).unwrap();
        assert_eq!(
            digest,
            "8104cb51cfa3a37f5cfe4930cc88a71fc7172c06e9cc2af4da27f39375d9e70a"
        );
    }
}
