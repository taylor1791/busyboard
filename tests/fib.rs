use busyboard::eater::{Cpu, Flag, I};
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn eater_fibonacci() {
    let out = Rc::new(RefCell::new(Vec::<u8>::new()));
    let fib = out.clone();
    let mut cpu = Cpu::from_asm(vec![
        I::lda(23), // 0
        I::out(),   // 2
        I::add(24), // 3
        I::sta(25), // 5
        I::lda(24), // 7
        I::sta(23), // 9
        I::lda(25), // 11
        I::sta(24), // 13
        I::jpc(19), // 15
        I::jmp(0),  // 17
        I::lda(23), // 19
        I::out(),   // 21
        I::hlt(),   // 22
    ], vec![
      0x00, // n    ,f_n
      0x01, // n + 1, f_{n-1}
      0x00  // temp
    ]).with_out(move |x| fib.borrow_mut().push(x));

    while !cpu.get(Flag::Halt) && !cpu.get(Flag::IllegalHalt) {
        cpu.step();
    }

    assert_eq!(out.borrow().to_vec(), vec![
        0x00, 0x01, 0x01, 0x02, 0x03, 0x05, 0x08, 0x0d, 0x15, 0x22, 0x37, 0x59, 0x90, 0xe9
    ]);
}
