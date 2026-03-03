// 1. Parse arguments from cli expect len 3
// 2. argument index 2 is a command list, add, remove, done
// 3. argument index 3 expect single string of text for the title
// example: task add "get out and work on my plantation"
// example: task list
// example: task remove 1 (row number that is showed in task list command)
// example: task done 1 (row number that is showed in task list command)

use help::help;

mod help;

//TODO implement state management making simplifying command handling
pub struct Manger;

type CResult<T> = Result<T, &'static str>;

pub trait Commands {
    fn process(&self, args: Vec<String>) -> CResult<String>;
}

impl Commands for Manger {
    fn process(&self, args: Vec<String>) -> CResult<String> {
        if args.len() <= 1 {
            return Err(help());
        }
        Ok(String::new())
    }
}

impl Manger {
    pub fn init() -> impl Commands {
        Manger {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARGS: [&str; 3] = ["./lib.rs", "add", "test task"];

    #[test]
    fn init_empty_args() {
        let r = Manger::init().process(vec![]);
        assert!(r.is_err());
    }
}
