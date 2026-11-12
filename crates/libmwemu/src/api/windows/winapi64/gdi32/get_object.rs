use crate::emu;

pub fn GetObjectA(emu: &mut emu::Emu) {
    get_object(emu, "GetObjectA");
}

fn get_object(emu: &mut emu::Emu, api: &str) {
    let h = emu.regs().rcx;
    let cb = emu.regs().rdx as u32;
    let pv = emu.regs().r8;
    log_red!(emu, "gdi32!{api} h:0x{:x} cb:{} pv:0x{:x}", h, cb, pv);
    if pv != 0 && cb > 0 {
        // Zero-fill the buffer first
        let fill = (cb as u64).min(64);
        for i in 0..fill {
            emu.maps.write_byte(pv + i, 0);
        }
        // Fill as BITMAP structure if buffer is large enough
        if cb >= 24 {
            emu.maps.write_dword(pv, 0); // bmType
            emu.maps.write_dword(pv + 4, 64); // bmWidth
            emu.maps.write_dword(pv + 8, 64); // bmHeight
            emu.maps.write_dword(pv + 12, 256); // bmWidthBytes
            emu.maps.write_word(pv + 16, 1); // bmPlanes
            emu.maps.write_word(pv + 18, 32); // bmBitsPixel
        }
        emu.regs_mut().rax = fill;
    } else {
        // When pv is NULL, return the required buffer size
        emu.regs_mut().rax = 24; // BITMAP struct size
    }
}

pub fn GetObjectW(emu: &mut emu::Emu) {
    get_object(emu, "GetObjectW");
}
