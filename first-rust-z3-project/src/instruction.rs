use crate::register::{Register, RegisterSet};

pub enum Instruction {
    AddInstruction {
        rs: Register,
        rt: Register,
        rd: Register,
    },
}

impl<'ctx> Instruction {
    pub fn apply(&self, set: &RegisterSet<'ctx>) -> RegisterSet<'ctx> {
        let mut new_set = set.clone();

        match self {
            Instruction::AddInstruction { rs, rt, rd } => {
                let rs = set.get_register(rs);
                let rt = set.get_register(rt);
                new_set.set_register(rd, rs + rt);

                new_set
            }
        }
    }
}

struct InstructionPacket {
    slot1: Instruction,
    slot2: Instruction,
    slot3: Instruction,
    slot4: Instruction,
}
