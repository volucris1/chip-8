use std::{
    fs::File,
    path::Path,
};

use chip8::Chip8;
mod chip8;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    program: String,
}

fn main() {
    env_logger::init();

    let mut chip8 = Chip8::new();

    // let path_to_roms = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/ROMs"));
    // let path_to_program = path_to_roms.join("TETRIS");
    let args = Args::parse();
    let path_to_program = Path::new(&args.program);
    let mut file = File::open(path_to_program).unwrap();
    chip8.load_from_file(&mut file);

    chip8.run();
}
