use log::debug;
use thiserror::Error;

use crate::core::memory::Address;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Instruction {
    ClearScreen,
    Call {
        address: Address,
    },
    Return,
    LoadVImm {
        reg_num: u8,
        imm: u8,
    },
    LoadIImm {
        imm: u16,
    },
    AddVImm {
        reg_num: u8,
        imm: u8,
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
    SkipIfEqVImm {
        reg_num: u8,
        imm: u8,
    },
    SkipIfNotEqVImm {
        reg_num: u8,
        imm: u8,
    },
    SkipIfEqualV2 {
        x_reg_num: u8,
        y_reg_num: u8,
    },
    SkipIfNotEqualV2 {
        x_reg_num: u8,
        y_reg_num: u8,
    },
    SetEqual {
        x_reg_num: u8,
        y_reg_num: u8,
    },
    BitWiseOrEqual {
        x_reg_num: u8,
        y_reg_num: u8,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Error)]
pub enum InstructionDecodeError {
    #[error("Unknown Instruction: {0:#04x}")]
    UnknownInstruction(u16),
}

impl TryFrom<u16> for Instruction {
    type Error = InstructionDecodeError;

    fn try_from(opcode: u16) -> Result<Self, Self::Error> {
        if opcode == 0x00E0 {
            return Ok(Instruction::ClearScreen);
        }

        if opcode == 0x00EE {
            return Ok(Instruction::Return);
        }

        match opcode & 0xF000 {
            0x1000 => {
                let address = Address(opcode & 0x0FFF);
                Ok(Instruction::JumpTo { address })
            }
            0x2000 => {
                let address = Address(opcode & 0x0FFF);
                Ok(Instruction::Call { address })
            }
            0x3000 => {
                let (reg_num, imm) = separate_register_and_imm(opcode);
                Ok(Instruction::SkipIfEqVImm { reg_num, imm })
            }
            0x4000 => {
                let (reg_num, imm) = separate_register_and_imm(opcode);
                Ok(Instruction::SkipIfNotEqVImm { reg_num, imm })
            }
            0x5000 => {
                let (x_reg_num, y_reg_num, last_nibble) = separate_two_registers_and_nibble(opcode);
                if last_nibble == 0 {
                    Ok(Instruction::SkipIfEqualV2 {
                        x_reg_num,
                        y_reg_num,
                    })
                } else {
                    Err(InstructionDecodeError::UnknownInstruction(opcode))
                }
            }
            0x6000 => {
                let (reg_num, imm) = separate_register_and_imm(opcode);
                Ok(Instruction::LoadVImm { reg_num, imm })
            }
            0x7000 => {
                let (reg_num, imm) = separate_register_and_imm(opcode);
                debug!("Register Number: {reg_num}; Immediate: {imm}");
                Ok(Instruction::AddVImm { reg_num, imm })
            }
            0x8000 => {
                let (x_reg_num, y_reg_num, last_nibble) = separate_two_registers_and_nibble(opcode);
                match last_nibble {
                    0 => Ok(Instruction::SetEqual {
                        x_reg_num,
                        y_reg_num,
                    }),
                    1 => Ok(Instruction::BitWiseOrEqual {
                        x_reg_num,
                        y_reg_num,
                    }),
                    _ => Err(InstructionDecodeError::UnknownInstruction(opcode)),
                }
            }
            0x9000 => {
                let (x_reg_num, y_reg_num, last_nibble) = separate_two_registers_and_nibble(opcode);
                if last_nibble == 0 {
                    Ok(Instruction::SkipIfNotEqualV2 {
                        x_reg_num,
                        y_reg_num,
                    })
                } else {
                    Err(InstructionDecodeError::UnknownInstruction(opcode))
                }
            }
            0xA000 => {
                let imm = opcode & 0xFFF;
                Ok(Instruction::LoadIImm { imm })
            }
            0xD000 => {
                let (x_reg_num, y_reg_num, sprite_length) =
                    separate_two_registers_and_nibble(opcode);
                Ok(Instruction::Draw {
                    x_reg_num,
                    y_reg_num,
                    sprite_length,
                })
            }
            0x0000 => {
                let address = Address(opcode & 0x0FFF);
                Ok(Instruction::Sys { address })
            }
            _ => Err(InstructionDecodeError::UnknownInstruction(opcode)),
        }
    }
}

fn separate_register_and_imm(opcode: u16) -> (u8, u8) {
    let register_index = ((opcode & 0x0F00) >> 8) as u8;
    let immediate = (opcode & 0x00FF) as u8;
    (register_index, immediate)
}

fn separate_two_registers_and_nibble(opcode: u16) -> (u8, u8, u8) {
    let register_index_1 = ((opcode & 0x0F00) >> 8) as u8;
    let register_index_2 = ((opcode & 0x00F0) >> 4) as u8;
    let nibble = (opcode & 0x000F) as u8;
    (register_index_1, register_index_2, nibble)
}
