use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn fseek(emu: &mut emu::Emu) {
    let fp = WinApiAbi::arg(emu, 0);
    let offset = WinApiAbi::arg(emu, 1) as i32 as i64;
    let origin = WinApiAbi::arg(emu, 2) as u32;
    let new_pos = helper::handler_seek(fp, offset, origin);
    log_red!(
        emu,
        "msvcrt!fseek fp:0x{:x} offset:{} origin:{} => pos:{}",
        fp,
        offset,
        origin,
        new_pos
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = if new_pos >= 0 { 0 } else { u64::MAX };
}
