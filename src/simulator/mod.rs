mod disassemble;
mod hexdump;
mod registers;
mod out;

use crate::{eater::{Cpu, Flag}, ui::ActionLoop};
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    prelude::{Layout, Line, Rect, Stylize, Widget},
    widgets::{Block, Padding, Paragraph},
};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Simulator {
    cpu: Cpu,
    mode: Mode,
    out: Rc<RefCell<Out>>,
    ui: Ui,
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

pub struct Out {
    data: [u8; 16],
    n: usize,
    new: bool,
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

        let out = Rc::new(RefCell::new(Out {
            data: [0; 16],
            n: 0,
            new: false,
        }));

        let cpu_out = out.clone();
        let cpu = cpu.with_out(move |data| {
            let mut out = cpu_out.borrow_mut();
            let n = out.n;

            out.data[n % 16] = data;
            out.new = true;
            out.n += 1;

            if n >= 512 {
                out.n = 256;
            }
        });

        Self { cpu, mode: Mode::Execute, out: out.clone(), ui }
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
        self.out.borrow_mut().new = false;
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
        let bytes = self.cpu.read_bytes(0, self.cpu.len() as u8);

        let chrome_height = 2;
        let instructions = Line::from(vec![
            format!(" {}:", self.mode.to_string()).bold().magenta(),
            " Step ".bold().into(),
            "<s>".blue().bold(),
            " Quit ".bold().into(),
            "<q> ".blue().bold(),
        ]);
        let chrome = Block::bordered()
            .title("Simulator")
            .title_bottom(instructions.centered());

        let disassembled = crate::eater::disassemble(&bytes);
        let disassembly = disassemble::disassemble(
            &disassembled,
            self.cpu.ip(),
            &bytes,
            &self.ui.previous_bytes
        );
        let disassembly_height = disassembly.len() as u16 + 1; // Instructions + padding
        let disassembly = Paragraph::new(disassembly)
            .block(Block::new().padding(Padding::horizontal(1)));

        let registers_width = 9 + 2; // The word "Registers" plus right padding
        let register_height = 7; // Title, AX, IP, C, H, I, padding
        let registers = registers::registers(&self.cpu, &self.ui);

        let out_height = 2 + 1; // 2 Lines plus bottom padding
        let out = out::out(self.out.borrow());

        // Each byte is 2 characters, plus a space (or a colon), horizontal padding, and a border.
        let dump_width = 17 * 3 + 2 + 2;
        let dump_height = 1 + ((bytes.len() + 15) / 16) as u16 + 2; // Title + Lines + border
        let dump = hexdump::hexdump(self.cpu.ip(), &bytes, &self.ui.previous_bytes);

        let width = dump_width + 2; // Add 2 for the border
        let height = chrome_height + disassembly_height.max(register_height) + out_height + dump_height;
        let area = Rect::new(area.x, area.y, width, area.height.min(height));
        let areas = Layout::vertical(vec![
            ratatui::prelude::Constraint::Length(disassembly_height),
            ratatui::prelude::Constraint::Length(out_height),
            ratatui::prelude::Constraint::Length(dump_height),
        ]).split(Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2));
        let register_area = Rect::new(areas[0].width - registers_width, areas[0].y, registers_width, register_height);

        chrome.render(area, buffer);
        disassembly.render(areas[0], buffer);
        registers.render(register_area, buffer);
        out.render(areas[1], buffer);
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
