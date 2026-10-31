use crate::mir::id::{BlockId, VRegId};
use crate::mir::function::MirFunction;
use crate::mir::instr::MirInstr;
use crate::mir::opcode::{Backend, OpCode};
use crate::mir::operand::{MirOperand, OpFlags, Reg};
use crate::types::Type;
use std::fmt::Write;

pub fn print_function<B: Backend>(f: &MirFunction<B>) -> String {
    let mut out = String::new();
    let _ = write!(out, "mfn {}(", f.name);

    for (i, &arg) in f.args.iter().enumerate() {
        if i > 0 { let _ = write!(out, ", "); }
        let _ = write!(out, "%{}{}", arg.raw(), type_suffix(f.vreg(arg).ty));
    }

    let _ = write!(out, ")\n");

    for (block_id, block) in f.blocks.iter_enumerated() {
        let _ = writeln!(out, "{}:", block_label(block_id));

        for phi in &block.phis {
            let _ = write!(out, " %{}{} = phi", phi.dest.raw(), type_suffix(f.vreg(phi.dest).ty));
            for (pred, val) in &phi.incoming {
                let _ = write!(out, " [{}: %{}]", block_label(*pred), val.raw());
            }
            let _ = writeln!(out);
        }

        for inst in &block.instrs {
            let _ = writeln!(out, "  {}", print_instr(f, inst));
        }
    }
    out
}

fn block_label(id: BlockId) -> String {
    format!("bb{}", id.raw())
}

fn print_instr<B: Backend>(f: &MirFunction<B>, inst: &MirInstr<B>) -> String {
    let mut s = String::new();

    let def = inst.operands.iter().find(|o| o.flags().contains(OpFlags::DEF));
    if let Some(d) = def {
        let _ = write!(s, "{} = ", print_operand_bare(f, d));
    }
    let _ = write!(s, "{}", inst.opcode.name());

    for (i, op) in inst.operands.iter()
        .filter(|o| o.flags().contains(OpFlags::USE))
        .enumerate()
    {
        if i == 0 { let _ = write!(s, " "); } else { let _ = write!(s, ", "); }
        let _ = write!(s, "{}", print_operand_bare(f, op));
    }
    s
}

fn print_operand_bare<B: Backend>(f: &MirFunction<B>, op: &MirOperand) -> String {
    match op {
        MirOperand::Reg(Reg::Virtual(v), _) => {
            format!("%{}{}", v.raw(), type_suffix(f.vreg(*v).ty))
        }
        MirOperand::Reg(Reg::Physical(p), _) => format!("r{}", p.raw()),
        MirOperand::Reg(Reg::Stack(id), _) => format!("stack{}", id.raw()),
        MirOperand::Imm(v, _) => format!("{v}"),
        MirOperand::FImm(v, _) => format!("{v}"),
        MirOperand::Block(b, _) => block_label(*b),
    }
}

fn type_suffix(ty: Type) -> String {
    match ty {
        Type::Int(w) => format!(":i{w}"),
        Type::Float(w) => format!(":f{w}"),
        Type::Ptr(w) => format!(":p{w}"),
        Type::Vec(n, w) => format!(":v{n}i{w}"),
        Type::CondFlag => ":cc".into(),
        Type::Special => ":sp".into(),
    }
}

#[allow(dead_code)]
fn vreg_name(id: VRegId) -> String {
    format!("%{}", id.raw())
}