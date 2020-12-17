use z3::{
    ast::{Array, Dynamic, BV},
    Context, FuncDecl, Sort,
};

pub enum Memory<'ctx> {
    ArrayMemory { array: Array<'ctx> },
    FunctionMemory { func: FuncDecl<'ctx> },
}

pub enum MemoryType {
    ArrayMemory,
    FunctionMemory,
}

impl<'ctx> Memory<'ctx> {
    pub fn new(ctx: &'ctx Context, typ: MemoryType) -> Self {
        match typ {
            MemoryType::ArrayMemory => Self::ArrayMemory {
                array: Array::new_const(
                    ctx,
                    "Memory",
                    &Sort::bitvector(ctx, 32),
                    &Sort::bitvector(ctx, 8),
                ),
            },
            MemoryType::FunctionMemory => {
                // TODO: Handle contraints
                Self::FunctionMemory {
                    func: FuncDecl::new(
                        ctx,
                        "Memory",
                        &[&Sort::bitvector(ctx, 32)],
                        &Sort::bitvector(ctx, 8),
                    ),
                }
            }
        }
    }

    pub fn load_byte(&self, idx: &BV<'ctx>) -> BV<'ctx> {
        match self {
            Memory::ArrayMemory { array } => array.select(idx).as_bv().unwrap(),
            Memory::FunctionMemory { func } => {
                func.apply(&[&Dynamic::from_ast(idx)]).as_bv().unwrap()
            }
        }
    }

    pub fn store_byte(&mut self, idx: &BV<'ctx>, val: &BV<'ctx>) {
        match self {
            Memory::ArrayMemory { ref mut array } => {
                *array = array.store(idx, val);
            }
            Memory::FunctionMemory { func } => unimplemented!(),
        }
    }
}
