use crate::emu;

pub fn ToAsciiEx(emu: &mut emu::Emu) {
    let virt_key = emu.regs().rcx as u32;
    let scan_code = emu.regs().rdx as u32;
    let _lp_key_state = emu.regs().r8;
    let lp_char = emu.regs().r9;
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _hkl = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let ch = if (0x20..=0x7e).contains(&virt_key) {
        virt_key as u8
    } else {
        0
    };
    if lp_char != 0 {
        emu.maps.write_word(lp_char, ch as u16);
    }
    log_red!(
        emu,
        "user32!ToAsciiEx vk:0x{:x} scan:0x{:x} => 0x{:x}",
        virt_key,
        scan_code,
        ch
    );
    emu.regs_mut().rax = if ch != 0 { 1 } else { 0 };
}
