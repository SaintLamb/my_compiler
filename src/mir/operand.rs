use crate::mir::id::{FrameId, PRegId, VRegId};
use crate::mir::types::{Cond, Type};
use crate::support::Symbol;
use bitflags::bitflags;

bitflags! {
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

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Reg(Reg, OpFlags),
    Imm(u64, OpFlags),  
    FImm(f64, OpFlags), 
    Reloc(Symbol, OpFlags), 
    Cond(Cond, OpFlags),
    Literal(Symbol, OpFlags), 
    Unused,
}

impl Operand {
    pub fn vreg_def(v: VRegId) -> Self {
        Operand::Reg(Reg::Virtual(v), OpFlags::DEF)
    }

    pub fn vreg_use(v: VRegId) -> Self {
        Operand::Reg(Reg::Virtual(v), OpFlags::USE)
    }

    pub fn preg_def(p: PRegId) -> Self {
        Operand::Reg(Reg::Physical(p), OpFlags::DEF)
    }

    pub fn preg_use(p: PRegId) -> Self {
        Operand::Reg(Reg::Physical(p), OpFlags::USE)
    }

    pub fn frame_use(f: FrameId) -> Self {
        Operand::Reg(Reg::Stack(f), OpFlags::USE)
    }

    pub fn imm(v: i64) -> Self {
        Operand::Imm(v as u64, OpFlags::USE)
    }
    pub fn imm_ex(v: u64) -> Self {
        Operand::Imm(v, OpFlags::USE)
    }

    pub fn fimm(v: f64) -> Self {
        Operand::FImm(v, OpFlags::USE)
    }

    pub fn block(s: Symbol) -> Self {
        Operand::Reloc(s, OpFlags::USE)
    }

    pub fn cond(c: Cond) -> Self {
        Operand::Cond(c, OpFlags::USE)
    }

    pub fn flags(&self) -> OpFlags {
        match self {
            Operand::Reg(_, f) 
            | Operand::Imm(_, f) 
            | Operand::FImm(_, f) 
            | Operand::Reloc(_, f) 
            | Operand::Cond(_, f) 
            | Operand::Literal(_, f) => *f,
            Operand::Unused => OpFlags::empty(),
        }
    }

    pub fn is_reg(&self) -> bool {
        matches!(self, Operand::Reg(..))
    }

    pub fn is_vreg(&self) -> bool {
        matches!(self, Operand::Reg(Reg::Virtual(_), _))
    }

    pub fn is_preg(&self) -> bool {
        matches!(self, Operand::Reg(Reg::Physical(_), _))
    }

    pub fn is_frame(&self) -> bool {
        matches!(self, Operand::Reg(Reg::Stack(_), _))
    }

    pub fn is_imm(&self) -> bool {
        matches!(self, Operand::Imm(..) | Operand::FImm(..))
    }

    pub fn as_vreg(&self) -> Option<VRegId> {
        match self {
            Operand::Reg(Reg::Virtual(v), _) => Some(*v),
            _ => None,
        }
    }

    pub fn as_block(&self) -> Option<&Symbol> {
        match self {
            Operand::Reloc(s, _) => Some(s),
            _ => None,
        }
    }

    pub fn imm_value(&self) -> Option<u64> {
        match self {
            Operand::Imm(v, _) => Some(*v),
            _ => None,
        }
    }
    
    pub fn type_hint(&self) -> Option<Type> {
        None 
    }
}