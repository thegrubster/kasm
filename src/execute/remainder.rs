use crate::{
    constant::{Byte, Half, Quarter, Word},
    error::ExecuteError,
    instruction::Remainder,
    operand::Operand,
    register::Register,
    registers::RegisterOperations,
    Processor,
};

impl Processor {
    pub fn rem(&mut self, instruction: Remainder) -> Result<(), ExecuteError> {
        match instruction {
            Remainder::Byte(r, o) => self.rem_byte(r, o),
            Remainder::Quarter(r, o) => self.rem_quarter(r, o),
            Remainder::Half(r, o) => self.rem_half(r, o),
            Remainder::Word(r, o) => self.rem_word(r, o),
        }
    }

    fn rem_byte(&mut self, register: Register, operand: Operand<Byte>) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.get_byte_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_rem(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn rem_quarter(
        &mut self,
        register: Register,
        operand: Operand<Quarter>,
    ) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.get_quarter_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_rem(b);
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn rem_half(&mut self, register: Register, operand: Operand<Half>) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.get_half_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_rem(b);
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn rem_word(&mut self, register: Register, operand: Operand<Word>) -> Result<(), ExecuteError> {
        let a = self.registers.get_reg_val(register);
        let b = self.get_word_operand_val(operand);

        if b == 0 {
            return Err(ExecuteError::DivideByZero);
        }

        let (result, overflow) = a.overflowing_rem(b);
        self.flags.set_from_word(result, overflow);
        self.registers[register] = result;

        Ok(())
    }
}

#[cfg(test)]
mod byte {
    use crate::{
        constant::Byte, error::ExecuteError, operand::Operand, register::Register, Processor,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::DivideByZero);

        let actual = p.rem_byte(Register::A, Operand::Immediate(0));

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();

        p.rem_byte(Register::A, Operand::Immediate(Byte::MAX))?;

        assert_eq!(p.registers[Register::A], 0);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 4;

        p.rem_byte(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], 0);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 6;

        p.rem_byte(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], 2);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod quarter {
    use crate::{
        constant::Quarter, error::ExecuteError, operand::Operand, register::Register, Processor,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::DivideByZero);

        let actual = p.rem_quarter(Register::A, Operand::Immediate(0));

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();

        p.rem_quarter(Register::A, Operand::Immediate(Quarter::MAX))?;

        assert_eq!(p.registers[Register::A], 0);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 4;

        p.rem_quarter(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], 0);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 6;

        p.rem_byte(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], 2);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod half {
    use crate::{
        constant::Half, error::ExecuteError, operand::Operand, register::Register, Processor,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::DivideByZero);

        let actual = p.rem_half(Register::A, Operand::Immediate(0));

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();

        p.rem_half(Register::A, Operand::Immediate(Half::MAX))?;

        assert_eq!(p.registers[Register::A], 0);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 4;

        p.rem_half(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], 0);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 6;

        p.rem_byte(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], 2);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }
}

#[cfg(test)]
mod word {
    use crate::{
        constant::Word, error::ExecuteError, operand::Operand, register::Register, Processor,
    };

    #[test]
    fn divide_by_zero_error() {
        let mut p = Processor::new().unwrap();
        let expected = Err(ExecuteError::DivideByZero);

        let actual = p.rem_word(Register::A, Operand::Immediate(0));

        assert_eq!(actual, expected);
    }

    #[test]
    fn dividend_is_zero() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();

        p.rem_word(Register::A, Operand::Immediate(Word::MAX))?;

        assert_eq!(p.registers[Register::A], 0);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn dividing_two_registers() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 4;

        p.rem_word(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], 0);
        assert!(!p.flags.overflow);
        assert!(p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }

    #[test]
    fn uneven_division() -> Result<(), ExecuteError> {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = 20;
        p.registers[Register::B] = 6;

        p.rem_byte(Register::A, Operand::Register(Register::B))?;

        assert_eq!(p.registers[Register::A], 2);
        assert!(!p.flags.overflow);
        assert!(!p.flags.zero);
        assert!(!p.flags.sign);

        Ok(())
    }
}
