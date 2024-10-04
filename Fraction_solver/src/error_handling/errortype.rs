#[derive(Debug)]
// Defining my own custom errors
// Enums are helpful to represent different variants of errors
pub enum Errortype {
    InvalidInput,
    // InvalidChoice,
    // EmptyInput,
    // ZeroInput,
    // CustomError(String),
}

// Struct will hold the formatted output
pub struct CustomFormatter {
    output: String,
}

impl CustomFormatter {
    // Used to create new instances of CustomFormatter
    // -> Self indicates that this method will return an instance of the struct it belongs to
    pub fn new() -> Self {
        CustomFormatter {
            output: String::new(),
        }
    }

    // Creating a custom write method to append strings
    pub fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    // Converting the output to a string
    pub fn to_string(&self) -> &str {
        &self.output
    }
}

// Implementing methods on Errortype to display the custom errors
impl Errortype {
    pub fn frmt(&self, formatter: &mut CustomFormatter) {
        match self {
            Errortype::InvalidInput => {
                formatter.write("\n\nInvalid Input! Please Enter a Valid Input\n\n");
            } // Errortype::InvalidChoice => {
              //     formatter.write("\n\nInvalid Input! Only 1, 2, 3 are valid inputs as of now!\n\n");
              // }
              // Errortype::EmptyInput => {
              //     formatter.write("\n\nNo Input Detected! Redirecting....\n\n");
              // }
              // Errortype::ZeroInput => {
              //     formatter
              //         .write("\n\nZero is NOT a valid input for this Mathematical Operation\n\n");
              // }

              // Handling custom error messages
              // Errortype::CustomError(msg) => {
              //     formatter.write(&format!("\n\n{}\n\n", msg));
              // }
        }
    }
}
