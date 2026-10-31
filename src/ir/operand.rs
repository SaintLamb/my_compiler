use crate::ir::id::{BlockId, ValueId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operand {
    Value(ValueId),
    Imm(i64),
    Block(BlockId),
}

impl Operand {
    pub fn value(v: ValueId) -> Self { Operand::Value(v) }
    pub fn imm(v: i64) -> Self { Operand::Imm(v) }
    pub fn block(b: BlockId) -> Self { Operand::Block(b) }
}