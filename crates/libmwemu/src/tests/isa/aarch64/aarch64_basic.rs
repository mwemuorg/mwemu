use crate::tests::helpers;
use crate::*;

#[test]
fn aarch64_add_1_plus_1() {
    helpers::setup();

    // mov x0, #1       -> 0xd2800020
    // mov x1, #1       -> 0xd2800021
    // add x2, x0, x1   -> 0x8b010002
    let code: [u8; 12] = [
        0x20, 0x00, 0x80, 0xd2, 0x21, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x01, 0x8b,
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);

    emu.step(); // mov x0, #1
    assert_eq!(emu.regs_aarch64().x[0], 1);

    emu.step(); // mov x1, #1
    assert_eq!(emu.regs_aarch64().x[1], 1);

    emu.step(); // add x2, x0, x1
    assert_eq!(emu.regs_aarch64().x[2], 2);
}

#[test]
fn aarch64_sub_sets_flags() {
    helpers::setup();

    // mov x0, #5       -> 0xd28000a0
    // mov x1, #5       -> 0xd28000a1
    // subs x2, x0, x1  -> 0xeb010002
    let code: [u8; 12] = [
        0xa0, 0x00, 0x80, 0xd2, 0xa1, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x01, 0xeb,
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);

    emu.step(); // mov x0, #5
    emu.step(); // mov x1, #5
    emu.step(); // subs x2, x0, x1

    assert_eq!(emu.regs_aarch64().x[2], 0);
    assert!(emu.regs_aarch64().nzcv.z); // zero flag set
    assert!(!emu.regs_aarch64().nzcv.n); // not negative
    assert!(emu.regs_aarch64().nzcv.c); // ARM: carry = NOT borrow, so set when a >= b
}

#[test]
fn aarch64_str_ldr_stack() {
    helpers::setup();

    // sub sp, sp, #0x10    -> 0xd10043ff
    // mov x0, #42          -> 0xd2800540
    // str x0, [sp]         -> 0xf90003e0
    // mov x0, #0           -> 0xd2800000
    // ldr x1, [sp]         -> 0xf94003e1
    let code: [u8; 20] = [
        0xff, 0x43, 0x00, 0xd1, 0x40, 0x05, 0x80, 0xd2, 0xe0, 0x03, 0x00, 0xf9, 0x00, 0x00, 0x80,
        0xd2, 0xe1, 0x03, 0x40, 0xf9,
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);

    emu.step(); // sub sp, sp, #0x10
    emu.step(); // mov x0, #42
    assert_eq!(emu.regs_aarch64().x[0], 42);

    emu.step(); // str x0, [sp]
    emu.step(); // mov x0, #0
    assert_eq!(emu.regs_aarch64().x[0], 0);

    emu.step(); // ldr x1, [sp]
    assert_eq!(emu.regs_aarch64().x[1], 42);
}

#[test]
fn aarch64_branch_and_link() {
    helpers::setup();

    // bl #8             -> 0x94000002  (skip next instruction, jump +8)
    // mov x0, #0xdead   -> 0xd29bd5a0  (should be skipped)
    // mov x0, #0xbeef   -> 0xd297dde0  (branch target)
    let code: [u8; 12] = [
        0x02, 0x00, 0x00, 0x94, 0xa0, 0xd5, 0x9b, 0xd2, 0xe0, 0xdd, 0x97, 0xd2,
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);
    let base = emu.regs_aarch64().pc;

    emu.step(); // bl #8
    assert_eq!(emu.regs_aarch64().pc, base + 8); // jumped to 3rd instruction
    assert_eq!(emu.regs_aarch64().x[30], base + 4); // LR = return address

    emu.step(); // mov x0, #0xbeef
    assert_eq!(emu.regs_aarch64().x[0], 0xbeef);
}

#[test]
fn aarch64_cbz_taken() {
    helpers::setup();

    // mov x0, #0        -> 0xd2800000
    // cbz x0, #8        -> 0xb4000040  (skip next, jump +8)
    // mov x1, #0xdead   -> 0xd29bd5a1  (should be skipped)
    // mov x1, #1        -> 0xd2800021  (branch target)
    let code: [u8; 16] = [
        0x00, 0x00, 0x80, 0xd2, 0x40, 0x00, 0x00, 0xb4, 0xa1, 0xd5, 0x9b, 0xd2, 0x21, 0x00, 0x80,
        0xd2,
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);

    emu.step(); // mov x0, #0
    emu.step(); // cbz x0, #8 -> taken since x0 == 0

    let base = emu.cfg.code_base_addr;
    assert_eq!(emu.regs_aarch64().pc, base + 12); // jumped to 4th instruction

    emu.step(); // mov x1, #1
    assert_eq!(emu.regs_aarch64().x[1], 1);
}

