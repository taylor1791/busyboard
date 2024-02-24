use super::{Cpu, Flag};

pub struct I(Instr);

impl I {
    /// No operation; simply increments the instruction pointer.
    /// ```
    /// use busyboard::eater::{Cpu, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::nop(),
    ///     I::hlt()
    /// ], vec![]);
    ///
    /// cpu.step();
    /// assert_eq!(cpu.ip(), 1);
    /// ```
    pub fn nop() -> Self {
        I(Instr::Nop(Nop))
    }

    /// Load the immediate value specified by the instruction data into register A.
    /// ```
    /// use busyboard::eater::{Cpu, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::ldi(0x0D),
    ///     I::hlt()
    /// ], vec![]);
    ///
    /// cpu.step();
    /// assert_eq!(cpu.a(), 0x0D);
    /// assert_eq!(cpu.ip(), 2);
    /// ```
    pub fn ldi(value: u8) -> Self {
        I(Instr::Ldi(Ldi(value)))
    }

    /// Load the value from RAM at the address specified by the instruction data into register A.
    /// ```
    /// use busyboard::eater::{Cpu, I};
    /// let mut cpu = Cpu::from_asm(vec![ I::lda(0x03) ], vec![0x0D, 0x0E, 0x0F]);
    /// cpu.step();
    /// assert_eq!(cpu.a(), 0x0E);
    /// assert_eq!(cpu.ip(), 2);
    /// ```
    pub fn lda(address: u8) -> Self {
        I(Instr::Lda(Lda(address)))
    }

    /// Store the contents of register A at the memory address specified by the instruction data.
    /// ```
    /// use busyboard::eater::{Cpu, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::lda(0x04),
    ///     I::sta(0x05)
    /// ], vec![0xCD, 0x00]);
    ///
    /// cpu.step();
    /// cpu.step();
    /// assert_eq!(cpu.read_bytes(0x04, 2), [0xCD, 0xCD]);
    /// assert_eq!(cpu.ip(), 4);
    /// ```
    pub fn sta(address: u8) -> Self {
        I(Instr::Sta(Sta(address)))
    }

    /// Calculate the sum of A and the value at the address specified by the instruction data and store the result in A.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::lda(0x07),
    ///     I::add(0x08),
    ///     I::add(0x06),
    /// ], vec![0x01, 0x3F, 0xC0]);
    ///
    /// cpu.step();
    /// cpu.step();
    /// assert_eq!(cpu.a(), 0xFF);
    /// assert!(!cpu.get(Flag::Carry));
    ///
    /// cpu.step();
    /// assert_eq!(cpu.a(), 0x00);
    /// // assert!(cpu.get(Flag::Carry));
    /// // assert_eq!(cpu.ip(), 6);
    /// ```
    pub fn add(address: u8) -> Self {
        I(Instr::Add(Add(address)))
    }

    //////////////////
    /// Calculate the difference of A and the value at the address specified by the instruction data and store the result in A.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::lda(0x07),
    ///     I::sub(0x08),
    ///     I::sub(0x09)
    /// ], vec![0x01, 0xC0, 0x3F, 0x82]);
    ///
    /// cpu.step();
    /// cpu.step();
    /// assert_eq!(cpu.a(), 0x81);
    /// assert!(!cpu.get(Flag::Carry));
    ///
    /// cpu.step();
    /// assert_eq!(cpu.a(), 0xFF);
    /// assert!(cpu.get(Flag::Carry));
    /// assert_eq!(cpu.ip(), 6);
    /// ```
    pub fn sub(address: u8) -> Self {
        I(Instr::Sub(Sub(address)))
    }

    /// Jump to the address specified by the instruction data.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::jmp(0x02),
    ///     I::jmp(0xCC)
    /// ], vec![0x15]);
    ///
    /// cpu.step();
    /// assert_eq!(cpu.ip(), 0x2);
    ///
    /// cpu.step();
    /// assert!(cpu.get(Flag::IllegalHalt));
    /// ```
    pub fn jmp(address: u8) -> Self {
        I(Instr::Jmp(Jmp(address)))
    }

