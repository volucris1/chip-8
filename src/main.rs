use std::{path::Path, fs::File};

use chip8::Chip8;

mod chip8;

fn main() {
    let mut chip8 = Chip8::new();

    let path_to_roms = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/ROMs"));
    let path_to_program = path_to_roms.join("Space Invaders [David Winter] (alt).ch8");
    let mut file = File::open(path_to_program).unwrap();
    chip8.load_from_file(&mut file);

    chip8.run();
}
