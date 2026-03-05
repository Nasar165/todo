use std::{fmt::format, slice::Iter};

use task::Task;

use crate::{Manger, command::Command};

pub trait Add {
    fn process(&self, mut args: Iter<String>, manager: &Manger) -> Result<String, &'static str> {
        let Some(v) = args.next() else {
            return Err("add command requires a title");
        };

        if v.trim().is_empty() {
            return Err("task can't be an empty string or whitespace");
        }

        let t = Task::from(v.as_str());
        if manager.io.write(format!("{}\n", t).as_str()).is_err() {
            return Err("failed to write task to file ");
        }

        Ok(format!("{t} has been added"))
    }
}

impl Add for Command {}

#[cfg(test)]
mod test {
    use std::{fs, io};

    use super::*;

    struct Mock;

    impl Add for Mock {}

    fn clean() -> io::Result<()> {
        fs::remove_file("./todo.txt")
    }

    fn new_manager() -> Manger {
        Manger {
            io: Box::new(files::FileManager::new_manager("./todo.txt").unwrap()),
        }
    }

    #[test]
    fn test_fail_add_task() {
        let manager = new_manager();
        let r = Add::process(&Mock {}, [].iter(), &manager);
        clean().unwrap();
        assert!(r.is_err())
    }

    #[test]
    fn test_fail_empty_title() {
        let manager = new_manager();
        let r = Add::process(&Mock {}, [String::from(" ")].iter(), &manager);
        clean().unwrap();
        assert!(r.is_err())
    }

    #[test]
    fn add_task() {
        let manager = new_manager();
        let r = Add::process(
            &Mock {},
            [String::from("a super new task")].iter(),
            &manager,
        );
        clean().unwrap();
        assert!(r.is_ok())
    }
}
