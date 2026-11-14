use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn fread(emu: &mut emu::Emu) {
    let buf = WinApiAbi::arg(emu, 0);
    let size = WinApiAbi::arg(emu, 1) as u32;
    let count = WinApiAbi::arg(emu, 2) as u32;
    let fp = WinApiAbi::arg(emu, 3);
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
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::handler_set_eof(fp);
    emu.regs_mut().rax = 0;
}
