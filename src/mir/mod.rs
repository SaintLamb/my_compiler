pub mod arm;
pub mod builder;
pub mod function;
pub mod id;
pub mod instr;
pub mod opcode;
pub mod operand;
pub mod printer;
pub mod riscv;
pub mod verifier;

pub use builder::MirBuilder;
pub use function::{FrameInfo, MirBlock, MirFunction, Phi, StackSlot, StackSlotUsage, VRegInfo};
pub use id::{BlockId, FrameId, InstId, PRegId, VRegId};
pub use instr::MirInstr;
pub use opcode::{OpCode, TargetKind, Backend};
pub use operand::{MirOperand, OpFlags, Reg};
pub use printer::print_function;
pub use verifier::{verify, Stage as MirStage, VerifyError as MirVerifyError};
