use std::slice::Iter;

use crate::command::Command;

pub trait Remove {
    fn process(&self, args: Iter<String>) -> Result<String, &'static str> {
        todo!()
    }
}

impl Remove for Command {}
