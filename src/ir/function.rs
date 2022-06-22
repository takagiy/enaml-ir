use super::basic_block::BasicBlock;
use typed_arena::Arena;

pub struct Function<'ctx> {
    is_entry: bool,
    basic_blocks: Arena<BasicBlock<'ctx>>,
}

impl<'ctx> Function<'ctx> {
    // typed-arena safens this operation
    #[allow(clippy::mut_from_ref)]
    pub fn add_basic_block(&'ctx self) -> &mut BasicBlock<'ctx> {
        self.basic_blocks.alloc(BasicBlock::new())
    }
}
