use crate::emu;
use crate::winapi::helper;

pub fn ftell(emu: &mut emu::Emu) {
    let fp = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pos = helper::handler_tell(fp);
    log_red!(emu, "msvcrt!ftell fp:0x{:x} => {}", fp, pos);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = pos as u64;
}
