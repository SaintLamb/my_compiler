pub trait Opcode: Copy + std::fmt::Debug + Eq + std::hash::Hash + 'static {
    fn name(self) -> &'static str;
    fn is_terminator(self) -> bool;
    fn branch_target_operand_indices(self) -> &'static [usize];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GenericOp {
    // ---- 控制流 ----
    Br,
    BrCond,
    Ret,
    Phi,
    Call,

    // ---- 内存 ----
    Load,
    Store,
    FrameIndex,
    GlobalValue,

    // ---- 整数算术 ----
    Add,
    Sub,
    Mul,

    // ---- 位运算 ----
    And,
    Or,
    Xor,
    Shl,
    Lshr,
    Ashr,

    // ---- 有符号/无符号除余 ----
    Sdiv,
    Srem,
    Udiv,
    Urem,

    // ---- 浮点 ----
    Fadd,
    Fsub,
    Fmul,
    Fdiv,
    Fneg,

    // ---- 比较 ----
    Icmp,
    Fcmp,

    // ---- 类型转换 ----
    Trunc,
    Zext,
    Sext,
    Fptrunc,
    Fpext,
    Fptosi,
    Sitofp,

    // ---- 值操作 ----
    Copy,
    Select,
    Constant,
}

impl GenericOp {
    pub fn name(self) -> &'static str {
        match self {
            GenericOp::Br => "br",
            GenericOp::BrCond => "brcond",
            GenericOp::Ret => "ret",
            GenericOp::Phi => "phi",
            GenericOp::Call => "call",
            GenericOp::Load => "load",
            GenericOp::Store => "store",
            GenericOp::FrameIndex => "frame_index",
            GenericOp::GlobalValue => "global_value",
            GenericOp::Add => "add",
            GenericOp::Sub => "sub",
            GenericOp::Mul => "mul",
            GenericOp::And => "and",
            GenericOp::Or => "or",
            GenericOp::Xor => "xor",
            GenericOp::Shl => "shl",
            GenericOp::Lshr => "lshr",
            GenericOp::Ashr => "ashr",
            GenericOp::Sdiv => "sdiv",
            GenericOp::Srem => "srem",
            GenericOp::Udiv => "udiv",
            GenericOp::Urem => "urem",
            GenericOp::Fadd => "fadd",
            GenericOp::Fsub => "fsub",
            GenericOp::Fmul => "fmul",
            GenericOp::Fdiv => "fdiv",
            GenericOp::Fneg => "fneg",
            GenericOp::Icmp => "icmp",
            GenericOp::Fcmp => "fcmp",
            GenericOp::Trunc => "trunc",
            GenericOp::Zext => "zext",
            GenericOp::Sext => "sext",
            GenericOp::Fptrunc => "fptrunc",
            GenericOp::Fpext => "fpext",
            GenericOp::Fptosi => "fptosi",
            GenericOp::Sitofp => "sitofp",
            GenericOp::Copy => "copy",
            GenericOp::Select => "select",
            GenericOp::Constant => "constant",
        }
    }

    pub fn is_terminator(self) -> bool {
        matches!(self, GenericOp::Br | GenericOp::BrCond | GenericOp::Ret)
    }

    pub fn branch_target_operand_indices(self) -> &'static [usize] {
        match self {
            GenericOp::Br => &[0],
            GenericOp::BrCond => &[1, 2],
            _ => &[],
        }
    }
}

impl Opcode for GenericOp {
    fn name(self) -> &'static str {
        GenericOp::name(self)
    }

    fn is_terminator(self) -> bool {
        GenericOp::is_terminator(self)
    }
    
    fn branch_target_operand_indices(self) -> &'static [usize] {
        GenericOp::branch_target_operand_indices(self)
    }
}