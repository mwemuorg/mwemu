use crate::emu;

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx as u32;
    let subkey_ptr = emu.regs().rdx;
    let options = emu.regs().r8 as u32;
    let sam = emu.regs().r9 as u32;
    let result = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let subkey = emu.maps.read_wide_string(subkey_ptr);
    log_red!(emu, "kernel32!RegOpenKeyExW {}", subkey);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
