use cli_utils_rs::bin_selector;
use std::env;

fn main() {
    let args = env::args();
    match bin_selector(args) {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("{}", err),
    }
}
