use std::cell::RefCell;
use std::rc::Rc;

use iced_x86::{Code, Encoder, Instruction, MemoryOperand, OpCodeOperandKind, Register};
use itertools::iproduct;

use crate::arch::OperatingSystem;
use crate::arch::x86::regs::{Regs64, U256};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::utils::bits::{low_bits_u64, low_bits_u128};
use crate::{emu32, emu64};

const DATA: u64 = 0x3000_0000;
const PAGE: u64 = 0x1000;

fn prepare(is_x64: bool, code: &[u8]) -> Emu {
    let mut emu = if is_x64 { emu64() } else { emu32() };
    emu.os = OperatingSystem::Linux;
    emu.cfg.console_enabled = false;
    emu.cfg.soft_exception_recovery = false;
    emu.cfg.max_instructions = Some(128);
    emu.cfg.enable_threading = false;
    emu.disable_ctrlc();
    let mut bytes = code.to_vec();
    bytes.resize(64, 0x90);
    emu.load_code_bytes(&bytes).unwrap();
    emu.maps
        .create_map("data", DATA, PAGE, Permission::READ_WRITE)
        .unwrap();
    emu.regs_mut().rdi = DATA;
    emu.regs_mut().rsi = DATA + 0x100;
    emu
}

fn encode(is_x64: bool, instruction: Instruction) -> Vec<u8> {
    let mut encoder = Encoder::new(if is_x64 { 64 } else { 32 });
    encoder.encode(&instruction, 0).unwrap();
    encoder.take_buffer()
}

struct PackedRegisters {
    dst: Register,
    src: Register,
    bits: u32,
}

/// Encode a named MMX/SSE instruction with either a register source or [EDI/RDI].
fn prepare_packed(is_x64: bool, code: Code, memory: bool) -> (Emu, PackedRegisters) {
    let registers = match code.op_code().op0_kind() {
        OpCodeOperandKind::mm_reg => PackedRegisters {
            dst: Register::MM0,
            src: Register::MM1,
            bits: u64::BITS,
        },
        OpCodeOperandKind::xmm_reg => PackedRegisters {
            dst: Register::XMM0,
            src: Register::XMM1,
            bits: u128::BITS,
        },
        kind => panic!("expected a packed destination for {code:?}, got {kind:?}"),
    };
    let instruction = if memory {
        let address = if is_x64 { Register::RDI } else { Register::EDI };
        Instruction::with2(code, registers.dst, MemoryOperand::with_base(address)).unwrap()
    } else {
        Instruction::with2(code, registers.dst, registers.src).unwrap()
    };
    (prepare(is_x64, &encode(is_x64, instruction)), registers)
}

fn prepare_instruction(is_x64: bool, instruction: Instruction) -> Emu {
    prepare(is_x64, &encode(is_x64, instruction))
}

fn packed(lanes: &[i64], bits: u32) -> u128 {
    let mask = (1u128 << bits) - 1;
    lanes.iter().enumerate().fold(0, |value, (i, lane)| {
        value | ((*lane as u128 & mask) << (i as u32 * bits))
    })
}

#[test]
fn arithmetic_shift_keeps_signed_lanes_separate() {
    for (is_x64, code) in iproduct!(
        [false, true],
        [Code::Psraw_mm_mmm64, Code::Psraw_xmm_xmmm128]
    ) {
        let (mut emu, registers) = prepare_packed(is_x64, code, false);
        let lanes = (registers.bits / u16::BITS) as usize;
        let input = [-2, 4, -8, 16, -32, 64, -128, 256];
        let expected = [-1, 2, -4, 8, -16, 32, -64, 128];
        emu.regs_mut()
            .set_xmm_reg(registers.dst, packed(&input[..lanes], u16::BITS));
        emu.regs_mut().set_xmm_reg(registers.src, 1);
        assert!(emu.step(), "{code:?} x64={is_x64}");
        assert_eq!(
            emu.regs().get_xmm_reg(registers.dst),
            packed(&expected[..lanes], u16::BITS),
            "{code:?} x64={is_x64}"
        );
    }
}

#[test]
fn arithmetic_shift_uses_full_low_qword_count() {
    let cases = [
        ([Code::Psraw_mm_mmm64, Code::Psraw_xmm_xmmm128], u16::BITS),
        ([Code::Psrad_mm_mmm64, Code::Psrad_xmm_xmmm128], u32::BITS),
    ];
    for (instructions, bits) in cases {
        for (is_x64, code, count) in
            iproduct!([false, true], instructions, [16u64, 32, 255, 256, u64::MAX])
        {
            let (mut emu, registers) = prepare_packed(is_x64, code, false);
            let input: Vec<_> = (0..registers.bits / bits)
                .map(|i| if i % 2 == 0 { -32768 } else { 16384 })
                .collect();
            let expected: Vec<_> = input
                .iter()
                .map(|v| v >> count.min((bits - 1) as u64))
                .collect();
            emu.regs_mut()
                .set_xmm_reg(registers.dst, packed(&input, bits));
            emu.regs_mut().set_xmm_reg(registers.src, count as u128);
            assert!(emu.step(), "{code:?} x64={is_x64} count={count}");
            assert_eq!(
                emu.regs().get_xmm_reg(registers.dst),
                packed(&expected, bits),
                "{code:?} x64={is_x64} count={count}"
            );
        }
    }
}

#[test]
fn packed_saturation_keeps_both_sources_for_mmx_and_xmm() {
    struct PackCase<'a> {
        instructions: [Code; 2],
        input_bits: u32,
        dst: &'a [i64],
        src: &'a [i64],
        expected_dst: &'a [i64],
        expected_src: &'a [i64],
    }
    let word_min = i16::MIN as i64;
    let word_max = i16::MAX as i64;
    let byte_min = i8::MIN as i64;
    let byte_max = i8::MAX as i64;
    let unsigned_byte_max = u8::MAX as i64;
    let cases = [
        PackCase {
            instructions: [Code::Packssdw_mm_mmm64, Code::Packssdw_xmm_xmmm128],
            input_bits: u32::BITS,
            dst: &[40_000, -40_000, 100, -100],
            src: &[word_max, word_min, word_max + 1, word_min - 1],
            expected_dst: &[word_max, word_min, 100, -100],
            expected_src: &[word_max, word_min, word_max, word_min],
        },
        PackCase {
            instructions: [Code::Packsswb_mm_mmm64, Code::Packsswb_xmm_xmmm128],
            input_bits: u16::BITS,
            dst: &[
                300,
                -300,
                100,
                -100,
                byte_max,
                byte_min,
                byte_max + 1,
                byte_min - 1,
            ],
            src: &[-1, 0, 1, 255, -256, 126, -127, 256],
            expected_dst: &[
                byte_max, byte_min, 100, -100, byte_max, byte_min, byte_max, byte_min,
            ],
            expected_src: &[-1, 0, 1, byte_max, byte_min, 126, -127, byte_max],
        },
        PackCase {
            instructions: [Code::Packuswb_mm_mmm64, Code::Packuswb_xmm_xmmm128],
            input_bits: u16::BITS,
            dst: &[300, -300, 100, -100, 127, -128, unsigned_byte_max + 1, -1],
            src: &[1, 2, 3, 4, 5, 6, 7, 8],
            expected_dst: &[unsigned_byte_max, 0, 100, 0, 127, 0, unsigned_byte_max, 0],
            expected_src: &[1, 2, 3, 4, 5, 6, 7, 8],
        },
    ];
    for case in &cases {
        for (is_x64, code) in iproduct!([false, true], case.instructions) {
            let (mut emu, registers) = prepare_packed(is_x64, code, false);
            let lanes = (registers.bits / case.input_bits) as usize;
            emu.regs_mut()
                .set_xmm_reg(registers.dst, packed(&case.dst[..lanes], case.input_bits));
            emu.regs_mut()
                .set_xmm_reg(registers.src, packed(&case.src[..lanes], case.input_bits));
            let expected: Vec<_> = case.expected_dst[..lanes]
                .iter()
                .chain(&case.expected_src[..lanes])
                .copied()
                .collect();
            assert!(emu.step(), "{code:?} x64={is_x64}");
            assert_eq!(
                emu.regs().get_xmm_reg(registers.dst),
                packed(&expected, case.input_bits / 2),
                "{code:?} x64={is_x64}"
            );
        }
    }
}

