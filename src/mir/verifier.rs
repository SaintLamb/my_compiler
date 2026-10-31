use crate::id::IndexVec;
use crate::mir::id::{BlockId, VRegId};
use crate::mir::function::MirFunction;
use crate::mir::opcode::{Backend, OpCode};
use crate::mir::operand::{MirOperand, OpFlags, Reg};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    PostLowering,
    PostRegAlloc,
    PostFrameLayout,
}

#[derive(Debug)]
pub enum VerifyError {
    VRegMultipleDefs(VRegId),
    VRegNoDef(VRegId),
    VRegUndefinedUse(VRegId),
    BlockNoTerminator(BlockId),
    PhiPredMismatch(BlockId),
    SuccPredMismatch(BlockId),
    VRegMissingAfterRegAlloc(VRegId),
    FrameSizeMissingAfterLayout,
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::VRegMultipleDefs(v) => write!(f, "vreg %{} has multiple defs", v.raw()),
            VerifyError::VRegNoDef(v) => write!(f, "vreg %{} has no def", v.raw()),
            VerifyError::VRegUndefinedUse(v) => write!(f, "vreg %{} used but never defined", v.raw()),
            VerifyError::BlockNoTerminator(b) => write!(f, "bb{} has no terminator", b.raw()),
            VerifyError::PhiPredMismatch(b) => write!(f, "phi in bb{} doesn't match preds", b.raw()),
            VerifyError::SuccPredMismatch(b) => write!(f, "bb{} succ/pred mismatch", b.raw()),
            VerifyError::VRegMissingAfterRegAlloc(v) => write!(f, "vreg %{} has no assigned reg", v.raw()),
            VerifyError::FrameSizeMissingAfterLayout => write!(f, "frame_size is None after frame layout"),
        }
    }
}

pub fn verify<B: Backend>(f: &MirFunction<B>, stage: Stage) -> Result<(), Vec<VerifyError>> {
    let mut errors = Vec::new();

    let mut def_counts: IndexVec<VRegId, u32> = IndexVec::with_len(f.vregs.len(), 0);
    let mut use_counts: IndexVec<VRegId, u32> = IndexVec::with_len(f.vregs.len(), 0);

    for block in f.blocks.iter() {
        for phi in &block.phis {
            def_counts[phi.dest] += 1;
            for (_, v) in &phi.incoming {
                use_counts[*v] += 1;
            }
        }
        for inst in &block.instrs {
            for op in &inst.operands {
                if let MirOperand::Reg(Reg::Virtual(v), flags) = op {
                    if flags.contains(OpFlags::DEF) { def_counts[*v] += 1; }
                    if flags.contains(OpFlags::USE) { use_counts[*v] += 1; }
                }
            }
        }
    }

    for (v_id, _) in f.vregs.iter_enumerated() {
        let defs = def_counts[v_id];
        if defs > 1 {
            errors.push(VerifyError::VRegMultipleDefs(v_id));
        } else if defs == 0 && !f.is_arg(v_id) {
            errors.push(VerifyError::VRegNoDef(v_id));
        }
    }

    for (v_id, _) in f.vregs.iter_enumerated() {
        if use_counts[v_id] > 0 && def_counts[v_id] == 0 && !f.is_arg(v_id) {
            errors.push(VerifyError::VRegUndefinedUse(v_id));
        }
    }

    for (block_id, block) in f.blocks.iter_enumerated() {
        if !block.instrs.last().map_or(false, |i| i.is_terminator()) {
            errors.push(VerifyError::BlockNoTerminator(block_id));
        }
        for succ in &block.succs {
            if !f.block(*succ).preds.contains(&block_id) {
                errors.push(VerifyError::SuccPredMismatch(block_id));
                break;
            }
        }
        for pred in &block.preds {
            if !f.block(*pred).succs.contains(&block_id) {
                errors.push(VerifyError::SuccPredMismatch(block_id));
                break;
            }
        }
        for phi in &block.phis {
            if phi.incoming.len() != block.preds.len()
                || !phi.incoming.iter().all(|(p, _)| block.preds.contains(p))
            {
                errors.push(VerifyError::PhiPredMismatch(block_id));
                break;
            }
        }
    }

    if stage >= Stage::PostRegAlloc {
        for (v_id, info) in f.vregs.iter_enumerated() {
            if info.assigned.is_none() && !f.is_arg(v_id) {
                errors.push(VerifyError::VRegMissingAfterRegAlloc(v_id));
            }
        }
    }

    if stage == Stage::PostFrameLayout && f.frame.frame_size.is_none() {
        errors.push(VerifyError::FrameSizeMissingAfterLayout);
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}