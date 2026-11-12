use crate::emu;
use crate::winapi::helper;

pub fn fwrite(emu: &mut emu::Emu) {
    let buff_ptr = emu.regs().rcx;
    let size = emu.regs().rdx as u32;
    let nemb = emu.regs().r8 as u32;
    let file = emu.regs().r9;
    let filename = helper::handler_get_uri(file);
    log_red!(
        emu,
        "msvcrt!fwrite `{}` 0x{:x} {}",
        filename,
        buff_ptr,
        size
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = size as u64;
}
