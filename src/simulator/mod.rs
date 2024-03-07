mod hexdump;
mod registers;

use crate::{eater::{Cpu, Flag}, ui::ActionLoop};
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    prelude::{Layout, Rect, Stylize, Widget},
    widgets::{Block, Padding, Paragraph},
};

pub struct Simulator {
    cpu: Cpu,
    mode: Mode,
    ui: Ui
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Execute,
    Exit,
    Step,
}

#[derive(Debug)]
pub enum Action {
    Mode(Mode),
    Quit,
    Step,
}

pub struct Ui {
    previous_ax: u8,
    previous_bytes: Vec<u8>,
    previous_ip: u8,
    previous_flag_c: bool,
    previous_flag_h: bool,
    previous_flag_i: bool,
}

impl Simulator {
    pub fn from(cpu: Cpu) -> Self {
        let ui = Ui {
            previous_ax: cpu.a(),
            previous_bytes: cpu.read_bytes(0, cpu.len() as u8).to_vec(),
            previous_ip: cpu.ip(),
            previous_flag_c: cpu.get(Flag::Carry),
            previous_flag_h: cpu.get(Flag::Halt),
            previous_flag_i: cpu.get(Flag::IllegalHalt),
        };

        Self { cpu, mode: Mode::Execute, ui }
    }
}

impl ActionLoop for Simulator {
    type Action = Action;

    fn action(&self, key: KeyEvent) -> Option<Self::Action> {
        match key.code {
            KeyCode::Char('q') => return Some(Action::Quit),
            KeyCode::Char('s') if self.mode == Mode::Step => return Some(Action::Step),
            KeyCode::Char('s') => return Some(Action::Mode(Mode::Step)),
            _ => None,
        }
    }

    fn exited(&self) -> bool {
        self.mode == Mode::Exit
    }

    fn deadline_expired(&self) -> Option<Self::Action> {
        if self.mode == Mode::Execute {
            Some(Action::Step)
        } else {
            None
        }
    }

    fn update(&mut self, action: Self::Action) {
        self.ui.previous_bytes = self.cpu.read_bytes(0, self.cpu.len() as u8).to_vec();
        self.ui.previous_ax = self.cpu.a();
        self.ui.previous_ip = self.cpu.ip();
        self.ui.previous_flag_c = self.cpu.get(Flag::Carry);
        self.ui.previous_flag_h = self.cpu.get(Flag::Halt);
        self.ui.previous_flag_i = self.cpu.get(Flag::IllegalHalt);

         match action {
             Action::Mode(mode) => self.mode = mode,
             Action::Quit => self.mode = Mode::Exit,
             Action::Step => {
                 self.cpu.step();
             }
         }
    }
}

impl ratatui::widgets::WidgetRef for Simulator {
    fn render_ref(&self, area: Rect, buffer: &mut ratatui::prelude::Buffer) {
        let area = Layout::horizontal(vec![
            // | 00: XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX |
            ratatui::prelude::Constraint::Length(3 * 17 + 2 + 2),
        ]).split(area)[0];

        let areas = Layout::vertical(vec![
            ratatui::prelude::Constraint::Max(16),
            ratatui::prelude::Constraint::Length(3),
            ratatui::prelude::Constraint::Max(16),
        ]).split(area);

        let registers = registers::registers(&self.cpu, &self.ui);
        let mode = Paragraph::new(format!("Mode: {}", self.mode).bold())
            .block(Block::new().padding(Padding::uniform(1)));

        let dump = hexdump::hexdump(
            self.cpu.ip(),
            &self.cpu.read_bytes(0, self.cpu.len() as u8),
            &self.ui.previous_bytes,
        );

        registers.render(areas[0], buffer);
        mode.render(areas[1], buffer);
        dump.render(areas[2], buffer);
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Execute => write!(f, "Execute"),
            Mode::Exit => write!(f, "Exiting"),
            Mode::Step => write!(f, "Step"),
        }
    }
}
