pub extern crate colored;

#[macro_export]
macro_rules! print_err {
    ($error:expr) => {
        use handle_errors::colored::Colorize;
        println!(
            "\n{} {} {} {}{}",
            "Error:".red(),
            $error,
            file!().red(),
            "line: ".red(),
            line!()
        )
    };
}
