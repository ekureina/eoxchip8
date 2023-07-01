use std::{fs::File, io::Read, path::PathBuf, time::Duration};

use clap::Parser;
use rc8::core::cpu::main::Executor;

#[derive(Debug, Parser, PartialEq, Eq, PartialOrd, Ord)]
struct Chip8RunArgs {
    #[arg(short, long)]
    program_path: PathBuf,
}

fn main() {
    let args = Chip8RunArgs::parse();
    let mut rom = File::open(args.program_path).unwrap();
    let mut program = vec![];
    rom.read_to_end(&mut program).unwrap();
    let mut executor = Executor::new();
    executor.load_program(&program).unwrap();
    loop {
        executor.execute_once().unwrap();
        println!("{}", executor.get_display());
        std::thread::sleep(Duration::from_secs(1));
    }
}