#[test]
fn implicit_and_non_temporal_stores_apply_write_hooks() {
    let cases: &[(&str, &[u8], u32, u128)] = &[
        (
            "movntdq",
            &[0x66, 0x0f, 0xe7, 0x07],
            128,
            0x8877665544332211_1122334455667788,
        ),
        (
            "movntps",
            &[0x0f, 0x2b, 0x07],
            128,
            0x8877665544332211_1122334455667788,
        ),
        (
            "movntpd",
            &[0x66, 0x0f, 0x2b, 0x07],
            128,
            0x8877665544332211_1122334455667788,
        ),
        ("movhps", &[0x0f, 0x17, 0x07], 64, 0x8877665544332211),
        ("movlps", &[0x0f, 0x13, 0x07], 64, 0x1122334455667788),
        ("push", &[0x50], 64, 0x1122334455667788),
    ];
    for &(name, code, bits, expected) in cases {
        let mut emu = prepare(true, code);
        emu.regs_mut().rax = 0x1122334455667788;
        emu.regs_mut().rsp = DATA + 8;
        emu.regs_mut()
            .set_xmm_reg(Register::XMM0, 0x8877665544332211_1122334455667788);
        let events = Rc::new(RefCell::new(Vec::new()));
        let observed = Rc::clone(&events);
        let pc = emu.pc();
        emu.set_hook_on_memory_write(move |emu, ip, address, size, value| {
            assert_eq!(emu.maps.read_bytes(DATA, 16).as_ref(), &[0; 16]);
            observed.borrow_mut().push((ip, address, size, value));
            !value
        });
        assert!(emu.step(), "{name}");
        assert_eq!(*events.borrow(), vec![(pc, DATA, bits, expected)], "{name}");
        assert_eq!(
            emu.maps.read_bytes(DATA, bits as usize / 8).as_ref(),
            &(!expected).to_le_bytes()[..bits as usize / 8],
            "{name}"
        );
    }
}

#[test]
fn ymm_hook_observes_and_replaces_both_halves_before_store() {
    let mut emu = prepare(true, &[0xc5, 0xfe, 0x7f, 0x07]);
    let input: Vec<_> = (0..32).collect();
    emu.regs_mut()
        .set_ymm_reg(Register::YMM0, U256::from_little_endian(&input));
    let events = Rc::new(RefCell::new(Vec::new()));
    let observed = Rc::clone(&events);
    let pc = emu.pc();
    emu.set_hook_on_memory_write(move |emu, ip, address, size, value| {
        assert_eq!(emu.maps.read_bytes(DATA, 32).as_ref(), &[0; 32]);
        observed.borrow_mut().push((ip, address, size, value));
        !value
    });
    assert!(emu.step());
    let expected: Vec<_> = input
        .chunks_exact(16)
        .enumerate()
        .map(|(i, bytes)| {
            (
                pc,
                DATA + i as u64 * 16,
                128,
                u128::from_le_bytes(bytes.try_into().unwrap()),
            )
        })
        .collect();
    assert_eq!(*events.borrow(), expected);
    assert_eq!(
        emu.maps.read_bytes(DATA, 32).as_ref(),
        input.iter().map(|v| !v).collect::<Vec<_>>()
    );
}

#[test]
fn simd_stores_fail_without_partial_write() {
    for code in [&[0xf3, 0x0f, 0x7f, 0x07][..], &[0xc5, 0xfe, 0x7f, 0x07][..]] {
        let mut emu = prepare(true, code);
        emu.maps
            .create_map("read_only", DATA + PAGE, PAGE, Permission::READ)
            .unwrap();
        emu.regs_mut().rdi = DATA + PAGE - 8;
        emu.regs_mut().set_xmm_reg(Register::XMM0, u128::MAX);
        emu.regs_mut()
            .set_ymm_reg(Register::YMM0, U256::from_little_endian(&[0xff; 32]));
        let pc = emu.pc();
        assert!(!emu.step());
        assert_eq!(emu.pc(), pc);
        assert_eq!(emu.maps.read_bytes(DATA + PAGE - 8, 8).as_ref(), &[0; 8]);
    }
}

#[test]
fn rep_fault_preserves_failed_iteration_and_instruction_pointer() {
    for (is_x64, width, copy, cached) in
        iproduct!([false, true], [1u32, 2, 4, 8], [false, true], [false, true])
    {
        if width == 8 && !is_x64 {
            continue;
        }
        let mut code = vec![0xf3];
        if width == 2 {
            code.push(0x66);
        } else if width == 8 {
            code.push(0x48);
        }
        code.push(match (copy, width) {
            (true, 1) => 0xa4,
            (true, _) => 0xa5,
            (false, 1) => 0xaa,
            _ => 0xab,
        });
        let len = code.len() as u64;
        let mut emu = prepare(is_x64, &code);
        emu.maps
            .create_map("read_only", DATA + PAGE, PAGE, Permission::READ)
            .unwrap();
        emu.regs_mut().rdi = DATA + PAGE - width as u64;
        emu.regs_mut().rcx = 3;
        emu.regs_mut().rax = 0x42;
        let pc = emu.pc();
        if cached {
            let _ = emu.run(Some(pc + len));
        } else {
            assert!(!emu.step());
        }
        assert_eq!(
            emu.regs().rcx,
            2,
            "x64={is_x64} width={width} copy={copy} cached={cached}"
        );
        assert_eq!(emu.regs().rdi, DATA + PAGE);
        assert_eq!(emu.pc(), pc);
        assert!(
            emu.maps
                .protect_memory(DATA + PAGE, PAGE, 4, Permission::READ_WRITE)
                .is_ok()
        );
        if cached {
            assert!(emu.run(Some(pc + len)).is_ok());
        } else {
            assert!(emu.step());
        }
        assert_eq!(emu.regs().rcx, 0);
        assert_eq!(emu.regs().rdi, DATA + PAGE + 2 * width as u64);
        assert_eq!(emu.pc(), pc + len);
    }
}

#[test]
fn skipped_instructions_keep_hook_pc_and_cached_decode_in_sync() {
    for (is_x64, cached) in iproduct!([false, true], [false, true]) {
        let mut emu = prepare(is_x64, &[0x90, 0xb8, 0x78, 0x56, 0x34, 0x12]);
        let pc = emu.pc();
        let events = Rc::new(RefCell::new(Vec::new()));
        let observed = Rc::clone(&events);
        emu.set_hook_on_pre_instruction(move |emu, ip, ins, _| {
            observed
                .borrow_mut()
                .push((ip, ins.as_x86().ip(), emu.pc()));
            ip != pc
        });
        if cached {
            emu.run(Some(pc + 6)).unwrap();
        } else {
            assert!(emu.step());
            assert!(emu.step());
        }
        assert_eq!(
            *events.borrow(),
            vec![(pc, pc, pc), (pc + 1, pc + 1, pc + 1)]
        );
        assert_eq!(emu.pc(), pc + 6);
        assert_eq!(emu.regs().get_eax(), 0x12345678);
    }
}

#[test]
fn logical_shifts_handle_large_counts_and_mmx_memory_width() {
    struct ShiftCase {
        instructions: [Code; 2],
        lane_bits: u32,
        left: bool,
    }
    let cases = [
        ShiftCase {
            instructions: [Code::Psrlw_mm_mmm64, Code::Psrlw_xmm_xmmm128],
            lane_bits: u16::BITS,
            left: false,
        },
        ShiftCase {
            instructions: [Code::Psrld_mm_mmm64, Code::Psrld_xmm_xmmm128],
            lane_bits: u32::BITS,
            left: false,
        },
        ShiftCase {
            instructions: [Code::Psrlq_mm_mmm64, Code::Psrlq_xmm_xmmm128],
            lane_bits: u64::BITS,
            left: false,
        },
        ShiftCase {
            instructions: [Code::Psllw_mm_mmm64, Code::Psllw_xmm_xmmm128],
            lane_bits: u16::BITS,
            left: true,
        },
        ShiftCase {
            instructions: [Code::Pslld_mm_mmm64, Code::Pslld_xmm_xmmm128],
            lane_bits: u32::BITS,
            left: true,
        },
        ShiftCase {
            instructions: [Code::Psllq_mm_mmm64, Code::Psllq_xmm_xmmm128],
            lane_bits: u64::BITS,
            left: true,
        },
    ];
    for case in &cases {
        for (is_x64, code, count, memory) in iproduct!(
            [false, true],
            case.instructions,
            [0u64, 1, 15, 16, 31, 32, 63, 64, 255, 256, u64::MAX],
            [false, true]
        ) {
            let (mut emu, registers) = prepare_packed(is_x64, code, memory);
            let input = if registers.bits == u128::BITS {
                u128::MAX
            } else {
                u64::MAX as u128
            };
            emu.regs_mut().set_xmm_reg(registers.dst, input);
            // Upper qword must not participate in the shift count.
            emu.regs_mut()
                .set_xmm_reg(registers.src, count as u128 | (1u128 << u64::BITS));
            let addr = DATA + PAGE - (registers.bits / u8::BITS) as u64;
            emu.regs_mut().rdi = addr;
            assert!(emu.maps.write_qword(addr, count));
            if registers.bits == u128::BITS {
                assert!(
                    emu.maps
                        .write_qword(addr + size_of::<u64>() as u64, u64::MAX)
                );
            }
            assert!(
                emu.step(),
                "{code:?} x64={is_x64} count={count} memory={memory}"
            );
            let mask = u64::MAX >> (u64::BITS - case.lane_bits);
            let lane = if count >= case.lane_bits as u64 {
                0
            } else if case.left {
                (u64::MAX << count) & mask
            } else {
                mask >> count
            };
            let expected = (0..registers.bits / case.lane_bits).fold(0u128, |value, i| {
                value | ((lane as u128) << (i * case.lane_bits))
            });
            assert_eq!(
                emu.regs().get_xmm_reg(registers.dst),
                expected,
                "{code:?} x64={is_x64} count={count} memory={memory}"
            );
        }
    }
}

