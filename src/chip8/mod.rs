use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    process::exit,
    thread,
    time::Duration,
};

use {
    rand::{
        prelude::ThreadRng,
        thread_rng,
        Rng,
    },
    sdl2::keyboard::Keycode,
};

use sdl2::{
    event::Event,
    pixels::Color,
    rect::Rect,
};

use self::{
    opcode::Opcode,
    stack::Stack,
    timers::Timers,
};

// use self::screen::Screen;
use super::chip8::screen::Screen;

mod font;
mod opcode;
mod screen;
mod stack;
mod tests;
mod timers;

pub struct Chip8 {
    /// Chip8 was commonly implemented in 4K system
    /// First 512 bytes occupies by machine
    /// ┌─────────────────────┐ <- 0xFFF End of Chip-8 RAM
    /// │                     │
    /// │                     │
    /// │                     │
    /// │                     │
    /// │                     │
    /// │                     │
    /// │                     │
    /// │                     │
    /// │                     │
    /// │                     │
    /// │                     │
    /// ├─────────────────────┤ <- 0x200 Start of Chip-8 program
    /// │                     │
    /// │                     │
    /// │                     │
    /// │                     │
    /// └─────────────────────┘ <- 0x000 Start of Chip-8 RAM
    memory: [u8; 1024 * 4],
    pc: usize,
    opcode: Opcode,
    stack: Stack,
    /// Chip8 have 16 8 bit registers named from 0x0 to 0xF
    /// 0xF is carry flag, while in substraction, it's the "no borrow flag" and
    /// set to 1 when drawing to detect pixel collision
    v: [u8; 16],
    screen: Screen,
    i: usize,
    rng: ThreadRng,
    timers: Timers,
    keys: HashSet<Keycode>,
    need_redraw: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0; 0x1000];
        let pc = 0x200;
        let opcode = Opcode::new();
        let stack = Stack::new();
        let v = [0; 16];
        let screen = Screen::new();
        let i = 0;
        let rng = thread_rng();
        let timers = Timers::new();
        let keys = HashSet::new();
        let need_redraw = false;

        font::load_font(&mut memory);

        Self {
            memory,
            pc,
            opcode,
            stack,
            v,
            screen,
            i,
            rng,
            timers,
            keys,
            need_redraw,
        }
    }
}

impl Chip8 {
    pub fn load_from_vec(&mut self, program: &Vec<u8>) {
        self.memory[0x200..].copy_from_slice(&program);
    }

    pub fn load_from_file(&mut self, program: &mut File) {
        let mem = &mut self.memory[0x200..];
        program.read(mem).unwrap();
    }

    pub fn run(&mut self) {
        let sdl_cxt = sdl2::init().unwrap();
        let sdl_video_ss = sdl_cxt.video().unwrap();

        let scale = 10;

        let sdl_window = sdl_video_ss
            .window("Chip-8 emulator", 64 * scale, 32 * scale)
            .position_centered()
            .build()
            .unwrap();

        let mut sdl_canvas = sdl_window.into_canvas().build().unwrap();

        let mut events = sdl_cxt.event_pump().unwrap();

        let bg_color = Color::RGB(0, 0, 0);
        let draw_color = Color::RGB(255, 255, 255);

        // let mut prev_keys = HashSet::new();

        while self.pc < self.memory.len() {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return,
                    _ => {}
                }
            }
            // let keys = events
            //     .keyboard_state()
            //     .pressed_scancodes()
            //     .filter_map(Keycode::from_scancode)
            //     .collect();

            // self.keys = &keys - &prev_keys;
            self.timers.countdown();

            // if self.keypad_waiting {
            //     continue;
            // }

