use crate::{
    constant::Word, error::ExecuteError, instruction::Instruction, operand::Operand,
    registers::RegisterOperations, utils::FromBytes, Interpreter,
};

mod addition;
mod and;
mod compare;
mod division;
mod jump;
mod multiplication;
mod not;
mod or;
mod pop;
mod print_register;
mod print_stack;
mod push;
mod remainder;
mod set;
mod subtraction;
mod test;
mod xor;

impl Interpreter {
    fn get_operand_value<T>(&self, operand: Operand<T>) -> T
    where
        T: FromBytes,
    {
        match operand {
            Operand::Register(register) => self.registers.get::<T>(register),
            Operand::Immediate(value) => value,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), ExecuteError> {
        use Instruction::*;

        match instruction {
            Stop => self.stop()?,
            Set(set_ins) => self.set(set_ins),
            Addition(add_ins) => self.add(add_ins),
            Subtraction(sub_ins) => self.sub(sub_ins),
            Multiplication(mul_ins) => self.mul(mul_ins),
            Division(div_ins) => self.div(div_ins)?,
            Remainder(rem_ins) => self.rem(rem_ins)?,
            Push(push_ins) => self.push(push_ins)?,
            Pop(pop_ins) => self.pop(pop_ins)?,
            Call(operand) => self.call(operand)?,
            Return => self.ret()?,
            And(and_ins) => self.and(and_ins),
            Or(or_ins) => self.or(or_ins),
            Xor(xor_ins) => self.xor(xor_ins),
            Not(not_ins) => self.not(not_ins),
            Test(test_ins) => self.test(test_ins),
            Compare(compare_ins) => self.compare(compare_ins),
            Jump(jump_ins) => self.jump(jump_ins),
            PrintRegister(print_reg_ins) => self.print_register(print_reg_ins)?,
            PrintStack(print_stack_ins) => self.print_stack(print_stack_ins)?,
        }

        Ok(())
    }

    fn stop(&mut self) -> Result<(), ExecuteError> {
        self.running = false;

        if self.config.print_instructions_executed {
            writeln!(
                self.output,
                "Instructions Executed: {}",
                self.config.instructions_executed
            )
            .map_err(|err| ExecuteError::IO(err.to_string()))?;
        }

        Ok(())
    }

    fn call(&mut self, operand: Operand<Word>) -> Result<(), ExecuteError> {
        let destination = self.get_operand_value(operand);
        self.push_value(destination)?;

        Ok(())
    }

    fn ret(&mut self) -> Result<(), ExecuteError> {
        let destination = self.pop_value::<Word>()?;
        self.program_counter = destination;

        Ok(())
    }
}
