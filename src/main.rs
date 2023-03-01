
use std::env;

mod emulator;


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No chip8 iso file given");
        std::process::exit(1);
    }
    let path = args[1].as_str();
    emulator::Emulator::new().load(path).run();
}