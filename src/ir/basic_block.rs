use typed_arena::Arena;

use super::{
    builder::context::Context,
    instruction::Instruction,
    variable::{AnyVar, BothIntAnyVar, PointerVar, UIntVar, VarIdExt},
};

pub struct BasicBlock<'ctx> {
    pub instructions: Arena<Instruction<'ctx>>,
}

impl<'ctx> BasicBlock<'ctx> {
    pub fn new() -> Self {
        Self {
            instructions: Arena::new(),
        }
    }

    fn push(&mut self, instruction: Instruction<'ctx>) -> &mut Instruction<'ctx> {
        self.instructions.alloc(instruction)
    }
}

impl<'ctx> Default for BasicBlock<'ctx> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BasicBlockBuilder<'ctx> {
    context: &'ctx Context,
    basic_block: &'ctx mut BasicBlock<'ctx>,
}

impl<'ctx> BasicBlockBuilder<'ctx> {
    pub(crate) fn new(context: &'ctx Context, basic_block: &'ctx mut BasicBlock<'ctx>) -> Self {
        Self {
            context,
            basic_block,
        }
    }

    pub fn build_add<'a, T: BothIntAnyVar<'ctx>>(&'ctx mut self, a: T, b: T) -> T {
        self.basic_block
            .push(Instruction::add(a.id(), b.id()))
            .assume_unchecked()
    }

    pub fn build_mul<T: BothIntAnyVar<'ctx>>(&'ctx mut self, a: T, b: T) -> T {
        self.basic_block
            .push(Instruction::mul(a.id(), b.id()))
            .assume_unchecked()
    }

    pub fn build_sub<T: BothIntAnyVar<'ctx>>(&'ctx mut self, a: T, b: T) -> T {
        self.basic_block
            .push(Instruction::sub(a.id(), b.id()))
            .assume_unchecked()
    }

    pub fn build_div<T: BothIntAnyVar<'ctx>>(&'ctx mut self, a: T, b: T) -> T {
        self.basic_block
            .push(Instruction::div(a.id(), b.id()))
            .assume_unchecked()
    }

    pub fn build_finish(&'ctx mut self, offset: PointerVar<'ctx>, length: UIntVar<'ctx, 32>) {
        self.basic_block
            .push(Instruction::finish(offset.id(), length.id()));
    }
}
