use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use clap::Parser;
use eoxchip8::core::cpu::main::Executor;
use iced::{time, Font, Length};
use iced::{Element, Task, Theme};
use log::{error, info};

pub fn main() -> iced::Result {
    env_logger::init();

    let args = Chip8RunArgs::parse();

    iced::application("EoxChip8", EoxChip8GUI::update, EoxChip8GUI::view)
        .subscription(EoxChip8GUI::subscription)
        .theme(EoxChip8GUI::theme)
        .default_font(Font::MONOSPACE)
        .run_with(|| EoxChip8GUI::new(args))
}

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EoxChip8GUI {
    executor: RefCell<Executor>,
    cycle_time: Duration,
}

#[derive(Debug, Clone)]
enum EoxMessage {
    Tick(Instant),
}

impl EoxChip8GUI {
    fn new(flags: Chip8RunArgs) -> (Self, Task<EoxMessage>) {
        let mut rom = File::open(flags.program_path).unwrap();
        let mut program = vec![];
        rom.read_to_end(&mut program).unwrap();

        let mut executor = Executor::new(flags.legacy_shift);
        executor.load_program(&program).unwrap();

        let cycle_time = Duration::from_secs(1) / flags.opcodes_per_second;

        (
            EoxChip8GUI {
                executor: RefCell::new(executor),
                cycle_time,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: EoxMessage) -> Task<EoxMessage> {
        match message {
            EoxMessage::Tick(instant) => {
                info!("{instant:?}");

                if let Err(error) = self.executor.borrow_mut().execute_once() {
                    error!("{error}");
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, EoxMessage> {
        let mut executor = self.executor.borrow_mut();
        let display = executor.get_display_mut();
        let display_text = format!("{}", display);
        display.render();
        iced::widget::text(display_text)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<EoxMessage> {
        time::every(self.cycle_time).map(EoxMessage::Tick)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
