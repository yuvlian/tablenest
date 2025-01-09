use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

pub const BUFFER_SIZE: usize = 20 * 1024 * 1024;

pub fn create_reader<P: AsRef<Path>>(path: P) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::with_capacity(BUFFER_SIZE, file))
}

pub fn create_writer<P: AsRef<Path>>(path: P) -> io::Result<BufWriter<File>> {
    let file = File::create(path)?;
    Ok(BufWriter::with_capacity(BUFFER_SIZE, file))
}
