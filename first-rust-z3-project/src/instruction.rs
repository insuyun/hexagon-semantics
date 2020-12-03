use crate::register::{Register, RegisterSet};

pub enum Instruction {
    Nop,
    AddInstruction {
        rs: Register,
        rt: Register,
        rd: Register,
    },
}

impl<'ctx> Instruction {
    pub fn apply(&self, old_set: &RegisterSet<'ctx>, set: &mut RegisterSet<'ctx>) {
        match self {
            Instruction::AddInstruction { rs, rt, rd } => {
                let rs = old_set.get_register(rs);
                let rt = old_set.get_register(rt);
                set.set_register(rd, rs + rt);
            }
            Instruction::Nop => {}
        }
    }
}

pub struct InstructionPacket {
    pub slot1: Instruction,
    pub slot2: Instruction,
    pub slot3: Instruction,
    pub slot4: Instruction,
}

impl<'ctx> InstructionPacket {
    pub fn apply(&self, set: &RegisterSet<'ctx>) -> RegisterSet<'ctx> {
        let mut new_set = set.clone();

        self.slot1.apply(set, &mut new_set);
        self.slot2.apply(set, &mut new_set);
        self.slot3.apply(set, &mut new_set);
        self.slot4.apply(set, &mut new_set);

        new_set
    }
}
