use super::{I, IBuilder, Instruction};

pub enum Flag {
    Carry = 0,
    Halt = 1,
    IllegalHalt = 2,
}

pub struct Cpu {
    pub (super) a: u8,
    pub (super) ip: u8,
    pub (super) flags: u8,
    pub (super) ram: Vec<u8>,
    pub (super) out: Box<dyn FnMut(u8)>,
}

impl Cpu {
    /// Create a new CPU with the given program and data.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::hlt(),
    /// ], vec![ ]);
    /// ```
    pub fn from_asm(asm: Vec<I>, data: Vec<u8>) -> Self {
        let ram = asm.into_iter().flat_map(|i| i.assemble()).chain(data).collect();

        Cpu {
            a: 0,
            ip: 0,
            flags: 0,
            ram,
            out: Box::from(default_out),
        }
    }

    /// Returns the contents of the A register.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::ldi(0xef),
    ///     I::hlt()
    /// ], vec![]);
    ///
    /// cpu.step();
    /// assert_eq!(cpu.a(), 0xef);
    /// ```
    pub fn a(&self) -> u8 {
        self.a
    }

    /// Returns the value of the given flag.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![], vec![]);
    ///
    /// cpu.step();
    /// assert_eq!(cpu.get(Flag::IllegalHalt), true);
    ///```
    pub fn get(&self, flag: Flag) -> bool {
        self.flags & (1 << flag as u8) != 0
    }

    /// Sets the IP to the given value.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///    I::ldi(0xdf),
    ///    I::hlt(),
    /// ], vec![]);
    ///
    /// cpu.goto(2);
    /// cpu.step();
    /// assert_eq!(cpu.a(), 0x0);
    /// assert_eq!(cpu.ip(), 2);
    /// ```
    pub fn goto(&mut self, ip: u8) {
        self.ip = ip;
    }

    /// Returns the contents of the instruction pointer.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::ldi(0xef),
    ///     I::hlt()
    /// ], vec![]);
    ///
    /// cpu.step();
    /// assert_eq!(cpu.ip(), 2);
    /// ```
    pub fn ip(&self) -> u8 {
        self.ip
    }

    /// Returns the number of bytes in the RAM.
    pub fn len(&self) -> usize {
        self.ram.len()
    }

    /// Read the value at the given address in RAM.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::hlt(),
    /// ], vec![]);
    ///
    /// assert_eq!(cpu.read(0), Some(15));
    /// ```
    pub fn read(&mut self, adr: u8) -> Option<u8> {
        if adr as usize >= self.ram.len() {
            return None
        }

        Some(self.ram[adr as usize])
    }

    pub fn read_bytes(&self, adr: u8, len: u8) -> &[u8] {
        let start = adr as usize;
        let end = (adr as usize + len as usize).min(self.ram.len());

        &self.ram[start..end]
    }

    pub (super) fn set(&mut self, flag: Flag) {
       self.flags |= 1 << flag as u8;
    }

    /// Execute the next instruction in the program.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///    I::ldi(0x01),
    ///    I::jpz(0x00),
    ///    I::hlt(),
    /// ], vec![]);
    ///
    /// cpu.step();
    /// cpu.step();
    /// cpu.step();
    /// assert!(cpu.get(Flag::Halt));
    /// ```
    pub fn step(&mut self) {
        if self.get(Flag::Halt) || self.get(Flag::IllegalHalt) {
            return;
        }

        if let Some(instruction) = decode(self) {
            instruction.execute(self);

            if self.get(Flag::Halt) || self.get(Flag::IllegalHalt) {
                return;
            }

            let next_ip = instruction.next(self);
            if next_ip as usize >= self.ram.len() {
                self.set(Flag::IllegalHalt);
            } else {
                self.ip = next_ip;
            }
        } else {
            self.set(Flag::IllegalHalt);
        }
    }

    /// Use the given function to handle output.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    ///
    /// struct X(u8);
    ///
    /// let mut i = X(0);
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::out(),
    ///     I::out(),
    ///     I::hlt(),
    /// ], vec![]).with_out(move |id| {
    ///     println!("Value: {} ", id);
    /// });
    ///
    /// cpu.step();
    /// cpu.step();
    /// ```
    pub fn with_out<F>(mut self, out: F) -> Self
    where
        F: FnMut(u8) + 'static,
    {
        self.out = Box::from(out);
        self
    }

    pub (super) fn unset(&mut self, flag: Flag) {
       self.flags &= !(1 << flag as u8);
    }

    /// Write the given value to the given address in RAM.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    /// ], vec![]);
    ///
    /// cpu.write(0, 0x01);
    /// cpu.write(1, 0x96);
    /// cpu.write(2, 0x0f);
    /// cpu.step();
    ///
    /// assert_eq!(cpu.read(0), Some(0x01));
    /// assert_eq!(cpu.read(1), Some(0x96));
    /// assert_eq!(cpu.a(), 0x96);
    pub fn write(&mut self, adr: u8, val: u8) {
        if adr as usize >= self.ram.len() {
            let padding = vec![0; adr as usize - self.ram.len() + 1];
            self.ram.extend(padding);
        }

        self.ram[adr as usize] = val;
    }
}

fn decode(cpu: &mut Cpu) -> Option<I> {
    if let Some(opcode) = cpu.read(cpu.ip) {
        return match I::from_opcode(opcode) {
            IBuilder::Complete(instruction) => return Some(instruction),
            IBuilder::NeedsData(incomplete) => if let Some(id) = cpu.read(cpu.ip + 1) {
                Some(incomplete.with_data(id))
            } else {
                None
            }
            _ => None
        }
    }

    None
}

fn default_out(value: u8) {
    print!("{} ", value);
}
