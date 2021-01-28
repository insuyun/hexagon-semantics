extern crate z3;

mod cpu_state;
mod instruction;
mod memory;
mod register;
mod simulator;

use crate::cpu_state::CpuState;
use crate::instruction::{Alu32Instruction, Instruction, InstructionPacket};
use crate::register::Register;
use crate::simulator::run_simulator;
use z3::{Config, Context};

fn main() -> std::io::Result<()> {
    let cfg = Config::default();
    let ctx = Context::new(&cfg);

    let mut cpu_state = CpuState::new(&ctx);

    let inst = Instruction::Alu32(Alu32Instruction::Add {
        rs: Register::R(0),
        rt: Register::R(1),
        rd: Register::R(2),
    });

    let packet = InstructionPacket {
        slot1: inst,
        slot2: Instruction::Nop,
        slot3: Instruction::Nop,
        slot4: Instruction::Nop,
    };

    cpu_state.apply_instruction_packet(&packet);

    println!("{:?}", cpu_state.get_register(Register::R(2)));

    run_simulator(&packet)
}
