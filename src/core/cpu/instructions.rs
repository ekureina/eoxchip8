use thiserror::Error;

use crate::core::memory::Address;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Instruction {
    ClearScreen,
    LoadVImm {
        reg_num: u8,
        imm: u8,
    },
    LoadIImm {
        imm: u16,
    },
    Draw {
        x_reg_num: u8,
        y_reg_num: u8,
        sprite_length: u8,
    },
    JumpTo {
        address: Address,
    },
    Sys {
        address: Address,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Error)]
pub enum InstructionDecodeError {
    #[error("Unknown Instruction: {0:#04x}")]
    UnknownInstruction(u16),
}

impl TryFrom<u16> for Instruction {
    type Error = InstructionDecodeError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0x00E0 {
            return Ok(Instruction::ClearScreen);
        }

        match value & 0xF000 {
            0x6000 => {
                let register_num = ((value & 0x0F00) >> 8) as u8;
                let imm = (value & 0x00FF) as u8;
                Ok(Instruction::LoadVImm {
                    reg_num: register_num,
                    imm,
                })
            }
            0xA000 => {
                let imm = value & 0xFFF;
                Ok(Instruction::LoadIImm { imm })
            }
            0xD000 => {
                let x_reg_num = ((value & 0x0F00) >> 8) as u8;
                let y_reg_num = ((value & 0x00F0) >> 4) as u8;
                let sprite_length = (value & 0x000F) as u8;
                Ok(Instruction::Draw {
                    x_reg_num,
                    y_reg_num,
                    sprite_length,
                })
            }
            0x0000 => {
                let address = Address(value & 0x0FFF);
                Ok(Instruction::Sys { address })
            }
            0x1000 => {
                let address = Address(value & 0x0FFF);
                Ok(Instruction::JumpTo { address })
            }
            _ => Err(InstructionDecodeError::UnknownInstruction(value)),
        }
    }
}
