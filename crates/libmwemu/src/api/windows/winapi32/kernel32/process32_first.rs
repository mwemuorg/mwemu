use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn Process32First(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let lppe = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!Process32First hndl: 0x{:x} lppe: 0x{:x}",
        handle,
        lppe
    );
    if !helper::handler_exist(handle) {
        emu.regs_mut().rax = 0;
    } else {
        // PROCESSENTRY32: szExeFile at +36 on 32-bit (ULONG_PTR th32DefaultHeapID is 4 bytes)
        emu.maps.write_dword(lppe + 8, 4); // th32ProcessID
        emu.maps.write_string(lppe + 36, "smss.exe");
        emu.regs_mut().rax = 1;
    }
    stdcall_ret(emu, 2);
}
