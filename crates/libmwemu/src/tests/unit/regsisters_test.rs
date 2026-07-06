use iced_x86::Register;

use crate::{regs64::U256, tests::helpers, *};

#[test]
pub fn regsisters_test() {
    helpers::setup();

    let mut emu = emu64();
    let regs = emu.regs_mut();

    // ====== 1. Direct 64 bits registers ======
    regs.rax = 0x123456789ABCDEF0;
    assert_eq!(regs.rax, 0x123456789ABCDEF0);

    regs.rbx = 0xCAFEBABECAFEBABE;
    assert_eq!(regs.rbx, 0xCAFEBABECAFEBABE);

    regs.rsp = 0x7FFF_FFFF_FFFF;
    regs.rbp = 0x5555_5555_5555;
    assert_eq!(regs.rsp, 0x7FFF_FFFF_FFFF);
    assert_eq!(regs.rbp, 0x5555_5555_5555);

    regs.cr0 = 0x80000011;
    assert_eq!(regs.cr0, 0x80000011);

    regs.msr = 0xDEADBEEF;
    assert_eq!(regs.msr, 0xDEADBEEF);

    regs.tr3 = 0x1234;
    assert_eq!(regs.tr3, 0x1234);

    regs.gs = 0xABCD;
    regs.fs = 0xDCBA;
    assert_eq!(regs.gs, 0xABCD);
    assert_eq!(regs.fs, 0xDCBA);

    // ====== 2. subregisters using methods ======
    regs.set_ax(0xBEEF);
    assert_eq!(regs.get_ax(), 0xBEEF);
    assert_eq!(regs.get_al(), 0xEF);
    assert_eq!(regs.get_ah(), 0xBE);

    regs.set_al(0x44);
    assert_eq!(regs.get_ax(), 0xBE44);
    assert_eq!(regs.get_al(), 0x44);

    regs.set_ah(0x22);
    assert_eq!(regs.get_ax(), 0x2244);
    assert_eq!(regs.get_ah(), 0x22);

    regs.set_eax(0x11223344);
    assert_eq!(regs.get_eax(), 0x11223344);
    assert_eq!(regs.get_ax(), 0x3344);
    assert_eq!(regs.get_al(), 0x44);

    regs.set_r8d(0x55667788);
    assert_eq!(regs.get_r8d(), 0x55667788);
    regs.set_r8w(0x99AA);
    assert_eq!(regs.get_r8w(), 0x99AA);
    regs.set_r8l(0xBB);
    assert_eq!(regs.get_r8l(), 0xBB);
    regs.set_r8h(0xCC);
    assert_eq!(regs.get_r8h(), 0xCC);

    // ====== 3. access by register name ======
    regs.set_by_name("eax", 0xAABBCCDD);
    assert_eq!(regs.get_by_name("eax"), 0xAABBCCDD);
    regs.set_by_name("al", 0xEE);
    assert_eq!(regs.get_by_name("al"), 0xEE);
    assert_eq!(regs.get_eax() & 0xFF, 0xEE);

    // ====== 4. XMM ======
    let xmm_val: u128 = 0x112233445566778899AABBCCDDEEFF00;
    assert!(regs.is_xmm(Register::XMM1));
    regs.set_xmm_reg(Register::XMM1, xmm_val);
    assert_eq!(regs.get_xmm_reg(Register::XMM1), xmm_val);
    regs.set_xmm_by_name("xmm1", xmm_val);
    assert_eq!(regs.get_xmm_by_name("xmm1"), xmm_val);

    // ====== 5. YMM ======
    let ymm_val = U256::from_big_endian(&[
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0xFF, 0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
        0x11, 0x00,
    ]);
    assert!(regs.is_ymm(Register::YMM1));
    regs.set_ymm_reg(Register::YMM1, ymm_val);
    assert_eq!(regs.get_ymm_reg(Register::YMM1), ymm_val);
    regs.set_ymm_by_name("ymm1", ymm_val);
    assert_eq!(regs.get_ymm_by_name("ymm1"), ymm_val);

    // ====== 6. Auxiliar methods ======
    regs.set_reg(Register::RAX, 0xCAFED00DDEADBEEF);
    assert_eq!(regs.get_reg(Register::RAX), 0xCAFED00DDEADBEEF);

    assert!(regs.is_reg("rax"));
    assert!(!regs.is_reg("xyz"));

    assert!(regs.is_xmm_by_name("xmm2"));
    assert!(regs.is_ymm_by_name("ymm3"));

    assert_eq!(regs.get_size(Register::RAX), 64);
    assert_eq!(regs.get_size(Register::AL), 8);

    // ====== 7. sanitize32 (should remove high part) ======
    regs.rax = 0xAABBCCDDEEFF7788;
    regs.sanitize32();
    assert_eq!(regs.rax, 0x00000000EEFF7788);

    // ====== 8. rand & clear ======
    regs.rand();
    let r1 = regs.rax;
    regs.rand();
    let r2 = regs.rax;
    assert_ne!(r1, r2); // rand should change something

    regs.clear::<64>();
    assert_eq!(regs.rax, 0);
    assert_eq!(regs.rbx, 0);
    assert_eq!(regs.rcx, 0);
}
