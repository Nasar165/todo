use manager::{Cli, Manger};
use std::env::args;

/// fetch arguments from cli and submit
/// for processing. Any error here will
/// trigger a panic
fn run() {
    let args: Vec<String> = args().collect();
    let mut args = args.iter();
    args.next();
    let m = Manger::init();
    match m.command(args) {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("failed to process command \n{}", e),
    }
}

fn main() {
    run();
}
