use crate::emu::Emu;
use crate::syscall::linux;
use crate::syscall::windows;
use crate::{color, exception::types};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    debug_assert!(ins.op_count() == 1);

    let interrupt = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let handle_interrupts = if let Some(mut hook_fn) = emu.hooks.hook_on_interrupt.take() {
        let rip = emu.regs().rip;
        let result = hook_fn(emu, rip, interrupt);
        emu.hooks.hook_on_interrupt = Some(hook_fn);
        result
    } else {
        true
    };

    if handle_interrupts {
        match interrupt {
            0x80 => {
                // Do not set `emu.linux` here: it would mis-route later `syscall` on PE/SSDT.
                if emu.os.is_linux() {
                    if emu.cfg.is_x64() {
                        linux::syscall64::gateway(emu);
                    } else {
                        linux::syscall32::gateway(emu);
                    }
                } else if emu.cfg.is_x64() {
                    windows::syscall64::gateway(emu);
                } else {
                    windows::syscall32::gateway(emu);
                }
            }

            0x29 => {
                let code = emu.regs().rcx;
                log::trace!("int 0x29: __fastfail({}) {}", code, fastfail_name(code));
                if code == 2 {
                    let (cmp_pos, cmp_ip, cmp_left, cmp_right) = emu.last_cmp;
                    let cs = emu.call_stack().clone();
                    log::info!("--- STACK COOKIE CHECK FAILURE ---");
                    if !cs.is_empty() {
                        log::info!("  call stack:");
                        let mut prev: Option<(u64, u64)> = None;
                        let mut repeat: u64 = 0;
                        for (call_site, target) in cs.iter() {
                            let entry = (*call_site, *target);
                            if prev == Some(entry) {
                                repeat += 1;
                                continue;
                            }
                            if repeat > 0 {
                                log::info!("      ... x{}", repeat + 1);
                            }
                            prev = Some(entry);
                            repeat = 0;
                            if let Some((fb, fe)) = emu.resolve_function_range(*call_site) {
                                log::info!(
                                    "    call 0x{:x} at 0x{:x} (fn 0x{:x}..0x{:x})",
                                    target,
                                    call_site,
                                    fb,
                                    fe
                                );
                            } else {
                                log::info!("    call 0x{:x} at 0x{:x}", target, call_site);
                            }
                        }
                        if repeat > 0 {
                            log::info!("      ... x{}", repeat + 1);
                        }
                    }
                    log::info!(
                        "  last cmp: 0x{:x} vs 0x{:x} (at pos:{} rip:0x{:x})",
                        cmp_left,
                        cmp_right,
                        cmp_pos,
                        cmp_ip
                    );
                    if let Some(cookie) = emu.read_security_cookie() {
                        log::info!("  __security_cookie = 0x{:x}", cookie);
                    }
                    let (sub_pos, sub_ip, sub_left, sub_right) = emu.last_sub;
                    if sub_pos > cmp_pos {
                        log::info!(
                            "  last sub: 0x{:x} - 0x{:x} (at pos:{} rip:0x{:x}) — MORE RECENT than cmp",
                            sub_left,
                            sub_right,
                            sub_pos,
                            sub_ip
                        );
                    }
                    log::info!("  ZF={} at int 0x29", emu.flag_zf());
                    if cmp_left == cmp_right {
                        log::warn!(
                            "  last cmp had EQUAL operands but __fastfail fired — the branch was likely decided by a different instruction (sub/test/xor)"
                        );
                    }
                    let cookie_check_fn = emu.resolve_function_range(cmp_ip);
                    for (call_site, target) in cs.iter() {
                        let target_fn = emu.resolve_function_range(*target);
                        if target_fn.is_some() && target_fn == cookie_check_fn {
                            if let Some((fb, fe)) = emu.resolve_function_range(*call_site) {
                                log::info!(
                                    "  vulnerable function: 0x{:x}..0x{:x} (called cookie check from 0x{:x})",
                                    fb,
                                    fe,
                                    call_site
                                );
                            } else {
                                log::info!("  cookie check called from 0x{:x}", call_site);
                            }
                            break;
                        }
                    }
                }
                if emu.kernel_driver_active() {
                    let mut reason = format!("__fastfail({}) {}", code, fastfail_name(code));
                    if code == 2 {
                        let (cmp_pos, cmp_ip, left, right) = emu.last_cmp;
                        reason.push_str(&format!(
                            " cmp 0x{:x} vs 0x{:x} (cmp at 0x{:x}, {} steps ago)",
                            left,
                            right,
                            cmp_ip,
                            emu.pos.saturating_sub(cmp_pos)
                        ));
                    }
                    emu.kernel_abort(reason);
                } else {
                    emu.stop();
                }
                return false;
            }

            0x03 => {
                emu.show_instruction(
                    color!("Red"),
                    &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
                );
                log::trace!("/!\\ int 0x3 sigtrap!!!!");
                emu.exception(types::ExceptionType::Int3);
                // If no exception handler is installed (no SEH/VEH), treat int3 as
                // clean process exit rather than an emulation error. On real Windows,
                // hitting int3 without a debugger terminates the process.
                if emu.seh() == 0 && emu.veh() == 0 && emu.uef() == 0 {
                    emu.stop();
                    return true;
                }
                return false;
            }

            0xdc => {
                log::trace!("/!\\ direct syscall: NtAlpcSendWaitReceivePort");
            }

            _ => {
                log::trace!("unimplemented interrupt {}", interrupt);
                return false;
            }
        }
    }
    true
}

/// Names of the common `__fastfail` codes, for the abort report.
fn fastfail_name(code: u64) -> &'static str {
    match code {
        0 => "FAST_FAIL_LEGACY_GS_VIOLATION",
        1 => "FAST_FAIL_VTGUARD_CHECK_FAILURE",
        2 => "FAST_FAIL_STACK_COOKIE_CHECK_FAILURE",
        3 => "FAST_FAIL_CORRUPT_LIST_ENTRY",
        4 => "FAST_FAIL_INCORRECT_STACK",
        5 => "FAST_FAIL_INVALID_ARG",
        6 => "FAST_FAIL_GS_COOKIE_INIT",
        7 => "FAST_FAIL_FATAL_APP_EXIT",
        8 => "FAST_FAIL_RANGE_CHECK_FAILURE",
        9 => "FAST_FAIL_UNSAFE_REGISTRY_ACCESS",
        10 => "FAST_FAIL_GUARD_ICALL_CHECK_FAILURE",
        _ => "",
    }
}
