use crate::emu;
use crate::winapi::helper;

pub fn _read(emu: &mut emu::Emu) {
    let fd = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let count = emu.regs().r8 as u32;
    let name = helper::handler_get_uri(fd);
    log_red!(
        emu,
        "msvcrt!_read fd:{} '{}' buf:0x{:x} count:{}",
        fd,
        name,
        buf,
        count
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
