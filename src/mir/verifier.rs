use crate::mir::function::MirFunction;
use crate::mir::id::{BlockId, VRegId, Idx};
use crate::mir::operand::{OpFlags, Operand, Reg};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    PostLowering,
    PostLegalize,
    PostRegBank,
    PostIsel,
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
    ClassMissingAfterRegBank(VRegId),
    SuccPredMismatch(BlockId),
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::VRegMultipleDefs(v) => {
                write!(f, "vreg %{} has multiple definitions", v.raw())
            }
            VerifyError::VRegNoDef(v) => {
                write!(f, "vreg %{} has no definition", v.raw())
            }
            VerifyError::VRegUndefinedUse(v) => {
                write!(f, "vreg %{} used but never defined", v.raw())
            }
            VerifyError::BlockNoTerminator(b) => {
                write!(f, "block bb{} has no terminator", b.raw())
            }
            VerifyError::PhiPredMismatch(b) => {
                write!(f, "phi in bb{} doesn't match preds", b.raw())
            }
            VerifyError::ClassMissingAfterRegBank(v) => {
                write!(f, "vreg %{} has no class after regbank", v.raw())
            }
            VerifyError::SuccPredMismatch(b) => {
                write!(f, "bb{} succ/pred mismatch", b.raw())
            }
        }
    }
}

impl std::error::Error for VerifyError {}

pub fn verify(f: &MirFunction, stage: Stage) -> Result<(), Vec<VerifyError>> {
    let mut errors = Vec::new();

    let mut def_counts = vec![0u32; f.vregs.len()];
    let mut use_counts = vec![0u32; f.vregs.len()];

    for block in f.blocks.iter() {
        for phi in &block.phis {
            def_counts[phi.dest.index()] += 1;
            for (_, val) in &phi.incoming {
                use_counts[val.index()] += 1;
            }
        }
        for instr in &block.instrs {
            for op in &instr.operands {
                if let Operand::Reg(Reg::Virtual(v), flags) = op {
                    if flags.contains(OpFlags::DEF) {
                        def_counts[v.index()] += 1;
                    }
                    if flags.contains(OpFlags::USE) {
                        use_counts[v.index()] += 1;
                    }
                }
            }
        }
    }

    for (v_id, _info) in f.vregs.iter_enumerated() {
        let defs = def_counts[v_id.index()];
        if defs > 1 {
            errors.push(VerifyError::VRegMultipleDefs(v_id));
        } else if defs == 0 {
            errors.push(VerifyError::VRegNoDef(v_id));
        }
    }

    for (block_id, block) in f.blocks.iter_enumerated() {
        if !block.instrs.last().map_or(false, |i| i.is_terminator()) {
            errors.push(VerifyError::BlockNoTerminator(block_id));
        }
    }

    for (block_id, block) in f.blocks.iter_enumerated() {
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
    }

    for (block_id, block) in f.blocks.iter_enumerated() {
        for phi in &block.phis {
            if phi.incoming.len() != block.preds.len() {
                errors.push(VerifyError::PhiPredMismatch(block_id));
                break;
            }
            if !phi
                .incoming
                .iter()
                .all(|(pred, _)| block.preds.contains(pred))
            {
                errors.push(VerifyError::PhiPredMismatch(block_id));
                break;
            }
        }
    }

    for (v_id, _info) in f.vregs.iter_enumerated() {
        if use_counts[v_id.index()] > 0 && def_counts[v_id.index()] == 0 {
            errors.push(VerifyError::VRegUndefinedUse(v_id));
        }
    }

    let need_class = matches!(
        stage,
        Stage::PostRegBank
            | Stage::PostIsel
            | Stage::PostRegAlloc
            | Stage::PostFrameLayout
    );
    if need_class {
        for (v_id, info) in f.vregs.iter_enumerated() {
            if info.class.is_none() {
                errors.push(VerifyError::ClassMissingAfterRegBank(v_id));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}