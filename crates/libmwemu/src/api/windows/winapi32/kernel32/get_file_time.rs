use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetFileTime(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let creation_ptr = arg(emu, 1) as u64;
    let access_ptr = arg(emu, 2) as u64;
    let write_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetFileTime h:0x{:x} creation:0x{:x} access:0x{:x} write:0x{:x}",
        h_file,
        creation_ptr,
        access_ptr,
        write_ptr
    );
    let filetime: u64 = 132514560000000000;
    if creation_ptr != 0 {
        emu.maps.write_qword(creation_ptr, filetime);
    }
    if access_ptr != 0 {
        emu.maps.write_qword(access_ptr, filetime);
    }
    if write_ptr != 0 {
        emu.maps.write_qword(write_ptr, filetime);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
