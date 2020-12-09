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
pub const INSTRUCTION_SIZE: i64 = 16;

pub struct InstructionPacket {
    pub slot1: Instruction,
    pub slot2: Instruction,
    pub slot3: Instruction,
    pub slot4: Instruction,
}