            if self.need_redraw {
                sdl_canvas.set_draw_color(bg_color);
                sdl_canvas.clear();
                sdl_canvas.set_draw_color(draw_color);

                for py in 0..32 {
                    for px in 0..64 {
                        if self.screen.vram()[py][px] == 1 {
                            let x = ((px as u32) * scale) as i32;
                            let y = ((py as u32) * scale) as i32;
                            let w = (1 * scale) as u32;
                            let h = (1 * scale) as u32;
                            let rect = Rect::new(x, y, w, h);
                            sdl_canvas.draw_rect(rect).unwrap();
                            sdl_canvas.fill_rect(rect).unwrap();
                        }
                    }
                }

                sdl_canvas.present();

                thread::sleep(Duration::new(0, 1_000_000_000 / 60));
                self.need_redraw = false;
            }

            self.pc += 2;
            self.opcode
                .set_from_u8(self.memory[self.pc], self.memory[self.pc + 1]);

            // println!("Code: {:X}", self.opcode.code());
            match self.opcode.code() & 0xF000 {
                0x0000 => match self.opcode.code() & 0x00FF {
                    0x00E0 => self.cls_00e0(),
                    0x00EE => self.ret_00ee(),
                    _ => (),
                },
                0x1000 => self.jp_1nnn(),
                0x2000 => self.call_2nnn(),
                0x3000 => self.se_3xnn(),
                0x4000 => self.sne_4xnn(),
                0x5000 => self.se_5xy0(),
                0x6000 => self.ld_6xnn(),
                0x7000 => self.add_7xnn(),
                0x8000 => match self.opcode.code() & 0x000F {
                    0x0000 => self.ld_8xy0(),
                    0x0001 => self.or_8xy1(),
                    0x0002 => self.and_8xy2(),
                    0x0003 => self.xor_8xy3(),
                    0x0004 => self.add_8xy4(),
                    0x0005 => self.sub_8xy5(),
                    0x0006 => self.shr_8xy6(),
                    0x0007 => self.subn_8xy7(),
                    0x000E => self.shl_8x0e(),
                    _ => (),
                },
                0x9000 => self.sne_9xy0(),
                0xA000 => self.ld_annn(),
                0xB000 => self.jp_bnnn(),
                0xC000 => self.rnd_cxnn(),
                0xD000 => self.drw_dxyn(),
                0xE000 => match self.opcode.code() & 0x00FF {
                    0x009E => self.skp_ex9e(),
                    0x00A1 => self.sknp_exa1(),
                    _ => (),
                },
                0xF000 => match self.opcode.code() & 0x00FF {
                    0x0007 => self.ld_fx07(),
                    0x000A => self.ld_fx0a(),
                    0x0015 => self.ld_fx15(),
                    0x0018 => self.ld_fx18(),
                    0x001E => self.add_fx1e(),
                    0x0029 => self.ld_fx29(),
                    0x0033 => self.ld_fx33(),
                    0x0055 => self.ld_fx55(),
                    0x0065 => self.ld_fx65(),
                    _ => (),
                },
                _ => (),
            }
        }
    }
}

impl Chip8 {
    /// Clear the display
    fn cls_00e0(&mut self) {
        self.screen.clear();
        self.need_redraw = true;
    }
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of
    /// the stack, then subtracts 1 from the stack pointer.
    fn ret_00ee(&mut self) {
        log::debug!(
            "Return from subroutine. Current PC: {}, previous: {}",
            self.pc,
            self.stack.stack().last().unwrap()
        );
        self.pc = self.stack.ret() as usize;
        log::debug!("Returned from subrouting. Current PC: {}", self.pc);
    }
    /// Jump to location nnn.
    ///
    /// The interpreter sets the program counter to nnn.
    fn jp_1nnn(&mut self) {
        self.pc = self.opcode.nnn() - 2;
    }
    /// Call subroutine at nnn.
    ///
    /// The interpreter increments the stack pointer, then puts the current PC
    /// on the top of the stack. The PC is then set to nnn.
    fn call_2nnn(&mut self) {
        self.stack.push(self.pc);

        let nnn = self.opcode.nnn();
        self.pc = nnn - 2;
    }
    /// Skip next instruction if Vx = kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are equal,
    /// increments the program counter by 2.
    fn se_3xnn(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];
        let nn = self.opcode.nn();

