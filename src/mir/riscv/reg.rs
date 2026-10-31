#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum RiscvReg {
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30, X31,
}

impl RiscvReg {
    pub const ZERO: Self = Self::X0;
    pub const RA: Self = Self::X1;
    pub const SP: Self = Self::X2;
    pub const A0: Self = Self::X10;
    pub const A1: Self = Self::X11;
    pub const A2: Self = Self::X12;
    pub const A3: Self = Self::X13;
    pub const A4: Self = Self::X14;
    pub const A5: Self = Self::X15;
    pub const A6: Self = Self::X16;
    pub const A7: Self = Self::X17;

    pub fn raw(self) -> u16 {
        self as u16
    }
}