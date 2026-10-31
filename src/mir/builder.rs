use crate::mir::function::{MirFunction, StackSlotUsage};
use crate::mir::id::{BlockId, FrameId, InstId, VRegId};
use crate::mir::instr::{MirInstr, Phi};
use crate::mir::operand::{OpFlags, Operand, Reg};
use crate::mir::opcode::GenericOp;
use crate::mir::types::Type;

pub struct Builder<'a> {
    func: &'a mut MirFunction,
}

impl<'a> Builder<'a> {
    pub fn new(func: &'a mut MirFunction) -> Self {
        Builder { func }
    }

    pub fn func(&self) -> &MirFunction {
        self.func
    }

    pub fn new_block(&mut self) -> BlockId {
        self.func.new_block()
    }

    pub fn new_vreg(&mut self, ty: Type) -> VRegId {
        self.func.new_vreg(ty)
    }

    pub fn push(
        &mut self,
        block: BlockId,
        opcode: GenericOp,
        operands: Vec<Operand>,
    ) -> InstId {
        let inst_idx = u32::try_from(self.func.block(block).instrs.len())
            .expect("block has too many instructions");
        let inst_id = InstId::new(inst_idx);

        for op in &operands {
            if op.flags().contains(OpFlags::DEF) {
                if let Operand::Reg(Reg::Virtual(v), _) = op {
                    debug_assert!(
                        self.func.vreg(*v).def.is_none(),
                        "vreg {v:?} defined twice"
                    );
                    self.func.vreg_mut(*v).def = Some((block, inst_id));
                }
            }
        }

        if opcode == GenericOp::Call {
            self.func.is_leaf = false;
        }

        let inst = MirInstr { opcode, operands };
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

    pub fn create_slot(
        &mut self,
        size: u32,
        align: u32,
        usage: StackSlotUsage,
    ) -> FrameId {
        self.func.frame.create_slot(size, align, usage)
    }

    pub fn create_arg_slot(
        &mut self,
        size: u32,
        align: u32,
        usage: StackSlotUsage,
        arg_index: u32,
    ) -> FrameId {
        self.func.frame.create_arg_slot(size, align, usage, arg_index)
    }

    pub fn set_entry(&mut self, block: BlockId) {
        self.func.entry = block;
    }

    pub fn set_is_leaf(&mut self, leaf: bool) {
        self.func.is_leaf = leaf;
    }

    pub fn set_is_entry(&mut self, entry: bool) {
        self.func.is_entry = entry;
    }
}