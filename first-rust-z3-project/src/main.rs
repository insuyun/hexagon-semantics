extern crate z3;

mod instruction;
mod register;

use z3::{Config, Context};

use crate::instruction::AddInstruction;
use crate::register::{Register, RegisterSet};

fn main() {
    let cfg = Config::default();
    let ctx = Context::new(&cfg);

    let set = RegisterSet::new(&ctx);

    let inst = AddInstruction {
        rs: Register::R(0),
        rt: Register::R(1),
        rd: Register::R(2),
    };

    let set = inst.apply(&set);

    println!("{:?}", set.get_register(&Register::R(2)));
}
