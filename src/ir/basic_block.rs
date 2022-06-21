use super::{
    builder::context::Context,
    instruction::Instruction,
    variable::{AnyVar, BothIntAnyVar, PointerVar, UIntVar},
};

pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
}

impl BasicBlock {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

impl Default for BasicBlock {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BasicBlockBuilder<'ctx> {
    context: &'ctx Context,
    basic_block: &'ctx mut BasicBlock,
}

impl<'ctx> BasicBlockBuilder<'ctx> {
    pub(crate) fn new(context: &'ctx Context, basic_block: &'ctx mut BasicBlock) -> Self {
        Self {
            context,
            basic_block,
        }
    }

    pub fn build_add<T: BothIntAnyVar>(&mut self, a: T, b: T) -> T {
        let id = self.context.next_id();
        self.basic_block.push(Instruction::add(id, a.id(), b.id()));
        T::new(id)
    }

    pub fn build_mul<T: BothIntAnyVar>(&mut self, a: T, b: T) -> T {
        let id = self.context.next_id();
        self.basic_block.push(Instruction::mul(id, a.id(), b.id()));
        T::new(id)
    }

    pub fn build_sub<T: BothIntAnyVar>(&mut self, a: T, b: T) -> T {
        let id = self.context.next_id();
        self.basic_block.push(Instruction::sub(id, a.id(), b.id()));
        T::new(id)
    }

    pub fn build_div<T: BothIntAnyVar>(&mut self, a: T, b: T) -> T {
        let id = self.context.next_id();
        self.basic_block.push(Instruction::div(id, a.id(), b.id()));
        T::new(id)
    }

    pub fn build_finish(&mut self, offset: PointerVar, length: UIntVar<32>) {
        self.basic_block
            .push(Instruction::finish(offset.id(), length.id()));
    }
}
