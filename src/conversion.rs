use crate::dnt::{ColumnType, DntHeader, RowData};
use crate::io_utils::{create_reader, create_writer};
use byteorder::{LittleEndian, ReadBytesExt};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::Instant;

pub fn convert_to_tsv(input_path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
    let start = Instant::now();

    let mut reader = create_reader(input_path)?;
    let header = DntHeader::read(&mut reader)?;

    let mut rows = Vec::with_capacity(header.row_count as usize);

    for _ in 0..header.row_count {
        let mut raw_data = Vec::new();
        let row_id = reader.read_u32::<LittleEndian>()?;

        for (_, column) in header.columns.iter().skip(1) {
            match column.column_type {
                ColumnType::String => {
                    let length = reader.read_i16::<LittleEndian>()?;
                    raw_data.extend_from_slice(&length.to_le_bytes());
                    if length > 0 {
                        let mut buffer = vec![0; length as usize];
                        reader.read_exact(&mut buffer)?;
                        raw_data.extend(buffer);
                    }
                }
                ColumnType::Int32
                | ColumnType::UInt32
                | ColumnType::Float32
                | ColumnType::Float32Alt => {
                    let mut buffer = [0; 4];
                    reader.read_exact(&mut buffer)?;
                    raw_data.extend_from_slice(&buffer);
                }
                ColumnType::Float64 => {
                    let mut buffer = [0; 8];
                    reader.read_exact(&mut buffer)?;
                    raw_data.extend_from_slice(&buffer);
                }
            }
        }
        rows.push(RowData { row_id, raw_data });
    }

    let processed_rows: Vec<String> = rows
        .par_iter()
        .map(|row| {
            let mut cursor = std::io::Cursor::new(&row.raw_data);
            let mut values = Vec::with_capacity(header.columns.len());
            values.push(row.row_id.to_string());

            for (_, column) in header.columns.iter().skip(1) {
                let value = match column.column_type {
                    ColumnType::String => {
                        let length = cursor.read_i16::<LittleEndian>().unwrap() as usize;
                        if length > 0 {
                            let mut buffer = vec![0; length];
                            cursor.read_exact(&mut buffer).unwrap();
                            String::from_utf8_lossy(&buffer).into_owned()
                        } else {
                            String::new()
                        }
                    }
                    ColumnType::Int32 | ColumnType::UInt32 => {
                        cursor.read_i32::<LittleEndian>().unwrap().to_string()
                    }
                    ColumnType::Float32 | ColumnType::Float32Alt => {
                        cursor.read_f32::<LittleEndian>().unwrap().to_string()
                    }
                    ColumnType::Float64 => cursor.read_f64::<LittleEndian>().unwrap().to_string(),
                };
                values.push(value);
            }
            values.join("\t")
        })
        .collect();

    let mut writer = create_writer(output_path)?;
    writeln!(
        writer,
        "{}",
        header
            .columns
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .join("\t")
    )?;

    let row_to_write = processed_rows.join("\n");

    writeln!(writer, "{}", row_to_write)?;
    writer.flush()?;

    println!("Conversion completed in {:?}", start.elapsed());
    Ok(())
}
