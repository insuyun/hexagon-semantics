extern crate z3;

mod instruction;
mod register;

use crate::instruction::{Instruction, InstructionPacket};
use crate::register::{CpuState, Register};
use z3::{Config, Context};

fn main() {
    let cfg = Config::default();
    let ctx = Context::new(&cfg);

    let mut cpu_state = CpuState::new(&ctx);

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

    cpu_state.apply_instruction_packet(packet);

    println!("{:?}", cpu_state.get_register(Register::R(2)));
}
