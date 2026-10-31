use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

/*
BOOL CheckRemoteDebuggerPresent(
  [in]      HANDLE hProcess,
  [in, out] PBOOL  pbDebuggerPresent
);
*/
pub fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    let process = emu.regs().rcx;
    let present_ptr = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!CheckRemoteDebuggerPresent hProcess: 0x{:x} pbDebuggerPresent: 0x{:x}",
        process,
        present_ptr
    );

    // Anti-debug code probes the argument validation too: a hook that always
    // answers "no debugger" gives itself away on NULL or bogus arguments.
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
}
