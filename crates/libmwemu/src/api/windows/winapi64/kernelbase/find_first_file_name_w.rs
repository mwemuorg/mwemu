use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileNameW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let find_data_ptr = emu.regs().rdx;
    let path = emu.maps.read_wide_string(path_ptr);
    let hndl = helper::handler_create(&format!("find_{}", path));
    if find_data_ptr != 0 {
        emu.maps.write_dword(find_data_ptr, 0x80); // dwFileAttributes = NORMAL
    }
    log_red!(
        emu,
        "kernelbase!FindFirstFileNameW '{}' => 0x{:x}",
        path,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
