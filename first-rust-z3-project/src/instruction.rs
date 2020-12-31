use crate::register::{Register, Register64};

#[allow(dead_code)]
pub enum Instruction {
    Nop,
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
    /* CR class */
    AddToPC {
        rd: Register,
        imm: u32,
    },
    /* JR class */
    Callr {
        rs: Register,
    },
    /* J class */
    Jump {
        r: i64,
    },
    /* LD class */
    LoadByteIo {
        rs: Register,
        rd: Register,
        imm: i32,
    },
    /* MEMOP class */
    /* NV class */
    /* ST class */
    StoreByteIo {
        rs: Register,
        rt: Register,
        imm: i32,
    }, /* SYSTEM class */
       /* XTYPE class */
}

// TODO: we don't know the instruction size
pub const INSTRUCTION_SIZE: i64 = 16;

pub struct InstructionPacket {
    pub slot1: Instruction,
    pub slot2: Instruction,
    pub slot3: Instruction,
    pub slot4: Instruction,
}