#[test]
fn cmpxchg_stores_observe_hooks_on_match_and_mismatch() {
    let cases: &[(&[u8], u32)] = &[
        (&[0x0f, 0xb0, 0x1f], 8),
        (&[0x66, 0x0f, 0xb1, 0x1f], 16),
        (&[0x0f, 0xb1, 0x1f], 32),
        (&[0x48, 0x0f, 0xb1, 0x1f], 64),
        (&[0x0f, 0xc7, 0x0f], 64),
        (&[0x48, 0x0f, 0xc7, 0x0f], 128),
    ];
    for ((code, bits), equal, writable) in iproduct!(cases, [false, true], [false, true]) {
        let mut emu = prepare(true, code);
        emu.regs_mut().rax = if equal { 7 } else { 9 };
        emu.regs_mut().rdx = 0;
        emu.regs_mut().rbx = 42;
        emu.regs_mut().rcx = 0;
        assert!(emu.maps.write_qword(DATA, 7));
        if !writable {
            emu.maps
                .protect_memory(DATA, PAGE, 2, Permission::READ)
                .unwrap();
        }
        let pc = emu.pc();
        let flags = emu.flags().dump();
        let events = Rc::new(RefCell::new(Vec::new()));
        let captured = events.clone();
        emu.set_hook_on_memory_write(move |emu, ip, addr, width, value| {
            assert_eq!(emu.maps.read_qword(DATA), Some(7));
            captured.borrow_mut().push((ip, addr, width, value));
            99
        });
        assert_eq!(emu.step(), writable);
        assert_eq!(
            *events.borrow(),
            vec![(pc, DATA, *bits, if equal { 42 } else { 7 })]
        );
        assert_eq!(
            emu.maps.read_byte(DATA),
            Some(if writable { 99 } else { 7 })
        );
        if writable {
            assert_eq!(emu.flag_zf(), equal);
            assert_eq!(emu.regs().rax, 7);
        } else {
            assert_eq!(emu.pc(), pc);
            assert_eq!(emu.flags().dump(), flags);
            assert_eq!(emu.regs().rax, if equal { 7 } else { 9 });
        }
    }
}

#[test]
fn push_width_failure_and_host_operations_keep_their_contracts() {
    for (is_x64, short, writable) in iproduct!([false, true], [false, true], [false, true]) {
        let code: &[u8] = if short {
            &[0x66, 0x6a, 0xff]
        } else {
            &[0x6a, 0xff]
        };
        let mut emu = prepare(is_x64, code);
        let bits = if short {
            16
        } else if is_x64 {
            64
        } else {
            32
        };
        let sp = DATA + (bits / 8) as u64;
        emu.set_sp(sp);
        let events = Rc::new(RefCell::new(Vec::new()));
        let captured = events.clone();
        emu.set_hook_on_memory_write(move |_, ip, addr, width, value| {
            captured.borrow_mut().push((ip, addr, width));
            value
        });
        // Host/bootstrap pushes and direct writes do not invoke CPU hooks.
        assert!(emu.stack_push16(123));
        emu.set_sp(sp);
        assert!(emu.maps.write_bytes(DATA, &[0; 8]));
        assert!(events.borrow().is_empty());
        if !writable {
            emu.maps
                .protect_memory(DATA, PAGE, 2, Permission::READ)
                .unwrap();
        }
        let pc = emu.pc();
        assert_eq!(emu.step(), writable);
        assert_eq!(*events.borrow(), vec![(pc, DATA, bits)]);
        assert_eq!(emu.sp(), if writable { DATA } else { sp });
        assert_eq!(
            emu.maps.read_byte(DATA),
            Some(if writable { 255 } else { 0 })
        );
        if !writable {
            assert_eq!(emu.pc(), pc);
        }
    }
}

#[test]
fn xmm_and_ymm_loads_fail_without_clobbering_destination() {
    for wide in [false, true] {
        let code: &[u8] = if wide {
            &[0xc5, 0xfe, 0x6f, 0x07]
        } else {
            &[0xf3, 0x0f, 0x6f, 0x07]
        };
        let mut emu = prepare(true, code);
        emu.regs_mut().rdi = DATA + PAGE - 8;
        emu.regs_mut().set_ymm_reg(Register::YMM0, U256::from(123));
        let pc = emu.pc();
        assert!(!emu.step());
        assert_eq!(emu.pc(), pc);
        assert_eq!(emu.regs().get_ymm_reg(Register::YMM0), U256::from(123));
    }
}

#[test]
fn rep_read_fault_keeps_the_failed_element_pending() {
    for (is_x64, cached) in iproduct!([false, true], [false, true]) {
        let mut emu = prepare(is_x64, &[0xf3, 0xa4]);
        emu.regs_mut().rsi = DATA + PAGE - 1;
        emu.regs_mut().rcx = 3;
        assert!(emu.maps.write_byte(DATA + PAGE - 1, 42));
        let pc = emu.pc();
        if cached {
            let _ = emu.run(Some(pc + 2));
            assert!(emu.fault_count > 0);
        } else {
            assert!(!emu.step());
        }
        assert_eq!(emu.pc(), pc);
        assert_eq!(emu.regs().rcx, 2);
        assert_eq!(emu.regs().rsi, DATA + PAGE);
        assert_eq!(emu.regs().rdi, DATA + 1);
        assert_eq!(emu.maps.read_word(DATA), Some(42));
    }
}

#[test]
fn fpu_control_and_environment_stores_use_encoded_layout_and_hooks() {
    for (is_x64, short, writable) in iproduct!([false, true], [false, true], [false, true]) {
        let code: &[u8] = if short {
            &[0x66, 0xd9, 0x37]
        } else {
            &[0xd9, 0x37]
        };
        let mut emu = prepare(is_x64, code);
        let fpu = emu.fpu_mut();
        fpu.set_ctrl(0x300);
        fpu.status = 0x4100;
        fpu.tag = 0xfffc;
        fpu.ip = 0x1234_5678;
        fpu.code_segment = 0x23;
        fpu.opcode = 0x321;
        fpu.operand_ptr = 0x2345_6789;
        fpu.data_segment = 0x2b;
        let expected: Vec<u8> = if short {
            [0x300u16, 0x4100, 0xfffc, 0x5678, 0x23, 0x6789, 0x2b]
                .into_iter()
                .flat_map(u16::to_le_bytes)
                .collect()
        } else {
            [
                0x300u32,
                0x4100,
                0xfffc,
                0x1234_5678,
                0x0321_0023,
                0x2345_6789,
                0x2b,
            ]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect()
        };
        assert!(emu.maps.write_bytes(DATA, &[0xaa; 32]));
        if !writable {
            emu.maps
                .protect_memory(DATA, PAGE, 2, Permission::READ)
                .unwrap();
        }
        let observed = Rc::new(RefCell::new(Vec::new()));
        let captured = observed.clone();
        emu.set_hook_on_memory_write(move |emu, _, _, bits, value| {
            assert_eq!(emu.maps.read_bytes(DATA, 32).as_ref(), &[0xaa; 32]);
            captured
                .borrow_mut()
                .extend_from_slice(&value.to_le_bytes()[..bits as usize / 8]);
            value
        });
        let pc = emu.pc();
        assert_eq!(emu.step(), writable);
        assert_eq!(*observed.borrow(), expected);
        assert_eq!(emu.fpu().ip, 0x1234_5678);
        assert_eq!(emu.fpu().get_ctrl(), if writable { 0x33f } else { 0x300 });
        if writable {
            assert_eq!(emu.maps.read_bytes(DATA, expected.len()).as_ref(), expected);
        } else {
            assert_eq!(emu.pc(), pc);
        }
        assert_eq!(emu.maps.read_dword(DATA + 28), Some(0xaaaa_aaaa));
    }
    let mut emu = prepare(true, &[0xd9, 0x3f]); // fnstcw [rdi]
    emu.fpu_mut().set_ctrl(0x37f);
    emu.set_hook_on_memory_write(|_, _, addr, bits, value| {
        assert_eq!((addr, bits, value), (DATA, 16, 0x37f));
        0x123
    });
    assert!(emu.step());
    assert_eq!(emu.maps.read_word(DATA), Some(0x123));
}

#[test]
fn mmx_moves_use_the_architectural_register_width() {
    for (is_x64, dword) in iproduct!([false, true], [false, true]) {
        // MOVD/MOVQ mm0,[rdi]; MOVD/MOVQ [rsi],mm0.
        let code = [
            0x0f,
            if dword { 0x6e } else { 0x6f },
            0x07,
            0x0f,
            if dword { 0x7e } else { 0x7f },
            0x06,
        ];
        let mut emu = prepare(is_x64, &code);
        let size = if dword { 4 } else { 8 };
        let source = DATA + PAGE - size;
        emu.regs_mut().rdi = source;
        emu.regs_mut().rsi = DATA;
        assert!(emu.maps.write_bytes(
            source,
            &0x8877_6655_4433_2211u64.to_le_bytes()[..size as usize]
        ));
        assert!(emu.step());
        assert_eq!(emu.regs().get_size(Register::MM0), 64);
        assert_eq!(
            emu.regs().get_xmm_reg(Register::MM0),
            if dword {
                0x4433_2211
            } else {
                0x8877_6655_4433_2211
            }
        );
        assert!(emu.step());
        assert_eq!(
            emu.maps.read_bytes(DATA, size as usize),
            emu.maps.read_bytes(source, size as usize)
        );
    }
}

