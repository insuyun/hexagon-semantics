use z3::{ast::BV, Context};

use crate::register::{Register, RegisterSet};

#[allow(dead_code)]
pub enum JInstruction {
    Jump { r: i64 },
}

impl<'ctx> JInstruction {
    pub fn apply_instruction(
        &self,
        ctx: &'ctx Context,
        new_set: &mut RegisterSet<'ctx>,
        old_set: &RegisterSet<'ctx>,
    ) {
        match self {
            Self::Jump { r } => {
                let pc = old_set.get_register(Register::PC);
                new_set.set_register(Register::PC, pc + BV::from_i64(&ctx, *r, 32));
            }
        }
    }
}
