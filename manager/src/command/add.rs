use std::slice::Iter;

use crate::command::Command;

pub trait Add {
    fn process(&self, args: Iter<String>) -> Result<String, &'static str> {
        todo!()
    }
}

impl Add for Command {}
