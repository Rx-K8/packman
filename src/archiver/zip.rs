use std::io::Read;
use std::io::{BufReader, Seek, Write};
use std::path::PathBuf;
use zip::write::SimpleFileOptions;

use std::fs::File;
use zip::result::ZipResult;
use zip::write::ZipWriter;

fn process_file<W: Write + Seek>(zw: &mut ZipWriter<W>, target: PathBuf) -> ZipResult<()> {
    let mut reader = BufReader::new(File::open(&target)?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let options = SimpleFileOptions::default();
    let file_name = target.to_str().unwrap();
    zw.start_file(file_name, options)?;
    zw.write(contents.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file() -> ZipResult<()> {
        let target = PathBuf::from("test.txt");
        let mut zw = ZipWriter::new(File::create("test1.zip")?);
        let result = process_file(&mut zw, target);
        assert!(result.is_ok());
        Ok(())
    }
}
