use cli_utils_rs::bin_selector;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    match bin_selector(args) {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("{}", err),
    }
}
