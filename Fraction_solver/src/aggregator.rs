pub struct Inputs {
    pub num: u32,
    pub fac: u32,
}

impl Inputs {
    pub fn check_fac(&self) {
        if self.num % self.fac == 0 {
            println!("\n {} is definitely a factor of {} ", self.fac, self.num);
        } else {
            println!("\n {} is not a factor of {}", self.fac, self.num);
        }
    }

    // Following function checks whether a number is prime or not
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
