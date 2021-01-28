use z3::{
    ast::{Ast, BV},
    Context,
};

use std::fmt;

use crate::register::{Register, Register64, RegisterSet};

#[allow(dead_code)]
pub enum Alu32Instruction {
    /* ALU32 class */
    /* ALU32/ALU class */
    Add {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Addi {
        rs: Register,
        rd: Register,
        imm: i32, // 16-bit
    },
    AddSat {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    And {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Andi {
        rs: Register,
        rd: Register,
        imm: i32, // 10-bit
    },
    Andn {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Not {
        rs: Register,
        rd: Register,
    },
    Or {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Ori {
        rs: Register,
        rd: Register,
        imm: i32, // 10-bit
    },
    Orn {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Xor {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Sub {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Subi {
        rs: Register,
        rd: Register,
        imm: i32, // 16-bit
    },
    SubSat {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Sxtb {
        rs: Register,
        rd: Register,
    },
    Sxth {
        rs: Register,
        rd: Register,
    },
    TransferSi {
        rd: Register,
        imm: i32, // 16-bit
    },
    TransferUiHi {
        rd: Register,
        imm: u32, // 16-bit
    },
    TransferUiLo {
        rd: Register,
        imm: u32, // 16-bit
    },
    Transfer {
        rs: Register,
        rd: Register,
    },
    Vaddh {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    VaddhSat {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    VadduhSat {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Vavgh {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    VavghRnd {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Vnavgh {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Vsubh {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    VsubhSat {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    VsubuhSat {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    Zxth {
        rs: Register,
        rd: Register,
    },
    /* ALU32/PERM class */
    CombineRhRh {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    CombineRhRl {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    CombineRlRh {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    CombineRlRl {
        rs: Register,
        rt: Register,
        rd: Register,
    },
    CombineIi1 {
        imm1: i64, // 8-bit
        imm2: i64, // 8-bit
        rdd: Register64,
    },
    CombineIi2 {
        imm1: i64, // 8-bit
        imm2: u64, // 6-bit
        rdd: Register64,
    },
    CombineIr {
        imm: i64, // 8-bit
        rs: Register,
        rdd: Register64,
    },
    CombineRi {
        imm: i64, // 8-bit
        rs: Register,
        rdd: Register64,
    },
    CombineRr {
        rs: Register,
        rt: Register,
        rdd: Register64,
    },
    MuxII {
        imm1: i64, // 8-bit
        imm2: i64, // 8-bit
        pu: Register,
        rd: Register,
    },
    MuxIR {
        imm: i64, // 8-bit
        rs: Register,
        pu: Register,
        rd: Register,
    },
    MuxRI {
        imm: i64, // 8-bit
        rs: Register,
        pu: Register,
        rd: Register,
    },
    MuxRR {
        rs: Register,
        rt: Register,
        pu: Register,
        rd: Register,
    },
    Aslh {
        rs: Register,
        rd: Register,
    },
    Asrh {
        rs: Register,
        rd: Register,
    },
    Packhl {
        rs: Register,
        rt: Register,
        rdd: Register64,
    },
}

impl<'ctx> Alu32Instruction {
    pub fn apply_instruction(
        &self,
        ctx: &'ctx Context,
        new_set: &mut RegisterSet<'ctx>,
        old_set: &RegisterSet<'ctx>,
    ) {
        match self {
            Self::Add { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rs + rt);
            }
            Self::Addi { rs, rd, imm } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs + BV::from_i64(&ctx, *imm as i64, 32));
            }
            Self::AddSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);
                let overflow_unsigned = rs.bvadd_no_overflow(&rt, false);
                let overflow_signed = rs.bvadd_no_overflow(&rt, true);

                // TODO: Make general saturation function?
                // underflow
                let result = (overflow_signed.clone() & overflow_unsigned).ite(
                    &BV::from_i64(&ctx, i32::MIN as i64, 32),
                    // overflow
                    &(overflow_signed.clone())
                        .ite(&BV::from_i64(&ctx, i32::MAX as i64, 32), &(rs + rt)),
                );

                let ovf = overflow_signed.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

                new_set.set_register(*rd, result);
                new_set.set_register(Register::USR, new_usr);
            }
            Self::And { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rs & rt);
            }
            Self::Andi { rs, rd, imm } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs & BV::from_i64(&ctx, *imm as i64, 32));
            }
            Self::Andn { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rs & rt.bvnot());
            }
            Self::Not { rs, rd } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs.bvnot());
            }
            Self::Or { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rs | rt);
            }
            Self::Ori { rs, rd, imm } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs | BV::from_i64(&ctx, *imm as i64, 32));
            }
            Self::Orn { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rs | rt.bvnot());
            }
            Self::Xor { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rs ^ rt);
            }
            Self::Sub { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rt - rs);
            }
            Self::Subi { rs, rd, imm } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, BV::from_i64(&ctx, *imm as i64, 32) - rs);
            }
            Self::SubSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);
                // TODO: Check this saturation logic
                let overflow_unsigned = rt.bvadd_no_overflow(&rs.bvneg(), false);
                let overflow_signed = rt.bvadd_no_overflow(&rs.bvneg(), true);

                // TODO: Make general saturation function?
                // underflow
                let result = (overflow_signed.clone() & overflow_unsigned).ite(
                    &BV::from_i64(&ctx, i32::MIN as i64, 32),
                    // overflow
                    &(overflow_signed.clone())
                        .ite(&BV::from_i64(&ctx, i32::MAX as i64, 32), &(rt - rs)),
                );

                let ovf = overflow_signed.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

                new_set.set_register(*rd, result);
                new_set.set_register(Register::USR, new_usr);
            }
            Self::Sxtb { rs, rd } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs.extract(7, 0).sign_ext(24));
            }
            Self::Sxth { rs, rd } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs.extract(15, 0).sign_ext(16));
            }
            Self::TransferSi { rd, imm } => {
                new_set.set_register(*rd, BV::from_i64(&ctx, *imm as i64, 16).sign_ext(16));
            }
            Self::TransferUiHi { rd, imm } => {
                let rd_val = old_set.get_register(*rd);
                new_set.set_register(
                    *rd,
                    (rd_val & BV::from_u64(&ctx, 0x0000FFFF, 32))
                        | BV::from_u64(&ctx, *imm as u64, 32).bvshl(&BV::from_u64(&ctx, 16, 32)),
                );
            }
            Self::TransferUiLo { rd, imm } => {
                let rd_val = old_set.get_register(*rd);
                new_set.set_register(
                    *rd,
                    (rd_val & BV::from_u64(&ctx, 0xFFFF0000, 32))
                        | BV::from_u64(&ctx, *imm as u64, 32),
                );
            }
            Self::Transfer { rs, rd } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs.clone());
            }
            Self::Vaddh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh = rs.extract(31, 16) + rt.extract(31, 16);
                let rdl = rs.extract(15, 0) + rt.extract(15, 0);

                new_set.set_register(*rd, rdh.concat(&rdl));
            }
            Self::VaddhSat { rs, rt, rd } => {
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
                        &BV::from_i64(&ctx, i16::MIN as i64, 16),
                        // overflow
                        &(overflow_signed.clone())
                            .ite(&BV::from_i64(&ctx, i16::MAX as i64, 16), &(rs + rt)),
                    );

                    let ovf =
                        overflow_signed.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                    let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

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
                        &BV::from_i64(&ctx, i16::MIN as i64, 16),
                        // overflow
                        &(overflow_signed.clone())
                            .ite(&BV::from_i64(&ctx, i16::MAX as i64, 16), &(rs + rt)),
                    );

                    let ovf =
                        overflow_signed.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                    let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                new_set.set_register(*rd, rdh.concat(&rdl));
                new_set.set_register(Register::USR, usr);
            }
            Self::VadduhSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);

                let (rdh, usr) = {
                    let rs = rs.extract(31, 16);
                    let rt = rt.extract(31, 16);

                    let overflow_unsigned = rs.bvadd_no_overflow(&rt, false);

                    // overflow
                    let result =
                        overflow_unsigned.ite(&BV::from_u64(&ctx, u16::MAX as u64, 16), &(rs + rt));

                    let ovf =
                        overflow_unsigned.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                    let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                let (rdl, usr) = {
                    let rs = rs.extract(15, 0);
                    let rt = rt.extract(15, 0);

                    let overflow_unsigned = rs.bvadd_no_overflow(&rt, false);

                    // overflow
                    let result =
                        overflow_unsigned.ite(&BV::from_u64(&ctx, u16::MAX as u64, 16), &(rs + rt));

                    let ovf =
                        overflow_unsigned.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                    let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                new_set.set_register(*rd, rdh.concat(&rdl));
                new_set.set_register(Register::USR, usr);
            }
            Self::Vsubh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh = rt.extract(31, 16) - rs.extract(31, 16);
                let rdl = rt.extract(15, 0) - rs.extract(15, 0);

                new_set.set_register(*rd, rdh.concat(&rdl));
            }
            Self::VsubhSat { rs, rt, rd } => {
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
                        &BV::from_i64(&ctx, i16::MIN as i64, 16),
                        // overflow
                        &(overflow_signed.clone())
                            .ite(&BV::from_i64(&ctx, i16::MAX as i64, 16), &(rt - rs)),
                    );

                    let ovf =
                        overflow_signed.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                    let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

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
                        &BV::from_i64(&ctx, i16::MIN as i64, 16),
                        // overflow
                        &(overflow_signed.clone())
                            .ite(&BV::from_i64(&ctx, i16::MAX as i64, 16), &(rt - rs)),
                    );

                    let ovf =
                        overflow_signed.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                    let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                new_set.set_register(*rd, rdh.concat(&rdl));
                new_set.set_register(Register::USR, usr);
            }
            Self::VsubuhSat { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let usr = old_set.get_register(Register::USR);

                let (rdh, usr) = {
                    let rs = rs.extract(31, 16);
                    let rt = rt.extract(31, 16);

                    let overflow_unsigned = rt.bvsub_no_overflow(&rs);

                    // overflow
                    let result =
                        overflow_unsigned.ite(&BV::from_u64(&ctx, u16::MAX as u64, 16), &(rt - rs));

                    let ovf =
                        overflow_unsigned.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                    let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                let (rdl, usr) = {
                    let rs = rs.extract(15, 0);
                    let rt = rt.extract(15, 0);

                    let overflow_unsigned = rt.bvsub_no_overflow(&rs);

                    // overflow
                    let result =
                        overflow_unsigned.ite(&BV::from_u64(&ctx, u16::MAX as u64, 16), &(rt - rs));

                    let ovf =
                        overflow_unsigned.ite(&BV::from_u64(&ctx, 1, 1), &BV::from_u64(&ctx, 0, 1));
                    let new_usr = (usr & BV::from_u64(&ctx, 0xFFFFFFFE, 32)) | ovf;

                    (result, new_usr)
                };

                new_set.set_register(*rd, rdh.concat(&rdl));
                new_set.set_register(Register::USR, usr);
            }
            Self::Vavgh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh =
                    (rs.extract(31, 16) + rt.extract(31, 16)).bvashr(&BV::from_u64(&ctx, 1, 16));
                let rdl =
                    (rs.extract(15, 0) + rt.extract(15, 0)).bvashr(&BV::from_u64(&ctx, 1, 16));

                new_set.set_register(*rd, rdh.concat(&rdl));
            }
            Self::VavghRnd { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh = (rs.extract(31, 16) + rt.extract(31, 16) + &BV::from_u64(&ctx, 1, 16))
                    .bvashr(&BV::from_u64(&ctx, 1, 16));
                let rdl = (rs.extract(15, 0) + rt.extract(15, 0) + &BV::from_u64(&ctx, 1, 16))
                    .bvashr(&BV::from_u64(&ctx, 1, 16));

                new_set.set_register(*rd, rdh.concat(&rdl));
            }

            Self::Vnavgh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                let rdh =
                    (rt.extract(31, 16) - rs.extract(31, 16)).bvashr(&BV::from_u64(&ctx, 1, 16));
                let rdl =
                    (rt.extract(15, 0) - rs.extract(15, 0)).bvashr(&BV::from_u64(&ctx, 1, 16));

                new_set.set_register(*rd, rdh.concat(&rdl));
            }
            Self::Zxth { rs, rd } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs.extract(15, 0).zero_ext(16));
            }
            Self::CombineRhRh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rt.extract(31, 16).concat(&rs.extract(31, 16)));
            }
            Self::CombineRhRl { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rt.extract(31, 16).concat(&rs.extract(15, 0)));
            }
            Self::CombineRlRh { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rt.extract(15, 0).concat(&rs.extract(31, 16)));
            }
            Self::CombineRlRl { rs, rt, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register(*rd, rt.extract(15, 0).concat(&rs.extract(15, 0)));
            }
            Self::CombineIi1 { imm1, imm2, rdd } => {
                let hi = BV::from_i64(&ctx, *imm1, 8).sign_ext(24);
                let lo = BV::from_i64(&ctx, *imm2, 8).zero_ext(24);
                new_set.set_register_64(*rdd, hi.concat(&lo));
            }
            Self::CombineIi2 { imm1, imm2, rdd } => {
                let hi = BV::from_i64(&ctx, *imm1, 8).zero_ext(24);
                let lo = BV::from_u64(&ctx, *imm2, 6).sign_ext(26);
                new_set.set_register_64(*rdd, hi.concat(&lo));
            }
            Self::CombineIr { imm, rs, rdd } => {
                let hi = BV::from_i64(&ctx, *imm, 8).sign_ext(24);
                let rs = old_set.get_register(*rs);
                new_set.set_register_64(*rdd, hi.concat(&rs));
            }
            Self::CombineRi { imm, rs, rdd } => {
                let lo = BV::from_i64(&ctx, *imm, 8).sign_ext(24);
                let rs = old_set.get_register(*rs);
                new_set.set_register_64(*rdd, rs.concat(&lo));
            }
            Self::CombineRr { rs, rt, rdd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                new_set.set_register_64(*rdd, rs.concat(&rt));
            }
            Self::MuxII { imm1, imm2, pu, rd } => {
                let imm1 = BV::from_i64(&ctx, *imm1, 8).sign_ext(24);
                let imm2 = BV::from_i64(&ctx, *imm2, 8).zero_ext(24);
                let pu = old_set.get_register(*pu);

                new_set.set_register(
                    *rd,
                    pu.extract(0, 0)
                        ._eq(&BV::from_u64(&ctx, 1, 1))
                        .ite(&imm1, &imm2),
                );
            }
            Self::MuxIR { imm, rs, pu, rd } => {
                let imm = BV::from_i64(&ctx, *imm, 8).sign_ext(24);
                let rs = old_set.get_register(*rs);
                let pu = old_set.get_register(*pu);

                new_set.set_register(
                    *rd,
                    pu.extract(0, 0)
                        ._eq(&BV::from_u64(&ctx, 1, 1))
                        .ite(&imm, &rs),
                );
            }
            Self::MuxRI { imm, rs, pu, rd } => {
                let imm = BV::from_i64(&ctx, *imm, 8).sign_ext(24);
                let rs = old_set.get_register(*rs);
                let pu = old_set.get_register(*pu);

                new_set.set_register(
                    *rd,
                    pu.extract(0, 0)
                        ._eq(&BV::from_u64(&ctx, 1, 1))
                        .ite(&rs, &imm),
                );
            }
            Self::MuxRR { rs, rt, pu, rd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);
                let pu = old_set.get_register(*pu);

                new_set.set_register(
                    *rd,
                    pu.extract(0, 0)
                        ._eq(&BV::from_u64(&ctx, 1, 1))
                        .ite(&rs, &rt),
                );
            }
            Self::Aslh { rs, rd } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs.bvshl(&BV::from_u64(&ctx, 16, 32)));
            }
            Self::Asrh { rs, rd } => {
                let rs = old_set.get_register(*rs);
                new_set.set_register(*rd, rs.bvashr(&BV::from_u64(&ctx, 16, 32)));
            }
            Self::Packhl { rs, rt, rdd } => {
                let rs = old_set.get_register(*rs);
                let rt = old_set.get_register(*rt);

                new_set.set_register_64(
                    *rdd,
                    rs.extract(31, 16)
                        .concat(&rt.extract(31, 16))
                        .concat(&rs.extract(15, 0))
                        .concat(&rt.extract(15, 0)),
                );
            }
        }
    }
}

