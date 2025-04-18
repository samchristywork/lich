#[macro_export] macro_rules! invalid_arguments {
    ($name:expr, $arguments:expr, $expected:expr) => {
        Err(format!(
            "Invalid arguments for '{}'.\n\nFound: {:?}\n\nExpected one of:\n{}",
            $name, $arguments, $expected
                .iter()
                .map(|s| format!("- {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        ))
    };
}
