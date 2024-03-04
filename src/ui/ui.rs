use crate::eater::Cpu;
use crossterm::{
    ExecutableCommand,
    self,
    terminal,
    event::{self, KeyCode, KeyEvent, KeyEventKind},
};
use ratatui::prelude::{CrosstermBackend, Stylize, Widget};
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    time::Duration,
    io::{Result, Stdout, stdout},
};

type Tui = ratatui::terminal::Terminal<CrosstermBackend<Stdout>>;

pub struct Ui {
    clock_speed: Duration,
    cpu: Cpu,
    mode: Mode,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Execute,
    Exit,
    Step,
}

impl Widget for &Ui {
    fn render(self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        ratatui::widgets::Paragraph::new(format!("a: {:03} ip: {:03}", self.cpu.a(), self.cpu.ip()))
            .white()
            .on_blue()
            .render(area, buffer);
    }
}

impl Ui {
    pub fn new(cpu: Cpu) -> Self {
        let output = Rc::new(RefCell::new(Vec::<u8>::new()));

        let me = Self {
            clock_speed: Duration::from_millis(1000),
            cpu: cpu.with_out(move |byte| {
                output.borrow_mut().push(byte);
            }),
            mode: Mode::Execute,
        };

        me
    }

    pub fn start(&mut self) -> Result<()> {
        let terminal = self.setup()?;
        self.main_loop(terminal)?;
        self.cleanup()?;

        Ok(())
    }

    fn setup(&mut self) -> Result<Tui> {
        stdout().execute(terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        let mut terminal = Tui::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(terminal)
    }

    fn cleanup(&mut self) -> Result<()> {
        stdout().execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn main_loop(&mut self, mut terminal: Tui) -> Result<()> {
        while self.mode != Mode::Exit {
            terminal.draw(|frame| self.render(frame))?;
            let action = self.input();
            self.update(action);
        }

        Ok(())
    }

    fn render(&self, frame: &mut ratatui::prelude::Frame) {
        frame.render_widget(self, frame.size());
    }

    fn input(&mut self) -> Action {
        let start = std::time::Instant::now();

        loop {
            if self.mode == Mode::Execute && start.elapsed() > self.clock_speed {
                return Action::Step;
            }

            let deadline = match self.mode {
                Mode::Execute => self.clock_speed - start.elapsed(),
                _ => Duration::from_secs(60), // Can be arbitrarily long
            };

            if event::poll(deadline).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    if let Some(action) = self.action(key) {
                        return action;
                    }
                }
            }
        }
    }

    fn action(&mut self, key: KeyEvent) -> Option<Action> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Some(Action::Quit),
                KeyCode::Char('s') if self.mode == Mode::Step => return Some(Action::Step),
                KeyCode::Char('s') => return Some(Action::Mode(Mode::Step)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Mode(mode) => self.mode = mode,
            Action::Quit => self.mode = Mode::Exit,
            Action::Step => {
                self.cpu.step();
            }
        }
    }
}

#[derive(Debug)]
enum Action {
    Mode(Mode),
    Quit,
    Step,
}
