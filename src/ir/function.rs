use crate::id::IndexVec;
use crate::ir::id::{BlockId, InstId, ValueId};
use crate::ir::instr::IrInstr;
use crate::support::Symbol;
use crate::types::Type;

#[derive(Clone, Debug)]
pub struct Phi {
    pub dest: ValueId,
    pub incoming: Vec<(BlockId, ValueId)>,
}

#[derive(Clone, Debug, Default)]
pub struct IrBlock {
    pub phis: Vec<Phi>,
    pub instrs: Vec<IrInstr>,
    pub preds: Vec<BlockId>,
    pub succs: Vec<BlockId>,
}

#[derive(Clone, Debug)]
pub struct ValueInfo {
    pub ty: Type,
    pub def: Option<(BlockId, InstId)>,
}

#[derive(Clone, Debug)]
pub struct IrFunction {
    pub name: Symbol,
    pub blocks: IndexVec<BlockId, IrBlock>,
    pub values: IndexVec<ValueId, ValueInfo>,
    pub args: Vec<ValueId>,
    pub entry: BlockId,
    pub exit_blocks: Vec<BlockId>,
}

impl IrFunction {
    pub fn new(name: Symbol) -> Self {
        let mut f = IrFunction {
            name,
            blocks: IndexVec::new(),
            values: IndexVec::new(),
            args: Vec::new(),
            entry: BlockId::new(0),
            exit_blocks: Vec::new(),
        };
        f.entry = f.new_block();
        f
    }

    pub fn new_block(&mut self) -> BlockId {
        self.blocks.push(IrBlock::default())
    }

    pub fn new_value(&mut self, ty: Type) -> ValueId {
        self.values.push(ValueInfo { ty, def: None })
    }

    pub fn new_arg(&mut self, ty: Type) -> ValueId {
        let v = self.new_value(ty);
        self.args.push(v);
        v
    }

    pub fn block(&self, id: BlockId) -> &IrBlock { &self.blocks[id] }
    pub fn block_mut(&mut self, id: BlockId) -> &mut IrBlock { &mut self.blocks[id] }
    pub fn value(&self, id: ValueId) -> &ValueInfo { &self.values[id] }
    pub fn value_mut(&mut self, id: ValueId) -> &mut ValueInfo { &mut self.values[id] }
    pub fn is_arg(&self, v: ValueId) -> bool { self.args.contains(&v) }
}