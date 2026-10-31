use crate::id::IndexVec;
use crate::ir::id::{BlockId, ValueId};
use crate::ir::function::IrFunction;
use crate::ir::operand::Operand;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    PostLowering,
}

#[derive(Debug)]
pub enum VerifyError {
    ValueMultipleDefs(ValueId),
    ValueNoDef(ValueId),
    ValueUndefinedUse(ValueId),
    BlockNoTerminator(BlockId),
    PhiPredMismatch(BlockId),
    SuccPredMismatch(BlockId),
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::ValueMultipleDefs(v) => write!(f, "value %{} has multiple defs", v.raw()),
            VerifyError::ValueNoDef(v) => write!(f, "value %{} has no def", v.raw()),
            VerifyError::ValueUndefinedUse(v) => write!(f, "value %{} used but never defined", v.raw()),
            VerifyError::BlockNoTerminator(b) => write!(f, "bb{} has no terminator", b.raw()),
            VerifyError::PhiPredMismatch(b) => write!(f, "phi in bb{} doesn't match preds", b.raw()),
            VerifyError::SuccPredMismatch(b) => write!(f, "bb{} succ/pred mismatch", b.raw()),
        }
    }
}

impl std::error::Error for VerifyError {}

pub fn verify(f: &IrFunction, _stage: Stage) -> Result<(), Vec<VerifyError>> {
    let mut errors = Vec::new();

    let mut def_counts: IndexVec<ValueId, u32> = IndexVec::with_len(f.values.len(), 0);
    let mut use_counts: IndexVec<ValueId, u32> = IndexVec::with_len(f.values.len(), 0);

    for block in f.blocks.iter() {
        for phi in &block.phis {
            def_counts[phi.dest] += 1;
            for (_, v) in &phi.incoming {
                use_counts[*v] += 1;
            }
        }
        for inst in &block.instrs {
            if let Some(r) = inst.result {
                def_counts[r] += 1;
            }
            for op in &inst.operands {
                if let Operand::Value(v) = op {
                    use_counts[*v] += 1;
                }
            }
        }
    }

    for (v_id, _) in f.values.iter_enumerated() {
        let defs = def_counts[v_id];
        if defs > 1 {
            errors.push(VerifyError::ValueMultipleDefs(v_id));
        } else if defs == 0 && !f.is_arg(v_id) {
            errors.push(VerifyError::ValueNoDef(v_id));
        }
    }

    for (v_id, _) in f.values.iter_enumerated() {
        if use_counts[v_id] > 0 && def_counts[v_id] == 0 && !f.is_arg(v_id) {
            errors.push(VerifyError::ValueUndefinedUse(v_id));
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
            if phi.incoming.len() != block.preds.len()
                || !phi.incoming.iter().all(|(p, _)| block.preds.contains(p))
            {
                errors.push(VerifyError::PhiPredMismatch(block_id));
                break;
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}