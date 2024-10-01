use std::io;
mod aggregator;
use aggregator::Inputs;

fn main() {
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
}