#[test]
fn skipped_rep_and_redirecting_hooks_leave_no_stale_decode_state() {
    for (is_x64, cached, redirect) in iproduct!([false, true], [false, true], [false, true]) {
        let mut emu = prepare(is_x64, &[0xf3, 0xaa, 0x90, 0xb8, 42, 0, 0, 0]);
        emu.regs_mut().rcx = 3;
        let pc = emu.pc();
        let observed = Rc::new(RefCell::new(Vec::new()));
        let captured = observed.clone();
        emu.set_hook_on_pre_instruction(move |emu, ip, ins, _| {
            captured.borrow_mut().push((ip, ins.as_x86().ip()));
            if ip == pc {
                if redirect {
                    emu.set_pc(pc + 3);
                    emu.force_reload = true;
                }
                return false;
            }
            true
        });
        if cached {
            emu.run(Some(pc + 8)).unwrap();
        } else {
            while emu.pc() != pc + 8 {
                assert!(emu.step());
            }
        }
        assert_eq!(emu.regs().rax, 42);
        assert_eq!(emu.regs().rcx, 3);
        assert_eq!(emu.maps.read_byte(DATA), Some(0));
        assert!(emu.rep.is_none());
        let expected = if redirect {
            vec![(pc, pc), (pc + 3, pc + 3)]
        } else {
            vec![(pc, pc), (pc + 2, pc + 2), (pc + 3, pc + 3)]
        };
        assert_eq!(*observed.borrow(), expected);
    }
}

#[test]
fn stack_instruction_families_observe_hooks_and_restore_sp_on_failure() {
    let cases: &[(&[u8], usize, bool)] = &[
        (&[0x66, 0x60], 8, false),
        (&[0x60], 8, false), // PUSHA/PUSHAD
        (&[0x66, 0x9c], 1, false),
        (&[0x9c], 1, false),
        (&[0x9c], 1, true),
        (&[0xc8, 0x10, 0x00, 0x00], 1, false),
        (&[0xc8, 0x10, 0x00, 0x00], 1, true),
        (&[0xe8, 0, 0, 0, 0], 1, false),
        (&[0xe8, 0, 0, 0, 0], 1, true),
    ];
    for ((code, pushes, is_x64), writable) in iproduct!(cases, [false, true]) {
        let mut emu = prepare(*is_x64, code);
        let sp = DATA + 128;
        emu.set_sp(sp);
        let bp = emu.regs().rbp;
        if !writable {
            emu.maps
                .protect_memory(DATA, PAGE, 2, Permission::READ)
                .unwrap();
        }
        let events = Rc::new(RefCell::new(Vec::new()));
        let captured = events.clone();
        emu.set_hook_on_memory_write(move |_, _, addr, bits, value| {
            captured.borrow_mut().push((addr, bits));
            value
        });
        let pc = emu.pc();
        assert_eq!(emu.step(), writable, "code={code:02x?}");
        assert_eq!(events.borrow().len(), if writable { *pushes } else { 1 });
        assert!(
            events
                .borrow()
                .iter()
                .all(|(addr, _)| (DATA..sp).contains(addr))
        );
        if !writable {
            assert_eq!(emu.sp(), sp);
            assert_eq!(emu.regs().rbp, bp);
            assert_eq!(emu.pc(), pc);
        }
    }
}

#[test]
fn narrow_simd_stores_propagate_write_failures() {
    let codes: &[&[u8]] = &[
        &[0xf2, 0x0f, 0x11, 0x07],          // MOVSD [rdi],xmm0
        &[0xc5, 0xf9, 0xd6, 0x07],          // VMOVQ [rdi],xmm0
        &[0x66, 0x0f, 0x3a, 0x15, 0x07, 0], // PEXTRW [rdi],xmm0,0
        &[0xc4, 0xe3, 0x79, 0x1d, 0x07, 0], // VCVTPS2PH [rdi],xmm0,0
    ];
    for (is_x64, code) in iproduct!([false, true], codes) {
        let mut emu = prepare(is_x64, code);
        assert!(emu.maps.write_bytes(DATA, &[0xaa; 16]));
        emu.maps
            .protect_memory(DATA, PAGE, 2, Permission::READ)
            .unwrap();
        let pc = emu.pc();
        assert!(!emu.step(), "code={code:02x?}");
        assert_eq!(emu.pc(), pc);
        assert_eq!(emu.maps.read_bytes(DATA, 16).as_ref(), &[0xaa; 16]);
    }
}

#[test]
fn enter_restores_frame_registers_after_a_nested_frame_read_fault() {
    for is_x64 in [false, true] {
        let mut emu = prepare(is_x64, &[0xc8, 0x10, 0, 2]);
        let sp = DATA + 128;
        let bp = DATA + PAGE + 16;
        emu.set_sp(sp);
        emu.regs_mut().rbp = bp;
        let pc = emu.pc();
        assert!(!emu.step());
        assert_eq!(emu.pc(), pc);
        assert_eq!(emu.sp(), sp);
        assert_eq!(emu.regs().rbp, bp);
    }
}

#[test]
fn modeled_fxsave_stores_apply_hooks_and_report_failure() {
    for (is_x64, writable) in iproduct!([false, true], [false, true]) {
        let mut emu = prepare(is_x64, &[0x0f, 0xae, 0x07]);
        assert!(emu.maps.write_bytes(DATA, &[0xaa; 48]));
        if !writable {
            emu.maps
                .protect_memory(DATA, PAGE, 2, Permission::READ)
                .unwrap();
        }
        let events = Rc::new(RefCell::new(Vec::new()));
        let captured = events.clone();
        emu.set_hook_on_memory_write(move |emu, _, addr, bits, _| {
            assert_eq!(emu.maps.read_bytes(DATA, 48).as_ref(), &[0xaa; 48]);
            captured.borrow_mut().push((addr, bits));
            u128::from_le_bytes([0x42; 16])
        });
        let pc = emu.pc();
        assert_eq!(emu.step(), writable);
        assert_eq!(*events.borrow(), vec![(DATA, 128), (DATA + 16, 128)]);
        assert_eq!(
            emu.maps.read_bytes(DATA, 32).as_ref(),
            &[if writable { 0x42 } else { 0xaa }; 32]
        );
        assert_eq!(emu.maps.read_bytes(DATA + 32, 16).as_ref(), &[0xaa; 16]);
        if !writable {
            assert_eq!(emu.pc(), pc);
        }
    }
}

#[test]
fn low_bits_and_subregister_writes_preserve_their_width_contract() {
    for bits in [0, 1, 8, 16, 32, 63, 64, 65, 127, 128, 129] {
        for value in [0, 1, 0xa5a5_a5a5_a5a5_a5a5, u64::MAX] {
            let expected =
                (0..bits.min(u64::BITS)).fold(0, |result, bit| result | (value & (1u64 << bit)));
            assert_eq!(low_bits_u64(value, bits), expected);
        }
        for value in [0, 1, u64::MAX as u128, u128::MAX] {
            let expected =
                (0..bits.min(u128::BITS)).fold(0, |result, bit| result | (value & (1u128 << bit)));
            assert_eq!(low_bits_u128(value, bits), expected);
        }
    }
    let mut regs = Regs64::new();
    for (reg, expected) in [
        (Register::AL, 0x1122_3344_5566_77fe),
        (Register::AH, 0x1122_3344_5566_fe88),
        (Register::AX, 0x1122_3344_5566_bafe),
        (Register::EAX, 0x0000_0000_dead_bafe),
        (Register::RAX, 0xffff_ffff_dead_bafe),
    ] {
        regs.rax = 0x1122_3344_5566_7788;
        regs.set_reg(reg, 0xffff_ffff_dead_bafe);
        assert_eq!(regs.rax, expected, "{reg:?}");
    }
    for reg in [
        Register::MM0,
        Register::MM1,
        Register::MM2,
        Register::MM3,
        Register::MM4,
        Register::MM5,
        Register::MM6,
        Register::MM7,
    ] {
        regs.set_xmm_reg(reg, u128::MAX);
        assert_eq!(regs.get_xmm_reg(reg), u64::MAX as u128);
    }
    regs.mm0 = u128::MAX;
    assert_eq!(regs.get_xmm_reg(Register::MM0), u64::MAX as u128);
}

struct UnpackCase {
    instructions: &'static [Code],
    lane_bits: u32,
    high_half: bool,
}

