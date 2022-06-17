pub struct Opcode {
    lr: u16,
}

impl Opcode {
    pub fn new() -> Self {
        let lr = 0x0000;

        Self { lr }
    }

    pub fn code(&self) -> u16 {
        self.lr
    }

    pub fn set_from_u8(&mut self, l: u8, r: u8) {
        let l = l as u16;
        let r = r as u16;

        self.lr = (l << 8) | r;
    }

    pub fn set_from_u16(&mut self, lr: u16) {
        self.lr = lr;
    }
}

impl Opcode {
    pub fn nnn(&mut self) -> usize {
        (self.lr & 0x0FFF) as usize
    }

    pub fn nn(&mut self) -> u8 {
        (self.lr & 0x00FF) as u8
    }

    pub fn n(&mut self) -> u8 {
        (self.lr & 0x000F) as u8
    }

    pub fn x(&mut self) -> usize {
        ((self.lr & 0x0F00) >> 8) as usize
    }

    pub fn y(&mut self) -> usize {
        ((self.lr & 0x00F0) >> 4) as usize
    }
}
