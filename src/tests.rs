#[cfg(test)]
mod tests {
    use crate::*;

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
