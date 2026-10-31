use my_compiler::ir::*;
use my_compiler::types::Type;

#[test]
fn ir_add_with_args_verifies() {
    let mut f = IrFunction::new("add".into());
    let entry = f.entry;
    let mut b = IrBuilder::new(&mut f);

    let a = b.new_arg(Type::I32);
    let c = b.new_arg(Type::I32);
    let r = b.new_value(Type::I32);

    b.push(
        entry,
        GenericOp::Add,
        vec![Operand::value(a), Operand::value(c)],
        Some(r),
    );
    b.push(entry, GenericOp::Ret, vec![Operand::value(r)], None);

    assert_eq!(f.args, vec![a, c]);
    if let Err(errs) = verify(&f, Stage::PostLowering) {
        for e in &errs { eprintln!("verify error: {e}"); }
        panic!("verify failed");
    }

    let out = print_function(&f);
    println!("{out}");
    assert!(out.contains("fn add(%0:i32, %1:i32):"));
    assert!(out.contains("%2:i32 = add %0:i32, %1:i32"));
    assert!(out.contains("ret %2:i32"));
}