use colored::*;

#[macro_export]
macro_rules! print_err {
    ($error:expr) => {
        println!(
            "\n{} {} {} {}{}",
            "Error:".red(),
            $error,
            file!().red(),
            "line:".red(),
            format!("{}", line!()).red()
        )
    };
}
