// 1. Parse arguments from cli expect len 3
// 2. argument index 2 is a command list, add, remove, done
// 3. argument index 3 expect single string of text for the title
// example: task add "get out and work on my plantation"
// example: task list
// example: task remove 1 (row number that is showed in task list command)
// example: task done 1 (row number that is showed in task list command)

use std::slice::Iter;

use crate::command::{Command, add::Add, done::Done, help, list::List, remove::Remove};

mod command;
pub struct Manger {
    io: Box<dyn files::FileIO>,
}

type CResult<T> = Result<T, &'static str>;

pub trait Cli {
    /// process arguments into commands capable of executing
    /// desired functionality
    fn command(&self, args: Iter<String>) -> CResult<String>;
}

impl Cli for Manger {
    fn command(&self, mut args: Iter<String>) -> CResult<String> {
        let c = Command {};
        let help = help::Help::process(&c, [].iter());
        let Some(v) = args.next() else {
            return help;
        };

        match v.as_str() {
            "add" => Add::process(&c, args, self),
            "list" => List::process(&c, args, self),
            "done" => Done::process(&c, args, self),
            "remove" => Remove::process(&c, args, self),
            _ => help,
        }
    }
}

impl Manger {
    /// creates a new cli with a file manager that will open or create
    /// a new file at current working directory. The function will panic
    /// if the FileIO fails to open or create todo.txt
    pub fn init() -> impl Cli {
        match files::FileManager::new_manager("./todo.txt") {
            Ok(f) => Manger { io: Box::new(f) },
            Err(e) => panic!("{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_empty_args() {
        let args = [];
        let r = Manger::init().command(args.iter());
        assert!(r.unwrap().contains("Usage: todo <command> <arguments>"));
    }
}
