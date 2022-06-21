use super::basic_block::BasicBlock;
use typed_arena::Arena;

pub struct Function {
    is_entry: bool,
    basic_blocks: Arena<BasicBlock>,
}

impl Function {
    // typed-arena safens this operation
    #[allow(clippy::mut_from_ref)]
    pub fn build_basic_block(&self) -> &mut BasicBlock {
        self.basic_blocks.alloc(BasicBlock::new())
    }
}
