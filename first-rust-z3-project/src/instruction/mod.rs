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

// TODO: we don't know the instruction size
pub const INSTRUCTION_SIZE: i64 = 16;

pub struct InstructionPacket {
    pub slot1: Instruction,
    pub slot2: Instruction,
    pub slot3: Instruction,
    pub slot4: Instruction,
}
