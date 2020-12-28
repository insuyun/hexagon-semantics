use z3::{
    ast::{Array, Ast, Bool, Dynamic, BV},
    Context, FuncDecl, Sort,
};

pub enum Memory<'ctx> {
    ArrayMemory {
        array: Array<'ctx>,
    },
    FunctionMemory {
        context: &'ctx Context,
        func: FuncDecl<'ctx>,
        constraints: Vec<Bool<'ctx>>,
    },
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
                    "ArrayMemory",
                    &Sort::bitvector(ctx, 32),
                    &Sort::bitvector(ctx, 8),
                ),
            },
            MemoryType::FunctionMemory => {
                // TODO: Handle contraints
                Self::FunctionMemory {
                    context: ctx,
                    func: FuncDecl::new(
                        ctx,
                        "FunctionMemory",
                        &[&Sort::bitvector(ctx, 32)],
                        &Sort::bitvector(ctx, 8),
                    ),
                    constraints: Vec::new(),
                }
            }
        }
    }

    pub fn load_byte(&self, idx: &BV<'ctx>) -> BV<'ctx> {
        match self {
            Memory::ArrayMemory { array } => array.select(idx).as_bv().unwrap(),
            Memory::FunctionMemory {
                context,
                func,
                constraints,
            } => func.apply(&[&Dynamic::from_ast(idx)]).as_bv().unwrap(),
        }
    }

    pub fn store_byte(&mut self, idx: &BV<'ctx>, val: &BV<'ctx>) {
        match self {
            Memory::ArrayMemory { ref mut array } => {
                *array = array.store(idx, val);
            }
            Memory::FunctionMemory {
                context,
                func,
                constraints,
            } => {
                let new_func = FuncDecl::new(
                    context,
                    "FunctionMemory",
                    &[&Sort::bitvector(context, 32)],
                    &Sort::bitvector(context, 8),
                );

                // new_func(x) == If(x == idx, val, func(x)))
                let x = BV::fresh_const(&context, "FunctionMemory_tmp", 32);
                let y = func.apply(&[&Dynamic::from_ast(&x)]).as_bv().unwrap();
                let new_constraint = new_func
                    .apply(&[&Dynamic::from_ast(&x)])
                    .as_bv()
                    .unwrap()
                    ._eq(&x._eq(idx).ite(val, &y));

                constraints.push(new_constraint);
                *func = new_func;
                // tmp3._eq();
            }
        }
    }
}
