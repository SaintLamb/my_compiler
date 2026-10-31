use my_compiler::mir::opcode::Riscv;
use my_compiler::mir::operand::MirOperand;
use my_compiler::mir::printer::print_function;
use my_compiler::mir::riscv::opcode::RiscvOpCode;
use my_compiler::mir::verifier::{verify, Stage};
use my_compiler::mir::{MirBuilder, MirFunction};
use my_compiler::support::Symbol;
use my_compiler::types::Type;

type MirFnR = MirFunction<Riscv>;

#[test]
fn test_print_add() {
    let mut f: MirFnR = MirFunction::new(Symbol::from("add"));
    let entry = f.entry;

    let mut b = MirBuilder::new(&mut f);
    let a = b.new_vreg(Type::I32);
    let c = b.new_vreg(Type::I32);
    let r = b.new_vreg(Type::I32);

    b.push(entry, RiscvOpCode::Li, vec![
        MirOperand::vreg_def(a),
        MirOperand::imm_use(42),
    ]);
    b.push(entry, RiscvOpCode::Li, vec![
        MirOperand::vreg_def(c),
        MirOperand::imm_use(7),
    ]);
    b.push(entry, RiscvOpCode::AddW, vec![
        MirOperand::vreg_def(r),
        MirOperand::vreg_use(a),
        MirOperand::vreg_use(c),
    ]);
    b.push(entry, RiscvOpCode::Ret, vec![MirOperand::vreg_use(r)]);
    drop(b);

    if let Err(errs) = verify(&f, Stage::PostLowering) {
        for e in &errs {
            eprintln!("verify error: {e}");
        }
        panic!("verify failed");
    }

    let out = print_function(&f);
    println!("{out}");
}

#[test]
fn test_print_add_with_args() {
    let mut f: MirFnR = MirFunction::new(Symbol::from("add"));
    let entry = f.entry;

    let mut b = MirBuilder::new(&mut f);
    let a = b.new_arg(Type::I32);
    let c = b.new_arg(Type::I32);
    let r = b.new_vreg(Type::I32);

    b.push(entry, RiscvOpCode::AddW, vec![
        MirOperand::vreg_def(r),
        MirOperand::vreg_use(a),
        MirOperand::vreg_use(c),
    ]);
    b.push(entry, RiscvOpCode::Ret, vec![MirOperand::vreg_use(r)]);
    drop(b);

    assert_eq!(f.args, vec![a, c]);

    if let Err(errs) = verify(&f, Stage::PostLowering) {
        for e in &errs {
            eprintln!("verify error: {e}");
        }
        panic!("verify failed");
    }

    let out = print_function(&f);
    println!("{out}");

    // ⚠️ 断言关键字按你的 printer 实际输出对齐
    assert!(out.contains("addw") || out.contains("add.w"), "got:\n{out}");
    assert!(out.contains("ret"), "got:\n{out}");
}