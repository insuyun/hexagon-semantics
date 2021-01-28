mod alu32;
mod cr;
mod j;
mod jr;
mod ld;
mod st;

pub use alu32::Alu32Instruction;
pub use cr::CrInstruction;
pub use j::JInstruction;
pub use jr::JrInstruction;
pub use ld::LdInstruction;
pub use st::StInstruction;

use std::fmt;

#[allow(dead_code)]
pub enum Instruction {
    Nop,
    /* ALU32 class */
    Alu32(Alu32Instruction),
    /* CR class */
    Cr(CrInstruction),
    /* JR class */
    Jr(JrInstruction),
    /* J class */
    J(JInstruction),
    /* LD class */
    Ld(LdInstruction),
    /* MEMOP class */
    /* NV class */
    /* ST class */
    St(StInstruction),
    /* SYSTEM class */
    /* XTYPE class */
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nop => write!(f, "nop"),
            Self::Alu32(inst) => write!(f, "{}", inst),
            Self::Cr(inst) => write!(f, "{}", inst),
            Self::Jr(inst) => write!(f, "{}", inst),
            Self::J(inst) => write!(f, "{}", inst),
            Self::Ld(inst) => write!(f, "{}", inst),
            Self::St(inst) => write!(f, "{}", inst),
        }
    }
}

// TODO: we don't know the instruction size
pub const INSTRUCTION_SIZE: i64 = 16;

pub struct InstructionPacket {
    pub slot1: Instruction,
    pub slot2: Instruction,
    pub slot3: Instruction,
    pub slot4: Instruction,
}

impl fmt::Display for InstructionPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}; {}; {}; {};",
            self.slot1, self.slot2, self.slot3, self.slot4
        )
    }
}
