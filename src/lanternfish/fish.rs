pub struct Lanternfish {
    timer: u8,
}

impl Lanternfish {
    /// Creates a new lanternfish with internal timer set to `timer`
    pub fn new(timer: u8) -> Lanternfish {
        Lanternfish { timer }
    }

    /// Makes the lanternfish grow older by one day and may spawn a new
    /// Lanternfish.
    pub fn grow_up(&mut self) -> Option<Lanternfish> {
        if self.timer > 0 {
            self.timer -= 1;
            None
        } else {
            self.timer = 6;
            Some(Lanternfish::new(8))
        }
    }
}
