use crate::ir::id::{BlockId, InstId, ValueId};
use crate::ir::function::{IrFunction, Phi};
use crate::ir::instr::IrInstr;
use crate::ir::opcode::GenericOp;
use crate::ir::operand::Operand;
use crate::types::Type;

pub struct IrBuilder<'a> {
    func: &'a mut IrFunction,
}

impl<'a> IrBuilder<'a> {
    pub fn new(func: &'a mut IrFunction) -> Self {
        IrBuilder { func }
    }

    pub fn func(&self) -> &IrFunction { self.func }
    pub fn new_block(&mut self) -> BlockId { self.func.new_block() }
    pub fn new_value(&mut self, ty: Type) -> ValueId { self.func.new_value(ty) }
    pub fn new_arg(&mut self, ty: Type) -> ValueId { self.func.new_arg(ty) }

    pub fn push(
        &mut self,
        block: BlockId,
        opcode: GenericOp,
        operands: Vec<Operand>,
        result: Option<ValueId>,
    ) -> InstId {
        let idx = u32::try_from(self.func.block(block).instrs.len())
            .expect("block too many instructions");
        let inst_id = InstId::new(idx);

        if let Some(r) = result {
            debug_assert!(self.func.value(r).def.is_none(), "value {r:?} defined twice");
            self.func.value_mut(r).def = Some((block, inst_id));
        }

        if opcode == GenericOp::Ret && !self.func.exit_blocks.contains(&block) {
            self.func.exit_blocks.push(block);
        }

        let inst = IrInstr { opcode, operands, result };
        self.func.block_mut(block).instrs.push(inst);
        inst_id
    }

    pub fn add_phi(&mut self, block: BlockId, phi: Phi) {
        self.func.block_mut(block).phis.push(phi);
    }

    pub fn add_edge(&mut self, from: BlockId, to: BlockId) {
        self.func.block_mut(from).succs.push(to);
        self.func.block_mut(to).preds.push(from);
    }

    pub fn set_entry(&mut self, block: BlockId) {
        self.func.entry = block;
    }
}