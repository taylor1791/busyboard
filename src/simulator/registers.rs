use crate::eater::{Cpu, Flag};
use ratatui::{
    prelude::{Line, Stylize, Widget},
    widgets::{Block, Padding, Paragraph},
};
use super::Ui;

pub fn registers(cpu: &Cpu, ui: &Ui) -> impl Widget {
    let ax = format!("   AX: {:02x}", cpu.a());
    let ax = if cpu.a() != ui.previous_ax { ax.green() } else { ax.into() };

    let ip = format!("   IP: {:02x}", cpu.ip());
    let ip = ip.magenta();

    let c = format!("    C: {:01x}", cpu.get(Flag::Carry) as u8);
    let c = if cpu.get(Flag::Carry) != ui.previous_flag_c { c.green() } else { c.into() };

    let h = format!("    H: {:01x}", cpu.get(Flag::Halt) as u8);
    let h = if cpu.get(Flag::Halt) != ui.previous_flag_h { h.green() } else { h.into() };

    let i = format!("    I: {:01x}", cpu.get(Flag::IllegalHalt) as u8);
    let i = if cpu.get(Flag::IllegalHalt) != ui.previous_flag_i { i.green() } else { i.into() };

    let registers = Paragraph::new(vec![
        Line::from(ax),
        Line::from(ip),
        Line::from(c),
        Line::from(h),
        Line::from(i),
    ])
        .block(Block::new()
            .title_top(Line::from(" Registers ".bold()).left_aligned())
            .padding(Padding::horizontal(1))
        );

    registers
}
