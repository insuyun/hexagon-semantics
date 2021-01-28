use z3::{
    ast::{Ast, BV},
    Context,
};

use crate::instruction::{Instruction, InstructionPacket, INSTRUCTION_SIZE};
use crate::memory::{Memory, MemoryType};
use crate::register::{Register, Register64, RegisterSet};

pub struct CpuState<'ctx> {
    context: &'ctx Context,
    register_set: RegisterSet<'ctx>,
    memory: Memory<'ctx>,
}

impl<'ctx> CpuState<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        let register_set = RegisterSet::new(&ctx);
        let memory = Memory::new(ctx, MemoryType::ArrayMemory);
        Self {
            context: ctx,
            register_set,
            memory,
        }
    }

    pub fn get_register(&self, r: Register) -> BV<'ctx> {
        self.register_set.get_register(r)
    }

    pub fn get_register_64(&self, rr: Register64) -> BV<'ctx> {
        self.register_set.get_register_64(rr)
    }

    pub fn set_register(&mut self, r: Register, val: BV<'ctx>) {
        self.register_set.set_register(r, val);
    }

    pub fn set_register_64(&mut self, rr: Register64, val: BV<'ctx>) {
        self.register_set.set_register_64(rr, val);
    }

    fn apply_instruction(&mut self, inst: &Instruction, old_set: &RegisterSet<'ctx>) {
        match inst {
            Instruction::Nop => {}
            Instruction::Alu32(inst) => {
                inst.apply_instruction(&self.context, &mut self.register_set, old_set);
            }
            Instruction::Cr(inst) => {
                inst.apply_instruction(&self.context, &mut self.register_set, old_set);
            }
            Instruction::Jr(inst) => {
                inst.apply_instruction(&self.context, &mut self.register_set, old_set);
            }
            Instruction::J(inst) => {
                inst.apply_instruction(&self.context, &mut self.register_set, old_set);
            }
            Instruction::Ld(inst) => {
                inst.apply_instruction(
                    &self.context,
                    &mut self.memory,
                    &mut self.register_set,
                    old_set,
                );
            }
            Instruction::St(inst) => {
                inst.apply_instruction(
                    &self.context,
                    &mut self.memory,
                    &mut self.register_set,
                    old_set,
                );
            }
        }
    }

    pub fn apply_instruction_packet(&mut self, packet: &InstructionPacket) {
        let old_set = self.register_set.clone();

        self.apply_instruction(&packet.slot1, &old_set);
        self.apply_instruction(&packet.slot2, &old_set);
        self.apply_instruction(&packet.slot3, &old_set);
        self.apply_instruction(&packet.slot4, &old_set);
    }
}
