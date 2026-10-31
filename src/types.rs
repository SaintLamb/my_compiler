#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Int(u16),
    Float(u16),
    Ptr(u16),
    Vec(u16, u16),
    CondFlag,
    Special,
}

impl Type {
    pub const I32: Type = Type::Int(32);
    pub const I64: Type = Type::Int(64);
    pub const F32: Type = Type::Float(32);
    pub const F64: Type = Type::Float(64);
    pub const PTR: Type = Type::Ptr(64);

    pub fn bit_width(self) -> u16 {
        match self {
            Type::Int(w) | Type::Float(w) | Type::Ptr(w) => w,
            Type::Vec(n, w) => n.checked_mul(w).expect("type size overflow"),
            Type::CondFlag | Type::Special => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RegClass {
    Gpr,
    Fpr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cond {
    Al,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl Cond {
    pub fn reverse(self) -> Cond {
        match self {
            Cond::Lt => Cond::Gt,
            Cond::Gt => Cond::Lt,
            Cond::Le => Cond::Ge,
            Cond::Ge => Cond::Le,
            _ => self,
        }
    }

    pub fn flip(self) -> Cond {
        match self {
            Cond::Eq => Cond::Ne,
            Cond::Ne => Cond::Eq,
            Cond::Lt => Cond::Ge,
            Cond::Gt => Cond::Le,
            Cond::Le => Cond::Gt,
            Cond::Ge => Cond::Lt,
            Cond::Al => Cond::Al,
        }
    }
}