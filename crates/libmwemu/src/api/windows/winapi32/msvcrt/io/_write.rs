use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn _write(emu: &mut emu::Emu) {
    let fd = WinApiAbi::arg(emu, 0);
    let buf = WinApiAbi::arg(emu, 1);
    let count = WinApiAbi::arg(emu, 2) as u32;
    let name = helper::handler_get_uri(fd);
    log_red!(
        emu,
        "msvcrt!_write fd:{} '{}' buf:0x{:x} count:{}",
        fd,
        name,
        buf,
        count
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = count as u64;
}
