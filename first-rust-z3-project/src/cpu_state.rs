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
            Instruction::Sub { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rt - rs);
            }
            Instruction::Subi { rs, rd, imm } => {
                let rs = old_set.get_register(*rs);
                self.set_register(*rd, BV::from_i64(&self.context, *imm as i64, 32) - rs);
            }
            Instruction::SubSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);
                // TODO: Check this saturation logic
                let overflow_unsigned = rt.bvadd_no_overflow(&rs.bvneg(), false);
                let overflow_signed = rt.bvadd_no_overflow(&rs.bvneg(), true);

                // TODO: Make general saturation function?
                // underflow
                let result = (overflow_signed.clone() & overflow_unsigned).ite(
                    &BV::from_i64(&self.context, i32::MIN as i64, 32),
                    // overflow
                    &(overflow_signed.clone()).ite(
                        &BV::from_i64(&self.context, i32::MAX as i64, 32),
                        &(rt - rs),
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
            Instruction::Sxtb { rs, rd } => {
                let rs = old_set.get_register(*rs);
                self.set_register(*rd, rs.extract(7, 0).sign_ext(24));
            }
            Instruction::Sxth { rs, rd } => {
                let rs = old_set.get_register(*rs);
                self.set_register(*rd, rs.extract(15, 0).sign_ext(16));
            }
            Instruction::TransferSi { rd, imm } => {
                self.set_register(
                    *rd,
                    BV::from_i64(&self.context, *imm as i64, 16).sign_ext(16),
                );
            }
            Instruction::TransferUiHi { rd, imm } => {
                let rd_val = old_set.get_register(*rd);
                self.set_register(
                    *rd,
                    (rd_val & BV::from_u64(&self.context, 0x0000FFFF, 32))
                        | BV::from_u64(&self.context, *imm as u64, 32).bvshl(&BV::from_u64(
                            &self.context,
                            16,
                            32,
                        )),
                );
            }
            Instruction::TransferUiLo { rd, imm } => {
                let rd_val = old_set.get_register(*rd);
                self.set_register(
                    *rd,
                    (rd_val & BV::from_u64(&self.context, 0xFFFF0000, 32))
                        | BV::from_u64(&self.context, *imm as u64, 32),
                );
            }
            Instruction::Transfer { rs, rd } => {
                let rs = old_set.get_register(*rs);
                self.set_register(*rd, rs.clone());
            }
            Instruction::Vaddh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh = rs.extract(31, 16) + rt.extract(31, 16);
                let rdl = rs.extract(15, 0) + rt.extract(15, 0);

                self.set_register(*rd, rdh.concat(&rdl));
            }
            Instruction::VaddhSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);

                let (rdh, usr) = {
                    let rs = rs.extract(31, 16);
                    let rt = rt.extract(31, 16);

                    let overflow_unsigned = rs.bvadd_no_overflow(&rt, false);
                    let overflow_signed = rs.bvadd_no_overflow(&rt, true);

                    // TODO: Make general saturation function?
                    // underflow
                    let result = (overflow_signed.clone() & overflow_unsigned).ite(
                        &BV::from_i64(&self.context, i16::MIN as i64, 16),
                        // overflow
                        &(overflow_signed.clone()).ite(
                            &BV::from_i64(&self.context, i16::MAX as i64, 16),
                            &(rs + rt),
                        ),
                    );

                    let ovf = overflow_signed.ite(
                        &BV::from_u64(&self.context, 1, 1),
                        &BV::from_u64(&self.context, 0, 1),
                    );
                    let new_usr = (usr & BV::from_u64(&self.context, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                let (rdl, usr) = {
                    let rs = rs.extract(15, 0);
                    let rt = rt.extract(15, 0);

                    let overflow_unsigned = rs.bvadd_no_overflow(&rt, false);
                    let overflow_signed = rs.bvadd_no_overflow(&rt, true);

                    // TODO: Make general saturation function?
                    // underflow
                    let result = (overflow_signed.clone() & overflow_unsigned).ite(
                        &BV::from_i64(&self.context, i16::MIN as i64, 16),
                        // overflow
                        &(overflow_signed.clone()).ite(
                            &BV::from_i64(&self.context, i16::MAX as i64, 16),
                            &(rs + rt),
                        ),
                    );

                    let ovf = overflow_signed.ite(
                        &BV::from_u64(&self.context, 1, 1),
                        &BV::from_u64(&self.context, 0, 1),
                    );
                    let new_usr = (usr & BV::from_u64(&self.context, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                self.set_register(*rd, rdh.concat(&rdl));
                self.set_register(Register::USR, usr);
            }
            Instruction::VadduhSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);

                let (rdh, usr) = {
                    let rs = rs.extract(31, 16);
                    let rt = rt.extract(31, 16);

                    let overflow_unsigned = rs.bvadd_no_overflow(&rt, false);

                    // overflow
                    let result = overflow_unsigned.ite(
                        &BV::from_u64(&self.context, u16::MAX as u64, 16),
                        &(rs + rt),
                    );

                    let ovf = overflow_unsigned.ite(
                        &BV::from_u64(&self.context, 1, 1),
                        &BV::from_u64(&self.context, 0, 1),
                    );
                    let new_usr = (usr & BV::from_u64(&self.context, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                let (rdl, usr) = {
                    let rs = rs.extract(15, 0);
                    let rt = rt.extract(15, 0);

                    let overflow_unsigned = rs.bvadd_no_overflow(&rt, false);

                    // overflow
                    let result = overflow_unsigned.ite(
                        &BV::from_u64(&self.context, u16::MAX as u64, 16),
                        &(rs + rt),
                    );

                    let ovf = overflow_unsigned.ite(
                        &BV::from_u64(&self.context, 1, 1),
                        &BV::from_u64(&self.context, 0, 1),
                    );
                    let new_usr = (usr & BV::from_u64(&self.context, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                self.set_register(*rd, rdh.concat(&rdl));
                self.set_register(Register::USR, usr);
            }
            Instruction::Vsubh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh = rt.extract(31, 16) - rs.extract(31, 16);
                let rdl = rt.extract(15, 0) - rs.extract(15, 0);

                self.set_register(*rd, rdh.concat(&rdl));
            }
            Instruction::VsubhSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);

                let (rdh, usr) = {
                    let rs = rs.extract(31, 16);
                    let rt = rt.extract(31, 16);

                    let overflow_unsigned = rt.bvadd_no_overflow(&rs.bvneg(), false);
                    let overflow_signed = rt.bvadd_no_overflow(&rs.bvneg(), true);

                    // TODO: Make general saturation function?
                    // underflow
                    let result = (overflow_signed.clone() & overflow_unsigned).ite(
                        &BV::from_i64(&self.context, i16::MIN as i64, 16),
                        // overflow
                        &(overflow_signed.clone()).ite(
                            &BV::from_i64(&self.context, i16::MAX as i64, 16),
                            &(rt - rs),
                        ),
                    );

                    let ovf = overflow_signed.ite(
                        &BV::from_u64(&self.context, 1, 1),
                        &BV::from_u64(&self.context, 0, 1),
                    );
                    let new_usr = (usr & BV::from_u64(&self.context, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                let (rdl, usr) = {
                    let rs = rs.extract(15, 0);
                    let rt = rt.extract(15, 0);

                    let overflow_unsigned = rt.bvadd_no_overflow(&rs.bvneg(), false);
                    let overflow_signed = rt.bvadd_no_overflow(&rs.bvneg(), true);

                    // TODO: Make general saturation function?
                    // underflow
                    let result = (overflow_signed.clone() & overflow_unsigned).ite(
                        &BV::from_i64(&self.context, i16::MIN as i64, 16),
                        // overflow
                        &(overflow_signed.clone()).ite(
                            &BV::from_i64(&self.context, i16::MAX as i64, 16),
                            &(rt - rs),
                        ),
                    );

                    let ovf = overflow_signed.ite(
                        &BV::from_u64(&self.context, 1, 1),
                        &BV::from_u64(&self.context, 0, 1),
                    );
                    let new_usr = (usr & BV::from_u64(&self.context, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                self.set_register(*rd, rdh.concat(&rdl));
                self.set_register(Register::USR, usr);
            }
            Instruction::VsubuhSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);

                let (rdh, usr) = {
                    let rs = rs.extract(31, 16);
                    let rt = rt.extract(31, 16);

                    let overflow_unsigned = rt.bvsub_no_overflow(&rs);

                    // overflow
                    let result = overflow_unsigned.ite(
                        &BV::from_u64(&self.context, u16::MAX as u64, 16),
                        &(rt - rs),
                    );

                    let ovf = overflow_unsigned.ite(
                        &BV::from_u64(&self.context, 1, 1),
                        &BV::from_u64(&self.context, 0, 1),
                    );
                    let new_usr = (usr & BV::from_u64(&self.context, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                let (rdl, usr) = {
                    let rs = rs.extract(15, 0);
                    let rt = rt.extract(15, 0);

                    let overflow_unsigned = rt.bvsub_no_overflow(&rs);

                    // overflow
                    let result = overflow_unsigned.ite(
                        &BV::from_u64(&self.context, u16::MAX as u64, 16),
                        &(rt - rs),
                    );

                    let ovf = overflow_unsigned.ite(
                        &BV::from_u64(&self.context, 1, 1),
                        &BV::from_u64(&self.context, 0, 1),
                    );
                    let new_usr = (usr & BV::from_u64(&self.context, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                self.set_register(*rd, rdh.concat(&rdl));
                self.set_register(Register::USR, usr);
            }
            Instruction::Vavgh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh = (rs.extract(31, 16) + rt.extract(31, 16)).bvashr(&BV::from_u64(
                    &self.context,
                    1,
                    16,
                ));
                let rdl = (rs.extract(15, 0) + rt.extract(15, 0)).bvashr(&BV::from_u64(
                    &self.context,
                    1,
                    16,
                ));

                self.set_register(*rd, rdh.concat(&rdl));
            }
            Instruction::VavghRnd { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh =
                    (rs.extract(31, 16) + rt.extract(31, 16) + &BV::from_u64(&self.context, 1, 16))
                        .bvashr(&BV::from_u64(&self.context, 1, 16));
                let rdl =
                    (rs.extract(15, 0) + rt.extract(15, 0) + &BV::from_u64(&self.context, 1, 16))
                        .bvashr(&BV::from_u64(&self.context, 1, 16));

                self.set_register(*rd, rdh.concat(&rdl));
            }

            Instruction::Vnavgh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh = (rt.extract(31, 16) - rs.extract(31, 16)).bvashr(&BV::from_u64(
                    &self.context,
                    1,
                    16,
                ));
                let rdl = (rt.extract(15, 0) - rs.extract(15, 0)).bvashr(&BV::from_u64(
                    &self.context,
                    1,
                    16,
                ));

                self.set_register(*rd, rdh.concat(&rdl));
            }
            Instruction::Zxth { rs, rd } => {
                let rs = old_set.get_register(*rs);
                self.set_register(*rd, rs.extract(15, 0).zero_ext(16));
            }
            Instruction::CombineRhRh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rt.extract(31, 16).concat(&rs.extract(31, 16)));
            }
            Instruction::CombineRhRl { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rt.extract(31, 16).concat(&rs.extract(15, 0)));
            }
            Instruction::CombineRlRh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rt.extract(15, 0).concat(&rs.extract(31, 16)));
            }
            Instruction::CombineRlRl { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register(*rd, rt.extract(15, 0).concat(&rs.extract(15, 0)));
            }
            Instruction::CombineIi1 { imm1, imm2, rdd } => {
                let hi = BV::from_i64(&self.context, *imm1, 8).sign_ext(24);
                let lo = BV::from_i64(&self.context, *imm2, 8).zero_ext(24);
                self.set_register_64(*rdd, hi.concat(&lo));
            }
            Instruction::CombineIi2 { imm1, imm2, rdd } => {
                let hi = BV::from_i64(&self.context, *imm1, 8).zero_ext(24);
                let lo = BV::from_u64(&self.context, *imm2, 6).sign_ext(26);
                self.set_register_64(*rdd, hi.concat(&lo));
            }
            Instruction::CombineIr { imm, rs, rdd } => {
                let hi = BV::from_i64(&self.context, *imm, 8).sign_ext(24);
                let rs = old_set.get_register(*rs);
                self.set_register_64(*rdd, hi.concat(&rs));
            }
            Instruction::CombineRi { imm, rs, rdd } => {
                let lo = BV::from_i64(&self.context, *imm, 8).sign_ext(24);
                let rs = old_set.get_register(*rs);
                self.set_register_64(*rdd, rs.concat(&lo));
            }
            Instruction::CombineRr { rs, rt, rdd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                self.set_register_64(*rdd, rs.concat(&rt));
            }
            Instruction::MuxII { imm1, imm2, pu, rd } => {
                let imm1 = BV::from_i64(&self.context, *imm1, 8).sign_ext(24);
                let imm2 = BV::from_i64(&self.context, *imm2, 8).zero_ext(24);
                let pu = old_set.get_register(*pu);

                self.set_register(
                    *rd,
                    pu.extract(0, 0)
                        ._eq(&BV::from_u64(&self.context, 1, 1))
                        .ite(&imm1, &imm2),
                );
            }
            Instruction::MuxIR { imm, rs, pu, rd } => {
                let imm = BV::from_i64(&self.context, *imm, 8).sign_ext(24);
                let rs = old_set.get_register(*rs);
                let pu = old_set.get_register(*pu);

                self.set_register(
                    *rd,
                    pu.extract(0, 0)
                        ._eq(&BV::from_u64(&self.context, 1, 1))
                        .ite(&imm, &rs),
                );
            }
            Instruction::MuxRI { imm, rs, pu, rd } => {
                let imm = BV::from_i64(&self.context, *imm, 8).sign_ext(24);
                let rs = old_set.get_register(*rs);
                let pu = old_set.get_register(*pu);

                self.set_register(
                    *rd,
                    pu.extract(0, 0)
                        ._eq(&BV::from_u64(&self.context, 1, 1))
                        .ite(&rs, &imm),
                );
            }
            Instruction::MuxRR { rs, rt, pu, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let pu = old_set.get_register(*pu);

                self.set_register(
                    *rd,
                    pu.extract(0, 0)
                        ._eq(&BV::from_u64(&self.context, 1, 1))
                        .ite(&rs, &rt),
                );
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
                self.memory.store_byte(&addr, &old_set.get_register(*rt));
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
