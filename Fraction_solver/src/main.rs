// Connecting this main file to the custom library that handles logical stuff
mod basic_functions;
mod error_handling;
mod utils;
// importing that custom library, and the struct defined in it that is INPUT
// use crate::basic_functions::basic_maths;
use crate::error_handling::errortype::{CustomFormatter, Errortype};
use crate::utils::aggregator::{general_input, menu, pause_and_ask, Inputs};
fn main() {
    let mut remaining = 5;
    let mut final_choice = 0;
    menu(&mut remaining, &mut final_choice);
    // when input is valid and is a possible choice then following lines are executed

    match final_choice {
        1 => {
            let mut numerator = String::new();
            let mut factor_to_check = String::new();

            // looping to handle errors and take the right input
            loop {
                println!("\n -- Program will Quit after {remaining} wrong inputs -- \n");
                println!(" \n Press ctrl+z to force quit !\n ");

                // taking input for numerator
                println!("Enter The Numerator :");
                general_input(&mut numerator);
                // converting the input in an integer
                if numerator.trim().is_empty() {
                    println!("Empty Input Detected ! Redirecting.....");
                    continue;
                }
                let num: u32 = match numerator.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        numerator.clear();
                        let mut formatter = CustomFormatter::new();
                        Errortype::InvalidInput.frmt(&mut formatter);
                        println!("{}", formatter.to_string());
                        remaining -= 1;
                        // when attempts are exhausted , program quits
                        if remaining == 0 {
                            println!(
                            "\n\n| You have exceeded the maximum attempts. Exiting FRACSIDUS |\n\n"
                        );
                            break;
                        }
                        // if the input is valid then the program continues
                        continue;
                    }
                };

                // handling error prone scenarios and converting it into integer
                println!("Enter The Factor To Check : ");
                general_input(&mut factor_to_check);
                if factor_to_check.trim().is_empty() {
                    println!("Empty Input Detected ! Redirecting.....");
                    continue;
                }
                // handling error prone scenarios and converting it into integer
                let fac: u32 = match factor_to_check.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        factor_to_check.clear();
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
                        println!(" \n---Invalid Input--- \n");
                        remaining -= 1;
                        // when attempts are exhausted , program quits
                        if remaining == 0 {
                            println!(
                                " \n\n | You have exceeded the maximum attempts. Exiting | \n\n"
                            );
                            break;
                        }
                        // if the input is valid then the program continues
                        continue;
                    }
                };

                let inputs = Inputs { num, fac };
                // checks to ensure that factor isn't greater than the numerator in the input
                if inputs.num < inputs.fac {
                    println!("__ Factor input is larger than numerator input ! Error ! ___")
                }
                // handling zero as a factor
                else if inputs.fac == 0 {
                    println!("\n -- Any number divided by ZERO is UNDEFINED \n")
                } else {
                    inputs.check_fac();
                    let action = pause_and_ask();
                    match action.as_str() {
                        "exit" => {
                            numerator.clear();
                            factor_to_check.clear();
                            final_choice.to_string().clear();
                            return;
                        }
                        "continue" => {
                            numerator.clear();
                            factor_to_check.clear();
                            final_choice.to_string().clear();
                            continue;
                        }
                        "main_menu" => {
                            // MENU RECALLING WILL BE ADDED LAtER
                            numerator.clear();
                            factor_to_check.clear();
                            final_choice.to_string().clear();
                            break;
                        }
                        _ => {
                            println!("Invalid action: {}", action)
                        }
                    }
                }

                // Clear input buffers for the next iteration
                numerator.clear();
                factor_to_check.clear();
            }
        }
        2 => {
            let mut num_to_check_for_prime = String::new();
            // looping to handle errors and take the right input
            loop {
                println!("\nProgram will quit after {remaining} wrong inputs\n");
                println!(" \n Press ctrl+z to quit !\n ");

                // taking input for numerator
                println!("Enter The Number To Check Whether Its Prime or Not :");
                general_input(&mut num_to_check_for_prime);

                // converting the input in an integer
                let num: u32 = match num_to_check_for_prime.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!(" \nInvalid Input \n");
                        num_to_check_for_prime.clear();
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
                let inputs = Inputs { num, fac: 0 };
                if inputs.num == inputs.fac {
                    println!("\nYou are too dumb to Enter ZERO !!!\n");
                } else {
                    inputs.check_prime();
                    let action = pause_and_ask();
                    match action.as_str() {
                        "exit" => {
                            num_to_check_for_prime.clear();
                            final_choice.to_string().clear();
                            return;
                        }
                        "continue" => {
                            num_to_check_for_prime.clear();
                            final_choice.to_string().clear();
                            continue;
                        }
                        "main_menu" => {
                            // MENU RECALLING WILL BE ADDED LAtER
                            num_to_check_for_prime.clear();
                            final_choice.to_string().clear();
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
            final_choice.to_string().clear();
            println!("Exiting Program.......")
        }
        _ => {
            println!("Invalid Choice ! ")
        }
    };
}
