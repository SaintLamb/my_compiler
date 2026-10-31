use crate::ir::id::BlockId;
use crate::ir::function::IrFunction;
use crate::ir::instr::IrInstr;
use crate::ir::operand::Operand;
use crate::types::Type;
use std::fmt::Write;

pub fn print_function(f: &IrFunction) -> String {
    let mut out = String::new();

    let _ = write!(out, "fn {}(", f.name);
    for (i, &arg) in f.args.iter().enumerate() {
        if i > 0 { let _ = write!(out, ", "); }
        let _ = write!(out, "%{}{}", arg.raw(), type_suffix(f.value(arg).ty));
    }
    let _ = writeln!(out, "):");

    for (block_id, block) in f.blocks.iter_enumerated() {
        let _ = writeln!(out, "  {}:", block_label(block_id));

        for phi in &block.phis {
            let _ = write!(out, "    %{}{} = phi", phi.dest.raw(), type_suffix(f.value(phi.dest).ty));
            for (pred, val) in &phi.incoming {
                let _ = write!(out, " [{}: %{}]", block_label(*pred), val.raw());
            }
            let _ = writeln!(out);
        }

        for inst in &block.instrs {
            let _ = writeln!(out, "    {}", print_instr(f, inst));
        }
    }
    out
}

fn block_label(id: BlockId) -> String {
    format!("bb{}", id.raw())
}

fn print_instr(f: &IrFunction, inst: &IrInstr) -> String {
    let mut s = String::new();

    if let Some(r) = inst.result {
        let _ = write!(s, "%{}{} = ", r.raw(), type_suffix(f.value(r).ty));
    }
    let _ = write!(s, "{}", inst.opcode.name());

    for (i, op) in inst.operands.iter().enumerate() {
        if i == 0 { let _ = write!(s, " "); } else { let _ = write!(s, ", "); }
        let _ = write!(s, "{}", print_operand(f, op));
    }
    s
}

fn print_operand(f: &IrFunction, op: &Operand) -> String {
    match op {
        Operand::Value(v) => format!("%{}{}", v.raw(), type_suffix(f.value(*v).ty)),
        Operand::Imm(v) => format!("{v}"),
        Operand::Block(b) => block_label(*b),
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