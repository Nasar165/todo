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
";

// returns helpful information on how to used the
// cli and guiding the user towards intended usage.
pub fn help() -> &'static str {
    TODO_HELP
}

#[cfg(test)]
mod test {
    use crate::help::{TODO_HELP, help};

    #[test]
    fn get_help() {
        assert_eq!(help(), TODO_HELP)
    }
}
