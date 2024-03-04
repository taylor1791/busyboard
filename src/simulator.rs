use crate::{eater::Cpu, ui::ActionLoop};
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::prelude::{Stylize, Widget};

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
    fn render_ref(&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        ratatui::widgets::Paragraph::new(format!("a: {:03} ip: {:03}", self.cpu.a(), self.cpu.ip()))
            .white()
            .on_blue()
            .render(area, buffer);
    }
}