    /// Jump to the address specified by the instruction data if the A register is zero.
    /// ```
    /// use busyboard::eater::{Cpu, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::ldi(0x01),
    ///     I::jpz(0x04),
    ///     I::ldi(0x00),
    ///     I::jpz(0x00),
    /// ], vec![]);
    ///
    /// cpu.step();
    /// cpu.step();
    /// assert_eq!(cpu.ip(), 4);
    ///
    /// cpu.step();
    /// cpu.step();
    /// assert_eq!(cpu.ip(), 0);
    /// ```
    pub fn jpz(address: u8) -> Self {
        I(Instr::Jpz(Jpz(address)))
    }

    /// Jump to the address specified by the instruction data if the carry flag is set.
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///    I::jpc(0xDD),
    ///    I::ldi(0xFF),
    ///    I::add(0x01),
    ///    I::jpc(0x00),
    ///    I::hlt()
    /// ], vec![]);
    ///
    /// cpu.step();
    /// assert_eq!(cpu.ip(), 0x02);
    ///
    /// cpu.step();
    /// cpu.step();
    /// cpu.step();
    /// assert_eq!(cpu.ip(), 0x00);
    /// ```
    pub fn jpc(address: u8) -> Self {
        I(Instr::Jpc(Jpc(address)))
    }

    /// Call the `out` function with the contents of register A.
    /// ```
    /// use busyboard::eater::{Cpu, I};
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let calls = Rc::new(RefCell::new(vec![]));
    /// let c = calls.clone();
    ///
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::ldi(0x0D),
    ///     I::out(),
    ///     I::hlt()
    /// ], vec![]).with_out(move |id| {
    ///     c.borrow_mut().push(id);
    /// });
    ///
    /// cpu.step();
    /// cpu.step();
    /// assert_eq!(cpu.ip(), 3);
    /// assert_eq!(*calls.borrow(), vec![0x0D]);
    /// ```
    pub fn out() -> Self {
        I(Instr::Out(Out))
    }

    /// Halt the CPU
    /// ```
    /// use busyboard::eater::{Cpu, Flag, I};
    /// let mut cpu = Cpu::from_asm(vec![
    ///     I::hlt()
    /// ], vec![]);
    ///
    /// assert!(!cpu.get(Flag::Halt));
    /// cpu.step();
    /// cpu.step();
    /// assert!(cpu.get(Flag::Halt));
    /// assert_eq!(cpu.ip(), 0);
    /// ```
    pub fn hlt() -> Self {
        I(Instr::Hlt(Hlt))
    }

    pub (super) fn from_opcode(opcode: u8) -> IBuilder {
        if opcode == Nop::opcode() {
            IBuilder::Complete(I(Instr::Nop(Nop)))
        } else if opcode == Ldi::opcode() {
            IBuilder::NeedsData(IWithoutData(Instr::Ldi(Ldi(0))))
        } else if opcode == Lda::opcode() {
            IBuilder::NeedsData(IWithoutData(Instr::Lda(Lda(0))))
        } else if opcode == Sta::opcode() {
            IBuilder::NeedsData(IWithoutData(Instr::Sta(Sta(0))))
        } else if opcode == Add::opcode() {
            IBuilder::NeedsData(IWithoutData(Instr::Add(Add(0))))
        } else if opcode == Sub::opcode() {
            IBuilder::NeedsData(IWithoutData(Instr::Sub(Sub(0))))
        } else if opcode == Jmp::opcode() {
            IBuilder::NeedsData(IWithoutData(Instr::Jmp(Jmp(0))))
        } else if opcode == Jpz::opcode() {
            IBuilder::NeedsData(IWithoutData(Instr::Jpz(Jpz(0))))
        } else if opcode == Jpc::opcode() {
            IBuilder::NeedsData(IWithoutData(Instr::Jpc(Jpc(0))))
        } else if opcode == Out::opcode() {
            IBuilder::Complete(I(Instr::Out(Out)))
        } else if opcode == Hlt::opcode() {
            IBuilder::Complete(I(Instr::Hlt(Hlt)))
        } else {
            IBuilder::Invalid
        }
    }
}

pub (super) enum IBuilder {
    Invalid,
    Complete(I),
    NeedsData(IWithoutData),
}

pub (super) struct IWithoutData(Instr);

impl IWithoutData {
    pub (super) fn with_data(self, data: u8) -> I {
        match self.0 {
            Instr::Ldi(_) => I(Instr::Ldi(Ldi(data))),
            Instr::Lda(_) => I(Instr::Lda(Lda(data))),
            Instr::Sta(_) => I(Instr::Sta(Sta(data))),
            Instr::Add(_) => I(Instr::Add(Add(data))),
            Instr::Sub(_) => I(Instr::Sub(Sub(data))),
            Instr::Jmp(_) => I(Instr::Jmp(Jmp(data))),
            Instr::Jpz(_) => I(Instr::Jpz(Jpz(data))),
            Instr::Jpc(_) => I(Instr::Jpc(Jpc(data))),
            instr => I(instr),
        }
    }
}

