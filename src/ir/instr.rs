use crate::ir::id::ValueId;
use crate::ir::opcode::GenericOp;
use crate::ir::operand::Operand;

#[derive(Clone, Debug)]
pub struct IrInstr {
    pub opcode: GenericOp,
    pub operands: Vec<Operand>,
    pub result: Option<ValueId>,
}

impl IrInstr {
    pub fn new(opcode: GenericOp, operands: Vec<Operand>, result: Option<ValueId>) -> Self {
        IrInstr { opcode, operands, result }
    }

    pub fn is_terminator(&self) -> bool {
        self.opcode.is_terminator()
    }

    pub fn branch_targets(&self) -> impl Iterator<Item = &Operand> + '_ {
        self.opcode
            .branch_target_operand_indices()
            .iter()
            .filter_map(move |&i| self.operands.get(i))
    }
}