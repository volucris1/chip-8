pub struct Screen {
    vram: [[u8; 64]; 32],
}

impl Screen {
    pub fn new() -> Self {
        let vram = [[0; 64]; 32];

        Self { vram }
    }
    pub fn clear(&mut self) {
        self.vram = [[0; 64]; 32];
    }

    pub fn set_vram(&mut self, vram: [[u8; 64]; 32]) {
        self.vram = vram;
    }

    pub fn vram(&self) -> [[u8; 64]; 32] {
        self.vram
    }

    pub fn set_xy(&mut self, x: usize, y: usize, v: u8) {
        self.vram[y][x] ^= v;
    }

    pub fn vram_mut(&mut self) -> &mut [[u8; 64]; 32] {
        &mut self.vram
    }
}
