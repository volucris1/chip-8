use sdl2::keyboard::Keycode;

pub fn hex_to_key(h: u8) -> Option<Keycode> {
    log::debug!("Hex {h} to key");
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

pub fn key_to_hex(key: Keycode) -> Option<u8> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),

        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),

        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),

        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