impl fmt::Display for Alu32Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { rs, rt, rd } => {
                write!(f, "{} = add({}, {})", rd, rs, rt)
            }
            Self::Addi { rs, rd, imm } => {
                write!(f, "{} = add({}, {})", rd, rs, imm)
            }
            Self::AddSat { rs, rt, rd } => {
                write!(f, "{} = add({}, {}):sat", rd, rs, rt)
            }
            Self::And { rs, rt, rd } => {
                write!(f, "{} = and({}, {})", rd, rs, rt)
            }
            Self::Andi { rs, rd, imm } => {
                write!(f, "{} = and({}, {})", rd, rs, imm)
            }
            Self::Andn { rs, rt, rd } => {
                write!(f, "{} = and({}, ~{})", rd, rt, rs)
            }
            Self::Not { rs, rd } => {
                write!(f, "{} = not({})", rd, rs)
            }
            Self::Or { rs, rt, rd } => {
                write!(f, "{} = or({}, {})", rd, rs, rt)
            }
            Self::Ori { rs, rd, imm } => {
                write!(f, "{} = or({}, {})", rd, rs, imm)
            }
            Self::Orn { rs, rt, rd } => {
                write!(f, "{} = and({}, ~{})", rd, rt, rs)
            }
            Self::Xor { rs, rt, rd } => {
                write!(f, "{} = xor({}, {})", rd, rs, rt)
            }
            Self::Sub { rs, rt, rd } => {
                write!(f, "{} = sub({}, {})", rd, rt, rs)
            }
            Self::Subi { rs, rd, imm } => {
                write!(f, "{} = sub({}, {})", rd, imm, rs)
            }
            Self::SubSat { rs, rt, rd } => {
                write!(f, "{} = sub({}, {}):sat", rd, rt, rs)
            }
            Self::Sxtb { rs, rd } => {
                write!(f, "{} = sxtb({})", rd, rs)
            }
            Self::Sxth { rs, rd } => {
                write!(f, "{} = sxth({})", rd, rs)
            }
            Self::TransferSi { rd, imm } => {
                write!(f, "{} = {}", rd, imm)
            }
            Self::TransferUiHi { rd, imm } => {
                write!(f, "{}.H = {}", rd, imm)
            }
            Self::TransferUiLo { rd, imm } => {
                write!(f, "{}.L = {}", rd, imm)
            }
            Self::Transfer { rs, rd } => {
                write!(f, "{} = {}", rd, rs)
            }
            Self::Vaddh { rs, rt, rd } => {
                write!(f, "{} = vaddh({}, {})", rd, rs, rt)
            }
            Self::VaddhSat { rs, rt, rd } => {
                write!(f, "{} = vaddh({}, {}):sat", rd, rs, rt)
            }
            Self::VadduhSat { rs, rt, rd } => {
                write!(f, "{} = vadduh({}, {}):sat", rd, rs, rt)
            }
            Self::Vsubh { rs, rt, rd } => {
                write!(f, "{} = vsubh({}, {})", rd, rs, rt)
            }
            Self::VsubhSat { rs, rt, rd } => {
                write!(f, "{} = vsubh({}, {}):sat", rd, rs, rt)
            }
            Self::VsubuhSat { rs, rt, rd } => {
                write!(f, "{} = vsubuh({}, {}):sat", rd, rs, rt)
            }
            Self::Vavgh { rs, rt, rd } => {
                write!(f, "{} = vavgh({}, {})", rd, rs, rt)
            }
            Self::VavghRnd { rs, rt, rd } => {
                write!(f, "{} = vavgh({}, {}):rnd", rd, rs, rt)
            }

            Self::Vnavgh { rs, rt, rd } => {
                write!(f, "{} = vnavgh({}, {})", rd, rs, rt)
            }
            Self::Zxth { rs, rd } => {
                write!(f, "{} = zxth({})", rd, rs)
            }
            Self::CombineRhRh { rs, rt, rd } => {
                write!(f, "{} = combine({}.H, {}.H)", rd, rt, rs)
            }
            Self::CombineRhRl { rs, rt, rd } => {
                write!(f, "{} = combine({}.H, {}.L)", rd, rt, rs)
            }
            Self::CombineRlRh { rs, rt, rd } => {
                write!(f, "{} = combine({}.L, {}.H)", rd, rt, rs)
            }
            Self::CombineRlRl { rs, rt, rd } => {
                write!(f, "{} = combine({}.L, {}.L)", rd, rt, rs)
            }
            Self::CombineIi1 { imm1, imm2, rdd } => {
                write!(f, "{} = combine({}, {})", rdd, imm1, imm2)
            }
            Self::CombineIi2 { imm1, imm2, rdd } => {
                // TODO: Check how this instruction syntax is differentiated from CombineIi1
                write!(f, "{} = combine({}, {})", rdd, imm1, imm2)
            }
            Self::CombineIr { imm, rs, rdd } => {
                write!(f, "{} = combine({}, {})", rdd, imm, rs)
            }
            Self::CombineRi { imm, rs, rdd } => {
                write!(f, "{} = combine({}, {})", rdd, rs, imm)
            }
            Self::CombineRr { rs, rt, rdd } => {
                write!(f, "{} = combine({}, {})", rdd, rs, rt)
            }
            Self::MuxII { imm1, imm2, pu, rd } => {
                write!(f, "{} = mux({}, {}, {})", rd, pu, imm1, imm2)
            }
            Self::MuxIR { imm, rs, pu, rd } => {
                write!(f, "{} = mux({}, {}, {})", rd, pu, imm, rs)
            }
            Self::MuxRI { imm, rs, pu, rd } => {
                write!(f, "{} = mux({}, {}, {})", rd, pu, rs, imm)
            }
            Self::MuxRR { rs, rt, pu, rd } => {
                write!(f, "{} = mux({}, {}, {})", rd, pu, rs, rt)
            }
            Self::Aslh { rs, rd } => {
                write!(f, "{} = aslh({})", rd, rs)
            }
            Self::Asrh { rs, rd } => {
                write!(f, "{} = asrh({})", rd, rs)
            }
            Self::Packhl { rs, rt, rdd } => {
                write!(f, "{} = packhl({}, {})", rdd, rs, rt)
            }
        }
    }
}
