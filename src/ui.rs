use rfd::FileDialog;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn select_folder(title: &str) -> Option<PathBuf> {
    FileDialog::new().set_title(title).pick_folder()
}

pub fn process_files_in_directory(
    input_folder: &PathBuf,
    output_folder: &PathBuf,
    convert_fn: fn(&PathBuf, &PathBuf) -> std::io::Result<()>,
) {
    for entry in WalkDir::new(input_folder)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file()
            && entry.path().extension().and_then(|ext| ext.to_str()) == Some("dnt")
        {
            let input_file = entry.path().to_path_buf();
            let output_file = output_folder.join(entry.file_name());

            if let Err(e) = convert_fn(&input_file, &output_file) {
                eprintln!("Error converting file {}: {}", input_file.display(), e);
            } else {
                println!("Converted file: {}", input_file.display());
            }
        }
    }
}
