use crate::io_utils::{create_reader, create_writer};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::Instant;
use byteorder::WriteBytesExt;
use read_from::ReadFrom;
use crate::dnt::{Column, Header, FLOAT32, FLOAT64, INT32, LPNNTS, UINT32, UINT8};

pub fn convert_to_tsv(input_path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
    let start = Instant::now();

    let mut reader = create_reader(input_path)?;
    let mut writer = create_writer(output_path)?;
    let tab: u8 = 0x09;
    let new_line: u8 = 0x0a;

    // header
    let header = Header::read_from(&mut reader)?;
    println!("Rows: {}, Cols: {}", header.row_count, header.column_count);

    // columns
    let column_count = header.column_count.0 as usize + 1;
    let mut columns: Vec<Column> = Vec::with_capacity(column_count);
    columns.push(Column {
        name: LPNNTS("_RowID|3".to_string()),
        data_type: UINT8(2), // UINT32
    });
    writer.write("_RowID|3".as_bytes())?;
    for _ in 1..column_count {
        writer.write_u8(tab)?;
        let col = Column::read_from(&mut reader)?;
        writer.write(col.name.0.as_bytes())?;
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
            let cell = match columns[i].data_type.0 {
                1 => LPNNTS::read_from(&mut reader)?.to_string(),
                2 => INT32::read_from(&mut reader)?.to_string(),
                3 => UINT32::read_from(&mut reader)?.to_string(),
                4 => FLOAT32::read_from(&mut reader)?.to_string(),
                5 => FLOAT32::read_from(&mut reader)?.to_string(),
                6 => FLOAT64::read_from(&mut reader)?.to_string(),
                x => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid column type: {}", x))),
            };
            writer.write(cell.as_bytes())?;
        }
        writer.write_u8(new_line)?;
    }

    println!("Conversion completed in {:?}", start.elapsed());
    Ok(())
}
