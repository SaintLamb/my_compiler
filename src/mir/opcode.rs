use crate::mir::riscv::opcode::RiscvOpCode;

pub trait OpCode: Copy + std::fmt::Debug + Eq + std::hash::Hash + 'static {
    fn name(self) -> &'static str;
    fn is_terminator(self) -> bool;
    fn branch_target_operand_indices(self) -> &'static [usize];
}

pub trait Backend: 'static + Copy + std::fmt::Debug {
    type OpCode: OpCode;
    const KIND: TargetKind;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MirOpCode {
    Riscv(RiscvOpCode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetKind { Riscv, Arm }

#[derive(Clone, Copy, Debug)]
pub struct Riscv;

impl Backend for Riscv {
    type OpCode = RiscvOpCode;
    const KIND: TargetKind = TargetKind::Riscv;
}