use crate::emu;
use crate::winapi::helper;

pub fn fseek(emu: &mut emu::Emu) {
    let fp = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let offset = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32 as i64;
    let origin = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let new_pos = helper::handler_seek(fp, offset, origin);
    log_red!(
        emu,
        "msvcrt!fseek fp:0x{:x} offset:{} origin:{} => pos:{}",
        fp,
        offset,
        origin,
        new_pos
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = if new_pos >= 0 { 0 } else { u64::MAX };
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
