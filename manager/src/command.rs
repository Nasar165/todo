use std::slice::Iter;

use crate::Manger;

pub mod add;
pub mod done;
pub mod help;
pub mod list;
pub mod remove;

pub struct Command;

/// gets next value from the iterator and verifies that it's
/// a valid number and parsed into a usize. this function will also
/// return a string containing the content of todo text file.
fn process_index(
    mut args: Iter<String>,
    manager: &Manger,
) -> Result<(usize, String), &'static str> {
    let Some(index) = args.next() else {
        return Err("remove requires and index");
    };

    let index: usize = match index.parse() {
        Ok(i) => {
            if i == 0 {
                return Err("index can't be 0");
            }
            i
        }
        Err(_) => return Err("failed to process index"),
    };

    let mut buff = String::new();
    let Ok(_) = manager.io.read(&mut buff) else {
        return Err("failed to read todo file");
    };
    Ok((index, buff))
}

/// verifies that an index is within bounds
fn valid_index(len: usize, index: usize) -> Result<(), &'static str> {
    if len == 0 {
        return Err("length is 0, indicating not values");
    }

    let index = index - 1;
    if index > len {
        return Err("invalid index length");
    }
    Ok(())
}

/// write a list of task to a file by clearing the file and. then writing
/// the task to the file. This action is not bulletproof since there is a
/// risk that the write could fail leading to data loss.
/// it would be good to backup the file to tmp before clearing and writing.
fn write_to_file(tasks: &str, manager: &Manger) -> Result<(), &'static str> {
    if manager.io.clear_file().is_err() {
        return Err("failed to clear file before write");
    }

    if manager.io.write(tasks).is_err() {
        return Err("failed to write tasks to file");
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{fs, io};

    use super::*;

    fn clean() -> io::Result<()> {
        fs::remove_file("./todo.txt")
    }

    fn new_manager() -> Manger {
        Manger {
            io: Box::new(files::FileManager::new_manager("./todo.txt").unwrap()),
        }
    }

    #[test]
    fn write_file() {
        let manager = new_manager();
        let wr = write_to_file("test me\nwelcome\n", &manager);
        clean().unwrap();
        assert!(wr.is_ok())
    }

    #[test]
    fn get_index() {
        let manager = new_manager();
        let (index, _) = process_index([String::from("1")].iter(), &manager).unwrap();
        clean().unwrap();
        assert_eq!(index, 1)
    }

    #[test]
    fn index_is_valid() {
        assert!(valid_index(10, 5).is_ok());
        assert!(valid_index(0, 5).is_err());
        assert!(valid_index(0, 0).is_err());
    }
}
