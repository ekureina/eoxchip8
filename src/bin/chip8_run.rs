use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    time::{Duration, Instant},
};

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
    #[arg(short, long, default_value_t = 700)]
    opcodes_per_second: u32,
}

fn main() {
    env_logger::init();

    let args = Chip8RunArgs::parse();

    let mut rom = File::open(args.program_path).unwrap();
    let mut program = vec![];
    rom.read_to_end(&mut program).unwrap();

    let mut executor = Executor::new(args.legacy_shift);
    executor.load_program(&program).unwrap();

    let cycle_time = Duration::from_secs(1) / args.opcodes_per_second;

    loop {
        let start = Instant::now();
        if let Err(error) = executor.execute_once() {
            error!("{error}");
        }
        let display = executor.get_display_mut();
        if display.has_changed() {
            println!("{}", display);
            display.render();
        }
        let run_elapsed = start.elapsed();
        std::thread::sleep(cycle_time - run_elapsed);
    }
}
