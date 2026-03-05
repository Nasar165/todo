use std::{
    fs::{self, File},
    io::{self, Read, Seek, Write},
};

pub struct FileManager {
    file: Box<File>,
}

pub trait FileIO {
    fn write(&self, buf: &str) -> io::Result<()>;
    fn read(&self, buff: &mut String) -> io::Result<usize>;
}

impl FileIO for FileManager {
    // write a string to the file using write_all
    // to ensure that all bytes are written to the file
    fn write(&self, buf: &str) -> io::Result<()> {
        let mut file = self.file.as_ref();
        file.write_all(buf.as_bytes())
    }

    /// read all the bytes from the created file until EOF
    /// this function will seek to start 0 before performing
    /// read_to_string allocating the bytes to the buffer.
    fn read(&self, buff: &mut String) -> io::Result<usize> {
        let mut file = self.file.as_ref();
        file.seek(io::SeekFrom::Start(0))?;
        file.read_to_string(buff)
    }
}

impl FileManager {
    pub fn new_manager(path: &str) -> Result<impl FileIO, io::Error> {
        let file = FileManager::open(path)?;
        Ok(FileManager {
            file: Box::new(file),
        })
    }

    fn open(path: &str) -> io::Result<File> {
        fs::OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), io::Error>;

    const PATH: &str = "./test.md";

    fn clean() -> io::Result<()> {
        fs::remove_file(PATH)
    }

    #[test]
    fn open_file() {
        FileManager::open(PATH).expect("file did not open");
    }

    #[test]
    fn write_file() -> TestResult {
        let m = FileManager::new_manager(PATH)?;
        m.write("hello\n")?;
        clean()?;
        Ok(())
    }

    #[test]
    fn read_file() -> TestResult {
        let m = FileManager::new_manager(PATH)?;
        let t = "buff me\n";
        m.write(t)?;
        let mut text = String::new();
        let size = m.read(&mut text).expect("failed to read text from file");
        assert!(size > 0);
        clean()?;
        Ok(())
    }
}
