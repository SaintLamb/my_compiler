#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ArmReg {
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30,
    Sp,
    V0, V1, V2, V3, V4, V5, V6, V7,
    V8, V9, V10, V11, V12, V13, V14, V15,
    V16, V17, V18, V19, V20, V21, V22, V23,
    V24, V25, V26, V27, V28, V29, V30, V31,
}

impl ArmReg {
    pub const FP: Self = Self::X29;
    pub const LR: Self = Self::X30;

    pub fn raw(self) -> u16 {
        self as u16
    }
}