pub (super) trait Instruction {
    fn assemble(&self) -> Vec<u8>;
    fn execute(&self, cpu: &mut Cpu);
    fn next(&self, cpu: &Cpu) -> u8;
}

enum Instr {
    Nop(Nop),
    Ldi(Ldi),
    Lda(Lda),
    Sta(Sta),
    Add(Add),
    Sub(Sub),
    Jmp(Jmp),
    Jpz(Jpz),
    Jpc(Jpc),
    Out(Out),
    Hlt(Hlt),
}

struct Nop;
impl Nop {
    fn opcode() -> u8 {
        0
    }
}

struct Ldi(u8);
impl Ldi {
    fn opcode() -> u8 {
        1
    }
}

struct Lda(u8);
impl Lda {
    fn opcode() -> u8 {
        2
    }
}

struct Sta(u8);
impl Sta {
    fn opcode() -> u8 {
        3
    }
}

struct Add(u8);
impl Add {
    fn opcode() -> u8 {
        4
    }
}

struct Sub(u8);
impl Sub {
    fn opcode() -> u8 {
        5
    }
}

struct Jmp(u8);
impl Jmp {
    fn opcode() -> u8 {
        6
    }
}

struct Jpz(u8);
impl Jpz {
    fn opcode() -> u8 {
        7
    }
}

struct Jpc(u8);
impl Jpc {
    fn opcode() -> u8 {
        8
    }
}

struct Out;
impl Out {
    fn opcode() -> u8 {
        14
    }
}

struct Hlt;
impl Hlt {
    fn opcode() -> u8 {
        15
    }
}

impl Instruction for I {
    fn assemble(&self) -> Vec<u8> {
        self.0.assemble()
    }

    fn execute(&self, cpu: &mut Cpu) {
        self.0.execute(cpu)
    }

    fn next(&self, cpu: &Cpu) -> u8 {
        self.0.next(cpu)
    }
}

impl Instruction for Instr {
    fn assemble(&self) -> Vec<u8> {
        match self {
            Instr::Nop(nop) => nop.assemble(),
            Instr::Ldi(ldi) => ldi.assemble(),
            Instr::Lda(lda) => lda.assemble(),
            Instr::Sta(sta) => sta.assemble(),
            Instr::Add(add) => add.assemble(),
            Instr::Sub(sub) => sub.assemble(),
            Instr::Jmp(jmp) => jmp.assemble(),
            Instr::Jpz(jpz) => jpz.assemble(),
            Instr::Jpc(jpc) => jpc.assemble(),
            Instr::Out(out) => out.assemble(),
            Instr::Hlt(hlt) => hlt.assemble(),
        }
    }

    fn execute(&self, cpu: &mut Cpu) {
        match self {
            Instr::Nop(nop) => nop.execute(cpu),
            Instr::Ldi(ldi) => ldi.execute(cpu),
            Instr::Lda(lda) => lda.execute(cpu),
            Instr::Sta(sta) => sta.execute(cpu),
            Instr::Add(add) => add.execute(cpu),
            Instr::Sub(sub) => sub.execute(cpu),
            Instr::Jmp(jmp) => jmp.execute(cpu),
            Instr::Jpz(jpz) => jpz.execute(cpu),
            Instr::Jpc(jpc) => jpc.execute(cpu),
            Instr::Out(out) => out.execute(cpu),
            Instr::Hlt(hlt) => hlt.execute(cpu),
        }
    }

    fn next(&self, cpu: &Cpu) -> u8 {
        match self {
            Instr::Nop(nop) => nop.next(cpu),
            Instr::Ldi(ldi) => ldi.next(cpu),
            Instr::Lda(lda) => lda.next(cpu),
            Instr::Sta(sta) => sta.next(cpu),
            Instr::Add(add) => add.next(cpu),
            Instr::Sub(sub) => sub.next(cpu),
            Instr::Jmp(jmp) => jmp.next(cpu),
            Instr::Jpz(jpz) => jpz.next(cpu),
            Instr::Jpc(jpc) => jpc.next(cpu),
            Instr::Out(out) => out.next(cpu),
            Instr::Hlt(hlt) => hlt.next(cpu),
        }
    }
}

