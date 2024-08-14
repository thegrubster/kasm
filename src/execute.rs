use crate::{constant::{ExecuteTable, Word}, error::ExecuteError, register::Register, registers::RegisterOperations, Processor};

pub const EXECUTE_TABLE: ExecuteTable = [
    Processor::execute_stop,
    Processor::execute_set_byte,
    Processor::execute_set_quarter,
    Processor::execute_set_half,
    Processor::execute_set_word,
    Processor::execute_add_byte,
    Processor::execute_add_quarter,
    Processor::execute_add_half,
    Processor::execute_add_word,
    Processor::execute_sub_byte,
];

impl Processor {
    fn execute_stop(&mut self) -> Result<(), ExecuteError> {
        self.running = false;

        Ok(())
    }

    /// The underlying value to be set is confirmed applicable to the given size operation when decoded.
    ///
    /// Therefore the execute phase of setting a register is identical for each size variant.
    fn execute_set(&mut self) -> Result<(), ExecuteError> {
        let register = Register::try_from(self.registers[Register::P1])?;
        let value = self.registers[Register::P2];
        self.registers[register] = value;

        Ok(())
    }

    fn execute_set_byte(&mut self) -> Result<(), ExecuteError> {
        self.execute_set()
    }

    fn execute_set_quarter(&mut self) -> Result<(), ExecuteError> {
        self.execute_set()
    }

    fn execute_set_half(&mut self) -> Result<(), ExecuteError> {
        self.execute_set()
    }

    fn execute_set_word(&mut self) -> Result<(), ExecuteError> {
        self.execute_set()
    }

    fn execute_add_byte(&mut self) -> Result<(), ExecuteError> {
        let register = Register::try_from(self.registers[Register::P1])?;
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.registers.get_reg_val_as_byte(Register::P2);
        
        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn execute_add_quarter(&mut self) -> Result<(), ExecuteError> {
        let register = Register::try_from(self.registers[Register::P1])?;
        let a = self.registers.get_reg_val_as_quarter(register);
        let b = self.registers.get_reg_val_as_quarter(Register::P2);
        
        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_quarter(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn execute_add_half(&mut self) -> Result<(), ExecuteError> {
        let register = Register::try_from(self.registers[Register::P1])?;
        let a = self.registers.get_reg_val_as_half(register);
        let b = self.registers.get_reg_val_as_half(Register::P2);
        
        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_half(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn execute_add_word(&mut self) -> Result<(), ExecuteError> {
        let register = Register::try_from(self.registers[Register::P1])?;
        let a = self.registers.get_reg_val(register);
        let b = self.registers.get_reg_val(Register::P2);
        
        let (result, overflow) = a.overflowing_add(b);
        self.flags.set_from_word(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }

    fn execute_sub_byte(&mut self) -> Result<(), ExecuteError> {
        let register = Register::try_from(self.registers[Register::P1])?;
        let a = self.registers.get_reg_val_as_byte(register);
        let b = self.registers.get_reg_val_as_byte(Register::P2);

        let (result, overflow) = a.overflowing_sub(b);
        self.flags.set_from_byte(result, overflow);
        self.registers[register] = result as Word;

        Ok(())
    }
}

#[cfg(test)]
mod execute_set {
    use crate::{error::ExecuteError, register::Register, Processor};

    #[test]
    fn destination_is_valid_register() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 100;
        p.execute_set().unwrap();
        let expected = 100;
        let actual = p.registers[Register::A];
        assert_eq!(actual, expected);
    }

    #[test]
    fn destination_is_invalid_register() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::P1] = Register::P1.as_word();
        p.registers[Register::P2] = 100;
        let expected = Err(ExecuteError::InvalidRegisterCast(Register::P1.as_word()));
        let actual = p.execute_set();
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod execute_add_byte {
    use crate::{constant::{Byte, Word}, register::Register, Processor};

    #[test]
    fn add_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word;
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 1;
        p.execute_add_byte().unwrap();
        
        assert_eq!(p.registers[Register::A], 0);
        assert!(p.flags.zero);
        assert!(p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Byte::MAX as Word - 1;
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 1;
        p.execute_add_byte().unwrap();
        
        assert_eq!(p.registers[Register::A], Byte::MAX as Word);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod execute_add_quarter {
    use crate::{constant::{Quarter, Word}, register::Register, Processor};

    #[test]
    fn add_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word;
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 1;
        p.execute_add_quarter().unwrap();
        
        assert_eq!(p.registers[Register::A], 0);
        assert!(p.flags.zero);
        assert!(p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Quarter::MAX as Word - 1;
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 1;
        p.execute_add_quarter().unwrap();
        
        assert_eq!(p.registers[Register::A], Quarter::MAX as Word);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod execute_add_half {
    use crate::{constant::{Half, Word}, register::Register, Processor};

    #[test]
    fn add_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word;
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 1;
        p.execute_add_half().unwrap();
        
        assert_eq!(p.registers[Register::A], 0);
        assert!(p.flags.zero);
        assert!(p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Half::MAX as Word - 1;
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 1;
        p.execute_add_half().unwrap();
        
        assert_eq!(p.registers[Register::A], Half::MAX as Word);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}

#[cfg(test)]
mod execute_add_word {
    use crate::{constant::Word, register::Register, Processor};

    #[test]
    fn add_causes_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX;
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 1;
        p.execute_add_word().unwrap();
        
        assert_eq!(p.registers[Register::A], 0);
        assert!(p.flags.zero);
        assert!(p.flags.overflow);
        assert!(!p.flags.sign);
    }

    #[test]
    fn add_does_not_cause_overflow() {
        let mut p = Processor::new().unwrap();
        p.registers[Register::A] = Word::MAX - 1;
        p.registers[Register::P1] = Register::A.as_word();
        p.registers[Register::P2] = 1;
        p.execute_add_word().unwrap();
        
        assert_eq!(p.registers[Register::A], Word::MAX);
        assert!(!p.flags.zero);
        assert!(!p.flags.overflow);
        assert!(p.flags.sign);
    }
}

