use crate::tests::helpers;
use crate::*;

#[test]
// arithmetic calculations on an 64bits elf
pub fn elf64lin_cpu_arithmetics1() {
    helpers::setup();

    let mut emu = emu64();
    emu.load_code(&helpers::test_data_path("elf64lin_cpu_arithmetics1.bin"));

    assert_eq!(emu.flags().dump(), 0x202); // initial flags (match with gdb linux)

    emu.run_to(5); // position 5 is emulated
    assert_eq!(emu.regs().rax, 3);
    assert_eq!(emu.flags().dump(), 0x206);

    emu.run_to(6); // dec ax
    assert_eq!(emu.regs().rax, 2);
    assert_eq!(emu.flags().dump(), 0x202);

    emu.run_to(8); // last dec rax zero reached
    assert_eq!(emu.regs().rax, 0);
    assert_eq!(emu.flags().dump(), 0x246);

    emu.run_to(11); // neg ax
    assert_eq!(emu.regs().rax, 0x1122334455668888);
    assert_eq!(emu.flags().dump(), 0x297); // [ CF PF AF SF IF ]

    emu.run_to(14); // sar al, 1
    assert_eq!(emu.regs().rax, 0xffffffff556688c4);
    assert_eq!(emu.flags().dump(), 0x292);

    emu.run_to(23); // shl ax, 1
    assert_eq!(emu.regs().rax, 0x15596260);
    assert_eq!(emu.flags().dump(), 0xa17);

    emu.run_to(29); // shl rax, cl
    assert_eq!(emu.regs().rax, 0x55658980);
    assert_eq!(emu.flags().dump(), 0x212);

    emu.run_to(30); // shr al, 1
    assert_eq!(emu.regs().rax, 0x55658940);
    assert_eq!(emu.flags().dump(), 0xa12);

    emu.run_to(31); // shr ax, 1
    assert_eq!(emu.regs().rax, 0x556544a0);
    assert_eq!(emu.flags().dump(), 0xa16);

    emu.run_to(40); // imul eax
    assert_eq!(emu.regs().rax, 0x21000000);
    assert_eq!(emu.flags().dump(), 0xa17); // [ CF PF AF IF OF ]

    emu.run_to(41); // imul rax
    assert_eq!(emu.regs().rax, 0x441000000000000);
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]

    emu.run_to(43); // imul eax, eax
    assert_eq!(emu.regs().rax, 0);
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]

    emu.run_to(45); // imul rax, rax
    assert_eq!(emu.regs().rax, 0x1eace4a3c82fb840);
    assert_eq!(emu.flags().dump(), 0xa17); // [ CF PF AF IF OF ]

    emu.run_to(48); // imul  rax,2
    assert_eq!(emu.regs().rax, 0x120bdc200);
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]

    emu.run_to(49); // rcl al, 1
    assert_eq!(emu.regs().rax, 0x120bdc200);
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]

    emu.run_to(50); // rcl ax, 1
    assert_eq!(emu.regs().rax, 0x120bd8400);
    assert_eq!(emu.flags().dump(), 0x217); // [ CF PF AF IF ]

    emu.run_to(52); // rcl   rax,1
    assert_eq!(emu.regs().rax, 0x82f61002); // ERROR
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]

    emu.run_to(58); // rcr   ax,1
    assert_eq!(emu.regs().rax, 0x82f60800);
    assert_eq!(emu.flags().dump(), 0x217); // [ CF PF AF IF ]

    emu.run_to(64);
    assert_eq!(emu.regs().rax, 0x60bd8200);
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]

    emu.run_to(69);
}

#[test]
// arithmetic calculations on an 64bits elf
pub fn elf64lin_cpu_arithmetics2() {
    helpers::setup();

    let mut emu = emu64();
    emu.load_code(&helpers::test_data_path("elf64lin_cpu_arithmetics2.bin"));
    emu.flags_mut().f_if = true;
    emu.run_to(790022);

    assert_eq!(emu.regs().rax, 0xffffff03);
    assert_eq!(emu.regs().rbx, 0x100);
    assert_eq!(emu.regs().rcx, 0);
    assert_eq!(emu.regs().rdx, 0);
    assert_eq!(emu.regs().rsp, 0x402868);
    assert_eq!(emu.regs().rbp, 0x402868);
    assert_eq!(emu.regs().rip, 0x4013c1);

    assert!(emu.flags().dump() == 0x206);
}

#[test]
// arithmetic calculations on an 64bits elf
pub fn elf64lin_cpu_arithmetics3() {
    helpers::setup();

    let mut emu = emu64();
    emu.load_code(&helpers::test_data_path("elf64lin_cpu_arithmetics3.bin"));

    emu.run_to(1513234);

    assert_eq!(emu.regs().rax, 0xffffffffffffffff);
    assert_eq!(emu.regs().rbx, 0xffffffffffffffff);
    assert_eq!(emu.regs().rcx, 0);
    assert_eq!(emu.regs().rdx, 0);
    assert_eq!(emu.regs().rsi, 0x40204b);
    assert_eq!(emu.regs().rbp, 0);
    assert_eq!(emu.regs().rip, 0x401167);
}

#[test]
// arithmetic calculations on an 64bits elf
pub fn elf64lin_cpu_arithmetics4() {
    helpers::setup();

    let mut emu = emu64();
    emu.load_code(&helpers::test_data_path("elf64lin_cpu_arithmetics4.bin"));

    emu.run_to(294);

    assert_eq!(emu.regs().rax, 0x14);
    assert_eq!(emu.regs().rbx, 0x14);
    assert_eq!(emu.regs().rcx, 0);
    assert_eq!(emu.regs().rdx, 0);
    assert_eq!(emu.regs().rsi, 0x3333333333333333);
    assert_eq!(emu.regs().rdi, 0x4444444444444444);
    assert_eq!(emu.regs().rbp, 0);
    assert_eq!(emu.regs().r8, 0x5555555555555555);
    assert_eq!(emu.regs().r9, 0x6666666666666666);
    assert_eq!(emu.regs().r10, 0x7777777777777777);
    assert_eq!(emu.regs().r11, 0x8888888888888888);
    assert_eq!(emu.regs().r12, 0x9999999999999999);
    assert_eq!(emu.regs().r13, 0xaaaaaaaaaaaaaaaa);
    assert_eq!(emu.regs().r14, 0xbbbbbbbbbbbbbbbb);
    assert_eq!(emu.regs().r15, 0xcccccccccccccccc);
    assert_eq!(emu.regs().rip, 0x401188);
}
