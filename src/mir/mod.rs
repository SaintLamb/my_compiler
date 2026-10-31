pub mod id;
pub mod types;
pub mod opcode;
pub mod operand;
pub mod instr;
pub mod function;
pub mod builder;
pub mod printer;
pub mod verifier;

pub use id::{BlockId, FrameId, InstId, PRegId, VRegId};
pub use types::{Cond, RegClass, Type};
pub use opcode::{GenericOp, Opcode};
pub use operand::{OpFlags, Operand, Reg};
pub use instr::{MirInstr, Phi};
pub use function::{
    FrameInfo, MirBlock, MirFunction, StackSlot, StackSlotUsage, VRegInfo,
};
pub use builder::Builder;
pub use verifier::{verify, Stage, VerifyError};