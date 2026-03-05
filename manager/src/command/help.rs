use std::slice::Iter;

use crate::command::Command;

const TODO_HELP: &str = "todo is a simple rust cli designed to help keep track of simple tasks

Usage: todo <command> <arguments>

The commands are:
    list    lists all task with an index number
                - ex, todo list
    add     adds a new task
                - ex, todo add \"my super task\"
    done    marks a tasks as done using the index number from list
                - ex, todo done 1
    remove  removes a task from the list using the index number from list
                - ex, todo remove 2
    help    prints available commands with simple examples
                - ex, todo help
";

pub trait Help {
    fn process(&self, _args: Iter<String>) -> Result<String, &'static str> {
        Ok(TODO_HELP.to_string())
    }
}

impl Help for Command {}

#[cfg(test)]
mod test {
    use super::*;

    struct Mock;

    impl Help for Mock {}

    #[test]
    fn get_help() {
        let res = Help::process(&Mock {}, [].iter()).unwrap();
        assert_eq!(res, TODO_HELP)
    }
}
