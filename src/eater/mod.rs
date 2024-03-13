mod cpu;
mod disassemble;
mod instructions;

pub use cpu::{Cpu, Flag};
pub use instructions::I;
use instructions::{IBuilder, Instruction};
pub use disassemble::{Disassembly, disassemble};
