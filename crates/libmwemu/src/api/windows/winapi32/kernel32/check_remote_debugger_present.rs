use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;

pub fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    let process = arg(emu, 0) as u64;
    let present_ptr = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!CheckRemoteDebuggerPresent hProcess: 0x{:x} pbDebuggerPresent: 0x{:x}",
        process,
        present_ptr
    );

    if process == 0 || present_ptr == 0 {
        emu.set_last_error(constants::ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = 0;
        return;
    }

    let valid = process == constants::CURRENT_PROCESS_HANDLE
        || process == u64::MAX
        || helper::handler_exist(process);
    if !valid {
        emu.set_last_error(constants::ERROR_INVALID_HANDLE);
        emu.regs_mut().rax = 0;
        return;
    }

    emu.maps.write_dword(present_ptr, 0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
