use crate::mir::id::{BlockId, FrameId, PRegId, VRegId};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct OpFlags: u8 {
        const DEF      = 1 << 0;
        const USE      = 1 << 1;
        const IMPLICIT = 1 << 2;
        const CLOBBER  = 1 << 3;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Reg {
    Virtual(VRegId),
    Physical(PRegId),
    Stack(FrameId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MirOperand {
    Reg(Reg, OpFlags),
    Imm(i64, OpFlags),
    FImm(f64, OpFlags),
    Block(BlockId, OpFlags),
}

impl MirOperand {
    pub fn flags(&self) -> OpFlags {
        match self {
            MirOperand::Reg(_, f)
            | MirOperand::Imm(_, f)
            | MirOperand::FImm(_, f)
            | MirOperand::Block(_, f) => *f,
        }
    }

    pub fn vreg_def(v: VRegId) -> Self { 
        MirOperand::Reg(Reg::Virtual(v), OpFlags::DEF) 
    }

    pub fn vreg_use(v: VRegId) -> Self { 
        MirOperand::Reg(Reg::Virtual(v), OpFlags::USE) 
    }

    pub fn preg_def(p: PRegId) -> Self { 
        MirOperand::Reg(Reg::Physical(p), OpFlags::DEF) 
    }

    pub fn preg_use(p: PRegId) -> Self { 
        MirOperand::Reg(Reg::Physical(p), OpFlags::USE) 
    }

    pub fn stack_def(f: FrameId) -> Self { 
        MirOperand::Reg(Reg::Stack(f), OpFlags::DEF) 
    }

    pub fn stack_use(f: FrameId) -> Self { 
        MirOperand::Reg(Reg::Stack(f), OpFlags::USE) 
    }

    pub fn imm_use(v: i64) -> Self { 
        MirOperand::Imm(v, OpFlags::USE) 
    }

    pub fn block_use(b: BlockId) -> Self { 
        MirOperand::Block(b, OpFlags::USE) 
    }
}