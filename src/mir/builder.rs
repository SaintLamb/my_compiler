use crate::mir::id::{BlockId, FrameId, VRegId};
use crate::mir::function::{MirFunction, StackSlotUsage};
use crate::mir::instr::MirInstr;
use crate::mir::opcode::{Backend, OpCode};
use crate::mir::operand::MirOperand;
use crate::types::Type;

pub struct MirBuilder<'a, B: Backend> {
    func: &'a mut MirFunction<B>,
}

impl<'a, B: Backend> MirBuilder<'a, B> {
    pub fn new(func: &'a mut MirFunction<B>) -> Self {
        MirBuilder { func }
    }

    pub fn func(&self) -> &MirFunction<B> { self.func }
    pub fn new_block(&mut self) -> BlockId { self.func.new_block() }
    pub fn new_vreg(&mut self, ty: Type) -> VRegId { self.func.new_vreg(ty) }
    pub fn new_arg(&mut self, ty: Type) -> VRegId { self.func.new_arg(ty) }

    pub fn push(&mut self, block: BlockId, opcode: B::OpCode, operands: Vec<MirOperand>) {
        if opcode.is_terminator() && !self.func.exit_blocks.contains(&block) {
            self.func.exit_blocks.push(block);
        }
        self.func.block_mut(block).instrs.push(MirInstr::new(opcode, operands));
    }

    pub fn create_slot(&mut self, size: u32, align: u32, usage: StackSlotUsage) -> FrameId {
        self.func.frame.create_slot(size, align, usage)
    }

    pub fn add_edge(&mut self, from: BlockId, to: BlockId) {
        self.func.block_mut(from).succs.push(to);
        self.func.block_mut(to).preds.push(from);
    }

    pub fn set_entry(&mut self, block: BlockId) { self.func.entry = block; }
    pub fn set_is_leaf(&mut self, leaf: bool) { self.func.is_leaf = leaf; }
}