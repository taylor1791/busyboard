use busyboard::{
    eater::{Cpu, I},
    simulator::Simulator,
    ui::Ui,
};

fn main() -> std::io::Result<()> {
    // Count to 100 and then halt
    let cpu = Cpu::from_asm(vec![
        I::lda(14),
        I::add(13),
        I::sta(14),
        I::sub(15),
        I::jpz(12),
        I::jmp(0),
        I::hlt()
    ], vec![0x01, 0x00, 100]);

    let simulator = Simulator::from(cpu);
    Ui::new(std::time::Duration::from_millis(1000))
        .run(simulator)
}
