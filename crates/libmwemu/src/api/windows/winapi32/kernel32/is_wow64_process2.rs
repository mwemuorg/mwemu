use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn IsWow64Process2(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let p_process_machine = arg(emu, 1) as u64;
    let p_native_machine = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!IsWow64Process2 hProcess:0x{:x} pProcessMachine:0x{:x} pNativeMachine:0x{:x}",
        h_process,
        p_process_machine,
        p_native_machine
    );
    if p_process_machine != 0 {
        emu.maps.write_dword(p_process_machine, 0);
    }
    if p_native_machine != 0 {
        emu.maps.write_dword(p_native_machine, 0x8664);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
