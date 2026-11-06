use crate::emu;

pub fn RtlZeroMemory(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let len = emu.regs().rdx as usize;

    log_red!(emu, "kernel32!RtlZeroMemory dest:0x{:x} len:{}", dest, len);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    if dest != 0 && len > 0 && len <= 0x100_0000 {
        emu.maps.memset(dest, 0, len);
    }
}
