use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn QueryFullProcessImageNameA(emu: &mut emu::Emu) {
    let _h_process = arg(emu, 0) as u64;
    let _flags = arg(emu, 1) as u64;
    let name_ptr = arg(emu, 2) as u64;
    let size_ptr = arg(emu, 3) as u64;

    let name = "C:\\Windows\\System32\\emulated.exe";

    if name_ptr != 0 {
        emu.maps.write_string(name_ptr, name);
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, name.len() as u32);
    }

    log_red!(emu, "kernel32!QueryFullProcessImageNameA => '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
