use crate::mir::opcode::OpCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArmOp {
    Add, Sub, Mul, And, Orr, Eor,
    Ldr, Str,
    B, Bl, Bx, Cbz, Cbnz,
    Mov, MovZ, MovK,
    Li, Mv,
    CopyFromReg, CopyToReg,
}

impl OpCode for ArmOp {
    fn name(self) -> &'static str {
        match self {
            ArmOp::Add => "arm.add",
            ArmOp::Sub => "arm.sub",
            ArmOp::Mul => "arm.mul",
            ArmOp::And => "arm.and",
            ArmOp::Orr => "arm.orr",
            ArmOp::Eor => "arm.eor",
            ArmOp::Ldr => "arm.ldr",
            ArmOp::Str => "arm.str",
            ArmOp::B => "arm.b",
            ArmOp::Bl => "arm.bl",
            ArmOp::Bx => "arm.bx",
            ArmOp::Cbz => "arm.cbz",
            ArmOp::Cbnz => "arm.cbnz",
            ArmOp::Mov => "arm.mov",
            ArmOp::MovZ => "arm.movz",
            ArmOp::MovK => "arm.movk",
            ArmOp::Li => "arm.li",
            ArmOp::Mv => "arm.mv",
            ArmOp::CopyFromReg => "arm.copy_from_reg",
            ArmOp::CopyToReg => "arm.copy_to_reg",
        }
    }

    fn is_terminator(self) -> bool {
        matches!(self, ArmOp::B | ArmOp::Bl | ArmOp::Bx | ArmOp::Cbz | ArmOp::Cbnz)
    }

    fn branch_target_operand_indices(self) -> &'static [usize] {
        match self {
            ArmOp::B | ArmOp::Bl | ArmOp::Bx => &[0],
            ArmOp::Cbz | ArmOp::Cbnz => &[1],
            _ => &[],
        }
    }
}