const UNPACK_CASES: &[UnpackCase] = &[
    UnpackCase {
        instructions: &[Code::Punpcklbw_mm_mmm32, Code::Punpcklbw_xmm_xmmm128],
        lane_bits: u8::BITS,
        high_half: false,
    },
    UnpackCase {
        instructions: &[Code::Punpckhbw_mm_mmm64, Code::Punpckhbw_xmm_xmmm128],
        lane_bits: u8::BITS,
        high_half: true,
    },
    UnpackCase {
        instructions: &[Code::Punpcklwd_mm_mmm32, Code::Punpcklwd_xmm_xmmm128],
        lane_bits: u16::BITS,
        high_half: false,
    },
    UnpackCase {
        instructions: &[Code::Punpckhwd_mm_mmm64, Code::Punpckhwd_xmm_xmmm128],
        lane_bits: u16::BITS,
        high_half: true,
    },
    UnpackCase {
        instructions: &[Code::Punpckldq_mm_mmm32, Code::Punpckldq_xmm_xmmm128],
        lane_bits: u32::BITS,
        high_half: false,
    },
    UnpackCase {
        instructions: &[Code::Punpckhdq_mm_mmm64, Code::Punpckhdq_xmm_xmmm128],
        lane_bits: u32::BITS,
        high_half: true,
    },
    UnpackCase {
        instructions: &[Code::Punpcklqdq_xmm_xmmm128],
        lane_bits: u64::BITS,
        high_half: false,
    },
    UnpackCase {
        instructions: &[Code::Punpckhqdq_xmm_xmmm128],
        lane_bits: u64::BITS,
        high_half: true,
    },
];

#[test]
fn packed_unpack_interleaves_the_selected_half_at_mmx_and_xmm_widths() {
    for case in UNPACK_CASES {
        for (is_x64, &code, memory) in iproduct!([false, true], case.instructions, [false, true]) {
            let (mut emu, registers) = prepare_packed(is_x64, code, memory);
            let lanes = (registers.bits / case.lane_bits) as usize;
            let dst: Vec<i64> = (1..=lanes as i64).collect();
            let src: Vec<i64> = (101..101 + lanes as i64).collect();
            let half = lanes / 2;
            let start = if case.high_half { half } else { 0 };
            let expected: Vec<_> = dst[start..start + half]
                .iter()
                .zip(&src[start..start + half])
                .flat_map(|(&a, &b)| [a, b])
                .collect();
            emu.regs_mut()
                .set_xmm_reg(registers.dst, packed(&dst, case.lane_bits));
            emu.regs_mut()
                .set_xmm_reg(registers.src, packed(&src, case.lane_bits));
            let bytes = (registers.bits / u8::BITS) as usize;
            let address = DATA + PAGE - bytes as u64;
            emu.regs_mut().rdi = address;
            assert!(emu.maps.write_bytes(
                address,
                &packed(&src, case.lane_bits).to_le_bytes()[..bytes]
            ));
            assert!(emu.step(), "{code:?} x64={is_x64} memory={memory}");
            assert_eq!(
                emu.regs().get_xmm_reg(registers.dst),
                packed(&expected, case.lane_bits),
                "{code:?} x64={is_x64} memory={memory}"
            );
        }
    }
}

struct BinaryCase {
    instructions: [Code; 2],
    lane_bits: u32,
    dst: [i64; 2],
    src: [i64; 2],
    expected: [i64; 2],
}

// Repeat each pair across the register to test boundaries without leaking into adjacent lanes.
const BINARY_CASES: &[BinaryCase] = &[
    BinaryCase {
        instructions: [Code::Paddb_mm_mmm64, Code::Paddb_xmm_xmmm128],
        lane_bits: u8::BITS,
        dst: [u8::MAX as i64, 1],
        src: [1, 2],
        expected: [0, 3],
    },
    BinaryCase {
        instructions: [Code::Paddw_mm_mmm64, Code::Paddw_xmm_xmmm128],
        lane_bits: u16::BITS,
        dst: [u16::MAX as i64, 1],
        src: [1, 2],
        expected: [0, 3],
    },
    BinaryCase {
        instructions: [Code::Paddd_mm_mmm64, Code::Paddd_xmm_xmmm128],
        lane_bits: u32::BITS,
        dst: [u32::MAX as i64, 1],
        src: [1, 2],
        expected: [0, 3],
    },
    BinaryCase {
        instructions: [Code::Paddq_mm_mmm64, Code::Paddq_xmm_xmmm128],
        lane_bits: u64::BITS,
        dst: [-1, 1],
        src: [1, 2],
        expected: [0, 3],
    },
    BinaryCase {
        instructions: [Code::Psubb_mm_mmm64, Code::Psubb_xmm_xmmm128],
        lane_bits: u8::BITS,
        dst: [0, 3],
        src: [1, 2],
        expected: [u8::MAX as i64, 1],
    },
    BinaryCase {
        instructions: [Code::Psubw_mm_mmm64, Code::Psubw_xmm_xmmm128],
        lane_bits: u16::BITS,
        dst: [0, 3],
        src: [1, 2],
        expected: [u16::MAX as i64, 1],
    },
    BinaryCase {
        instructions: [Code::Psubd_mm_mmm64, Code::Psubd_xmm_xmmm128],
        lane_bits: u32::BITS,
        dst: [0, 3],
        src: [1, 2],
        expected: [u32::MAX as i64, 1],
    },
    BinaryCase {
        instructions: [Code::Psubq_mm_mmm64, Code::Psubq_xmm_xmmm128],
        lane_bits: u64::BITS,
        dst: [0, 3],
        src: [1, 2],
        expected: [-1, 1],
    },
    BinaryCase {
        instructions: [Code::Paddusb_mm_mmm64, Code::Paddusb_xmm_xmmm128],
        lane_bits: u8::BITS,
        dst: [u8::MAX as i64, 1],
        src: [1, 2],
        expected: [u8::MAX as i64, 3],
    },
    BinaryCase {
        instructions: [Code::Paddusw_mm_mmm64, Code::Paddusw_xmm_xmmm128],
        lane_bits: u16::BITS,
        dst: [u16::MAX as i64, 1],
        src: [1, 2],
        expected: [u16::MAX as i64, 3],
    },
    BinaryCase {
        instructions: [Code::Psubusb_mm_mmm64, Code::Psubusb_xmm_xmmm128],
        lane_bits: u8::BITS,
        dst: [0, 3],
        src: [1, 2],
        expected: [0, 1],
    },
    BinaryCase {
        instructions: [Code::Psubusw_mm_mmm64, Code::Psubusw_xmm_xmmm128],
        lane_bits: u16::BITS,
        dst: [0, 3],
        src: [1, 2],
        expected: [0, 1],
    },
    BinaryCase {
        instructions: [Code::Paddsb_mm_mmm64, Code::Paddsb_xmm_xmmm128],
        lane_bits: u8::BITS,
        dst: [i8::MAX as i64, i8::MIN as i64],
        src: [1, -1],
        expected: [i8::MAX as i64, i8::MIN as i64],
    },
    BinaryCase {
        instructions: [Code::Paddsw_mm_mmm64, Code::Paddsw_xmm_xmmm128],
        lane_bits: u16::BITS,
        dst: [i16::MAX as i64, i16::MIN as i64],
        src: [1, -1],
        expected: [i16::MAX as i64, i16::MIN as i64],
    },
    BinaryCase {
        instructions: [Code::Psubsb_mm_mmm64, Code::Psubsb_xmm_xmmm128],
        lane_bits: u8::BITS,
        dst: [i8::MAX as i64, i8::MIN as i64],
        src: [-1, 1],
        expected: [i8::MAX as i64, i8::MIN as i64],
    },
    BinaryCase {
        instructions: [Code::Psubsw_mm_mmm64, Code::Psubsw_xmm_xmmm128],
        lane_bits: u16::BITS,
        dst: [i16::MAX as i64, i16::MIN as i64],
        src: [-1, 1],
        expected: [i16::MAX as i64, i16::MIN as i64],
    },
    BinaryCase {
        instructions: [Code::Pcmpeqb_mm_mmm64, Code::Pcmpeqb_xmm_xmmm128],
        lane_bits: u8::BITS,
        dst: [1, -1],
        src: [1, 0],
        expected: [-1, 0],
    },
    BinaryCase {
        instructions: [Code::Pcmpeqw_mm_mmm64, Code::Pcmpeqw_xmm_xmmm128],
        lane_bits: u16::BITS,
        dst: [1, -1],
        src: [1, 0],
        expected: [-1, 0],
    },
    BinaryCase {
        instructions: [Code::Pcmpeqd_mm_mmm64, Code::Pcmpeqd_xmm_xmmm128],
        lane_bits: u32::BITS,
        dst: [1, -1],
        src: [1, 0],
        expected: [-1, 0],
    },
    BinaryCase {
        instructions: [Code::Pcmpgtd_mm_mmm64, Code::Pcmpgtd_xmm_xmmm128],
        lane_bits: u32::BITS,
        dst: [-1, 1],
        src: [0, -1],
        expected: [0, -1],
    },
];

#[test]
fn packed_arithmetic_wraps_saturates_and_compares_without_lane_leakage() {
    for case in BINARY_CASES {
        for (is_x64, code, memory) in iproduct!([false, true], case.instructions, [false, true]) {
            let (mut emu, registers) = prepare_packed(is_x64, code, memory);
            let expand = |lanes: [i64; 2]| -> Vec<i64> {
                lanes
                    .into_iter()
                    .cycle()
                    .take((registers.bits / case.lane_bits) as usize)
                    .collect()
            };
            let dst = packed(&expand(case.dst), case.lane_bits);
            let src = packed(&expand(case.src), case.lane_bits);
            emu.regs_mut().set_xmm_reg(registers.dst, dst);
            emu.regs_mut().set_xmm_reg(registers.src, src);
            let bytes = (registers.bits / u8::BITS) as usize;
            let address = DATA + PAGE - bytes as u64;
            emu.regs_mut().rdi = address;
            assert!(emu.maps.write_bytes(address, &src.to_le_bytes()[..bytes]));
            assert!(emu.step(), "{code:?} x64={is_x64} memory={memory}");
            assert_eq!(
                emu.regs().get_xmm_reg(registers.dst),
                packed(&expand(case.expected), case.lane_bits),
                "{code:?} x64={is_x64} memory={memory}"
            );
        }
    }
}

