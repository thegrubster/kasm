//! Defines the constants, type aliases and central types to the virtual processor.

use crate::{error::DecodeError, instruction::Instruction};
use phf::Map;
use std::str::SplitWhitespace;

pub type Byte = u8;
pub type Quarter = u16;
pub type Half = u32;
pub type Word = u64;

/// The amount of bytes in a mega byte.
const MEGA_BYTE: usize = 1_048_576;

/// The size of the stack in the virtual processor.
///
/// The value is in bytes.
pub const STACK_SIZE: usize = MEGA_BYTE * 4;

type DecodeFn = fn(SplitWhitespace) -> Result<Instruction, DecodeError>;
pub type DecodeTable = Map<&'static str, DecodeFn>;

pub const BYTE: usize = 1;
pub const BYTES_IN_QUARTER: usize = 2;
pub const BYTES_IN_HALF: usize = 4;
pub const BYTES_IN_WORD: usize = 8;
