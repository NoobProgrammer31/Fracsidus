// Comment update Date : 5 Oct 2024 : 10:40
// I am feeling too lazy to provide comments for other files and code as of now
// Connecting src/main.rs to other modules defined in src
// src/basic_functions , src/error_handling , src/utils
// bringing these modules into scope
mod basic_functions;
mod design;
mod error_handling;
mod utils;

use basic_functions::basic_maths::DualInputs;

// Bringing their functionalities into scope
use design::design_patterns::{
    print_confirm_menu, print_line, print_mode_message, print_welcome_message,
};
use error_handling::errortype::{handle_error, Errortype};
use utils::aggregator::{general_input, menu, pause_and_ask};

// Function to call actions
// will add it in future
// fn call_action() {}

// main function
fn main() {
    let mut attempts_for_mode = 3;
    let mut activate_mode_menu = true;

    'mode_loop: loop {
        if activate_mode_menu {
            let mut final_mode_choice = 0;
            print_mode_message();
            menu(
                &mut attempts_for_mode,
                &mut final_mode_choice,
                &mut activate_mode_menu,
            );

            match final_mode_choice {
                1 => {
                    let mut remaining = 5;

                    // This variable will control the execution of main menu
                    let mut activate_menu = true;

                    // loops the main menu
                    loop {
                        println!("hello....");
                        // used to handle the re execution of menu function
                        if activate_menu {
                            // to hold the choice of the user entered after menu display
                            let mut final_choice = 0;

                            // calling the menu function
                            // it is defined in src/utils/aggregator.rs
                            print_welcome_message();
                            menu(&mut remaining, &mut final_choice, &mut activate_menu);

                            // we get the choice entered by the user through the menu function
                            // and then we use that value with match to execute different features
                            match final_choice {
                                // if final choice was 1
                                1 => {
                                    // For Addition Of Fractions
                                    let mut numerator = String::new();
                                    let mut denominator = String::new();
                                    print_line();
                                    loop {
                                        println!(" -> Exiting After {remaining} wrong inputs ");

                                        // taking input and handling when the input is empty
                                        // will generalise it later during optimisation process
                                        // optimise later
                                        println!("Enter The Numerator");
                                        general_input(&mut numerator);
                                        // converting the input in an integer
                                        if numerator.trim().is_empty() {
                                            println!("\n### You Sure You Wanna Return To Main Menu ? ###\n");
                                            print_confirm_menu();
                                            let action = pause_and_ask();
                                            match action.as_str() {
                                                "exit" => {
                                                    numerator.clear();
                                                    return;
                                                }
                                                "continue" => {
                                                    numerator.clear();
                                                    continue;
                                                }
                                                "main_menu" => {
                                                    activate_menu = true;
                                                    break;
                                                }
                                                "mode_menu" => {
                                                    activate_mode_menu = true;
                                                    break;
                                                }
                                                _ => {
                                                    println!("Invalid action: {}", action)
                                                }
                                            }
                                        }
                                        let num: i32 = match numerator.trim().parse() {
                                            Ok(num) => {
                                                if num == 0 {
                                                    handle_error(Errortype::ZeroInput);
                                                }
                                                num
                                            }
                                            Err(_) => {
                                                numerator.clear();
                                                handle_error(Errortype::InvalidInput);
                                                remaining -= 1;
                                                // when attempts are exhausted , program quits
                                                if remaining == 0 {
                                                    activate_menu = false;
                                                    activate_mode_menu = false;
                                                    numerator.clear();
                                                    println!(
                            "\n\n| You have exceeded the maximum attempts. Exiting FRACSIDUS |\n\n");
                                                    break;
                                                }
                                                // if the input is valid then the program continues
                                                numerator.clear();
                                                continue;
                                            }
                                        };

                                        println!("Enter The Denominator : ");
                                        general_input(&mut denominator);
                                        if denominator.trim().is_empty() {
                                            println!("\n### You Sure You Wanna Return To Main Menu ? ###\n");
                                            print_confirm_menu();
                                            let action = pause_and_ask();
                                            match action.as_str() {
                                                "exit" => {
                                                    denominator.clear();
                                                    numerator.clear();
                                                    return;
                                                }
                                                "continue" => {
                                                    denominator.clear();
                                                    numerator.clear();
                                                    continue;
                                                }
                                                "main_menu" => {
                                                    denominator.clear();
                                                    numerator.clear();
                                                    activate_menu = true;
                                                    break;
                                                }
                                                "mode_menu" => {
                                                    denominator.clear();
                                                    numerator.clear();
                                                    activate_mode_menu = true;
                                                    break;
                                                }
                                                _ => {
                                                    println!("Invalid action: {}", action)
                                                }
                                            }
                                        }
                                        // handling error prone scenarios and converting it into integer
                                        let deno: i32 = match denominator.trim().parse() {
                                            Ok(num) => {
                                                if num == 0 {
                                                    handle_error(Errortype::ZeroInput);
                                                    continue;
                                                }
                                                num
                                            }
                                            Err(_) => {
                                                denominator.clear();
                                                // please read this
                                                // so I encountered a bug in my code which was
                                                // after you enter the valid input for numerator
                                                // and invalid input for factor_to_check
                                                // then input buffer for numerator was already filled
                                                // so code was taking me to the start of the loop
                                                // but even with the right input for numerator it was saying invalid input
                                                // I resolved it by clearing input buffer for numerator here also
                                                // IMPORTANT : Please always clear your nput buffers
                                                numerator.clear();
                                                handle_error(Errortype::InvalidInput);
                                                remaining -= 1;
                                                // when attempts are exhausted , program quits
                                                if remaining == 0 {
                                                    activate_menu = false;
                                                    activate_mode_menu = false;
                                                    println!(
                                " \n\n | You have exceeded the maximum attempts. Exiting | \n\n"
                            );
                                                    break;
                                                }
                                                // if the input is valid then the program continues
                                                numerator.clear();
                                                continue;
                                            }
                                        };
                                        let inputs = DualInputs {
                                            first_num: num,
                                            second_num: deno,
                                        };
                                        // checks to ensure that factor isn't greater than the numerator in the input
                                        if inputs.first_num < inputs.second_num {
                                            println!("__ Factor input is larger than numerator input ! Error ! ___")
                                        } else {
                                            inputs.check_fac();
                                            print_confirm_menu();
                                            let action = pause_and_ask();
                                            match action.as_str() {
                                                "exit" => {
                                                    numerator.clear();
                                                    denominator.clear();
                                                    return;
                                                }
                                                "continue" => {
                                                    numerator.clear();
                                                    denominator.clear();
                                                    continue;
                                                }
                                                "main_menu" => {
                                                    activate_menu = true;
                                                    break;
                                                }
                                                "mode_menu" => {
                                                    activate_mode_menu = true;
                                                    break;
                                                }
                                                _ => {
                                                    println!("Invalid action: {}", action)
                                                }
                                            }
                                        }

                                        // Clear input buffers for the next iteration
                                        numerator.clear();
                                        denominator.clear();
                                    }
                                }
                                2 => {
                                    let mut num_to_check_for_prime = String::new();
                                    println!(" \n -- (Press Enter) To Interrupt --\n ");
                                    // looping to handle errors and take the right input
                                    loop {
                                        println!(
                                            "\nProgram will quit after {remaining} wrong inputs\n"
                                        );

                                        // taking input for numerator
                                        println!(
                                            "Enter The Number To Check Whether Its Prime or Not :"
                                        );
                                        general_input(&mut num_to_check_for_prime);

                                        if num_to_check_for_prime.trim().is_empty() {
                                            println!("\n### You Sure You Wanna Return To Main Menu ? ###\n");
                                            print_confirm_menu();
                                            let action = pause_and_ask();
                                            match action.as_str() {
                                                "exit" => {
                                                    num_to_check_for_prime.clear();
                                                    return;
                                                }
                                                "continue" => {
                                                    num_to_check_for_prime.clear();
                                                    continue;
                                                }
                                                "main_menu" => {
                                                    activate_menu = true;
                                                    break;
                                                }
                                                "mode_menu" => {
                                                    activate_mode_menu = true;
                                                    break;
                                                }
                                                _ => {
                                                    println!("Invalid action: {}", action)
                                                }
                                            }
                                        }
                                        // converting the input in an integer
                                        let num: i32 = match num_to_check_for_prime.trim().parse() {
                                            Ok(num) => {
                                                if num == 0 {
                                                    handle_error(Errortype::ZeroInput);
                                                }
                                                num
                                            }
                                            Err(_) => {
                                                println!(" \nInvalid Input \n");
                                                num_to_check_for_prime.clear();
                                                remaining -= 1;
                                                // when attempts are exhausted , program quits
                                                if remaining == 0 {
                                                    activate_menu = false;
                                                    activate_mode_menu = false;
                                                    println!("You have exceeded the maximum attempts. Exiting.");
                                                    break;
                                                }
                                                // if the input is valid then the program continues
                                                continue;
                                            }
                                        };
                                        let inputs = DualInputs {
                                            first_num: num,
                                            second_num: 0,
                                        };
                                        if inputs.first_num == inputs.second_num {
                                            println!("\nYou are too dumb to Enter ZERO !!!\n");
                                        } else {
                                            inputs.check_prime();
                                            print_confirm_menu();
                                            let action = pause_and_ask();
                                            match action.as_str() {
                                                "exit" => {
                                                    num_to_check_for_prime.clear();
                                                    return;
                                                }
                                                "continue" => {
                                                    num_to_check_for_prime.clear();
                                                    continue;
                                                }
                                                "main_menu" => {
                                                    activate_menu = true;
                                                    break;
                                                }
                                                "mode_menu" => {
                                                    activate_mode_menu = true;
                                                    break;
                                                }
                                                _ => {
                                                    println!("Invalid action: {}", action)
                                                }
                                            }
                                        }

                                        // Clear input buffers for the next iteration
                                        num_to_check_for_prime.clear();
                                    }
                                }
                                3 => {
                                    println!("Exiting Program from main menu.......");
                                    break 'mode_loop;
                                }
                                _ => {
                                    println!("Invalid Choice ! ");
                                    continue;
                                }
                            };
                        } else {
                            break;
                        }
                    }
                }

                2 => {
                    println!("Advanced Features Will Be Added Later , Stay Tuned !!! ");
                    final_mode_choice.to_string().clear();
                    break 'mode_loop;
                }
                3 => {
                    println!("Exiting.........");
                    break 'mode_loop;
                }
                _ => {
                    println!("Invalid Choice");
                    continue;
                }
            }
        } else {
            break;
        }
    }
}
