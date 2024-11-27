pub const TIMER_FREQ: u8 = 60;

pub struct Timer {
    counter: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { counter: 0 }
    }

    pub fn tick(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        }
    }

    pub fn set(&mut self, value: u8) {
        self.counter = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick() {
        let mut timer = Timer::new();
        timer.counter += 3;

        timer.tick();
        assert_eq!(timer.counter, 2);
        timer.tick();
        assert_eq!(timer.counter, 1);
        timer.tick();
        assert_eq!(timer.counter, 0);
        timer.tick();
        assert_eq!(timer.counter, 0);
    }

    #[test]
    fn set() {
        let data = 9;
        let mut timer = Timer::new();

        timer.set(data);
        assert_eq!(timer.counter, data);
    }
}
