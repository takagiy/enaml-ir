use std::cell::Cell;

use crate::ir::variable::VarId;

#[non_exhaustive]
pub struct Context {
    next_id: Cell<VarId>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            next_id: Cell::new(0),
        }
    }

    pub fn next_id(&self) -> usize {
        self.next_id.replace(self.next_id.get() + 1)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
