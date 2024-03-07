mod hexdump;

use crate::{eater::Cpu, ui::ActionLoop};
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
    previous_bytes: Vec<u8>,
}

impl Simulator {
    pub fn from(cpu: Cpu) -> Self {
        let ui = Ui {
            previous_bytes: cpu.read_bytes(0, cpu.len() as u8).to_vec(),
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
            ratatui::prelude::Constraint::Length(3),
            ratatui::prelude::Constraint::Max(16),
        ]).split(area);

        let mode = Paragraph::new(format!("Mode: {}", self.mode).bold())
            .block(Block::new().padding(Padding::uniform(1)));

        let dump = hexdump::hexdump(
            self.cpu.ip(),
            &self.cpu.read_bytes(0, self.cpu.len() as u8),
            &self.ui.previous_bytes,
        );

        mode.render(areas[0], buffer);
        dump.render(areas[1], buffer);
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
