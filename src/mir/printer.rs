use crate::mir::function::MirFunction;
use crate::mir::id::{BlockId, VRegId};
use crate::mir::instr::MirInstr;
use crate::mir::operand::{OpFlags, Operand, Reg};
use crate::mir::types::Type;
use std::fmt::Write;

pub fn print_function(f: &MirFunction) -> String {
    let mut out = String::new();
    let _ = write!(out, "fn {}(", f.name);

    for (i, &arg) in f.args.iter().enumerate() {
        if i > 0 {
            let _ = write!(out, ", ");
        }
        let _ = write!(out, "%{}{}", arg.raw(), type_suffix(f.vreg(arg).ty));
    }

    let _ = writeln!(out, "):");

    for (block_id, block) in f.blocks.iter_enumerated() {
        let _ = writeln!(out, "  {}:", block_label(block_id));

        for phi in &block.phis {
            let _ = write!(
                out,
                "    %{}{} = phi",
                phi.dest.raw(),
                type_suffix(f.vreg(phi.dest).ty)
            );
            for (pred, val) in &phi.incoming {
                let _ = write!(out, " [{}: {}]", block_label(*pred), vreg_name(*val));
            }
            let _ = writeln!(out);
        }

        for instr in &block.instrs {
            let _ = writeln!(out, "    {}", print_instr(f, instr));
        }
    }
    out
}

fn block_label(id: BlockId) -> String {
    format!("bb{}", id.raw())
}

fn vreg_name(id: VRegId) -> String {
    format!("%{}", id.raw())
}

fn print_instr(f: &MirFunction, instr: &MirInstr) -> String {
    let mut s = String::new();

    let def = instr
        .operands
        .iter()
        .find(|o| o.flags().contains(OpFlags::DEF));

    if let Some(def_op) = def {
        let _ = write!(s, "{} = ", print_operand_bare(f, def_op));
    }

    // opcode
    let _ = write!(s, "{}", instr.opcode.name());

    let uses: Vec<_> = instr
        .operands
        .iter()
        .filter(|o| o.flags().contains(OpFlags::USE))
        .collect();
    for (i, op) in uses.iter().enumerate() {
        if i == 0 {
            let _ = write!(s, " ");
        } else {
            let _ = write!(s, ", ");
        }
        let _ = write!(s, "{}", print_operand_bare(f, op));
    }

    s
}

fn print_operand_bare(f: &MirFunction, op: &Operand) -> String {
    match op {
        Operand::Reg(Reg::Virtual(v), _) => {
            let info = f.vreg(*v);
            format!("%{}{}", v.raw(), type_suffix(info.ty))
        }
        Operand::Reg(Reg::Physical(p), _) => format!("r{}", p.raw()),
        Operand::Reg(Reg::Stack(id), _) => {
            let slot = f.slot(*id);
            let off = slot
                .offset
                .map(|o| format!("@{:+}", o))
                .unwrap_or_default();
            format!("stack{}({}{})", id.raw(), slot.size, off)
        }
        Operand::Imm(v, _) => format!("{v}"),
        Operand::FImm(v, _) => format!("{v}"),
        Operand::Reloc(s, _) => format!("reloc({s})"),
        Operand::Cond(c, _) => format!("cond({c:?})"),
        Operand::Literal(s, _) => format!("literal({s})"),
        Operand::Unused => "<unused>".into(),
    }
}

#[allow(dead_code)]
fn print_operand(f: &MirFunction, op: &Operand) -> String {
    match op {
        Operand::Reg(Reg::Virtual(v), flags) => {
            let info = f.vreg(*v);
            let cls = info
                .class
                .map(|c| format!(":{c:?}"))
                .unwrap_or_default();
            format!(
                "{}{}{}{}",
                vreg_name(*v),
                cls,
                type_suffix(info.ty),
                flag_suffix(*flags)
            )
        }
        Operand::Reg(Reg::Physical(p), flags) => {
            format!("r{}{}", p.raw(), flag_suffix(*flags))
        }
        Operand::Reg(Reg::Stack(id), flags) => {
            let slot = f.slot(*id);
            let off = slot
                .offset
                .map(|o| format!("@{:+}", o))
                .unwrap_or_default();
            format!(
                "stack{}({}{}{})",
                id.raw(),
                slot.size,
                off,
                flag_suffix(*flags)
            )
        }
        Operand::Imm(v, flags) => {
            format!("imm({}){}", *v as i64, flag_suffix(*flags))
        }
        Operand::FImm(v, flags) => {
            format!("fimm({}){}", v, flag_suffix(*flags))
        }
        Operand::Reloc(s, flags) => {
            format!("reloc({}){}", s, flag_suffix(*flags))
        }
        Operand::Cond(c, flags) => {
            format!("cond({:?}){}", c, flag_suffix(*flags))
        }
        Operand::Literal(s, flags) => {
            format!("literal({}){}", s, flag_suffix(*flags))
        }
        Operand::Unused => "<unused>".into(),
    }
}

fn type_suffix(ty: Type) -> String {
    match ty {
        Type::Int(w) => format!(":i{}", w),
        Type::Float(w) => format!(":f{}", w),
        Type::Ptr(w) => format!(":p{}", w),
        Type::Vec(n, w) => format!(":v{}i{}", n, w),
        Type::CondFlag => ":cc".into(),
        Type::Special => ":sp".into(),
    }
}

fn flag_suffix(flags: OpFlags) -> &'static str {
    if flags.contains(OpFlags::DEF) {
        "@def"
    } else if flags.contains(OpFlags::USE) {
        "@use"
    } else {
        ""
    }
}