use std::env;

fn main() {
    echo_function(env::args());
}

fn echo_function(mut args: impl Iterator<Item = String>) {
    args.next();
    let mut output: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        output.push(arg);
    }

    println!("{}", output.join(" "));
}