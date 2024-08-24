use crate::register::Register;

#[derive(Debug, PartialEq)]
pub enum Not {
    Byte(Register),
    Quarter(Register),
    Half(Register),
    Word(Register),
}
