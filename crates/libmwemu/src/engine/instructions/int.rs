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
                log::trace!("call_stack = {:?}", emu.call_stack());
                log::trace!("int 0x29: __fastfail {}", emu.regs().rcx);
                if emu.kernel_driver_active() {
                    let code = emu.regs().rcx;
                    emu.kernel_abort(format!("__fastfail({}) {}", code, fastfail_name(code)));
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
