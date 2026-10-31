use crate::mir::id::{BlockId, VRegId};
use crate::mir::operand::{OpFlags, Operand, Reg};
use crate::mir::opcode::GenericOp;

#[derive(Clone, Debug)]
pub struct MirInstr {
    pub opcode: GenericOp,
    pub operands: Vec<Operand>,
}

impl MirInstr {
    pub fn new(opcode: GenericOp, operands: Vec<Operand>) -> Self {
        MirInstr { opcode, operands }
    }

    pub fn def(&self) -> Option<&Operand> {
        self.operands
            .iter()
            .find(|o| o.flags().contains(OpFlags::DEF))
    }

    pub fn def_vreg(&self) -> Option<VRegId> {
        match self.def()? {
            Operand::Reg(Reg::Virtual(v), _) => Some(*v),
            _ => None,
        }
    }

    pub fn uses(&self) -> impl Iterator<Item = &Operand> + '_ {
        self.operands
            .iter()
            .filter(|o| o.flags().contains(OpFlags::USE))
    }

    pub fn use_vregs(&self) -> impl Iterator<Item = VRegId> + '_ {
        self.uses().filter_map(|o| match o {
            Operand::Reg(Reg::Virtual(v), _) => Some(*v),
            _ => None,
        })
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

#[derive(Clone, Debug)]
pub struct Phi {
    pub dest: VRegId,
    pub incoming: Vec<(BlockId, VRegId)>,
}

impl Phi {
    pub fn new(dest: VRegId) -> Self {
        Phi {
            dest,
            incoming: Vec::new(),
        }
    }

    pub fn add(&mut self, from: BlockId, val: VRegId) {
        self.incoming.push((from, val));
    }
}