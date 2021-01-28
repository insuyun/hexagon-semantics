use std::fmt;
use z3::{ast::BV, Context};

use crate::register::{Register, RegisterSet};

#[allow(dead_code)]
pub enum CrInstruction {
    AddToPC { rd: Register, imm: u32 },
}

impl<'ctx> CrInstruction {
    pub fn apply_instruction(
        &self,
        ctx: &'ctx Context,
        new_set: &mut RegisterSet<'ctx>,
        old_set: &RegisterSet<'ctx>,
    ) {
        match self {
            Self::AddToPC { rd, imm } => {
                let pc = old_set.get_register(Register::PC);
                new_set.set_register(*rd, pc + BV::from_u64(&ctx, *imm as u64, 32));
            }
        }
    }
}

impl fmt::Display for CrInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddToPC { rd, imm } => write!(f, "{} = add(pc, {})", rd, imm),
        }
    }
}
