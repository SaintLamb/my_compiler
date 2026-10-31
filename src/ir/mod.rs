pub mod builder;
pub mod function;
pub mod id;
pub mod instr;
pub mod opcode;
pub mod operand;
pub mod printer;
pub mod verifier;

pub use builder::IrBuilder;
pub use function::{IrBlock, IrFunction, Phi, ValueInfo};
pub use id::{BlockId, InstId, ValueId};
pub use instr::IrInstr;
pub use opcode::GenericOp;
pub use operand::Operand;
pub use printer::print_function;
pub use verifier::{verify, Stage, VerifyError};