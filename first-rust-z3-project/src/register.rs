use z3::{ast::BV, Context};

#[derive(Clone, Copy)]
pub enum Register {
    R(usize),
    C(usize),
    P(u32),
    SP,  // R29
    FP,  // R30
    LR,  // R31
    PC,  // C09
    USR, // C08
}

#[derive(Clone, Copy)]
pub enum Register64 {
    R(usize), // usize must be odd number
    C(usize), // usize must be odd number
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

    pub fn get_register(&self, r: Register) -> BV<'ctx> {
        match r {
            Register::R(v) => self.general[v],
            Register::C(v) => self.control[v],
            Register::P(v) => self.control[4].extract(8 * (v + 1) - 1, 8 * v),
            Register::SP => self.general[29],
            Register::FP => self.general[30],
            Register::LR => self.general[31],
            Register::PC => self.control[9],
            Register::USR => self.control[8],
        }
    }

    pub fn get_register_64(&self, rr: Register64) -> BV<'ctx> {
        match rr {
            Register64::R(v) => self.general[v].concat(&self.general[v - 1]),
            Register64::C(v) => self.control[v].concat(&self.control[v - 1]),
        }
    }

    pub fn set_register(&mut self, r: Register, val: BV<'ctx>) {
        match r {
            Register::R(v) => self.general[v] = val,
            Register::C(v) => self.control[v] = val,
            Register::SP => self.general[29] = val,
            Register::FP => self.general[30] = val,
            Register::LR => self.general[31] = val,
            Register::PC => self.control[9] = val,
            Register::USR => self.control[8] = val,
        };
    }

    pub fn set_register_64(&mut self, rr: Register64, val: BV<'ctx>) {
        match rr {
            Register64::R(v) => {
                self.general[v] = val.extract(63, 32);
                self.general[v - 1] = val.extract(31, 0);
            }
            Register64::C(v) => {
                self.control[v] = val.extract(63, 32);
                self.control[v - 1] = val.extract(31, 0);
            }
        }
    }
}
