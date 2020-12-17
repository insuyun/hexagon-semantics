use z3::{ast::BV, Context};

use crate::instruction::{Instruction, InstructionPacket, INSTRUCTION_SIZE};
use crate::memory::{Memory, MemoryType};

#[derive(Clone, Copy)]
pub enum Register {
    R(usize),
    C(usize),
    SP, // R29
    FP, // R30
    LR, // R31
    PC, // C09
}

#[derive(Clone)]
pub struct RegisterSet<'ctx> {
    general: [BV<'ctx>; 32],
    control: [BV<'ctx>; 32],
}

pub struct CpuState<'ctx> {
    context: &'ctx Context,
    register_set: RegisterSet<'ctx>,
    memory: Memory<'ctx>,
}

impl<'ctx> RegisterSet<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        Self {
            general: [
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
            ],
            control: [
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
            ],
        }
    }

    pub fn get_register(&self, r: Register) -> &BV<'ctx> {
        match r {
            Register::R(v) => &self.general[v],
            Register::C(v) => &self.control[v],
            Register::SP => &self.general[29],
            Register::FP => &self.general[30],
            Register::LR => &self.general[31],
            Register::PC => &self.control[9],
        }
    }

    pub fn set_register(&mut self, r: Register, val: BV<'ctx>) {
        match r {
            Register::R(v) => self.general[v] = val,
            Register::C(v) => self.control[v] = val,
            Register::SP => self.general[29] = val,
            Register::FP => self.general[30] = val,
            Register::LR => self.general[31] = val,
            Register::PC => self.control[9] = val,
        };
    }
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

    pub fn get_register(&self, r: Register) -> &BV<'ctx> {
        self.register_set.get_register(r)
    }

    pub fn set_register(&mut self, r: Register, val: BV<'ctx>) {
        self.register_set.set_register(r, val);
    }

    fn apply_instruction(&mut self, inst: &Instruction, old_set: &RegisterSet<'ctx>) {
        match inst {
            Instruction::Nop => {}
            Instruction::Add { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rs + rt);
            }
            Instruction::AddToPC { rd, imm } => {
                let pc = old_set.get_register(Register::PC);
                self.set_register(*rd, pc + BV::from_u64(&self.context, *imm as u64, 32));
            }
            Instruction::Callr { rs } => {
                let rs = old_set.get_register(*rs);
                let pc = old_set.get_register(Register::PC);
                self.set_register(
                    Register::LR,
                    pc + BV::from_i64(&self.context, INSTRUCTION_SIZE, 32),
                );
                self.set_register(Register::PC, rs.clone());
            }
            Instruction::Jump { r } => {
                let pc = old_set.get_register(Register::PC);
                self.set_register(Register::PC, pc + BV::from_i64(&self.context, *r, 32));
            }
            Instruction::LoadByteIo { rs, rd, imm } => {
                let addr = old_set.get_register(*rs) + BV::from_i64(&self.context, *imm as i64, 32);
                let value = self.memory.load_byte(&addr);
                self.set_register(*rd, value);
            }
            Instruction::StoreByteIo { rs, rt, imm } => {
                let addr = old_set.get_register(*rs) + BV::from_i64(&self.context, *imm as i64, 32);
                self.memory.store_byte(&addr, old_set.get_register(*rt));
            }
        }
    }

    pub fn apply_instruction_packet(&mut self, packet: InstructionPacket) {
        let old_set = self.register_set.clone();

        self.apply_instruction(&packet.slot1, &old_set);
        self.apply_instruction(&packet.slot2, &old_set);
        self.apply_instruction(&packet.slot3, &old_set);
        self.apply_instruction(&packet.slot4, &old_set);
    }
}
