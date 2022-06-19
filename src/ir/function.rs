use super::basic_block::BasicBlock;
use typed_arena::Arena;

pub struct Function {
    is_entry: bool,
    basic_blocks: Arena<BasicBlock>,
}

impl Function {}
