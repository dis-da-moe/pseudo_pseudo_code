#[cfg(test)]
mod test {
    use crate::parse_and_run;

    #[cfg(not(windows))]
    macro_rules! main_separator {
        () => {
            "/"
        };
    }

    #[cfg(windows)]
    macro_rules! main_separator {
        () => {
            r#"\"#
        };
    }

    macro_rules! example {
        ($file_name: expr) => {
            parse_and_run(
                include_str!(concat!(
                    "..",
                    main_separator!(),
                    "examples",
                    main_separator!(),
                    $file_name
                ))
                .to_owned(),
                $file_name.to_owned(),
            )
            .unwrap()
        };
    }
    #[test]
    fn examples() {
        example!("bubbleSort.psps");
    }
    #[test]
    #[should_panic]
    fn not_assigned(){
        example!("error.psps");
    }
}
