extern crate z3;

mod instruction;
mod register;

use z3::{Config, Context};

use crate::instruction::{Instruction, InstructionPacket};
use crate::register::{Register, RegisterSet};

fn main() {
    let cfg = Config::default();
    let ctx = Context::new(&cfg);

    let set = RegisterSet::new(&ctx);

    let inst = Instruction::Add {
        rs: Register::R(0),
        rt: Register::R(1),
        rd: Register::R(2),
    };

    let packet = InstructionPacket {
        slot1: inst,
        slot2: Instruction::Nop,
        slot3: Instruction::Nop,
        slot4: Instruction::Nop,
    };

    let set = packet.apply(&ctx, &set);

    println!("{:?}", set.get_register(Register::R(2)));
}
