use crate::mir::id::{BlockId, FrameId, IndexVec, InstrId, PRegId, VRegId};
use crate::mir::instr::{MirInstr, Phi};
use crate::mir::types::{RegClass, Type};
use crate::support::Symbol;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackSlotUsage {
    /// 传入参数（当前函数帧内的 incoming args）
    Arg,
    /// 传出参数区（调用其他函数时使用）
    CalleeArg,
    /// callee-saved 寄存器保存
    CalleeSave,
    /// 局部变量 / alloca 对象
    Local,
    /// 溢出槽
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
        usage: StackSlotUsage,
    ) -> FrameId {
        self.create_slot_with_arg_index(size, align, usage, None)
    }

    pub fn create_arg_slot(
        &mut self,
        size: u32,
        align: u32,
        usage: StackSlotUsage,
        arg_index: u32,
    ) -> FrameId {
        debug_assert!(
            matches!(usage, StackSlotUsage::Arg | StackSlotUsage::CalleeArg),
            "create_arg_slot 只用于 Arg / CalleeArg"
        );
        self.create_slot_with_arg_index(size, align, usage, Some(arg_index))
    }

    fn create_slot_with_arg_index(
        &mut self,
        size: u32,
        align: u32,
        usage: StackSlotUsage,
        arg_index: Option<u32>,
    ) -> FrameId {
        self.slots.push(StackSlot {
            size,
            align,
            offset: None,
            usage,
            arg_index,
        })
    }
}

#[derive(Clone, Debug)]
pub struct VRegInfo {
    pub ty: Type,
    pub class: Option<RegClass>,
    pub def: Option<(BlockId, InstrId)>,
    pub assigned: Option<PRegId>,
}

#[derive(Clone, Debug, Default)]
pub struct MirBlock {
    pub phis: Vec<Phi>,
    pub instrs: Vec<MirInstr>,
    pub preds: Vec<BlockId>,
    pub succs: Vec<BlockId>,
}

impl MirBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_before_terminator(&mut self, instr: MirInstr) {
        debug_assert!(
            !instr.is_terminator(),
            "insert_before_terminator 不接受 terminator"
        );
        let pos = self
            .instrs
            .iter()
            .position(|i| i.is_terminator())
            .unwrap_or(self.instrs.len());
        self.instrs.insert(pos, instr);
    }

    pub fn terminator(&self) -> Option<&MirInstr> {
        self.instrs.last().filter(|i| i.is_terminator())
    }
}

#[derive(Clone, Debug)]
pub struct MirFunction {
    pub name: Symbol,
    pub blocks: IndexVec<BlockId, MirBlock>,
    pub vregs: IndexVec<VRegId, VRegInfo>,
    pub args: Vec<VRegId>,
    pub frame: FrameInfo,
    pub entry: BlockId,
    pub is_leaf: bool,
    pub is_entry: bool,
}

impl MirFunction {
    pub fn new(name: Symbol) -> Self {
        let mut f = MirFunction {
            name,
            blocks: IndexVec::new(),
            vregs: IndexVec::new(),
            args: Vec::new(),
            frame: FrameInfo::default(),
            entry: BlockId::new(0),
            is_leaf: true,
            is_entry: false,
        };
        f.entry = f.new_block();
        f
    }

    pub fn new_block(&mut self) -> BlockId {
        self.blocks.push(MirBlock::new())
    }

    pub fn new_vreg(&mut self, ty: Type) -> VRegId {
        self.vregs.push(VRegInfo {
            ty,
            class: None,
            def: None,
            assigned: None,
        })
    }
    
    pub fn new_arg_vreg(&mut self, ty: Type) -> VRegId {
        let v = self.new_vreg(ty);
        self.args.push(v);
        v
    }

    pub fn is_arg(&self, v: VRegId) -> bool {
        self.args.contains(&v)
    }

    pub fn block(&self, id: BlockId) -> &MirBlock {
        &self.blocks[id]
    }

    pub fn block_mut(&mut self, id: BlockId) -> &mut MirBlock {
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
}