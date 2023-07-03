use log::debug;
use thiserror::Error;

use crate::core::memory::{
    memory_to_flip_instructions, Address, Chip8Display, MemoryAccessError, Ram,
};

use super::{
    instructions::{Instruction, InstructionDecodeError},
    registers::{RegisterI, RegisterPC, RegisterV},
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Executor {
    memory: Ram,
    gp_registers: [RegisterV; 16],
    display: Chip8Display,
    i: RegisterI,
    pc: RegisterPC,
    stack: Vec<Address>,
    legacy_shift: bool,
}

impl Executor {
    #[must_use]
    pub fn new(legacy_shift: bool) -> Self {
        Executor {
            legacy_shift,
            ..Default::default()
        }
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), MemoryAccessError> {
        self.memory.load_program(program)?;
        self.pc = RegisterPC::default();
        self.display = Chip8Display::default();
        self.gp_registers = [RegisterV::default(); 16];
        self.i = RegisterI::default();
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub fn execute_once(&mut self) -> Result<(), ExecutionError> {
        let pc = self.pc.get();
        debug!("PC: {:?}", self.pc);
        self.pc.inc();
        let instruction = self.memory.get_wide(pc)?.try_into()?;
        debug!("Instruction: {instruction:?}");
        match instruction {
            Instruction::ClearScreen => self.display.clear(),
            Instruction::Return => {
                let return_address = self.stack.pop().ok_or(ExecutionError::StackPopFail)?;
                self.pc.set(return_address);
            }
            Instruction::Call { address } => {
                self.stack.push(self.pc.get());
                self.pc.set(address);
            }
            Instruction::LoadVImm { reg_num, imm } => {
                self.gp_registers[reg_num as usize].set(imm);
            }
            Instruction::AddVImm { reg_num, imm } => {
                debug!("Register Number: {reg_num}; Immediate: {imm}");
                debug!(
                    "Register Data: {}",
                    self.gp_registers[reg_num as usize].get()
                );
                // Explicitly not setting the flags register here
                let _ = self.gp_registers[reg_num as usize].add(imm);
                debug!(
                    "Register Data: {}",
                    self.gp_registers[reg_num as usize].get()
                );
            }
            Instruction::SkipIfEqVImm { reg_num, imm } => {
                if self.gp_registers[reg_num as usize].get() == imm {
                    self.pc.inc();
                }
            }
            Instruction::SkipIfNotEqVImm { reg_num, imm } => {
                if self.gp_registers[reg_num as usize].get() != imm {
                    self.pc.inc();
                }
            }
            Instruction::SkipIfEqualV2 {
                x_reg_num,
                y_reg_num,
            } => {
                if self.gp_registers[x_reg_num as usize].get()
                    == self.gp_registers[y_reg_num as usize].get()
                {
                    self.pc.inc();
                }
            }
            Instruction::SkipIfNotEqualV2 {
                x_reg_num,
                y_reg_num,
            } => {
                if self.gp_registers[x_reg_num as usize].get()
                    != self.gp_registers[y_reg_num as usize].get()
                {
                    self.pc.inc();
                }
            }
            Instruction::LoadIImm { imm } => {
                self.i.set(imm);
            }
            Instruction::Draw {
                x_reg_num,
                y_reg_num,
                sprite_length,
            } => {
                self.draw_on_display(x_reg_num, y_reg_num, sprite_length)?;
            }
            Instruction::JumpTo { address } => {
                self.pc.set(address);
            }
            Instruction::SetEqual {
                x_reg_num,
                y_reg_num,
            } => {
                self.gp_registers[x_reg_num as usize]
                    .set(self.gp_registers[y_reg_num as usize].get());
            }
            Instruction::BitWiseOrEqual {
                x_reg_num,
                y_reg_num,
            } => {
                let current_val = self.gp_registers[x_reg_num as usize].get();
                let bit_or_val = self.gp_registers[y_reg_num as usize].get();
                self.gp_registers[x_reg_num as usize].set(current_val | bit_or_val);
            }
            Instruction::BitWiseAndEqual {
                x_reg_num,
                y_reg_num,
            } => {
                let current_val = self.gp_registers[x_reg_num as usize].get();
                let bit_or_val = self.gp_registers[y_reg_num as usize].get();
                self.gp_registers[x_reg_num as usize].set(current_val & bit_or_val);
            }
            Instruction::BitWiseXorEqual {
                x_reg_num,
                y_reg_num,
            } => {
                let current_val = self.gp_registers[x_reg_num as usize].get();
                let bit_or_val = self.gp_registers[y_reg_num as usize].get();
                self.gp_registers[x_reg_num as usize].set(current_val ^ bit_or_val);
            }
            Instruction::AddV2 {
                x_reg_num,
                y_reg_num,
            } => {
                let flag_result = self.gp_registers[x_reg_num as usize]
                    .add(self.gp_registers[y_reg_num as usize].get());
                self.set_flag_register(flag_result);
            }
            Instruction::SubV2 {
                x_reg_num,
                y_reg_num,
            } => {
                let flag_result = self.gp_registers[x_reg_num as usize]
                    .sub(self.gp_registers[y_reg_num as usize].get());
                self.set_flag_register(flag_result);
            }
            Instruction::SubNV2 {
                x_reg_num,
                y_reg_num,
            } => {
                self.set_flag_register(
                    self.gp_registers[y_reg_num as usize].get()
                        > self.gp_registers[x_reg_num as usize].get(),
                );
                let result = self.gp_registers[y_reg_num as usize]
                    .get()
                    .wrapping_sub(self.gp_registers[x_reg_num as usize].get());
                self.gp_registers[x_reg_num as usize].set(result);
            }
            Instruction::ShiftRight {
                x_reg_num,
                y_reg_num,
            } => {
                let used_register = if self.legacy_shift {
                    self.gp_registers[y_reg_num as usize].get()
                } else {
                    self.gp_registers[x_reg_num as usize].get()
                };
                self.gp_registers[x_reg_num as usize].set(used_register >> 1);
                self.set_flag_register((used_register & 0x1) != 0);
            }
            Instruction::ShiftLeft {
                x_reg_num,
                y_reg_num,
            } => {
                let used_register = if self.legacy_shift {
                    self.gp_registers[y_reg_num as usize].get()
                } else {
                    self.gp_registers[x_reg_num as usize].get()
                };
                self.gp_registers[x_reg_num as usize].set(used_register << 1);
                self.set_flag_register((used_register & 0x80) != 0);
            }
            Instruction::LoadRegistersFromMem { max_reg_num } => {
                let start_mem = self.i.get();
                for offset in 0..=max_reg_num {
                    self.gp_registers[offset as usize]
                        .set(self.memory.get(Address(start_mem + u16::from(offset)))?);
                }
            }
            Instruction::SaveRegistersToMem { max_reg_num } => {
                let start_mem = self.i.get();
                for offset in 0..=max_reg_num {
                    self.memory.set(
                        Address(start_mem + u16::from(offset)),
                        self.gp_registers[offset as usize].get(),
                    )?;
                }
            }
            Instruction::BCDRegister { register_num } => {
                let (first_digit, second_digit, third_digit) =
                    Executor::bcd(self.gp_registers[register_num as usize].get());
                let root_address = self.i.get();
                self.memory.set(Address(root_address), first_digit)?;
                self.memory.set(Address(root_address + 1), second_digit)?;
                self.memory.set(Address(root_address + 2), third_digit)?;
            }
            Instruction::AddIV { register_num } => {
                self.i.add(self.gp_registers[register_num as usize].get());
            }
            Instruction::Sys { .. } => {}
        }
        Ok(())
    }

    #[must_use]
    pub fn get_display_mut(&mut self) -> &mut Chip8Display {
        &mut self.display
    }

    #[allow(clippy::cast_possible_truncation)]
    fn draw_on_display(
        &mut self,
        x_reg_num: u8,
        y_reg_num: u8,
        sprite_length: u8,
    ) -> Result<(), MemoryAccessError> {
        let sprite_memory_start = self.i.get();
        let mut sprite_direct_memory = vec![];
        for offset in 0..sprite_length {
            sprite_direct_memory.push(
                self.memory
                    .get(Address(sprite_memory_start + u16::from(offset)))?,
            );
        }

        let sprite_flips = memory_to_flip_instructions(&sprite_direct_memory);

        let start_x = self.gp_registers[x_reg_num as usize].get();
        let start_y = self.gp_registers[y_reg_num as usize].get();
        for offset_x in 0..(u8::BITS as u8) {
            for offset_y in 0..sprite_length {
                if sprite_flips[offset_y as usize][offset_x as usize] {
                    self.display
                        .flip_pixel(start_x + offset_x, start_y + offset_y);
                }
            }
        }
        Ok(())
    }

    fn bcd(data: u8) -> (u8, u8, u8) {
        let first_digit = data / 100;
        let second_digit = (data % 100) / 10;
        let third_digit = data % 10;
        (first_digit, second_digit, third_digit)
    }

    fn set_flag_register(&mut self, flag: bool) {
        if flag {
            self.gp_registers[15].set(1);
        } else {
            self.gp_registers[15].set(0);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Error)]
pub enum ExecutionError {
    #[error("Erorr on accessing memory: '{0}'")]
    MemoryAccess(#[from] MemoryAccessError),
    #[error("Error on decoding instruction: '{0}'")]
    InstructionDecode(#[from] InstructionDecodeError),
    #[error("Issue popping the stack")]
    StackPopFail,
}
