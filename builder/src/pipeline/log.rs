#[macro_export]
macro_rules! err {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);

        // Format the output if the environment variable is not set or set to 1
        let formatted_output_enabled = std::env::var("FORMATTED_OUTPUT").map(|env| env == "1").unwrap_or(true);
        if formatted_output_enabled {
            use colored::Colorize;

            println!("{}", msg.red())
        } else {
            println!("{}", msg)
        }
    }};
}

#[macro_export]
macro_rules! warn {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);

        // Format the output if the environment variable is not set or set to 1
        let formatted_output_enabled = std::env::var("FORMATTED_OUTPUT").map(|env| env == "1").unwrap_or(true);
        if formatted_output_enabled {
            use colored::Colorize;

            println!("{}", msg.yellow())
        } else {
            println!("{}", msg)
        }
    }};
}

#[macro_export]
macro_rules! info {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);

        // Format the output if the environment variable is not set or set to 1
        let formatted_output_enabled = std::env::var("FORMATTED_OUTPUT").map(|env| env == "1").unwrap_or(true);
        if formatted_output_enabled {
            use colored::Colorize;

            println!("{}", msg.bold())
        } else {
            println!("{}", msg)
        }
    }};
}

#[macro_export]
macro_rules! progress {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);

        // Format the output if the environment variable is not set or set to 1
        let formatted_output_enabled = std::env::var("FORMATTED_OUTPUT").map(|env| env == "1").unwrap_or(true);
        if formatted_output_enabled {
            use colored::Colorize;

            println!("{}", msg.bright_black())
        } else {
            println!("{}", msg)
        }
    }};
}