/// AArch64 shellcode defined inline in Rust: loop that computes 1+2+3+4+5 = 15.
///
/// ```asm
/// mov x0, #0         ; accumulator
/// mov x1, #5         ; counter
/// loop:
///   add x0, x0, x1  ; x0 += x1
///   sub x1, x1, #1  ; x1--
///   cbnz x1, #-8    ; if x1 != 0, jump back to add
/// ```
///
/// 2 setup instructions + 3×5 loop iterations = 17 steps total.
/// Expected on exit: x0 == 15, x1 == 0.
#[test]
fn aarch64_shellcode_sum_loop() {
    helpers::setup();

    // Verified little-endian encodings:
    //   mov x0, #0       -> D2 80 00 00  (0xd2800000)
    //   mov x1, #5       -> D2 80 00 A1  (0xd28000a1)
    //   add x0, x0, x1   -> 8B 01 00 00  (0x8b010000)
    //   sub x1, x1, #1   -> D1 00 04 21  (0xd1000421)
    //   cbnz x1, #-8     -> B5 FF FF C1  (0xb5ffffc1)
    //                        offset=-8: imm19=-2 -> 0x7FFFE, branches back to add
    let code: [u8; 20] = [
        0x00, 0x00, 0x80, 0xd2, // mov x0, #0
        0xa1, 0x00, 0x80, 0xd2, // mov x1, #5
        0x00, 0x00, 0x01, 0x8b, // loop: add x0, x0, x1
        0x21, 0x04, 0x00, 0xd1, //       sub x1, x1, #1
        0xc1, 0xff, 0xff, 0xb5, //       cbnz x1, #-8
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);

    // 2 setup + 3 instructions x 5 iterations = 17 steps
    for _ in 0..17 {
        emu.step();
    }

    assert_eq!(
        emu.regs_aarch64().x[0],
        15,
        "x0 should be 1+2+3+4+5=15, got {}",
        emu.regs_aarch64().x[0]
    );
    assert_eq!(
        emu.regs_aarch64().x[1],
        0,
        "x1 (counter) should be 0 after loop"
    );
}

/// Same as `aarch64_shellcode_sum_loop` but using `run_to` instead of per-step calls.
/// Verifies AArch64 works correctly with the high-level execution API.
#[test]
fn aarch64_shellcode_sum_loop_run_to() {
    helpers::setup();

    let code: [u8; 20] = [
        0x00, 0x00, 0x80, 0xd2, // mov x0, #0
        0xa1, 0x00, 0x80, 0xd2, // mov x1, #5
        0x00, 0x00, 0x01, 0x8b, // loop: add x0, x0, x1
        0x21, 0x04, 0x00, 0xd1, //       sub x1, x1, #1
        0xc1, 0xff, 0xff, 0xb5, //       cbnz x1, #-8
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);

    emu.run_to(16).unwrap_or_else(|e| {
        panic!(
            "run_to(17) failed: {} (pos={} pc=0x{:x})",
            e,
            emu.pos,
            emu.regs_aarch64().pc
        );
    });

    assert!(
        emu.pos >= 16,
        "expected >= 17 instructions, got pos={}",
        emu.pos
    );
    assert_eq!(
        emu.regs_aarch64().x[0],
        15,
        "x0 should be 15, got {}",
        emu.regs_aarch64().x[0]
    );
    assert_eq!(emu.regs_aarch64().x[1], 0, "x1 should be 0 after loop");
}

#[test]
fn high_linux_mmap_branch_executes_guest_instructions() {
    use super::{emu_with, step_ok};
    use crate::arch::OperatingSystem;
    use crate::maps::mem64::Permission;

    let mut emu = emu_with(&[0xd61f_0000]); // br x0
    let address = 0x7ff0_0000_0000;
    emu.os = OperatingSystem::Linux;
    emu.maps
        .create_map("mmap_high", address, 4096, Permission::READ_WRITE_EXECUTE)
        .expect("guest code");
    emu.maps.write_dword(address, 0xd280_0541); // mov x1, #42
    emu.regs_aarch64_mut().x[0] = address;
    step_ok(&mut emu);
    assert_eq!(emu.pc(), address);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[1], 42);
}

#[test]
fn thread_pointer_survives_system_register_roundtrip() {
    use super::{emu_with, step_ok};

    let mut emu = emu_with(&[0xd51b_d053, 0xd53b_d044]); // msr tpidr_el0, x19; mrs x4, tpidr_el0
    emu.regs_aarch64_mut().x[19] = 0x1234_5000;
    step_ok(&mut emu);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[4], 0x1234_5000);
}

#[test]
fn cache_maintenance_observes_mapped_lines() {
    use super::{data_map, emu_with, step_ok};

    let mut emu = emu_with(&[0xd53b_0021, 0xd50b_7b20, 0xd50b_7520, 0xd50b_7520]);
    emu.regs_aarch64_mut().x[0] = data_map(&mut emu) + 7;
    step_ok(&mut emu); // mrs x1, ctr_el0
    assert_eq!(emu.regs_aarch64().x[1], 0x0004_0004);
    step_ok(&mut emu); // dc cvau, x0
    step_ok(&mut emu); // ic ivau, x0
    emu.regs_aarch64_mut().x[0] = 0;
    assert!(!emu.step());
}
