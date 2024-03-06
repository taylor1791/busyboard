mod hexdump;

use crate::{eater::Cpu, ui::ActionLoop};
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    prelude::{Layout, Rect, Widget},
    widgets::{Block, Padding, Paragraph},
};

pub struct Simulator {
    cpu: Cpu,
    mode: Mode,
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

impl Simulator {
    pub fn from(cpu: Cpu) -> Self {
        Self {
            cpu,
            mode: Mode::Execute,
        }
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

        let areas = Layout::vertical(vec![ratatui::prelude::Constraint::Max(16)]).split(area);
        let dump = hexdump::hexdump(self.cpu.ip(), &self.cpu.read_bytes(0, self.cpu.len() as u8));

        dump.render(areas[0], buffer);
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
