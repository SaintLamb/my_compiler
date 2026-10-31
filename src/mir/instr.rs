use crate::mir::opcode::{Backend, OpCode};
use crate::mir::operand::MirOperand;

#[derive(Clone, Debug)]
pub struct MirInstr<B: Backend> {
    pub opcode: B::OpCode,
    pub operands: Vec<MirOperand>,
}

impl<B: Backend> MirInstr<B> {
    pub fn new(opcode: B::OpCode, operands: Vec<MirOperand>) -> Self {
        MirInstr { opcode, operands }
    }

    pub fn is_terminator(&self) -> bool {
        self.opcode.is_terminator()
    }

    pub fn branch_targets(&self) -> impl Iterator<Item = &MirOperand> + '_ {
        self.opcode
            .branch_target_operand_indices()
            .iter()
            .filter_map(move |&i| self.operands.get(i))
    }
}