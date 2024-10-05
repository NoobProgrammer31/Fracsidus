pub trait DisplayError {
    fn summarize_error(&self) -> String;
}

#[derive(Debug)]
// Simulating ErrorKind-like enum with custom error types
pub enum Errortype {
    InvalidInput,
    InvalidChoice,
    EmptyInput,
    ZeroInput,
    // CustomError(String),
}

impl DisplayError for Errortype {
    fn summarize_error(&self) -> String {
        match self {
            Errortype::InvalidInput => {
                "\n-- Invalid Input! Please enter a valid number --\n".to_string()
            }
            Errortype::InvalidChoice => {
                "\n-- Invalid Choice! Only valid options are 1, 2, 3 --\n".to_string()
            }
            Errortype::EmptyInput => "\n-- No Input Detected! Please try again. --\n".to_string(),
            Errortype::ZeroInput => {
                "\n-- Zero is NOT a valid input for this operation. --\n".to_string()
            } // Errortype::CustomError(msg) => format!("Error: {}", msg),
        }
    }
}

pub fn handle_error(error: Errortype) {
    // Prints the summarized error using DisplayError trait
    println!("{}", error.summarize_error());
}
