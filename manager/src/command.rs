use std::slice::Iter;

use crate::Manger;

pub mod add;
pub mod done;
pub mod help;
pub mod list;
pub mod remove;

pub struct Command;

fn process_index(
    mut args: Iter<String>,
    manager: &Manger,
) -> Result<(usize, String), &'static str> {
    let Some(index) = args.next() else {
        return Err("remove requires and index");
    };

    let index: usize = match index.parse() {
        Ok(i) => {
            if i == 0 {
                return Err("index can't be 0");
            }
            i
        }
        Err(_) => return Err("failed to process index"),
    };

    let mut buff = String::new();
    let Ok(_) = manager.io.read(&mut buff) else {
        return Err("failed to read todo file");
    };
    Ok((index, buff))
}

fn write_to_file(tasks: &str, manager: &Manger) -> Result<(), &'static str> {
    if manager.io.clear_file().is_err() {
        return Err("failed to clear file before write");
    }

    if manager.io.write(tasks).is_err() {
        return Err("failed to write tasks to file");
    };

    Ok(())
}