#[test]
fn packed_memory_faults_preserve_the_destination() {
    let instructions = BINARY_CASES
        .iter()
        .flat_map(|case| case.instructions)
        .chain(
            UNPACK_CASES
                .iter()
                .filter(|case| case.lane_bits != u64::BITS)
                .flat_map(|case| case.instructions.iter().copied()),
        );
    for (is_x64, code) in iproduct!([false, true], instructions) {
        let (mut emu, registers) = prepare_packed(is_x64, code, true);
        emu.regs_mut().set_xmm_reg(registers.dst, 0x1234_5678);
        emu.regs_mut().rdi = DATA + PAGE;
        let pc = emu.pc();
        assert!(!emu.step(), "{code:?} x64={is_x64}");
        assert_eq!(emu.pc(), pc, "{code:?}");
        assert_eq!(
            emu.regs().get_xmm_reg(registers.dst),
            0x1234_5678,
            "{code:?}"
        );
        assert_eq!(emu.fault_count, 1, "{code:?}");
    }
}

#[test]
fn movemask_uses_all_and_only_the_encoded_register_bytes() {
    for (is_x64, xmm) in iproduct!([false, true], [false, true]) {
        let (code, src) = if xmm {
            (Code::Pmovmskb_r32_xmm, Register::XMM1)
        } else {
            (Code::Pmovmskb_r32_mm, Register::MM1)
        };
        let instruction = Instruction::with2(code, Register::EAX, src).unwrap();
        let mut emu = prepare_instruction(is_x64, instruction);
        let bytes = [
            0x80, 1, 0xff, 0, 0x7f, 0x81, 0x42, 0xfe, 0, 0xff, 0, 0xff, 0, 0x80, 0, 0xff,
        ];
        emu.regs_mut().set_xmm_reg(src, u128::from_le_bytes(bytes));
        emu.regs_mut().rax = u64::MAX;
        assert!(emu.step());
        assert_eq!(emu.regs().rax, if xmm { 0xaaa5 } else { 0xa5 });
    }
}

#[test]
fn packed_word_selectors_ignore_bits_above_the_register_width() {
    for (is_x64, xmm, insert, selector) in iproduct!(
        [false, true],
        [false, true],
        [false, true],
        [0u8, 3, 4, 7, 8, 255]
    ) {
        let (dst, src, lanes) = if xmm {
            (Register::XMM0, Register::XMM1, 8)
        } else {
            (Register::MM0, Register::MM1, 4)
        };
        let code = match (insert, xmm) {
            (true, false) => Code::Pinsrw_mm_r32m16_imm8,
            (true, true) => Code::Pinsrw_xmm_r32m16_imm8,
            (false, false) => Code::Pextrw_r32_mm_imm8,
            (false, true) => Code::Pextrw_r32_xmm_imm8,
        };
        let instruction = if insert {
            Instruction::with3(code, dst, Register::ECX, selector as u32).unwrap()
        } else {
            Instruction::with3(code, Register::EAX, src, selector as u32).unwrap()
        };
        let mut emu = prepare_instruction(is_x64, instruction);
        let mut words: Vec<i64> = (1..=lanes as i64).collect();
        let index = selector as usize % lanes;
        emu.regs_mut().set_xmm_reg(dst, packed(&words, u16::BITS));
        emu.regs_mut().set_xmm_reg(src, packed(&words, u16::BITS));
        emu.regs_mut().rcx = 0xdead_beef;
        emu.regs_mut().rax = u64::MAX;
        assert!(emu.step());
        if insert {
            words[index] = 0xbeef;
            assert_eq!(emu.regs().get_xmm_reg(dst), packed(&words, u16::BITS));
        } else {
            assert_eq!(emu.regs().rax, words[index] as u64);
        }
    }
}

#[test]
fn packed_byte_shuffle_masks_mmx_indices_and_honors_zero_selectors() {
    for (is_x64, code, memory) in iproduct!(
        [false, true],
        [Code::Pshufb_mm_mmm64, Code::Pshufb_xmm_xmmm128],
        [false, true]
    ) {
        let (mut emu, registers) = prepare_packed(is_x64, code, memory);
        let bytes = (registers.bits / u8::BITS) as usize;
        let data: Vec<i64> = (1..=bytes as i64).collect();
        let controls = [8u8, 15, 0x80, 0xff, 0x17, 0, 1, 2, 7, 6, 5, 4, 3, 2, 1, 0];
        let expected: &[i64] = if registers.bits == u128::BITS {
            &[9, 16, 0, 0, 8, 1, 2, 3, 8, 7, 6, 5, 4, 3, 2, 1]
        } else {
            &[1, 8, 0, 0, 8, 1, 2, 3]
        };
        emu.regs_mut()
            .set_xmm_reg(registers.dst, packed(&data, u8::BITS));
        emu.regs_mut()
            .set_xmm_reg(registers.src, u128::from_le_bytes(controls));
        assert!(emu.maps.write_bytes(DATA, &controls[..bytes]));
        assert!(emu.step(), "{code:?} x64={is_x64} memory={memory}");
        assert_eq!(
            emu.regs().get_xmm_reg(registers.dst),
            packed(expected, u8::BITS)
        );
    }
}

#[test]
fn horizontal_packed_arithmetic_places_both_sources_in_the_destination() {
    struct HorizontalCase {
        instructions: [Code; 2],
        lane_bits: u32,
        dst: [i64; 4],
        src: [i64; 4],
        expected_dst: [i64; 2],
        expected_src: [i64; 2],
    }
    let cases = [
        HorizontalCase {
            instructions: [Code::Phaddw_mm_mmm64, Code::Phaddw_xmm_xmmm128],
            lane_bits: u16::BITS,
            dst: [i16::MAX as i64, -1, i16::MIN as i64, 1],
            src: [30000, 10000, -30000, -10000],
            expected_dst: [32766, -i16::MAX as i64],
            expected_src: [-25536, 25536],
        },
        HorizontalCase {
            instructions: [Code::Phsubw_mm_mmm64, Code::Phsubw_xmm_xmmm128],
            lane_bits: u16::BITS,
            dst: [i16::MAX as i64, -1, i16::MIN as i64, 1],
            src: [30000, 10000, -30000, -10000],
            expected_dst: [i16::MIN as i64, i16::MAX as i64],
            expected_src: [20000, -20000],
        },
        HorizontalCase {
            instructions: [Code::Phaddsw_mm_mmm64, Code::Phaddsw_xmm_xmmm128],
            lane_bits: u16::BITS,
            dst: [i16::MAX as i64, -1, i16::MIN as i64, 1],
            src: [30000, 10000, -30000, -10000],
            expected_dst: [32766, -i16::MAX as i64],
            expected_src: [i16::MAX as i64, i16::MIN as i64],
        },
        HorizontalCase {
            instructions: [Code::Phsubsw_mm_mmm64, Code::Phsubsw_xmm_xmmm128],
            lane_bits: u16::BITS,
            dst: [i16::MAX as i64, -1, i16::MIN as i64, 1],
            src: [30000, 10000, -30000, -10000],
            expected_dst: [i16::MAX as i64, i16::MIN as i64],
            expected_src: [20000, -20000],
        },
        HorizontalCase {
            instructions: [Code::Phaddd_mm_mmm64, Code::Phaddd_xmm_xmmm128],
            lane_bits: u32::BITS,
            dst: [i32::MAX as i64, 1, i32::MIN as i64, -1],
            src: [100, 200, -100, -200],
            expected_dst: [i32::MIN as i64, i32::MAX as i64],
            expected_src: [300, -300],
        },
        HorizontalCase {
            instructions: [Code::Phsubd_mm_mmm64, Code::Phsubd_xmm_xmmm128],
            lane_bits: u32::BITS,
            dst: [i32::MIN as i64, 1, i32::MAX as i64, -1],
            src: [100, 200, -100, -200],
            expected_dst: [i32::MAX as i64, i32::MIN as i64],
            expected_src: [-100, 100],
        },
    ];
    for case in &cases {
        for (is_x64, code, memory) in iproduct!([false, true], case.instructions, [false, true]) {
            let (mut emu, registers) = prepare_packed(is_x64, code, memory);
            let lanes = (registers.bits / case.lane_bits) as usize;
            let dst: Vec<_> = case.dst.into_iter().cycle().take(lanes).collect();
            let src: Vec<_> = case.src.into_iter().cycle().take(lanes).collect();
            let expected: Vec<_> = case
                .expected_dst
                .into_iter()
                .cycle()
                .take(lanes / 2)
                .chain(case.expected_src.into_iter().cycle().take(lanes / 2))
                .collect();
            emu.regs_mut()
                .set_xmm_reg(registers.dst, packed(&dst, case.lane_bits));
            emu.regs_mut()
                .set_xmm_reg(registers.src, packed(&src, case.lane_bits));
            let bytes = (registers.bits / u8::BITS) as usize;
            assert!(
                emu.maps
                    .write_bytes(DATA, &packed(&src, case.lane_bits).to_le_bytes()[..bytes])
            );
            assert!(emu.step(), "{code:?} x64={is_x64} memory={memory}");
            assert_eq!(
                emu.regs().get_xmm_reg(registers.dst),
                packed(&expected, case.lane_bits),
                "{code:?} x64={is_x64} memory={memory}"
            );
        }
    }
}

