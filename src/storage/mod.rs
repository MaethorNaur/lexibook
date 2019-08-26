use std::fs;
use std::io::prelude::*;
use std::path::Path;
use zip::result::ZipError;
pub fn write(filename: &str, txt: &str) -> Result<(), ZipError> {
    let path = Path::new(filename);
    let file = fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file("test.txt", Default::default())?;
    zip.write_all(txt.as_bytes())?;
    zip.finish().map(|_| ())
}
