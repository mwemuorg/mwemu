use crate::emu;
use crate::winapi::helper;

pub fn fread(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx as u32;
    let count = emu.regs().r8 as u32;
    let fp = emu.regs().r9;
    let name = helper::handler_get_uri(fp);
    log_red!(
        emu,
        "msvcrt!fread buf:0x{:x} size:{} count:{} fp:0x{:x} '{}'",
        buf,
        size,
        count,
        fp,
        name
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::handler_set_eof(fp);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
