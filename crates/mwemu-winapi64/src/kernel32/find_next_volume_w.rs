use crate::emu;
use crate::winapi::helper;

pub fn FindNextVolumeW(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let volume_name_ptr = emu.regs().rdx;
    let buf_len = emu.regs().r8 as u32;
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
}
