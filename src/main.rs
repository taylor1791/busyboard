use busyboard::{
    eater::{Cpu, I},
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

    Ui::new(cpu).start()
}