impl Instruction for Nop {
    fn assemble(&self) -> Vec<u8> {
        vec![Nop::opcode()]
    }

    fn execute(&self, _cpu: &mut Cpu) {}

    fn next(&self, cpu: &Cpu) -> u8 {
        cpu.ip + 1
    }
}

impl Instruction for Ldi {
    fn assemble(&self) -> Vec<u8> {
        vec![Ldi::opcode(), self.0]
    }

    fn execute(&self, cpu: &mut Cpu) {
        cpu.a = self.0;
    }

    fn next(&self, cpu: &Cpu) -> u8 {
        cpu.ip + 2
    }
}

impl Instruction for Lda {
    fn assemble(&self) -> Vec<u8> {
        vec![Lda::opcode(), self.0]
    }

    fn execute(&self, cpu: &mut Cpu) {
        if let Some(a) = cpu.read(self.0) {
            cpu.a = a;
        }
    }

    fn next(&self, cpu: &Cpu) -> u8 {
        cpu.ip + 2
    }
}

impl Instruction for Sta {
    fn assemble(&self) -> Vec<u8> {
        vec![Sta::opcode(), self.0]
    }

    fn execute(&self, cpu: &mut Cpu) {
        cpu.write(self.0, cpu.a);
    }

    fn next(&self, cpu: &Cpu) -> u8 {
        cpu.ip + 2
    }
}

impl Instruction for Add {
    fn assemble(&self) -> Vec<u8> {
        vec![Add::opcode(), self.0]
    }

    fn execute(&self, cpu: &mut Cpu) {
        cpu.unset(Flag::Carry);

        if let Some(operand) = cpu.read(self.0) {
            if cpu.a > 0xFF - operand {
                cpu.set(Flag::Carry);
            }

            cpu.a = cpu.a.wrapping_add(operand);
        }
    }

    fn next(&self, cpu: &Cpu) -> u8 {
        cpu.ip + 2
    }
}

impl Instruction for Sub {
    fn assemble(&self) -> Vec<u8> {
        vec![Sub::opcode(), self.0]
    }

    fn execute(&self, cpu: &mut Cpu) {
        cpu.unset(Flag::Carry);

        if let Some(operand) = cpu.read(self.0) {
            if cpu.a < operand {
                cpu.set(Flag::Carry);
            }

            cpu.a = cpu.a.wrapping_sub(operand);
        }
    }

    fn next(&self, cpu: &Cpu) -> u8 {
        cpu.ip + 2
    }
}

impl Instruction for Jmp {
    fn assemble(&self) -> Vec<u8> {
        vec![Jmp::opcode(), self.0]
    }

    fn execute(&self, _cpu: &mut Cpu) {}

    fn next(&self, _cpu: &Cpu) -> u8 {
        self.0
    }
}

impl Instruction for Jpz { 
    fn assemble(&self) -> Vec<u8> {
        vec![Jpz::opcode(), self.0]
    }

    fn execute(&self, _cpu: &mut Cpu) {}

    fn next(&self, cpu: &Cpu) -> u8 {
        if cpu.a == 0 {
            self.0
        } else {
            cpu.ip + 2
        }
    }
}

impl Instruction for Jpc {
    fn assemble(&self) -> Vec<u8> {
        vec![Jpc::opcode(), self.0]
    }

    fn execute(&self, _cpu: &mut Cpu) {}

    fn next(&self, cpu: &Cpu) -> u8 {
        if cpu.get(Flag::Carry) {
            self.0
        } else {
            cpu.ip + 2
        }
    }
}

impl Instruction for Out {
    fn assemble(&self) -> Vec<u8> {
        vec![Out::opcode()]
    }

    fn execute(&self, cpu: &mut Cpu) {
        (cpu.out)(cpu.a);
    }

    fn next(&self, cpu: &Cpu) -> u8 {
        cpu.ip + 1
    }
}

impl Instruction for Hlt {
    fn assemble(&self) -> Vec<u8> {
        vec![Hlt::opcode()]
    }

    fn execute(&self, cpu: &mut Cpu) {
        cpu.set(Flag::Halt);
    }

    fn next(&self, cpu: &Cpu) -> u8 {
        cpu.ip
    }
}
