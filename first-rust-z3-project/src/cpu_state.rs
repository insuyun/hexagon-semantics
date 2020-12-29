use z3::{ast::BV, Context};

use crate::instruction::{Instruction, InstructionPacket, INSTRUCTION_SIZE};
use crate::memory::{Memory, MemoryType};
use crate::register::{Register, RegisterSet};

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
            Instruction::Addi { rs, rd, imm } => {
                let rs = old_set.get_register(*rs);
                self.set_register(*rd, rs + BV::from_i64(&self.context, *imm as i64, 32));
            }
            Instruction::AddSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);
                let overflow_unsigned = rs.bvadd_no_overflow(rt, false);
                let overflow_signed = rs.bvadd_no_overflow(rt, true);

                // TODO: Make general saturation function?
                // underflow
                let result = (overflow_signed.clone() & overflow_unsigned).ite(
                    &BV::from_i64(&self.context, i32::MIN as i64, 32),
                    // overflow
                    &(overflow_signed.clone()).ite(
                        &BV::from_i64(&self.context, i32::MAX as i64, 32),
                        &(rs + rt),
                    ),
                );

                let ovf = overflow_signed.ite(
                    &BV::from_u64(&self.context, 1, 1),
                    &BV::from_u64(&self.context, 0, 1),
                );
                let new_usr = (usr & BV::from_u64(&self.context, 0xFFFFFFFE, 32)) | ovf;

                self.set_register(*rd, result);
                self.set_register(Register::USR, new_usr);
            }
            Instruction::And { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rs & rt);
            }
            Instruction::Andi { rs, rd, imm } => {
                let rs = old_set.get_register(*rs);
                self.set_register(*rd, rs & BV::from_i64(&self.context, *imm as i64, 32));
            }
            Instruction::Andn { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rs & rt.bvnot());
            }
            Instruction::Not { rs, rd } => {
                let rs = old_set.get_register(*rs);
                self.set_register(*rd, rs.bvnot());
            }
            Instruction::Or { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rs | rt);
            }
            Instruction::Ori { rs, rd, imm } => {
                let rs = old_set.get_register(*rs);
                self.set_register(*rd, rs | BV::from_i64(&self.context, *imm as i64, 32));
            }
            Instruction::Orn { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rs | rt.bvnot());
            }
            Instruction::Xor { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rs ^ rt);
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
