use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use clap::Parser;
use eoxchip8::core::cpu::main::Executor;
use iced::keyboard::key::{Code, Physical};
use iced::keyboard::{Event, Key};
use iced::{Element, Task, Theme, event};
use iced::{Font, Length, time};
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

#[derive(Debug, PartialEq, Eq)]
struct EoxChip8GUI {
    executor: RefCell<Executor>,
    cycle_time: Duration,
    key_map: HashMap<Key, Physical>,
}

#[derive(Debug, Clone)]
enum EoxMessage {
    Tick(Instant),
    KeyDown(Key, Physical),
    KeyUp(Key),
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
                key_map: HashMap::new(),
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
            EoxMessage::KeyUp(key) => {
                if let Some(physical_key) = self.key_map.get(&key)
                    && let Some(key_num) = convert_key(*physical_key)
                    && let Err(error) = self.executor.borrow_mut().set_key_released(key_num)
                {
                    error!("{error}");
                }
            }
            EoxMessage::KeyDown(key, physical) => {
                self.key_map.insert(key, physical);
                if let Some(key_num) = convert_key(physical)
                    && let Err(error) = self.executor.borrow_mut().set_key_pressed(key_num)
                {
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
        iced::Subscription::batch([
            time::every(self.cycle_time).map(EoxMessage::Tick),
            event::listen_with(|event, _, _| match event {
                iced::Event::Keyboard(event) => match event {
                    Event::KeyPressed {
                        key,
                        physical_key,
                        text,
                        ..
                    } => {
                        info!("Pressed {text:?}");
                        if let Some(_) = convert_key(physical_key) {
                            Some(EoxMessage::KeyDown(key, physical_key))
                        } else {
                            None
                        }
                    }
                    Event::KeyReleased { key, .. } => Some(EoxMessage::KeyUp(key)),
                    _ => None,
                },
                _ => None,
            }),
        ])
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn convert_key(key: Physical) -> Option<u8> {
    match key {
        Physical::Code(Code::Digit1) => Some(1),
        Physical::Code(Code::Digit2) => Some(2),
        Physical::Code(Code::Digit3) => Some(3),
        Physical::Code(Code::Digit4) => Some(12),
        Physical::Code(Code::KeyQ) => Some(4),
        Physical::Code(Code::KeyW) => Some(5),
        Physical::Code(Code::KeyE) => Some(6),
        Physical::Code(Code::KeyR) => Some(13),
        Physical::Code(Code::KeyA) => Some(7),
        Physical::Code(Code::KeyS) => Some(8),
        Physical::Code(Code::KeyD) => Some(9),
        Physical::Code(Code::KeyF) => Some(14),
        Physical::Code(Code::KeyZ) => Some(10),
        Physical::Code(Code::KeyX) => Some(0),
        Physical::Code(Code::KeyC) => Some(11),
        Physical::Code(Code::KeyV) => Some(15),
        _ => None,
    }
}
