use my_compiler::mir::*;
use my_compiler::mir::types::Type;
use my_compiler::mir::opcode::GenericOp;
use my_compiler::mir::operand::Operand;
use my_compiler::mir::printer::print_function;
use my_compiler::support::Symbol;

#[test]
fn test_print_add() {
    let mut f = MirFunction::new(Symbol::from("add"));
    let entry = f.entry;  

    let mut b = Builder::new(&mut f);
    let a = b.new_vreg(Type::I32);
    let c = b.new_vreg(Type::I32);
    let r = b.new_vreg(Type::I32);

    b.push(entry, GenericOp::Constant, vec![
        Operand::vreg_def(a),
        Operand::imm(42),
    ]);
    b.push(entry, GenericOp::Constant, vec![
        Operand::vreg_def(c),
        Operand::imm(7),
    ]);

    b.push(entry, GenericOp::Add, vec![
        Operand::vreg_def(r),
        Operand::vreg_use(a),
        Operand::vreg_use(c),
    ]);
    b.push(entry, GenericOp::Ret, vec![Operand::vreg_use(r)]);

    if let Err(errs) = verify(&f, Stage::PostLowering) {
        for e in &errs {
            eprintln!("verify error: {e}");
        }
        panic!("verify failed");
    }

    println!("{}", print_function(&f));
}