use crate::ir::basic_block::{BasicBlock, BasicBlockBuilder};

#[non_exhaustive]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_builder<'ctx>(
        &'ctx self,
        basic_block: &'ctx mut BasicBlock<'ctx>,
    ) -> BasicBlockBuilder<'ctx> {
        BasicBlockBuilder::new(self, basic_block)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
