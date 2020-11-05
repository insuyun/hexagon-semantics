extern crate z3;

use z3::{
    ast::BV,
    Config, Context
};

enum Register {
    R(usize),
    C(usize)
}

#[derive(Clone)]
struct RegisterSet<'ctx> {
    general: [BV<'ctx>; 32],
    control: [BV<'ctx>; 32],
}

impl<'ctx> RegisterSet<'ctx> {
    fn new(ctx: &'ctx Context) -> Self {
        Self {
            general: [
                BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32), BV::fresh_const(&ctx, "R", 32),
            ],
            control: [
                BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32), BV::fresh_const(&ctx, "C", 32),
            ]
        }
    }
    fn get_register(&self, r: &Register) -> &BV<'ctx> {
        match r {
            Register::R(v) => &self.general[*v],
            Register::C(v) => &self.control[*v]
        }
    }

    fn set_register(&mut self, r: &Register, val: BV<'ctx>) {
        match r {
            Register::R(v) => { self.general[*v] = val },
            Register::C(v) => { self.control[*v] = val },
        };
    }
}

struct AddInstruction {
    rs: Register,
    rt: Register,
    rd: Register
}

impl<'ctx> AddInstruction {
    fn apply(&self, set: &RegisterSet<'ctx>) -> RegisterSet<'ctx> {
        let mut new_set = set.clone();

        let rs = set.get_register(&self.rs);
        let rt = set.get_register(&self.rt);
        new_set.set_register(&self.rd, rs + rt);

        new_set
    }
}

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
