// struct to store the results of all checks
// pub struct Results {
//     pub prime_check: bool,
//     pub co_prime_check: bool,
//     pub factor_check: bool,
// }

// Creating a fraction Simplifier
pub struct DualInputs {
    pub first_num: i32,
    pub second_num: i32,
}

impl DualInputs {
    // Factor Check
    pub fn check_fac(&self) {
        if self.first_num % self.second_num == 0 {
            println!(
                "
                _______________________________
                {} is definitely a factor of {} ",
                self.second_num, self.first_num
            );
        } else {
            println!(
                "
                _________________________
                {} is not a factor of {}",
                self.second_num, self.first_num
            );
        }
    }

    // Find Prime Factors
    // pub fn find_factors(&self) {}

    // Prime Check
    pub fn check_prime(&self) {
        if self.first_num < 2 {
            println!(
                "
                _____________
{} is Not a PRIME !",
                self.first_num,
            );
            return;
        }

        let limit = (self.first_num as f64).sqrt() as i32;

        for i in 2..=limit {
            if self.first_num % i == 0 {
                println!(
                    "
                ___________________________
                {} is a COMPOSITE Number ! \n",
                    self.first_num
                );
                return;
            }
        }
        println!(
            "
            _________________________
            {} is a PRIME NUMBER! \n",
            self.first_num
        );
    }

    // Twin Prime Check
    // need to run prime_check() function before calling this program
    // fn check_twin_prime(&self) -> bool {
    //     let difference = self.first_num - self.second_num;
    //     if difference == 2 {
    //         return true;
    //     }
    //     false
    // }
    //
    // fn check_co_prime(&self) {}
}
