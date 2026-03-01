use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

pub struct Manager {
    file: Box<File>,
}

impl Manager {
    pub fn open(path: &str) -> io::Result<File> {
        fs::OpenOptions::new().read(true).append(true).open(path)
    }

    pub fn create_file(path: &str) -> io::Result<File> {
        let r = Manager::file_exists(path)?;
        if r {
            Manager::open(path)
        } else {
            File::create_new(path)
        }
    }

    pub fn file_exists(path: &str) -> io::Result<bool> {
        fs::exists(path)
    }

    pub fn write(&self, buf: &str) -> io::Result<usize> {
        let mut file = self.file.as_ref();
        file.write(buf.as_bytes())
    }

    pub fn read(&self, buff: &mut String) -> io::Result<usize> {
        //TODO fix bug where read always returns an empty string
        let mut file = self.file.as_ref();
        file.read_to_string(buff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "./test.md";
    const FAILED_TO_CREATE: &str = "failed to create a new file";

    #[test]
    fn new_file() -> Result<(), io::Error> {
        let f = Manager::create_file(PATH);
        match f {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[test]
    fn open_file() {
        new_file().expect(FAILED_TO_CREATE);
        Manager::open(PATH).expect("file did not open");
    }

    #[test]
    fn file_exists() {
        new_file().expect(FAILED_TO_CREATE);
        let Ok(f) = Manager::file_exists(PATH) else {
            panic!("failed to verify file integrity")
        };
        assert!(f);
    }

    #[test]
    fn write_file() -> Result<(), io::Error> {
        let f = Manager::create_file(PATH)?;
        let m = Manager { file: Box::new(f) };
        let w = m.write("hello\n")?;
        assert!(w > 0);
        Ok(())
    }

    #[test]
    fn read_file() -> Result<(), io::Error> {
        let f = Manager::create_file(PATH)?;
        let m = Manager { file: Box::new(f) };
        let t = "buff me\n";
        m.write(t)?;
        let mut text = String::new();
        let read = m.read(&mut text).expect("failed to read text from file");
        if read == 0 {
            panic!("failed to read from file size: {}", read)
        }
        assert_eq!(text, t);
        Ok(())
    }
}
