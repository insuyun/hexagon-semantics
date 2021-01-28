use std::fmt;
use z3::{ast::BV, Context};

use crate::instruction::INSTRUCTION_SIZE;
use crate::register::{Register, RegisterSet};

#[allow(dead_code)]
pub enum JrInstruction {
    Callr { rs: Register },
}

impl<'ctx> JrInstruction {
    pub fn apply_instruction(
        &self,
        ctx: &'ctx Context,
        new_set: &mut RegisterSet<'ctx>,
        old_set: &RegisterSet<'ctx>,
    ) {
        match self {
            Self::Callr { rs } => {
                let rs = old_set.get_register(*rs);
                let pc = old_set.get_register(Register::PC);
                new_set.set_register(Register::LR, pc + BV::from_i64(&ctx, INSTRUCTION_SIZE, 32));
                new_set.set_register(Register::PC, rs.clone());
            }
        }
    }
}

impl fmt::Display for JrInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Callr { rs } => write!(f, "callr {}", rs),
        }
    }
}
