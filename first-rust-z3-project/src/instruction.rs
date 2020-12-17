use crate::register::Register;

pub enum Instruction {
    Nop,
    /* ALU32 class */
    Add {
        rs: Register,
        rt: Register,
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
