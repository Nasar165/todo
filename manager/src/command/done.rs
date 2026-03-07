use std::slice::Iter;

use task::Task;

use crate::{
    Manger,
    command::{Command, process_index, valid_index, write_to_file},
};

pub trait Done {
    fn process(&self, args: Iter<String>, manager: &Manger) -> Result<String, &'static str> {
        let (index, buff) = process_index(args, manager)?;
        let mut l = Task::from_string_to_list(&buff);
        valid_index(l.len(), index)?;

        let Some(t) = l.get_mut(index) else {
            return Err("failed to get tasks");
        };

        t.done();
        let result = t.to_string();
        let l: Vec<String> = l.iter().map(|f| f.save()).collect();
        write_to_file(&l.join("\n"), manager)?;
        Ok(format!("{} has been marked as done", result))
    }
}

impl Done for Command {}

#[cfg(test)]
mod test {
    use std::{fs, io};

    use crate::command::add;

    use super::*;

    struct Mock;

    impl Done for Mock {}

    fn clean() -> io::Result<()> {
        fs::remove_file("./todo.txt")
    }

    fn new_manager() -> Manger {
        Manger {
            io: Box::new(files::FileManager::new_manager("./todo.txt").unwrap()),
        }
    }

    #[test]
    fn done() {
        let manager = new_manager();
        let c = Command {};
        add::Add::process(&c, [String::from("welcome")].iter(), &manager).unwrap();
        let rm = Mock {}.process([String::from("1")].iter(), &manager);
        clean().unwrap();
        assert!(rm.is_ok())
    }
}
