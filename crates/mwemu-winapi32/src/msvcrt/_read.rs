use crate::emu;
use crate::winapi::helper;

pub fn _read(emu: &mut emu::Emu) {
    let fd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let name = helper::handler_get_uri(fd);
    log_red!(
        emu,
        "msvcrt!_read fd:{} '{}' buf:0x{:x} count:{}",
        fd,
        name,
        buf,
        count
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
