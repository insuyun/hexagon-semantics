use std::fmt;
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

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::R(v) => write!(f, "R{}", v),
            Self::C(v) => write!(f, "C{}", v),
            Self::P(v) => write!(f, "P{}", v),
            Self::SP => write!(f, "SP"),
            Self::FP => write!(f, "FP"),
            Self::LR => write!(f, "LR"),
            Self::PC => write!(f, "PC"),
            Self::USR => write!(f, "USR"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Register64 {
    R(usize), // usize must be odd number
    C(usize), // usize must be odd number
}

impl fmt::Display for Register64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::R(v) => write!(f, "R{}:{}", v, v - 1),
            Self::C(v) => write!(f, "C{}:{}", v, v - 1),
        }
    }
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
            Register::R(v) => self.general[v].clone(),
            Register::C(v) => self.control[v].clone(),
            Register::P(v) => self.control[4].extract(8 * (v + 1) - 1, 8 * v),
            Register::SP => self.general[29].clone(),
            Register::FP => self.general[30].clone(),
            Register::LR => self.general[31].clone(),
            Register::PC => self.control[9].clone(),
            Register::USR => self.control[8].clone(),
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
            _ => unimplemented!(),
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
