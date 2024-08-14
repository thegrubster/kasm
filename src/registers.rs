use crate::{constant::{Byte, Half, Quarter, Word, REGISTER_AMOUNT}, register::Register};

pub trait RegisterOperations {
    fn get_reg_as_byte(&self, register: Register) -> Byte;
    fn get_reg_as_quarter(&self, register: Register) -> Quarter;
    fn get_reg_as_half(&self, register: Register) -> Half;
    fn get_reg(&self, register: Register) -> Word;
}

pub type Registers = [Word; REGISTER_AMOUNT];

impl RegisterOperations for Registers {
    fn get_reg_as_byte(&self, register: Register) -> Byte {
        self[register].to_le_bytes()[0]
    }

    fn get_reg_as_quarter(&self, register: Register) -> Quarter {
        let mut bytes = [0; 2];
        bytes.copy_from_slice(&self[register].to_le_bytes()[0..2]);
        Quarter::from_le_bytes(bytes)
    }

    fn get_reg_as_half(&self, register: Register) -> Half {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self[register].to_le_bytes()[0..4]);
        Half::from_le_bytes(bytes)
    }

    fn get_reg(&self, register: Register) -> Word {
        self[register]
    }
}