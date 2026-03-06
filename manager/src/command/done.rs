use std::slice::Iter;

use task::Task;

use crate::{
    Manger,
    command::{Command, process_index, write_to_file},
};

pub trait Done {
    fn process(&self, args: Iter<String>, manager: &Manger) -> Result<String, &'static str> {
        let (index, buff) = process_index(args, manager)?;
        let mut l = Task::from_string_to_list(&buff);
        let len = l.len();
        let index = index - 1;
        if index > len {
            return Err("index is greater than the list");
        }

        let Some(t) = l.get_mut(index) else {
            return Err("failed to get tasks");
        };

        t.done();
        let result = t.to_string();
        let l: Vec<String> = l.iter().map(|f| f.to_string()).collect();
        write_to_file(&l.join("\n"), manager)?;
        Ok(format!("{} has been marked as done", result))
    }
}

impl Done for Command {}
