use sdl2::keyboard::Keycode;

pub fn hex_to_key(h: u8) -> Option<Keycode> {
    match h {
        0x1 => Some(Keycode::Num1),
        0x2 => Some(Keycode::Num2),
        0x3 => Some(Keycode::Num3),
        0xC => Some(Keycode::Num4),

        0x4 => Some(Keycode::Q),
        0x5 => Some(Keycode::W),
        0x6 => Some(Keycode::E),
        0xD => Some(Keycode::R),

        0x7 => Some(Keycode::A),
        0x8 => Some(Keycode::S),
        0x9 => Some(Keycode::D),
        0xE => Some(Keycode::F),

        0xA => Some(Keycode::Z),
        0x0 => Some(Keycode::X),
        0xB => Some(Keycode::C),
        0xF => Some(Keycode::V),
        _ => None,
    }
}
