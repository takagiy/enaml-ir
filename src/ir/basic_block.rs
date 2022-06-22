use id_arena::Arena;

use super::{
    instruction::Instruction,
    variable::{AnyVar, BothIntAnyVar, PointerVar, Type, UIntVar, Var},
};

pub struct BasicBlock {
    pub instructions: Arena<Var>,
}

impl BasicBlock {
    pub fn new() -> Self {
        Self {
            instructions: Arena::new(),
        }
    }

    fn push_none(&mut self, instruction: Instruction) {
        self.instructions.alloc(Var {
            source: instruction,
            ty: Type::None,
        });
    }

    fn push<T: AnyVar>(&mut self, instruction: Instruction) -> T {
        let id = self.instructions.alloc(Var {
            source: instruction,
            ty: Type::Pointer,
        });
        T::new(id)
    }
}

impl Default for BasicBlock {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BasicBlockBuilder<'bb> {
    basic_block: &'bb mut BasicBlock,
}

impl<'bb> BasicBlockBuilder<'bb> {
    pub(crate) fn new(basic_block: &'bb mut BasicBlock) -> Self {
        Self { basic_block }
    }

    pub fn build_add<T: BothIntAnyVar>(&mut self, a: T, b: T) -> T {
        self.basic_block.push(Instruction::add(a.into(), b.into()))
    }

    pub fn build_mul<T: BothIntAnyVar>(&mut self, a: T, b: T) -> T {
        self.basic_block.push(Instruction::mul(a.into(), b.into()))
    }

    pub fn build_sub<T: BothIntAnyVar>(&mut self, a: T, b: T) -> T {
        self.basic_block.push(Instruction::sub(a.into(), b.into()))
    }

    pub fn build_div<T: BothIntAnyVar>(&mut self, a: T, b: T) -> T {
        self.basic_block.push(Instruction::div(a.into(), b.into()))
    }

    pub fn build_finish(&mut self, offset: PointerVar, length: UIntVar<32>) {
        self.basic_block
            .push_none(Instruction::finish(offset.into(), length.into()));
    }
}
