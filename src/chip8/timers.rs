pub struct Timers {
    delay: u8,
    sound: u8,
}

impl Timers {
    pub fn new() -> Self {
        let delay = 0;
        let sound = 0;
        Self { delay, sound }
    }

    pub fn sound(&self) -> u8 {
        self.sound
    }

    pub fn delay(&self) -> u8 {
        self.delay
    }

    pub fn set_delay(&mut self, delay: u8) {
        self.delay = delay;
    }

    pub fn set_sound(&mut self, sound: u8) {
        self.sound = sound;
    }
}
