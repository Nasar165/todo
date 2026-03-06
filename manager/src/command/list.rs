use std::slice::Iter;

use crate::{Manger, command::Command};

pub trait List {
    fn process(&self, _args: Iter<String>, manager: &Manger) -> Result<String, &'static str> {
        let mut buff = String::new();
        let Ok(_) = manager.io.read(&mut buff) else {
            return Err("failed to read file");
        };

        let mut i = 0;
        let tasks: Vec<String> = buff
            .lines()
            .map(|l| {
                let t = task::Task::from(l);
                i += 1;
                format!("{i}: {}", t)
            })
            .collect();
        Ok(tasks.join("\n"))
    }
}

impl List for Command {}

#[cfg(test)]
mod test {
    use std::{fs, io};

    use super::*;

    struct Mock;

    impl List for Mock {}

    fn clean() -> io::Result<()> {
        fs::remove_file("./todo.txt")
    }

    #[test]
    fn list_all() {
        let manager = Manger {
            io: Box::new(files::FileManager::new_manager("./todo.txt").unwrap()),
        };
        let m = Mock {};
        let read = m.process([].iter(), &manager);
        clean().unwrap();
        assert!(read.is_ok());
    }
}
