use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn ftell(emu: &mut emu::Emu) {
    let fp = WinApiAbi::arg(emu, 0);
    let pos = helper::handler_tell(fp);
    log_red!(emu, "msvcrt!ftell fp:0x{:x} => {}", fp, pos);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = pos as u64;
}
