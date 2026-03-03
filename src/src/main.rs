use manager::{Commands, Manger};
use std::env::args;

// fetch arguments from cli and submit
// for processing. Any error here will
//trigger a panic
fn run() {
    let args: Vec<String> = args().collect();
    let m = Manger::init();
    match m.process(args) {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    run();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_process() {
        run();
    }
}
