use crate::register::{Register, RegisterSet};

pub struct AddInstruction {
    pub rs: Register,
    pub rt: Register,
    pub rd: Register,
}

impl<'ctx> AddInstruction {
    pub fn apply(&self, set: &RegisterSet<'ctx>) -> RegisterSet<'ctx> {
        let mut new_set = set.clone();

        let rs = set.get_register(&self.rs);
        let rt = set.get_register(&self.rt);
        new_set.set_register(&self.rd, rs + rt);

        new_set
    }
}

trait Instruction {}

struct InstructionPacket<'a> {
    slot1: &'a dyn Instruction,
    slot2: &'a dyn Instruction,
    slot3: &'a dyn Instruction,
    slot4: &'a dyn Instruction,
}
