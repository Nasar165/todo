use std::slice::Iter;

use crate::{
    Manger,
    command::{Command, process_index, write_to_file},
};

pub trait Remove {
    fn process(&self, args: Iter<String>, manager: &Manger) -> Result<String, &'static str> {
        let (index, buff) = process_index(args, manager)?;
        let mut l: Vec<&str> = buff.lines().collect();
        let len = l.len();
        let index = index - 1;
        if index > len {
            return Err("index is greater than the list");
        }

        let removed = l.remove(index);
        write_to_file(l.join("\n").as_str(), manager)?;
        Ok(format!("{} has been removed", removed))
    }
}

impl Remove for Command {}
