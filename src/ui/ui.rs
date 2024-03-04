use crossterm::{
    ExecutableCommand,
    self,
    terminal,
    event,
};
use ratatui::{
    widgets::WidgetRef,
    prelude::{CrosstermBackend, Widget},
};
use std::{
    time::Duration,
    io::{ Result, self, stdout},
};

type Tui = ratatui::terminal::Terminal<CrosstermBackend<io::Stdout>>;

pub struct Ui {
    action_deadline: Duration,
}

pub trait ActionLoop {
    type Action;

    fn action(&self, key: event::KeyEvent) -> Option<Self::Action>;
    fn exited(&self) -> bool;
    fn deadline_expired(&self) -> Option<Self::Action>;
    fn update(&mut self, action: Self::Action);
}

impl Ui {
    pub fn new(action_deadline: Duration) -> Self {
        Self { action_deadline }
    }

    pub fn run<W>(&self, widget: W) -> Result<()>
    where
        W: ActionLoop + WidgetRef,
    {
        let terminal = self.setup()?;
        self.main_loop(terminal, widget)?;
        self.cleanup()?;

        Ok(())
    }

    fn setup(&self) -> Result<Tui> {
        stdout().execute(terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        let mut terminal = Tui::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(terminal)
    }

    fn main_loop<W>(&self, mut terminal: Tui, mut widget: W) -> Result<()>
    where
        W: ActionLoop + WidgetRef,
    {
        while !widget.exited() {
            if let Some(action) = {
                terminal.draw(|frame| self.render_frame(&widget, frame))?;
                self.input(&widget)
            } {
                widget.update(action);
            }
        }

        Ok(())
    }

    fn render_frame<W>(&self, widget: W, frame: &mut ratatui::prelude::Frame)
    where
        W: Widget,
    {
        frame.render_widget(widget, frame.size());
    }

    fn input<'w, W>(&self, widget: &'w W) -> Option<<W as ActionLoop>::Action>
    where
        W: ActionLoop,
    {
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > self.action_deadline {
                return widget.deadline_expired();
            }

            let deadline = self.action_deadline - start.elapsed();
            if event::poll(deadline).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    if let Some(action) = widget.action(key) {
                        return Some(action);
                    }
                }
            }
        }
    }

    fn cleanup(&self) -> Result<()> {
        stdout().execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
