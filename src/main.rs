use std::env;
use std::process;

pub struct Config {
    pub args: Vec<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let mut output: Vec<String> = Vec::new();

        while let Some(arg) = args.next() {
            output.push(arg);
        }

        if output.is_empty() {
            return Err("No arguments found");
        }

        return Ok(Config {
            args: output,
        })
    }
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{}", config.args.join(" "));
}