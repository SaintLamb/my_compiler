use crate::mir::opcode::OpCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RiscvOpCode {
    // 算术
    AddW, AddD, SubW, SubD, MulW, MulD, DivW, DivD, RemW, RemD,
    And, Or, Xor, SllW, SllD, SrlW, SrlD, SraW, SraD,
    // 内存
    Lw, Ld, Sw, Sd,
    // 控制流
    Beq, Bne, Blt, Bge, J, Jal, Jalr, Ret,
    // 立即数 / copy
    Li, Mv,
    // ABI
    CopyFromReg, CopyToReg,
}

impl OpCode for RiscvOpCode {
    fn name(self) -> &'static str {
        match self {
            RiscvOpCode::AddW => "riscv.addw",
            RiscvOpCode::AddD => "riscv.addd",
            RiscvOpCode::SubW => "riscv.subw",
            RiscvOpCode::SubD => "riscv.subd",
            RiscvOpCode::MulW => "riscv.mulw",
            RiscvOpCode::MulD => "riscv.muld",
            RiscvOpCode::DivW => "riscv.divw",
            RiscvOpCode::DivD => "riscv.divd",
            RiscvOpCode::RemW => "riscv.remw",
            RiscvOpCode::RemD => "riscv.remd",
            RiscvOpCode::And => "riscv.and",
            RiscvOpCode::Or => "riscv.or",
            RiscvOpCode::Xor => "riscv.xor",
            RiscvOpCode::SllW => "riscv.sllw",
            RiscvOpCode::SllD => "riscv.slld",
            RiscvOpCode::SrlW => "riscv.srlw",
            RiscvOpCode::SrlD => "riscv.srld",
            RiscvOpCode::SraW => "riscv.sraw",
            RiscvOpCode::SraD => "riscv.srad",
            RiscvOpCode::Lw => "riscv.lw",
            RiscvOpCode::Ld => "riscv.ld",
            RiscvOpCode::Sw => "riscv.sw",
            RiscvOpCode::Sd => "riscv.sd",
            RiscvOpCode::Beq => "riscv.beq",
            RiscvOpCode::Bne => "riscv.bne",
            RiscvOpCode::Blt => "riscv.blt",
            RiscvOpCode::Bge => "riscv.bge",
            RiscvOpCode::J => "riscv.j",
            RiscvOpCode::Jal => "riscv.jal",
            RiscvOpCode::Jalr => "riscv.jalr",
            RiscvOpCode::Ret => "riscv.ret",
            RiscvOpCode::Li => "riscv.li",
            RiscvOpCode::Mv => "riscv.mv",
            RiscvOpCode::CopyFromReg => "riscv.copy_from_reg",
            RiscvOpCode::CopyToReg => "riscv.copy_to_reg",
        }
    }

    fn is_terminator(self) -> bool {
        matches!(self,
            RiscvOpCode::Beq | RiscvOpCode::Bne | RiscvOpCode::Blt | RiscvOpCode::Bge
            | RiscvOpCode::J | RiscvOpCode::Jal | RiscvOpCode::Jalr | RiscvOpCode::Ret)
    }

    fn branch_target_operand_indices(self) -> &'static [usize] {
        match self {
            RiscvOpCode::Beq | RiscvOpCode::Bne | RiscvOpCode::Blt | RiscvOpCode::Bge => &[2],
            RiscvOpCode::J | RiscvOpCode::Jal => &[0],
            RiscvOpCode::Jalr => &[0],
            _ => &[],
        }
    }
}