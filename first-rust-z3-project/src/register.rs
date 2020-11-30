use z3::{ast::BV, Context};

pub enum Register {
    R(usize),
    C(usize),
}

#[derive(Clone)]
pub struct RegisterSet<'ctx> {
    general: [BV<'ctx>; 32],
    control: [BV<'ctx>; 32],
}

impl<'ctx> RegisterSet<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        Self {
            general: [
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
                BV::fresh_const(&ctx, "R", 32),
            ],
            control: [
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
                BV::fresh_const(&ctx, "C", 32),
            ],
        }
    }

    pub fn get_register(&self, r: &Register) -> &BV<'ctx> {
        match r {
            Register::R(v) => &self.general[*v],
            Register::C(v) => &self.control[*v],
        }
    }

    pub fn set_register(&mut self, r: &Register, val: BV<'ctx>) {
        match r {
            Register::R(v) => self.general[*v] = val,
            Register::C(v) => self.control[*v] = val,
        };
    }
}
