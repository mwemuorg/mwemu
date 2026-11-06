use std::cell::RefCell;
use std::rc::Rc;

use iced_x86::Register;
use itertools::iproduct;

use crate::arch::OperatingSystem;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::{emu32, emu64};

fn prepare(is_x64: bool, code: &[u8]) -> Emu {
    let mut emu = if is_x64 { emu64() } else { emu32() };
    emu.os = OperatingSystem::Linux;
    emu.cfg.console_enabled = false;
    emu.cfg.max_instructions = Some(256);
    emu.load_code_bytes(code).unwrap();
    emu
}

#[test]
fn string_stores_report_each_write() {
    for (is_x64, width, copy, backwards, repeat, cached) in iproduct!(
        [false, true],
        [1u32, 2, 4, 8],
        [false, true],
        [false, true],
        [None, Some(0u64), Some(1), Some(32)],
        [false, true],
    ) {
        if width == 8 && !is_x64 {
            continue;
        }
        let mut code = Vec::new();
        if repeat.is_some() {
            code.push(0xf3);
        }
        if width == 2 {
            code.push(0x66);
        } else if width == 8 {
            code.push(0x48);
        }
        code.push(match (copy, width) {
            (true, 1) => 0xa4,
            (true, _) => 0xa5,
            (false, 1) => 0xaa,
            (false, _) => 0xab,
        });
        let instruction_len = code.len() as u64;
        code.resize(64, 0x90);
        let mut emu = prepare(is_x64, &code);
        let ip = emu.pc();
        let base = if is_x64 { 0x1_3000_0000 } else { 0x3000_0000 };
        emu.maps
            .create_map("data", base, 0x1000, Permission::READ_WRITE)
            .unwrap();
        assert!(emu.maps.write_bytes(base, &[0xcc; 0x1000]));
        let source = base + 0x100;
        let destination = base + 0x800;
        let count = repeat.unwrap_or(1);
        let stride = u64::from(width);
        let offset = if backwards {
            count.saturating_sub(1) * stride
        } else {
            0
        };
        let accumulator = 0x8877_6655_4433_2211u64;
        for index in 0..count {
            let value = accumulator + index;
            assert!(emu.maps.write_bytes(
                source + index * stride,
                &value.to_le_bytes()[..width as usize],
            ));
        }
        emu.regs_mut().rsi = source + offset;
        emu.regs_mut().rdi = destination + offset;
        emu.regs_mut().rax = accumulator;
        emu.regs_mut().rcx = repeat.unwrap_or(7);
        emu.flags_mut().f_df = backwards;
        let mask = (1u128 << (width * 8)) - 1;
        let events = Rc::new(RefCell::new(Vec::new()));
        let observed = Rc::clone(&events);
        emu.set_hook_on_memory_write(move |emu, ip, addr, bits, value| {
            assert_eq!(
                emu.maps.read_bytes(addr, width as usize),
                vec![0xcc; width as usize]
            );
            observed.borrow_mut().push((ip, addr, bits, value));
            value ^ mask
        });
        if cached {
            emu.run(Some(ip + instruction_len)).unwrap();
        } else {
            assert!(emu.step());
        }
        assert_eq!(emu.pc(), ip + instruction_len);
        let expected: Vec<_> = (0..count)
            .map(|iteration| {
                let index = if backwards {
                    count - 1 - iteration
                } else {
                    iteration
                };
                let value = if copy {
                    accumulator + index
                } else {
                    accumulator
                };
                (
                    ip,
                    destination + index * stride,
                    width * 8,
                    u128::from(value) & mask,
                )
            })
            .collect();
        assert_eq!(
            *events.borrow(),
            expected,
            "x64={is_x64} width={width} copy={copy} backwards={backwards} repeat={repeat:?} cached={cached}"
        );
        for (_, addr, _, value) in expected {
            assert_eq!(
                emu.maps.read_bytes(addr, width as usize),
                &(value ^ mask).to_le_bytes()[..width as usize]
            );
        }
        assert_eq!(emu.maps.read_byte(destination - 1), Some(0xcc));
        assert_eq!(emu.maps.read_byte(destination + count * stride), Some(0xcc));
        let final_offset = if backwards {
            offset.wrapping_sub(count * stride)
        } else {
            count * stride
        };
        assert_eq!(emu.regs().rdi, destination.wrapping_add(final_offset));
        assert_eq!(
            emu.regs().rsi,
            source.wrapping_add(if copy { final_offset } else { offset })
        );
        assert_eq!(emu.regs().rcx, if repeat.is_some() { 0 } else { 7 });
        assert_eq!(emu.flag_df(), backwards);
    }
}

#[test]
fn operand_stores_keep_hook_values_and_widths() {
    // mov [edi/rdi], eax; movdqu [edi/rdi], xmm0
    for (is_x64, (code, width)) in iproduct!(
        [false, true],
        [
            (&[0x89, 0x07][..], 4u32),
            (&[0xf3, 0x0f, 0x7f, 0x07][..], 16),
        ],
    ) {
        let mut emu = prepare(is_x64, code);
        let destination = emu.alloc("data", 0x1000, Permission::READ_WRITE);
        emu.regs_mut().rdi = destination;
        emu.regs_mut().rax = 0x1234_5678;
        let original = 0xfedc_ba98_7654_3210_1234_5678u128;
        emu.regs_mut().set_xmm_reg(Register::XMM0, original);
        let ip = emu.pc();
        let events = Rc::new(RefCell::new(Vec::new()));
        let observed = Rc::clone(&events);
        emu.set_hook_on_memory_write(move |_, ip, addr, bits, value| {
            observed.borrow_mut().push((ip, addr, bits, value));
            0x1122_3344_5566_7788_99aa_bbcc_ddee_ff00
        });
        assert!(emu.step());
        assert_eq!(
            *events.borrow(),
            [(
                ip,
                destination,
                width * 8,
                if width == 4 { 0x1234_5678 } else { original }
            )]
        );
        assert_eq!(
            emu.maps.read_bytes(destination, width as usize),
            &0x1122_3344_5566_7788_99aa_bbcc_ddee_ff00u128.to_le_bytes()[..width as usize]
        );
    }
}

#[test]
fn string_write_hook_can_supply_a_missing_destination() {
    for (is_x64, code) in iproduct!([false, true], [0xa4, 0xaa]) {
        let mut emu = prepare(is_x64, &[code]);
        let source = emu.alloc("source", 0x1000, Permission::READ_WRITE);
        assert!(emu.maps.write_byte(source, 0x12));
        let destination = 0x3000_0000;
        emu.regs_mut().rsi = source;
        emu.regs_mut().rdi = destination;
        emu.regs_mut().rax = 0x12;
        let ip = emu.pc();
        emu.set_hook_on_memory_write(move |emu, hook_ip, addr, bits, value| {
            assert_eq!((hook_ip, addr, bits, value), (ip, destination, 8, 0x12));
            assert!(!emu.maps.is_mapped(addr));
            emu.maps
                .create_map("destination", addr, 0x1000, Permission::READ_WRITE)
                .unwrap();
            0x5a
        });
        assert!(emu.step());
        assert_eq!(emu.maps.read_byte(destination), Some(0x5a));
    }
}