#[test]
fn audited_simd_read_faults_preserve_registers_and_flags() {
    let memory = MemoryOperand::with_base(Register::RDI);
    let mut instructions = Vec::new();
    for code in [
        Code::Addsubps_xmm_xmmm128,
        Code::Addsubpd_xmm_xmmm128,
        Code::Pabsb_xmm_xmmm128,
        Code::Aesenc_xmm_xmmm128,
        Code::Cvtps2dq_xmm_xmmm128,
        Code::Ptest_xmm_xmmm128,
        Code::VEX_Vsqrtps_ymm_ymmm256,
        Code::VEX_Vpmovzxbw_ymm_xmmm128,
        Code::VEX_Vpbroadcastd_ymm_xmmm32,
    ] {
        let dest = if code.op_code().op0_kind() == OpCodeOperandKind::ymm_reg {
            Register::YMM0
        } else {
            Register::XMM0
        };
        instructions.push(Instruction::with2(code, dest, memory).unwrap());
    }
    for code in [
        Code::VEX_Vaddps_ymm_ymm_ymmm256,
        Code::VEX_Vfmadd132ps_ymm_ymm_ymmm256,
        Code::VEX_Vpdpbusd_ymm_ymm_ymmm256,
        Code::VEX_Vpslld_ymm_ymm_xmmm128,
    ] {
        instructions
            .push(Instruction::with3(code, Register::YMM0, Register::YMM1, memory).unwrap());
    }
    instructions.push(
        Instruction::with3(
            Code::VEX_Vaddss_xmm_xmm_xmmm32,
            Register::XMM0,
            Register::XMM1,
            memory,
        )
        .unwrap(),
    );
    instructions.push(
        Instruction::with3(
            Code::VEX_Vpshufd_ymm_ymmm256_imm8,
            Register::YMM0,
            memory,
            0x1bu32,
        )
        .unwrap(),
    );
    instructions.push(
        Instruction::with4(
            Code::VEX_Vshufps_ymm_ymm_ymmm256_imm8,
            Register::YMM0,
            Register::YMM1,
            memory,
            0x1bu32,
        )
        .unwrap(),
    );
    for instruction in instructions {
        let code = instruction.code();
        let mut emu = prepare_instruction(true, instruction);
        emu.regs_mut().rdi = DATA + PAGE;
        let original = U256::from(u128::MAX);
        emu.regs_mut().set_ymm_reg(Register::YMM0, original);
        emu.flags_mut().load(0x8d7);
        let flags = emu.flags().dump();
        let pc = emu.pc();
        assert!(!emu.step(), "{code:?}");
        assert_eq!(emu.pc(), pc, "{code:?}");
        assert_eq!(emu.regs().get_ymm_reg(Register::YMM0), original, "{code:?}");
        assert_eq!(emu.flags().dump(), flags, "{code:?}");
    }
}

#[test]
fn audited_stores_report_write_faults() {
    let memory = MemoryOperand::with_base(Register::RDI);
    let instructions = [
        Instruction::with2(Code::VEX_Vmovd_rm32_xmm, memory, Register::XMM0).unwrap(),
        Instruction::with1(Code::Stmxcsr_m32, memory).unwrap(),
        Instruction::with1(Code::Fist_m32int, memory).unwrap(),
        Instruction::with2(Code::Movbe_m32_r32, memory, Register::EAX).unwrap(),
    ];
    for instruction in instructions {
        let code = instruction.code();
        let mut emu = prepare_instruction(true, instruction);
        assert!(emu.maps.write_dword(DATA, 0x1234_5678));
        emu.maps
            .protect_memory(DATA, PAGE, 2, Permission::READ)
            .unwrap();
        let pc = emu.pc();
        let fpu_ip = emu.fpu().ip;
        assert!(!emu.step(), "{code:?}");
        assert_eq!(emu.pc(), pc, "{code:?}");
        assert_eq!(emu.maps.read_dword(DATA), Some(0x1234_5678), "{code:?}");
        assert_eq!(emu.fpu().ip, fpu_ip, "{code:?}");
    }
}

#[test]
fn conditional_moves_read_memory_before_changing_destination() {
    for (code, flags) in iproduct!(
        [
            Code::Cmova_r32_rm32,
            Code::Cmovae_r32_rm32,
            Code::Cmovb_r32_rm32,
            Code::Cmovbe_r32_rm32,
            Code::Cmove_r32_rm32,
            Code::Cmovne_r32_rm32,
            Code::Cmovg_r32_rm32,
            Code::Cmovge_r32_rm32,
            Code::Cmovl_r32_rm32,
            Code::Cmovle_r32_rm32,
            Code::Cmovo_r32_rm32,
            Code::Cmovno_r32_rm32,
            Code::Cmovs_r32_rm32,
            Code::Cmovns_r32_rm32,
            Code::Cmovp_r32_rm32,
            Code::Cmovnp_r32_rm32,
        ],
        [0, 0x8d7]
    ) {
        let instruction =
            Instruction::with2(code, Register::EAX, MemoryOperand::with_base(Register::RDI))
                .unwrap();
        let mut emu = prepare_instruction(true, instruction);
        emu.regs_mut().rax = u64::MAX;
        emu.regs_mut().rdi = DATA + PAGE;
        emu.flags_mut().load(flags);
        let flags = emu.flags().dump();
        let pc = emu.pc();
        assert!(!emu.step(), "{code:?} flags={flags:x}");
        assert_eq!(emu.pc(), pc, "{code:?}");
        assert_eq!(emu.regs().rax, u64::MAX, "{code:?}");
        assert_eq!(emu.flags().dump(), flags, "{code:?}");
    }
    for ((code, dest, src, expected_taken, expected_not_taken), taken) in iproduct!(
        [
            (
                Code::Cmove_r16_rm16,
                Register::AX,
                Register::BX,
                !(u16::MAX as u64) | 42,
                u64::MAX
            ),
            (
                Code::Cmove_r32_rm32,
                Register::EAX,
                Register::EBX,
                42,
                u32::MAX as u64
            ),
            (
                Code::Cmove_r64_rm64,
                Register::RAX,
                Register::RBX,
                42,
                u64::MAX
            ),
        ],
        [false, true]
    ) {
        let mut emu = prepare_instruction(true, Instruction::with2(code, dest, src).unwrap());
        emu.regs_mut().rax = u64::MAX;
        emu.regs_mut().rbx = 42;
        emu.flags_mut().f_zf = taken;
        assert!(emu.step(), "{code:?}");
        assert_eq!(
            emu.regs().rax,
            if taken {
                expected_taken
            } else {
                expected_not_taken
            },
            "{code:?}"
        );
    }
}

#[test]
fn stack_pops_preserve_state_when_a_read_or_write_fails() {
    for available in [0, 4, 12, 20, 28, 32] {
        let mut emu = prepare_instruction(false, Instruction::with(Code::Popad));
        let registers = [
            Register::EDI,
            Register::ESI,
            Register::EBP,
            Register::ESP,
            Register::EBX,
            Register::EDX,
            Register::ECX,
            Register::EAX,
        ];
        let sp = DATA + PAGE - available;
        for (index, register) in registers.into_iter().enumerate() {
            emu.regs_mut().set_reg(register, 100 + index as u64);
            if index as u64 * 4 < available {
                assert!(emu.maps.write_dword(sp + index as u64 * 4, index as u32));
            }
        }
        emu.set_sp(sp);
        let pc = emu.pc();
        assert_eq!(emu.step(), available == 32, "available={available}");
        assert_eq!(emu.sp(), if available == 32 { sp + 32 } else { sp });
        if available != 32 {
            assert_eq!(emu.pc(), pc);
        }
        for (index, register) in registers.into_iter().enumerate() {
            if register != Register::ESP {
                assert_eq!(
                    emu.regs().get_reg(register),
                    if available == 32 {
                        index as u64
                    } else {
                        100 + index as u64
                    },
                    "{register:?} available={available}"
                );
            }
        }
    }
    let instruction =
        Instruction::with1(Code::Pop_rm64, MemoryOperand::with_base(Register::RDI)).unwrap();
    let mut emu = prepare_instruction(true, instruction);
    emu.set_sp(DATA + 128);
    assert!(emu.maps.write_qword(emu.sp(), 42));
    emu.maps
        .protect_memory(DATA, PAGE, 2, Permission::READ)
        .unwrap();
    let pc = emu.pc();
    assert!(!emu.step());
    assert_eq!(emu.sp(), DATA + 128);
    assert_eq!(emu.pc(), pc);
}

