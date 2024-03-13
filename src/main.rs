use busyboard::{
    eater::{Cpu, I},
    simulator::Simulator,
    ui::Ui,
};

fn main() -> std::io::Result<()> {
    // Count to 100 and then halt
    let cpu = Cpu::from_asm(vec![
        I::lda(15),
        I::add(14),
        I::sta(15),
        I::out(),
        I::sub(16),
        I::jpz(13),
        I::jmp(0),
        I::hlt()
    ], vec![0x01, 0x00, 100]);

    let simulator = Simulator::from(cpu);
    Ui::new().run(simulator)
}
