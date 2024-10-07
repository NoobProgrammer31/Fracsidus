use crate::design::design_patterns::{print_confirm_menu, print_welcome_message};
use crate::error_handling::errortype::{handle_error, Errortype};
use std::io;
// Function to take input
pub fn general_input(input: &mut String) -> String {
    io::stdin().read_line(input).expect("Failed To Read Line");
    input.to_string()
}

pub fn menu(remaining: &mut u32, final_choice: &mut u32, activate_menu: &mut bool) {
    print_welcome_message();
    println!(" \n-> Exiting After {remaining} wrong inputs ");
    // Defiining neccessary variables
    let possible_choices = [1, 2, 3];

    // Taking input in the variable choice through this loop
    // to handle invalid inputs explicitly
    // doesn't let the program crash
    // if valid input is found then it exits the loop
    // and then final_choice variable take that valid input

    let mut choice = String::new();
    loop {
        choice.clear();
        // taking input for the variable choice
        general_input(&mut choice);
        if choice.trim().is_empty() {
            handle_error(Errortype::EmptyInput);
            continue;
        }
        // converting the input into a u32 type
        let choice: u32 = match choice.trim().parse() {
            Ok(choice) => choice,
            // if input number is not a u32 type then we decrease the count of remaining attempts
            Err(_) => {
                handle_error(Errortype::InvalidInput);
                // clearing the input buffer for choice
                *remaining -= 1;
                println!(" \n-> Exiting After {remaining} wrong inputs ");
                // when attempts are exhausted , program quits
                if *remaining == 0 {
                    println!("You have exceeded the maximum attempts. Exiting.");
                    *activate_menu = false;
                    return;
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
            println!("choice is {choice}");
            handle_error(Errortype::InvalidChoice);
            *remaining -= 1;
            // if attempts are exhausted program exits
            if *remaining == 0 {
                *activate_menu = false;
                println!("You have exceeded the maximum attempts. Exiting.");
                return;
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
    let mut decision = String::new();
    loop {
        decision.clear();
        println!("\n -> Exiting After {remaining} Wrong Inputs");
        print_confirm_menu();
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
                    decision.clear();
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

// pub fn pause_and_confirm() -> String {
//     let possible_confirmation = [1, 2, 3];
//     let final_confirmation: u32;
//     let mut remaining = 2;
//     let mut confirmation = String::new();
//     loop {
//         confirmation.clear();
//         println!("\n -> Exiting After {remaining} Wrong Inputs");
//         print_confirm_menu();
//         general_input(&mut confirmation);
//         if confirmation.trim().is_empty() {
//             handle_error(Errortype::EmptyInput);
//             continue;
//         }
//         // converting the input into a u32 type
//         let confirmation: u32 = match confirmation.trim().parse() {
//             Ok(confirmation) => confirmation,
//             // if input number is not a u32 type then we decrease the count of remaining attempts
//             Err(_) => {
//                 handle_error(Errortype::InvalidInput);
//                 // clearing the input buffer for decision
//                 remaining -= 1;
//                 // when attempts are exhausted , program quits
//                 if remaining == 0 {
//                     confirmation.clear();
//                     println!(
//                         "\n\n| You have exceeded the maximum attempts. Exiting FRACSIDUS | \n\n"
//                     );
//                     // need to work on this condition on the loop that it will be used to break
//                     return "exit".to_string();
//                 }
//                 // if the input is valid then the program continues
//                 continue;
//             }
//         };
//         // checking whether the valid input is a possible decision or not
//         if possible_confirmation.contains(&confirmation) {
//             // if it is a possible decision then the loop breaks and goes out to next if statements
//             final_confirmation = confirmation;
//             break;
//         }
//         // if input is not a possible decision , following lines print an error message
//         // and also decrement the remaining count by 1
//         else {
//             handle_error(Errortype::InvalidChoice);
//             remaining -= 1;
//             // if attempts are exhausted program exits
//             if remaining == 0 {
//                 println!("\n\nYou have exceeded the maximum attempts. Exiting FRACSIDUS\n\n");
//                 return "exit".to_string();
//             }
//             // if there are attempts left loop continues
//             continue;
//         }
//     }
//     println!("final decision is : {final_confirmation}");
//     match final_confirmation {
//         1 => "continue".to_string(),
//         2 => "main_menu".to_string(),
//         3 => "exit".to_string(),
//         _ => "exit".to_string(),
//     }
// }