#[test]
fn simd_logical_operations_preserve_integer_flags() {
    for (code, expected) in [
        (Code::Pand_mm_mmm64, 0x50),
        (Code::Pand_xmm_xmmm128, 0x50),
        (Code::Por_mm_mmm64, 0xf5),
        (Code::Por_xmm_xmmm128, 0xf5),
        (Code::Pxor_mm_mmm64, 0xa5),
        (Code::Pxor_xmm_xmmm128, 0xa5),
    ] {
        let (mut emu, registers) = prepare_packed(true, code, false);
        emu.regs_mut().set_xmm_reg(registers.dst, 0xf0);
        emu.regs_mut().set_xmm_reg(registers.src, 0x55);
        emu.flags_mut().load(0x8d7);
        let flags = emu.flags().dump();
        assert!(emu.step(), "{code:?}");
        assert_eq!(emu.regs().get_xmm_reg(registers.dst), expected, "{code:?}");
        assert_eq!(emu.flags().dump(), flags, "{code:?}");
    }
}

#[test]
fn whole_vector_byte_shifts_saturate_and_keep_128_bit_lanes_separate() {
    for ((code, wide, vex, left), count) in iproduct!(
        [
            (Code::Psrldq_xmm_imm8, false, false, false),
            (Code::Pslldq_xmm_imm8, false, false, true),
            (Code::VEX_Vpsrldq_xmm_xmm_imm8, false, true, false),
            (Code::VEX_Vpslldq_xmm_xmm_imm8, false, true, true),
            (Code::VEX_Vpsrldq_ymm_ymm_imm8, true, true, false),
            (Code::VEX_Vpslldq_ymm_ymm_imm8, true, true, true),
        ],
        [0u32, 1, 15, 16, 17, 255]
    ) {
        let register = if wide { Register::YMM0 } else { Register::XMM0 };
        let instruction = if vex {
            Instruction::with3(code, register, register, count).unwrap()
        } else {
            Instruction::with2(code, register, count).unwrap()
        };
        let mut emu = prepare_instruction(true, instruction);
        emu.regs_mut().set_ymm_reg(
            Register::YMM0,
            U256::from(u128::MAX) | (U256::from(1) << u128::BITS),
        );
        let expected = |value: u128| {
            if count >= 16 {
                0
            } else if left {
                value << (count * 8)
            } else {
                value >> (count * 8)
            }
        };
        assert!(emu.step(), "{code:?} count={count}");
        assert_eq!(
            emu.regs().get_xmm_reg(Register::XMM0),
            expected(u128::MAX),
            "{code:?} count={count}"
        );
        if wide {
            assert_eq!(
                emu.regs().get_ymm_reg(Register::YMM0),
                U256::from(expected(u128::MAX)) | (U256::from(expected(1)) << u128::BITS),
                "{code:?} count={count}"
            );
        }
    }
}

#[test]
fn x87_integer_read_faults_preserve_fpu_state() {
    for code in [
        Code::Ficom_m16int,
        Code::Ficomp_m32int,
        Code::Fimul_m16int,
        Code::Fidiv_m32int,
    ] {
        let mut emu = prepare_instruction(
            true,
            Instruction::with1(code, MemoryOperand::with_base(Register::RDI)).unwrap(),
        );
        emu.regs_mut().rdi = DATA + PAGE;
        emu.fpu_mut().push_f64(3.0);
        assert_eq!(emu.fpu_mut().get_st(0), 3.0);
        let status = emu.fpu().status;
        let pc = emu.pc();
        assert!(!emu.step(), "{code:?}");
        assert_eq!(emu.pc(), pc, "{code:?}");
        assert_eq!(emu.fpu_mut().get_st(0), 3.0, "{code:?}");
        assert_eq!(emu.fpu().status, status, "{code:?}");
    }
}

#[test]
fn selector_privilege_adjustment_commits_flags_after_a_successful_write() {
    for (dest, src, writable, expected, changed) in [
        (0x10, 3, true, 0x13, true),
        (0x13, 0x20, true, 0x13, false),
        (0x10, 3, false, 0x10, true),
        (0x13, 0x20, false, 0x13, false),
    ] {
        let instruction = Instruction::with2(
            Code::Arpl_rm16_r16,
            MemoryOperand::with_base(Register::EDI),
            Register::AX,
        )
        .unwrap();
        let mut emu = prepare_instruction(false, instruction);
        assert!(emu.maps.write_word(DATA, dest));
        emu.regs_mut().set_ax(src);
        emu.flags_mut().f_zf = !changed;
        let original_flags = emu.flags().dump();
        if !writable {
            emu.maps
                .protect_memory(DATA, PAGE, 2, Permission::READ)
                .unwrap();
        }
        assert_eq!(emu.step(), writable);
        assert_eq!(emu.maps.read_word(DATA), Some(expected));
        if writable {
            assert_eq!(emu.flag_zf(), changed);
        } else {
            assert_eq!(emu.flags().dump(), original_flags);
        }
    }
}

#[test]
fn alternating_float_operations_share_lane_order_and_x86_nan_results() {
    for ((code, wide, vex), invalid) in iproduct!(
        [
            (Code::Addsubps_xmm_xmmm128, false, false),
            (Code::VEX_Vaddsubps_xmm_xmm_xmmm128, false, true),
            (Code::VEX_Vaddsubps_ymm_ymm_ymmm256, true, true),
        ],
        [false, true]
    ) {
        let dest = if wide { Register::YMM0 } else { Register::XMM0 };
        let src = if wide { Register::YMM1 } else { Register::XMM1 };
        let instruction = if vex {
            Instruction::with3(code, dest, dest, src).unwrap()
        } else {
            Instruction::with2(code, dest, src).unwrap()
        };
        let mut emu = prepare_instruction(true, instruction);
        let (a, b, expected) = if invalid {
            (
                [f32::INFINITY, f32::NEG_INFINITY, 30.0, 40.0],
                [f32::INFINITY, f32::INFINITY, 3.0, 4.0],
                [
                    0xffc0_0000,
                    0xffc0_0000,
                    27.0f32.to_bits(),
                    44.0f32.to_bits(),
                ],
            )
        } else {
            (
                [10.0, 20.0, 30.0, 40.0],
                [1.0, 2.0, 3.0, 4.0],
                [
                    9.0f32.to_bits(),
                    22.0f32.to_bits(),
                    27.0f32.to_bits(),
                    44.0f32.to_bits(),
                ],
            )
        };
        let pack_floats = |values: [f32; 4]| packed(&values.map(|v| v.to_bits() as i64), u32::BITS);
        let pair = |value| U256::from(value) | (U256::from(value) << u128::BITS);
        emu.regs_mut()
            .set_ymm_reg(Register::YMM0, pair(pack_floats(a)));
        emu.regs_mut()
            .set_ymm_reg(Register::YMM1, pair(pack_floats(b)));
        let expected = packed(&expected.map(|v| v as i64), u32::BITS);
        assert!(emu.step(), "{code:?} invalid={invalid}");
        assert_eq!(
            emu.regs().get_xmm_reg(Register::XMM0),
            expected,
            "{code:?} invalid={invalid}"
        );
        if wide {
            assert_eq!(emu.regs().get_ymm_reg(Register::YMM0), pair(expected));
        }
    }
    for ((code, wide, vex), invalid) in iproduct!(
        [
            (Code::Addsubpd_xmm_xmmm128, false, false),
            (Code::VEX_Vaddsubpd_xmm_xmm_xmmm128, false, true),
            (Code::VEX_Vaddsubpd_ymm_ymm_ymmm256, true, true),
        ],
        [false, true]
    ) {
        let dest = if wide { Register::YMM0 } else { Register::XMM0 };
        let src = if wide { Register::YMM1 } else { Register::XMM1 };
        let instruction = if vex {
            Instruction::with3(code, dest, dest, src).unwrap()
        } else {
            Instruction::with2(code, dest, src).unwrap()
        };
        let mut emu = prepare_instruction(true, instruction);
        let (a, b, expected) = if invalid {
            (
                [f64::INFINITY, f64::NEG_INFINITY],
                [f64::INFINITY, f64::INFINITY],
                [0xfff8_0000_0000_0000, 0xfff8_0000_0000_0000],
            )
        } else {
            (
                [10.0, 20.0],
                [1.0, 2.0],
                [9.0f64.to_bits(), 22.0f64.to_bits()],
            )
        };
        let pack_floats = |values: [f64; 2]| packed(&values.map(|v| v.to_bits() as i64), u64::BITS);
        let pair = |value| U256::from(value) | (U256::from(value) << u128::BITS);
        emu.regs_mut()
            .set_ymm_reg(Register::YMM0, pair(pack_floats(a)));
        emu.regs_mut()
            .set_ymm_reg(Register::YMM1, pair(pack_floats(b)));
        let expected = packed(&expected.map(|v| v as i64), u64::BITS);
        assert!(emu.step(), "{code:?} invalid={invalid}");
        assert_eq!(
            emu.regs().get_xmm_reg(Register::XMM0),
            expected,
            "{code:?} invalid={invalid}"
        );
        if wide {
            assert_eq!(emu.regs().get_ymm_reg(Register::YMM0), pair(expected));
        }
    }
}
