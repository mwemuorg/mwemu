//! Guest-call diagnostics belong at the ABI boundary, outside the memory operations.
use crate::emu::Emu;
use crate::winapi::common::memory::Call;
use std::fmt::Write;

pub(crate) fn api(
    emu: &mut Emu,
    name: &str,
    parameters: &str,
    returns_value: bool,
    handler: impl FnOnce(&mut Emu),
) {
    invoke(emu, Call::Api, name, parameters, returns_value, handler);
}

pub(crate) fn invoke(
    emu: &mut Emu,
    call: Call,
    name: &str,
    parameters: &str,
    returns_value: bool,
    handler: impl FnOnce(&mut Emu),
) {
    if !log::log_enabled!(log::Level::Trace) || (matches!(call, Call::Api) && emu.cfg.verbose < 1) {
        handler(emu);
        return;
    }
    // Capture inputs before stdcall cleanup, output writes or return-register updates.
    let mut arguments = String::new();
    for (index, parameter) in parameters.split_whitespace().enumerate() {
        if index != 0 {
            arguments.push_str(", ");
        }
        write!(&mut arguments, "{parameter}: 0x{:x}", call.arg(emu, index)).unwrap();
    }
    let number = emu.regs().rax;
    handler(emu);
    let result = if returns_value {
        format!(" = 0x{:x}", emu.regs().rax)
    } else {
        String::new()
    };
    match call {
        Call::Api => {
            log_red!(emu, "{}({}){}", name, arguments, result);
        }
        Call::Syscall => {
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
