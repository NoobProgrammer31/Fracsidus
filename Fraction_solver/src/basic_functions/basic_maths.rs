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
            println!(
                "
                _______________________________
                {} is definitely a factor of {} ",
                self.fac, self.num
            );
        } else {
            println!(
                "
                _________________________
                {} is not a factor of {}",
                self.fac, self.num
            );
        }
    }

    // Feature 2 : Checks whether a number is prime or not
    pub fn check_prime(&self) {
        if self.num < 2 {
            println!(
                "
                _____________
                 Not a Prime !"
            );
            return;
        }

        let limit = (self.num as f64).sqrt() as u32;

        for i in 2..=limit {
            if self.num % i == 0 {
                println!(
                    "
                ___________________________
                {} is NOT a PRIME Number ! \n",
                    self.num
                );
                return;
            }
        }
        println!(
            "
            _________________________
            {} is a PRIME NUMBER! \n",
            self.num
        );
    }
}
