use crate::emu;
use crate::winapi::helper;

pub fn ftell(emu: &mut emu::Emu) {
    let fp = emu.regs().rcx;
    let pos = helper::handler_tell(fp);
    log_red!(emu, "msvcrt!ftell fp:0x{:x} => {}", fp, pos);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = pos as u64;
}
