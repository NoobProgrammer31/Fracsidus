use crate::error_handling::errortype::{handle_error, Errortype};
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
        println!("\n-- FRACSIDUS will exit after {remaining} wrong attempts --\n");
        println!("\n Please Input Your Choice \n");

        // taking input for the variable choice
        general_input(&mut choice);
        if choice.trim().is_empty() {
            handle_error(Errortype::EmptyInput);
            continue;
        }
        // converting the input into a u32 type
        let choice: u32 = match choice.trim().parse() {
            Ok(choice) => {
                if choice == 0 {
                    handle_error(Errortype::ZeroInput);
                }
                choice
            }
            // if input number is not a u32 type then we decrease the count of remaining attempts
            Err(_) => {
                handle_error(Errortype::InvalidInput);
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
            handle_error(Errortype::InvalidChoice);
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
        println!("\n | Fracsidus will exit after {remaining} wrong attempts |");
        println!("\n Please Input Your Choice \n");

        println!("\n Continue -> 1 \n Return To Main Menu -> 2 \n Exit -> 3 ");
        // taking input for the variable decision
        println!("\n--Take a Decision--\n");
        general_input(&mut decision);
        if decision.trim().is_empty() {
            handle_error(Errortype::EmptyInput);
            continue;
        }
        // converting the input into a u32 type
        let decision: u32 = match decision.trim().parse() {
            Ok(decision) => {
                if decision == 0 {
                    handle_error(Errortype::ZeroInput);
                }
                decision
            }
            // if input number is not a u32 type then we decrease the count of remaining attempts
            Err(_) => {
                handle_error(Errortype::InvalidInput);
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
            handle_error(Errortype::InvalidChoice);
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
