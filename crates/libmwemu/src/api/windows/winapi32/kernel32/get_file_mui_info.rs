use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetFileMUIInfo(emu: &mut emu::Emu) {
    let flags = arg(emu, 0);
    let file_path_ptr = arg(emu, 1) as u64;
    let info_ptr = arg(emu, 2) as u64;
    let info_size_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetFileMUIInfo flags:0x{:x} path:0x{:x}",
        flags,
        file_path_ptr
    );
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
