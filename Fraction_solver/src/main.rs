// importing io crate for input functionality
use std::io;
// Connecting this main file to the custom library that handles logical stuff
mod aggregator;
// importing that custom library, and the struct defined in it that is INPUT
use aggregator::Inputs;

fn main() {
    println!("\n--- Fracsidus Welcomes YOU ---\n");
    println!("\n Enter Your Choice \n 1 -> Check Whether a Number is a factor or not \n 2 -> Check whether a Number is Prime or Not \n ");
    // Defiining neccessary variables
    let possible_choices = [1, 2];
    let mut choice = String::new();
    let mut final_choice = 0;
    let mut remaining = 5;

    // Taking input in the variable choice through this loop
    // to handle invalid inputs explicitly
    // doesn't let the program crash
    // if valid input is found then it exits the loop
    // and then final_choice variable take that valid input
    loop {
        println!("remaining attempts = {remaining}");
        println!("\n Please Input Your Choice \n");

        // taking input for the variable choice
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed To Read Line");

        // converting the input into a u32 type
        let choice: u32 = match choice.trim().parse() {
            Ok(choice) => choice,
            // if input number is not a u32 type then we decrease the count of remaining attempts
            Err(_) => {
                println!("Invalid Input! Only 1 or 2 are valid inputs as of now!");
                // clearing the input buffer for choice
                choice.clear();
                remaining -= 1;
                // when attempts are exhausted , program quits
                if remaining == 0 {
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
            final_choice = choice;
            break;
        }
        // if input is not a possible choice , following lines print an error message
        // and also decrement the remaining count by 1
        else {
            println!("Invalid Choice! Please select either 1 or 2.");
            remaining -= 1;
            // if attempts are exhausted program exits
            if remaining == 0 {
                println!("You have exceeded the maximum attempts. Exiting.");
                break;
            }
            // if there are attempts left loop continues
            continue;
        }
    }

    // when input is valid and is a possible choice then following lines are executed
    if final_choice == 1 {
        let mut numerator = String::new();
        let mut factor_to_check = String::new();

        // looping to handle errors and take the right input
        loop {
            println!(" \n Press ctrl+z to quit !\n ");

            // taking input for numerator
            println!("Enter The Numerator :");
            io::stdin()
                .read_line(&mut numerator)
                .expect("Failed To Read Line");

            // converting the input in an integer
            let num: u32 = match numerator.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!(" \nInvalid Input \n");
                    continue;
                }
            };

            // handling error prone scenarios and converting it into integer
            println!("Enter The Factor To Check ; ");
            io::stdin()
                .read_line(&mut factor_to_check)
                .expect("Failed To Read Line");

            // handling error prone scenarios and converting it into integer
            let fac: u32 = match factor_to_check.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!(" \nInvalid Input \n");
                    continue;
                }
            };

            let inputs = Inputs { num, fac };
            // checks to ensure that factor isn't greater than the numerator in the input
            if inputs.num < inputs.fac {
                println!("Factor input is larger than numerator input ! Error !!!")
            } else {
                inputs.check_fac()
            }

            // Clear input buffers for the next iteration
            numerator.clear();
            factor_to_check.clear();
        }
    } else if final_choice == 2 {
        // looping to handle errors and take the right input
        loop {
            let mut num_to_check_for_prime = String::new();
            println!(" \n Press ctrl+z to quit !\n ");

            // taking input for numerator
            println!("Enter The Number To Check Whether Its Prime or Not :");
            io::stdin()
                .read_line(&mut num_to_check_for_prime)
                .expect("Failed To Read Line");

            // converting the input in an integer
            let num: u32 = match num_to_check_for_prime.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!(" \nInvalid Input \n");
                    continue;
                }
            };
            let inputs = Inputs { num, fac: 0 };
            if inputs.num == inputs.fac {
                println!("You are too dumb to Enter ZERO !!!");
            } else {
                inputs.check_prime()
            }

            // Clear input buffers for the next iteration
            num_to_check_for_prime.clear();
        }
    }
}
