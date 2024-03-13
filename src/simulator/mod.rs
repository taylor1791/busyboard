mod disassemble;
mod instructions;
mod hexdump;
mod registers;
mod out;

use crate::{eater::{Cpu, Flag}, ui::ActionLoop};
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    prelude::{Layout, Rect, Widget},
    widgets::{Block, Padding, Paragraph},
};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

pub struct Simulator {
    cpu: Cpu,
    mode: Mode,
    out: Rc<RefCell<Out>>,
    rate: Duration,
    ui: Ui,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Edit(Edit),
    Execute,
    Exit,
    Step,
}

impl Mode {
    fn is_edit(&self) -> bool {
        match self {
            Mode::Edit(..) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Edit {
    IP,
    Data,
}

#[derive(Debug)]
pub enum Action {
    Increment,
    Mode(Mode),
    Quit,
    Shift,
    Step,
    Turbo,
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
        let rate = Duration::from_millis(1000);

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

        Self { cpu, rate, mode: Mode::Execute, out: out.clone(), ui }
    }

    pub fn is_turbo(&self) -> bool {
        self.rate != Duration::from_millis(1000)
    }
}

impl ActionLoop for Simulator {
    type Action = Action;

    fn action(&self, key: KeyEvent) -> Option<Self::Action> {
        match key.code {
            KeyCode::Char('a') if self.mode == Mode::Execute => Some(Action::Turbo),
            KeyCode::Char('s') if self.mode == Mode::Execute => Some(Action::Mode(Mode::Step)),
            KeyCode::Char('d') if self.mode == Mode::Execute => Some(Action::Mode(Mode::Edit(Edit::IP))),
            KeyCode::Char('a') if self.mode == Mode::Step => Some(Action::Mode(Mode::Execute)),
            KeyCode::Char('s') if self.mode == Mode::Step => Some(Action::Step),
            KeyCode::Char('d') if self.mode == Mode::Step => Some(Action::Mode(Mode::Edit(Edit::IP))),
            KeyCode::Char('a') if self.mode.is_edit() => Some(Action::Increment),
            KeyCode::Char('s') if self.mode.is_edit() => Some(Action::Shift),
            KeyCode::Char('d') if self.mode == Mode::Edit(Edit::IP) => Some(Action::Mode(Mode::Edit(Edit::Data))),
            KeyCode::Char('d') if self.mode == Mode::Edit(Edit::Data) => Some(Action::Mode(Mode::Step)),
            KeyCode::Char('q') => Some(Action::Quit),
            _ => None,
        }
    }

    fn exited(&self) -> bool {
        self.mode == Mode::Exit
    }

    fn deadline(&self) -> &Duration {
        &self.rate
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
            Action::Increment => {
                match self.mode {
                    Mode::Edit(Edit::IP) => self.cpu.goto(self.cpu.ip().wrapping_add(1)),
                    Mode::Edit(Edit::Data) => {
                        let value = self.cpu.read(self.cpu.ip()).unwrap_or(0_u8);
                        self.cpu.write(self.cpu.ip(), value.wrapping_add(1))
                    },
                    _ => (),
                }
            },
            Action::Mode(mode) => self.mode = mode,
            Action::Quit => self.mode = Mode::Exit,
            Action::Shift if self.mode == Mode::Edit(Edit::IP) => self.cpu.goto(self.cpu.ip().wrapping_mul(2)),
            Action::Shift if self.mode == Mode::Edit(Edit::Data) => {
                let value = self.cpu.read(self.cpu.ip()).unwrap_or(0_u8);
                self.cpu.write(self.cpu.ip(), value.wrapping_mul(2))
            },
            Action::Shift => (),
            Action::Step => {
                self.cpu.step();
            },
            Action::Turbo => {
                self.rate = if self.is_turbo() {
                    Duration::from_millis(1000)
                } else {
                    Duration::from_millis(50)
                };
            },
         }
    }
}

impl ratatui::widgets::WidgetRef for Simulator {
    fn render_ref(&self, area: Rect, buffer: &mut ratatui::prelude::Buffer) {
        let bytes = self.cpu.read_bytes(0, self.cpu.len() as u8);

        let chrome_height = 2;
        let instructions = instructions::instructions(&self.mode, self.is_turbo());
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
            Mode::Edit(..) => write!(f, "Edit"),
            Mode::Execute => write!(f, "Execute"),
            Mode::Exit => write!(f, "Exiting"),
            Mode::Step => write!(f, "Step"),
        }
    }
}
