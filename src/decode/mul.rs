use super::{
    decode_reg_and_byte_operand, decode_reg_and_half_operand, decode_reg_and_quarter_operand,
    decode_reg_and_word_operand,
};
use crate::{
    error::DecodeError,
    instruction::{Instruction, MulInstruction},
};
use std::str::SplitWhitespace;

pub struct MulDecoder;

impl MulDecoder {
    pub fn decode_mul_byte(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_byte_operand(iter)?;
        let instruction = MulInstruction::Byte(register, operand);

        Ok(Instruction::Mul(instruction))
    }

    pub fn decode_mul_quarter(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_quarter_operand(iter)?;
        let instruction = MulInstruction::Quarter(register, operand);

        Ok(Instruction::Mul(instruction))
    }

    pub fn decode_mul_half(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_half_operand(iter)?;
        let instruction = MulInstruction::Half(register, operand);

        Ok(Instruction::Mul(instruction))
    }

    pub fn decode_mul_word(iter: SplitWhitespace) -> Result<Instruction, DecodeError> {
        let (register, operand) = decode_reg_and_word_operand(iter)?;
        let instruction = MulInstruction::Word(register, operand);

        Ok(Instruction::Mul(instruction))
    }
}
