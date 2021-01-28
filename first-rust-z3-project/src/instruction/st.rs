use std::fmt;
use z3::{ast::BV, Context};

use crate::memory::Memory;
use crate::register::{Register, RegisterSet};

#[allow(dead_code)]
pub enum StInstruction {
    StoreByteIo {
        rs: Register,
        rt: Register,
        imm: i32,
    },
}

impl<'ctx> StInstruction {
    pub fn apply_instruction(
        &self,
        ctx: &'ctx Context,
        memory: &mut Memory<'ctx>,
        _new_set: &mut RegisterSet<'ctx>,
        old_set: &RegisterSet<'ctx>,
    ) {
        match self {
            Self::StoreByteIo { rs, rt, imm } => {
                let addr = old_set.get_register(*rs) + BV::from_i64(&ctx, *imm as i64, 32);
                memory.store_byte(&addr, &old_set.get_register(*rt));
            }
        }
    }
}

impl fmt::Display for StInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StoreByteIo { rs, rt, imm } => write!(f, "memb({} + {}) = {}", rs, imm, rt),
        }
    }
}
