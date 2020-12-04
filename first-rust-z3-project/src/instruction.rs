use crate::register::{Register, RegisterSet};

use z3::{ast::BV, Context};

pub enum Instruction {
    Nop,
    /* ALU32 class */
    Add {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    /* CR class */
    /* JR class */
    Callr {
        rs: Register,
    },
    /* J class */
    Jump {
        r: i64,
    }, /* LD class */
       /* MEMOP class */
       /* NV class */
       /* ST class */
       /* SYSTEM class */
       /* XTYPE class */
}

// TODO: we don't know the instruction size
const INSTRUCTION_SIZE: i64 = 16;

impl<'ctx> Instruction {
    pub fn apply(
        &self,
        ctx: &'ctx Context,
        old_set: &RegisterSet<'ctx>,
        set: &mut RegisterSet<'ctx>,
    ) {
        match self {
            Instruction::Nop => {}
            Instruction::Add { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                set.set_register(*rd, rs + rt);
            }
            Instruction::Callr { rs } => {
                let rs = old_set.get_register(*rs);
                let pc = old_set.get_register(Register::PC);
                set.set_register(Register::LR, pc + BV::from_i64(ctx, INSTRUCTION_SIZE, 32));
                set.set_register(Register::PC, rs.clone());
            }
            Instruction::Jump { r } => {
                let pc = old_set.get_register(Register::PC);
                set.set_register(Register::PC, pc + BV::from_i64(ctx, *r, 32));
            }
        }
    }
}

pub struct InstructionPacket {
    pub slot1: Instruction,
    pub slot2: Instruction,
    pub slot3: Instruction,
    pub slot4: Instruction,
}

impl<'ctx> InstructionPacket {
    pub fn apply(&self, ctx: &'ctx Context, set: &RegisterSet<'ctx>) -> RegisterSet<'ctx> {
        let mut new_set = set.clone();

        self.slot1.apply(ctx, set, &mut new_set);
        self.slot2.apply(ctx, set, &mut new_set);
        self.slot3.apply(ctx, set, &mut new_set);
        self.slot4.apply(ctx, set, &mut new_set);

        new_set
    }
}
