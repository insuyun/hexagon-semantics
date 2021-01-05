use z3::{ast::BV, Context};

use crate::memory::Memory;
use crate::register::{Register, RegisterSet};

#[allow(dead_code)]
pub enum LdInstruction {
    LoadByteIo {
        rs: Register,
        rd: Register,
        imm: i32,
    },
}

impl<'ctx> LdInstruction {
    pub fn apply_instruction(
        &self,
        ctx: &'ctx Context,
        memory: &Memory<'ctx>,
        new_set: &mut RegisterSet<'ctx>,
        old_set: &RegisterSet<'ctx>,
    ) {
        match self {
            Self::LoadByteIo { rs, rd, imm } => {
                let addr = old_set.get_register(*rs) + BV::from_i64(&ctx, *imm as i64, 32);
                let value = memory.load_byte(&addr);
                new_set.set_register(*rd, value);
            }
        }
    }
}
