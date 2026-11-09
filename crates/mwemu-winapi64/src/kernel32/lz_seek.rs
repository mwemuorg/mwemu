use crate::emu;

pub fn LZSeek(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let offset = emu.regs().rdx as i32;
    let origin = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!LZSeek hFile:0x{:x} offset:{} origin:{}",
        h_file,
        offset,
        origin
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
