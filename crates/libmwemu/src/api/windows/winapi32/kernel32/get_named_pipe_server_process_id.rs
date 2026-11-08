use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNamedPipeServerProcessId(emu: &mut emu::Emu) {
    let h_pipe = arg(emu, 0) as u64;
    let pid_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetNamedPipeServerProcessId h:0x{:x} ptr:0x{:x}",
        h_pipe,
        pid_ptr
    );
    if pid_ptr != 0 {
        emu.maps.write_dword(pid_ptr, 0x123);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