        if vx == nn {
            self.pc += 2;
        };
    }
    /// Skip next instruction if Vx != kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are not equal,
    /// increments the program counter by 2.
    fn sne_4xnn(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];
        let nn = self.opcode.nn();

        if vx != nn {
            self.pc += 2;
        };
    }
    /// Skip next instruction if Vx = Vy.
    ///
    /// The interpreter compares register Vx to register Vy, and if they are
    /// equal, increments the program counter by 2.
    fn se_5xy0(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];
        let y = self.opcode.y();
        let vy = self.v[y];

        if vx == vy {
            self.pc += 2;
        };
    }
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    fn ld_6xnn(&mut self) {
        let x = self.opcode.x();
        let nn = self.opcode.nn();
        self.v[x] = nn;
    }
    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx, then stores the result in
    /// Vx.
    fn add_7xnn(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];
        let nn = self.opcode.nn();

        self.v[x] = vx.wrapping_add(nn);
    }
    /// Set Vx = Vy.
    ///
    /// Stores the value of register Vy in register Vx.
    fn ld_8xy0(&mut self) {
        let x = self.opcode.x();
        let y = self.opcode.y();
        let vy = self.v[y];

        self.v[x] = vy;
    }
    /// Set Vx = Vx OR Vy.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result
    /// in Vx. A bitwise OR compares the corrseponding bits from two values, and
    /// if either bit is 1, then the same bit in the result is also 1.
    /// Otherwise, it is 0.
    fn or_8xy1(&mut self) {
        let x = self.opcode.x();
        let y = self.opcode.y();
        let vy = self.v[y];

        self.v[x] |= vy;
    }
    /// Set Vx = Vx AND Vy.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise AND compares the corrseponding bits from two
    /// values, and if both bits are 1, then the same bit in the result is also
    /// 1. Otherwise, it is 0.
    fn and_8xy2(&mut self) {
        let x = self.opcode.x();
        let y = self.opcode.y();
        let vy = self.v[y];

        self.v[x] &= vy;
    }
    /// Set Vx = Vx XOR Vy.
    ///
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores
    /// the result in Vx. An exclusive OR compares the corrseponding bits from
    /// two values, and if the bits are not both the same, then the
    /// corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn xor_8xy3(&mut self) {
        let x = self.opcode.x();
        let y = self.opcode.y();
        let vy = self.v[y];

        self.v[x] ^= vy;
    }
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together. If the result is greater
    /// than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the
    /// lowest 8 bits of the result are kept, and stored in Vx.
    fn add_8xy4(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];
        let y = self.opcode.y();
        let vy = self.v[y];

        let result = (vx as u16) + (vy as u16);

        self.v[0xF] = if result > 255 { 1 } else { 0 };

        self.v[x] = result as u8;
    }
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from
    /// Vx, and the results stored in Vx.
    fn sub_8xy5(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];
        let y = self.opcode.y();
        let vy = self.v[y];

        self.v[0xF] = if vx > vy { 1 } else { 0 };

        self.v[x] = vx.wrapping_sub(vy);
    }
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise
    /// 0. Then Vx is divided by 2.
    fn shr_8xy6(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];

        self.v[0xF] = vx & 0x1;

        self.v[x] >>= 1;
    }
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from
    /// Vy, and the results stored in Vx.
    fn subn_8xy7(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];
        let y = self.opcode.y();
        let vy = self.v[y];

        self.v[0xF] = if vy > vx { 1 } else { 0 };

        self.v[x] = vy.wrapping_sub(vx);
    }
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise
    /// to 0. Then Vx is multiplied by 2.
    fn shl_8x0e(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];

        self.v[0xF] = (vx & 0x1) >> 7;

        self.v[x] <<= 1;
    }
    /// Skip next instruction if Vx != Vy.
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the
    /// program counter is increased by 2.
    fn sne_9xy0(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];
        let y = self.opcode.y();
        let vy = self.v[y];

        if vx != vy {
            self.pc += 2;
        }
    }
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    fn ld_annn(&mut self) {
        let nnn = self.opcode.nnn();

        self.i = nnn;
    }
    /// Jump to location nnn + V0.
    ///
    /// The program counter is set to nnn plus the value of V0.
    fn jp_bnnn(&mut self) {
        let nnn = self.opcode.nnn();
        let v0 = self.v[0] as usize;

        self.pc = (nnn - 2) + v0;
    }
    /// Set Vx = random byte AND kk.
    ///
    /// The interpreter generates a random number from 0 to 255, which is then
    /// ANDed with the value kk. The results are stored in Vx. See instruction
    /// 8xy2 for more information on AND.
    fn rnd_cxnn(&mut self) {
        let x = self.opcode.x();
        let nn = self.opcode.nn();
        let rn = self.rng.gen::<u8>();

        self.v[x] = rn & nn;
    }
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF
    /// = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address
    /// stored in I. These bytes are then displayed as sprites on screen at
    /// coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If
    /// this causes any pixels to be erased, VF is set to 1, otherwise it is set
    /// to 0. If the sprite is positioned so part of it is outside the
    /// coordinates of the display, it wraps around to the opposite side of the
    /// screen. See instruction 8xy3 for more information on XOR, and section
    /// 2.4, Display, for more information on the Chip-8 screen and sprites.
    fn drw_dxyn(&mut self) {
        let x = self.opcode.x();
        let y = self.opcode.y();
        let n = self.opcode.n() as usize;

        self.v[0xF] = 0; // reset if collisons were before
        for byte in 0..n {
            let y = (self.v[y] as usize + byte) % 32;
            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % 64;
                let color = (self.memory[self.i + byte] >> (7 - bit)) & 0x1;
                self.v[0x0F] |= color & self.screen.vram()[y][x];
                self.screen.set_xy(x, y, color);
            }
        }

        self.need_redraw = true;
    }

    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the down position, PC is increased by 2.
    fn skp_ex9e(&mut self) {}
    /// Skip next instruction if key with the value of Vx is not pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the up position, PC is increased by 2.
    fn sknp_exa1(&mut self) {}
    /// Set Vx = delay timer value.
    ///
    /// The value of DT is placed into Vx
    fn ld_fx07(&mut self) {
        let x = self.opcode.x();

        self.v[x] = self.timers.delay();
    }
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in Vx.
    fn ld_fx0a(&mut self) {
        log::error!("ld_fx0a is not implemented");
        exit(-1);
    }
    /// Set delay timer = Vx.
    ///
    /// DT is set equal to the value of Vx.
    fn ld_fx15(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];

        self.timers.set_delay(vx);
    }
    /// Set sound timer = Vx.
    ///
    /// ST is set equal to the value of Vx.
    fn ld_fx18(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];

        self.timers.set_sound(vx);
    }
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    fn add_fx1e(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x] as usize;

        self.i += vx;
    }
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite
    /// corresponding to the value of Vx. See section 2.4, Display, for more
    /// information on the Chip-8 hexadecimal font.
    fn ld_fx29(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x] as usize;

        self.i = vx * 5;
    }
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds
    /// digit in memory at location in I, the tens digit at location I+1, and
    /// the ones digit at location I+2.
    fn ld_fx33(&mut self) {
        let x = self.opcode.x();
        let vx = self.v[x];

        let i = self.i;
        self.memory[i + 0] = vx / 100;
        self.memory[i + 1] = (vx / 10) % 10;
        self.memory[i + 2] = vx % 10;
    }
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into
    /// memory, starting at the address in I.
    fn ld_fx55(&mut self) {
        let x = self.opcode.x();
        for i in 0..(x + 1) {
            self.memory[self.i + i] = self.v[i];
        }
    }
    /// Read registers V0 through Vx from memory starting at location I.
    ///
    /// The interpreter reads values from memory starting at location I into
    /// registers V0 through Vx.
    fn ld_fx65(&mut self) {
        let x = self.opcode.x();
        for i in 0..(x + 1) {
            self.v[i] = self.memory[self.i + i];
        }
    }
}
