use crate::emu;

pub fn TrackPopupMenu(emu: &mut emu::Emu) {
    let hmenu = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let x = emu.regs().r8 as i32;
    let y = emu.regs().r9 as i32;
    let _reserved = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let hwnd = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _lp_tpm = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    log_red!(
        emu,
        "user32!TrackPopupMenu hmenu:0x{:x} flags:0x{:x} x:{} y:{} hwnd:0x{:x}",
        hmenu,
        flags,
        x,
        y,
        hwnd
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
