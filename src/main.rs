use crate::ui::{process_files_in_directory, select_folder};

mod conversion;
mod dnt;
mod io_utils;
mod ui;

fn main() -> std::io::Result<()> {
    println!("Select folder containing .dnt files");
    if let Some(input_folder) = select_folder("Select folder containing .dnt files") {
        println!("Select folder to output the converted files");
        if let Some(output_folder) = select_folder("Select folder to output the converted files") {
            process_files_in_directory(&input_folder, &output_folder, conversion::convert_to_tsv);
        }
    }

    Ok(())
}
