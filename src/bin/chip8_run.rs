use std::{fs::File, io::Read, path::PathBuf, time::Duration};

use clap::Parser;
use log::error;
use rc8::core::cpu::main::Executor;

#[derive(Debug, Parser, PartialEq, Eq, PartialOrd, Ord)]
#[command(author, version, about)]
struct Chip8RunArgs {
    #[arg(short, long)]
    program_path: PathBuf,
    // Use the original Chip-8 shift with Vx = Vy
    #[arg(short, long)]
    legacy_shift: bool,
}

fn main() {
    env_logger::init();

    let args = Chip8RunArgs::parse();
    let mut rom = File::open(args.program_path).unwrap();
    let mut program = vec![];
    rom.read_to_end(&mut program).unwrap();
    let mut executor = Executor::new(args.legacy_shift);
    executor.load_program(&program).unwrap();
    loop {
        if let Err(error) = executor.execute_once() {
            error!("{error}");
        }
        let display = executor.get_display_mut();
        if display.has_changed() {
            println!("{}", display);
            display.render();
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
