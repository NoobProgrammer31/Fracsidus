use std::io;

// Function to take input
pub fn general_input(input: &mut String) -> String {
    io::stdin().read_line(input).expect("Failed To Read Line");
    input.to_string()
}

pub fn menu(remaining: &mut u32, final_choice: &mut u32) {
    println!("\n--- Fracsidus Welcomes YOU ---\n");
    println!("\n Enter Your Choice \n 1 -> Check Whether a Number is a factor or not \n 2 -> Check whether a Number is Prime or Not \n 3 -> EXIT ! \n ");
    // Defiining neccessary variables
    let possible_choices = [1, 2, 3];

    // Taking input in the variable choice through this loop
    // to handle invalid inputs explicitly
    // doesn't let the program crash
    // if valid input is found then it exits the loop
    // and then final_choice variable take that valid input

    loop {
        let mut choice = String::new();
        println!("\n--FRACSIDUS will exit after {remaining} wrong attempts --\n");
        println!("\n Please Input Your Choice \n");

        // taking input for the variable choice
        general_input(&mut choice);
        // converting the input into a u32 type
        let choice: u32 = match choice.trim().parse() {
            Ok(choice) => choice,
            // if input number is not a u32 type then we decrease the count of remaining attempts
            Err(_) => {
                println!("Invalid Input! Only 1 or 2 are valid inputs as of now!");
                // clearing the input buffer for choice
                choice.clear();
                *remaining -= 1;
                // when attempts are exhausted , program quits
                if *remaining == 0 {
                    println!("You have exceeded the maximum attempts. Exiting.");
                    break;
                }
                // if the input is valid then the program continues
                continue;
            }
        };
        // checking whether the valid input is a possible choice or not
        if possible_choices.contains(&choice) {
            // if it is a possible choice then the loop breaks and goes out to next if statements
            *final_choice = choice;
            break;
        }
        // if input is not a possible choice , following lines print an error message
        // and also decrement the remaining count by 1
        else {
            println!("Invalid Choice! Please select either 1 or 2.");
            *remaining -= 1;
            // if attempts are exhausted program exits
            if *remaining == 0 {
                println!("You have exceeded the maximum attempts. Exiting.");
                break;
            }
            // if there are attempts left loop continues
            continue;
        }
    }
}

// Function to return to Main Menu Or Exit the program or continue
pub fn pause_and_ask() -> String {
    let possible_decision = [1, 2, 3];
    let final_decision: u32;
    let mut remaining = 5;
    loop {
        let mut decision = String::new();
        println!("| Fracsidus will exit after {remaining} wrong attempts |");
        println!("\n Please Input Your Choice \n");

        println!("\n Continue -> 1 \n Return To Main Menu -> 2 \n Exit -> 3 ");
        // taking input for the variable decision
        println!("\n--Take a Decision--\n");
        general_input(&mut decision);
        // converting the input into a u32 type
        let decision: u32 = match decision.trim().parse() {
            Ok(decision) => decision,
            // if input number is not a u32 type then we decrease the count of remaining attempts
            Err(_) => {
                println!("\n\nInvalid Input! Only 1, 2, 3 are valid inputs as of now!\n\n");
                // clearing the input buffer for decision
                remaining -= 1;
                // when attempts are exhausted , program quits
                if remaining == 0 {
                    println!(
                        "\n\n| You have exceeded the maximum attempts. Exiting FRACSIDUS | \n\n"
                    );
                    // need to work on this condition on the loop that it will be used to break
                    return "exit".to_string();
                }
                // if the input is valid then the program continues
                continue;
            }
        };
        // checking whether the valid input is a possible decision or not
        if possible_decision.contains(&decision) {
            // if it is a possible decision then the loop breaks and goes out to next if statements
            final_decision = decision;
            break;
        }
        // if input is not a possible decision , following lines print an error message
        // and also decrement the remaining count by 1
        else {
            println!("\n :( Invalid Choice! Please select either 1, 2, 3 \n");
            remaining -= 1;
            // if attempts are exhausted program exits
            if remaining == 0 {
                println!("\n\nYou have exceeded the maximum attempts. Exiting FRACSIDUS\n\n");
                return "exit".to_string();
            }
            // if there are attempts left loop continues
            continue;
        }
    }
    println!("final decision is : {final_decision}");
    match final_decision {
        1 => "continue".to_string(),
        2 => "main_menu".to_string(),
        3 => "exit".to_string(),
        _ => "exit".to_string(),
    }
}

// struct to modify later
pub struct Inputs {
    pub num: u32,
    pub fac: u32,
}

// implementing method on struct Input
impl Inputs {
    // Feature 1 : Checks whether a number is a factor of another number or not
    pub fn check_fac(&self) {
        if self.num % self.fac == 0 {
            println!("\n {} is definitely a factor of {} ", self.fac, self.num);
        } else {
            println!("\n {} is not a factor of {}", self.fac, self.num);
        }
    }

    // Feature 2 : Checks whether a number is prime or not
    pub fn check_prime(&self) {
        if self.num < 2 {
            println!("Not a Prime !");
            return;
        }

        let limit = (self.num as f64).sqrt() as u32;

        for i in 2..=limit {
            if self.num % i == 0 {
                println!("\n {} is NOT a PRIME Number ! \n", self.num);
                return;
            }
        }
        println!("\n {} is a PRIME NUMBER! \n", self.num);
    }
}
