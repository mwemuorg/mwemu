use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn ToUnicodeEx(emu: &mut emu::Emu) {
    let virt_key = emu.regs().rcx as u32;
    let scan_code = emu.regs().rdx as u32;
    let _lp_key_state = emu.regs().r8;
    let pwsz_buff = emu.regs().r9;
    let cch_buff = WinApiAbi::arg(emu, 4) as i32;
    let _flags = WinApiAbi::arg(emu, 5);
    let _hkl = WinApiAbi::arg(emu, 6);
    let ch = if (0x20..=0x7e).contains(&virt_key) {
        virt_key as u16
    } else {
        0
    };
    if pwsz_buff != 0 && cch_buff > 0 {
        emu.maps.write_word(pwsz_buff, ch);
        if cch_buff > 1 {
            emu.maps.write_word(pwsz_buff + 2, 0);
        }
    }
    log_red!(
        emu,
        "user32!ToUnicodeEx vk:0x{:x} scan:0x{:x} => 0x{:x}",
        virt_key,
        scan_code,
        ch
    );
    emu.regs_mut().rax = if ch != 0 { 1 } else { 0 };
}
