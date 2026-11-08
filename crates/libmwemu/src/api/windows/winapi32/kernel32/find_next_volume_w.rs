use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FindNextVolumeW(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let volume_name_ptr = arg(emu, 1) as u64;
    let buf_len = arg(emu, 2) as u64 as u32;
    log_red!(
        emu,
        "kernel32!FindNextVolumeW h:0x{:x} buf:0x{:x} len:{}",
        handle,
        volume_name_ptr,
        buf_len
    );
    if helper::handler_exist(handle) && volume_name_ptr != 0 && buf_len > 0 {
        emu.maps.write_wide_string(volume_name_ptr, "");
    }
    emu.regs_mut().rax = 0; // FALSE - no more volumes
    emu.set_last_error(0x12); // ERROR_NO_MORE_FILES
    stdcall_ret(emu, 3);
}
