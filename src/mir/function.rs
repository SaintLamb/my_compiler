use crate::id::IndexVec;
use crate::mir::id::{BlockId, FrameId, PRegId, VRegId};
use crate::mir::instr::MirInstr;
use crate::mir::opcode::Backend;
use crate::support::Symbol;
use crate::types::{RegClass, Type};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackSlotUsage {
    Arg,
    CalleeArg,
    CalleeSave,
    Local,
    Spill,
}

#[derive(Clone, Debug)]
pub struct StackSlot {
    pub size: u32,
    pub align: u32,
    pub offset: Option<i32>,
    pub usage: StackSlotUsage,
    pub arg_index: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct FrameInfo {
    pub slots: IndexVec<FrameId, StackSlot>,
    pub callee_saved: Vec<PRegId>,
    pub max_outgoing_args: u32,
    pub frame_size: Option<u32>,
    pub stack_align: u32,
}

impl Default for FrameInfo {
    fn default() -> Self {
        FrameInfo {
            slots: IndexVec::new(),
            callee_saved: Vec::new(),
            max_outgoing_args: 0,
            frame_size: None,
            stack_align: 16,
        }
    }
}

impl FrameInfo {
    pub fn create_slot(
        &mut self, 
        size: u32, 
        align: u32, 
        usage: StackSlotUsage
    ) -> FrameId {
        self.slots.push(StackSlot { 
            size, 
            align, 
            offset: None, 
            usage, arg_index: None 
        })
    }
}

#[derive(Clone, Debug)]
pub struct Phi {
    pub dest: VRegId,
    pub incoming: Vec<(BlockId, VRegId)>,
}

#[derive(Clone, Debug)]
pub struct VRegInfo {
    pub ty: Type,
    pub class: Option<RegClass>,
    pub assigned: Option<PRegId>,
}

#[derive(Clone, Debug)]
pub struct MirBlock<B: Backend> {
    pub phis: Vec<Phi>,
    pub instrs: Vec<MirInstr<B>>,
    pub preds: Vec<BlockId>,
    pub succs: Vec<BlockId>,
}

#[derive(Clone, Debug)]
pub struct MirFunction<B: Backend> {
    pub name: Symbol,
    pub blocks: IndexVec<BlockId, MirBlock<B>>,
    pub vregs: IndexVec<VRegId, VRegInfo>,
    pub frame: FrameInfo,
    pub args: Vec<VRegId>,
    pub entry: BlockId,
    pub exit_blocks: Vec<BlockId>,
    pub is_leaf: bool,
    pub is_entry: bool,
}

impl<B: Backend> MirFunction<B> {
    pub fn new(name: Symbol) -> Self {
        let mut f = MirFunction {
            name,
            blocks: IndexVec::new(),
            vregs: IndexVec::new(),
            frame: FrameInfo::default(),
            args: Vec::new(),
            entry: BlockId::new(0),
            exit_blocks: Vec::new(),
            is_leaf: true,
            is_entry: false,
        };
        f.entry = f.new_block();
        f
    }

    pub fn new_block(&mut self) -> BlockId {
        self.blocks.push(MirBlock::default())
    }

    pub fn new_vreg(&mut self, ty: Type) -> VRegId {
        self.vregs.push(VRegInfo { ty, class: None, assigned: None })
    }

    pub fn new_arg(&mut self, ty: Type) -> VRegId {
        let v = self.new_vreg(ty);
        self.args.push(v);
        v
    }

    pub fn block(&self, id: BlockId) -> &MirBlock<B> { 
        &self.blocks[id] 
    }

    pub fn block_mut(&mut self, id: BlockId) -> &mut MirBlock<B> { 
        &mut self.blocks[id] 
    }

    pub fn vreg(&self, id: VRegId) -> &VRegInfo { 
        &self.vregs[id] 
    }

    pub fn vreg_mut(&mut self, id: VRegId) -> &mut VRegInfo { 
        &mut self.vregs[id] 
    }

    pub fn slot(&self, id: FrameId) -> &StackSlot { 
        &self.frame.slots[id] 
    }

    pub fn is_arg(&self, v: VRegId) -> bool { 
        self.args.contains(&v) 
    }
}

impl<B: Backend> Default for MirBlock<B> {
    fn default() -> Self {
        Self {
            phis: Vec::new(),
            instrs: Vec::new(),
            preds: Vec::new(),
            succs: Vec::new(),
        }
    }
}