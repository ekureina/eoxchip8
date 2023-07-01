use thiserror::Error;

use crate::core::memory::{Chip8Display, MemoryAccessError, Ram};

use super::{
    instructions::{Instruction, InstructionDecodeError},
    registers::{RegisterI, RegisterPC, RegisterV},
};

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Executor {
    memory: Ram,
    gp_registers: [RegisterV; 16],
    display: Chip8Display,
    i: RegisterI,
    pc: RegisterPC,
}

impl Executor {
    #[must_use]
    pub fn new() -> Self {
        Executor::default()
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), MemoryAccessError> {
        self.memory.load_program(program)?;
        self.pc = RegisterPC::default();
        self.display = Chip8Display::default();
        self.gp_registers = [RegisterV::default(); 16];
        self.i = RegisterI::default();
        Ok(())
    }

    pub fn execute_once(&mut self) -> Result<(), ExecutionError> {
        match self.memory.get_wide(self.pc.get())?.try_into()? {
            Instruction::ClearScreen => self.display.clear(),
            Instruction::LoadVImm { reg_num, imm } => {
                self.gp_registers[reg_num as usize].set(imm);
            }
            Instruction::LoadIImm { imm } => {
                self.i.set(imm);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Error)]
pub enum ExecutionError {
    #[error("Erorr on accessing memory: '{0}'")]
    MemoryAccess(#[from] MemoryAccessError),
    #[error("Error on decoding instruction: '{0}'")]
    InstructionDecode(#[from] InstructionDecodeError),
}
