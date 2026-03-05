use std::slice::Iter;

use crate::command::Command;

pub trait List {
    fn process(&self, args: Iter<String>) -> Result<String, &'static str> {
        todo!()
    }
}

impl List for Command {}
