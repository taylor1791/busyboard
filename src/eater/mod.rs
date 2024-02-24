mod cpu;
mod instructions;

pub use cpu::{Cpu, Flag};
pub use instructions::I;
use instructions::{IBuilder, Instruction};
