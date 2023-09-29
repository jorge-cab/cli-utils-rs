use std::env;

fn main() {
    let output = run_echo(env::args());

    println!("{}", output.join(" "));
}

pub fn run_echo(mut args: impl Iterator<Item = String>) -> Vec<String> {
    args.next();
    let mut output: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        output.push(arg);
    }

    return output;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn echo_test() {
        let args = vec!["nil", "arg0"];
        let result = run_echo(args.into_iter().map(|elm| String::from(elm)));

        assert_eq!(result, vec!["arg0"].into_iter().map(|elm| String::from(elm)).collect::<Vec<String>>());

        let args = vec!["nil", "arg0", "arg1", "arg2"];
        let result = run_echo(args.into_iter().map(|elm| String::from(elm)));

        assert_eq!(result, vec!["arg0", "arg1", "arg2"].into_iter().map(|elm| String::from(elm)).collect::<Vec<String>>());
    }
}
