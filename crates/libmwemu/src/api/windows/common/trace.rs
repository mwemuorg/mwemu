//! Guest-call diagnostics belong at the ABI boundary, outside the memory operations.
use crate::api::abi::windows::{
    CallingConvention::Stdcall,
    WindowsAbi,
    WindowsAbi::{WinApi, WinInterrupt, WinSyscall, WinSysenter},
};
use crate::emu::Emu;
use std::fmt::Write;

pub(crate) fn api(
    emu: &mut Emu,
    name: &str,
    parameters: &str,
    returns_value: bool,
    handler: impl FnOnce(&mut Emu),
) {
    let call = WinApi(Stdcall);
    invoke(emu, call, name, parameters, returns_value, handler);
}

pub(crate) fn invoke(
    emu: &mut Emu,
    call: WindowsAbi,
    name: &str,
    parameters: &str,
    returns_value: bool,
    handler: impl FnOnce(&mut Emu),
) {
    if !log::log_enabled!(log::Level::Trace) || (matches!(call, WinApi(_)) && emu.cfg.verbose < 1) {
        handler(emu);
        return;
    }
    // Capture inputs before stdcall cleanup, output writes or return-register updates.
    let mut arguments = String::new();
    for (index, parameter) in parameters.split_whitespace().enumerate() {
        if index != 0 {
            arguments.push_str(", ");
        }
        match call.arg(emu, index) {
            Ok(value) => {
                let _ = write!(&mut arguments, "{parameter}: 0x{value:x}");
            }
            Err(error) => {
                let _ = write!(&mut arguments, "{parameter}: <{:?}>", error.fault);
            }
        }
    }
    let number = emu.regs().rax;
    handler(emu);
    let result = if emu.api_policy.stopped() {
        " (stopped)".to_string()
    } else if returns_value {
        format!(" = 0x{:x}", emu.regs().rax)
    } else {
        String::new()
    };
    match call {
        WinApi(_) => {
            log_red!(emu, "{}({}){}", name, arguments, result);
        }
        WinSyscall | WinSysenter | WinInterrupt => {
            log_orange!(
                emu,
                "syscall 0x{:x}: {}({}){}",
                number,
                name,
                arguments,
                result
            );
        }
    }
}
