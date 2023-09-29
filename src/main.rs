use std::env;
use cli_utils_rs::run_echo;

fn main() {
    let output = run_echo(env::args());

    println!("{}", output.join(" "));
}