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
}
