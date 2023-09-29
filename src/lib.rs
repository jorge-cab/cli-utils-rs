pub fn run_echo(mut args: impl Iterator<Item = String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        output.push(arg);
    }

    return output;
}

pub fn bin_selector(mut args: impl Iterator<Item = String>) {
    args.next();

    match args.next() {
        Some(arg) if arg == "echo" => println!("{}", run_echo(args).join(" ")),
        Some(arg) => eprintln!("Invalid Command: {}", arg),
        None => eprintln!("No arguments received!"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn echo_test() {
        let args = vec!["arg0"];
        let result = run_echo(args.into_iter().map(|elm| String::from(elm)));

        assert_eq!(
            result,
            vec!["arg0"]
                .into_iter()
                .map(|elm| String::from(elm))
                .collect::<Vec<String>>()
        );

        let args = vec!["arg0", "arg1", "arg2"];
        let result = run_echo(args.into_iter().map(|elm| String::from(elm)));

        assert_eq!(
            result,
            vec!["arg0", "arg1", "arg2"]
                .into_iter()
                .map(|elm| String::from(elm))
                .collect::<Vec<String>>()
        );
    }
}
