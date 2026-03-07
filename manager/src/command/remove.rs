use std::slice::Iter;

use crate::{
    Manger,
    command::{Command, process_index, valid_index, write_to_file},
};

pub trait Remove {
    fn process(&self, args: Iter<String>, manager: &Manger) -> Result<String, &'static str> {
        let (index, buff) = process_index(args, manager)?;
        let mut l: Vec<&str> = buff.lines().collect();
        let index = valid_index(l.len(), index)?;
        let removed = l.remove(index);
        write_to_file(l.join("\n").as_str(), manager)?;
        Ok(format!("{} has been removed", removed))
    }
}

impl Remove for Command {}

#[cfg(test)]
mod test {
    use std::{fs, io};

    use crate::command::add;

    use super::*;

    struct Mock;

    impl Remove for Mock {}

    fn clean() -> io::Result<()> {
        fs::remove_file("./todo.txt")
    }

    fn new_manager() -> Manger {
        Manger {
            io: Box::new(files::FileManager::new_manager("./todo.txt").unwrap()),
        }
    }

    #[test]
    fn remove() {
        let manager = new_manager();
        let c = Command {};
        add::Add::process(&c, [String::from("welcome")].iter(), &manager).unwrap();
        let rm = Mock {}.process([String::from("1")].iter(), &manager);
        clean().unwrap();
        assert!(rm.is_ok())
    }
}
