use std::fs;

pub fn run_echo(mut args: impl Iterator<Item = String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        output.push(arg);
    }

    return output;
}

pub fn run_cat(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a filepath"),
    };

    if let Ok(content) = fs::read_to_string(file_path) {
        return Ok(content);
    } else {
        return Err("File does not exist.");
    };
}

pub fn bin_selector(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    match args.next() {
        Some(arg) if arg == "echo" => Ok(run_echo(args).join(" ")),
        Some(arg) if arg == "cat" => run_cat(args),
        Some(_) => Err("Invalid Command"),
        None => Err("No arguments received!"),
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

    #[test]
    fn bin_selector_test() {
        let args = vec!["echo", "asdf"];

        let result = bin_selector(args.into_iter().map(|elm| String::from(elm)));

        assert_eq!("asdf", result.unwrap());
    }
}
