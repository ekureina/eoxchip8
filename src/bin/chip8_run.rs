use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use clap::Parser;
use eoxchip8::core::cpu::main::Executor;
use iced::widget::Text;
use iced::{executor, time, Font, Length};
use iced::{Application, Command, Element, Settings, Theme};
use log::{error, info};

pub fn main() -> iced::Result {
    env_logger::init();

    let args = Chip8RunArgs::parse();
    let mut settings = Settings::with_flags(args);
    settings.default_font = Font::MONOSPACE;

    EoxChip8GUI::run(settings)
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

impl Application for EoxChip8GUI {
    type Executor = executor::Default;
    type Flags = Chip8RunArgs;
    type Message = EoxMessage;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
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
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("ExoChip8")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            EoxMessage::Tick(instant) => {
                info!("{instant:?}");

                if let Err(error) = self.executor.borrow_mut().execute_once() {
                    error!("{error}");
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut executor = self.executor.borrow_mut();
        let display = executor.get_display_mut();
        let display_text = format!("{}", display);
        display.render();
        Text::new(display_text)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(self.cycle_time).map(EoxMessage::Tick)
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
