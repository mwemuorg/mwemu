use crate::emu;

pub fn RegGetKeySecurity(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let sec_info = emu.regs().rdx as u32;
    let sec_desc = emu.regs().r8;
    let cb_sec_desc = emu.regs().r9;
    log_red!(
        emu,
        "kernelbase!RegGetKeySecurity hkey:0x{:x} info:0x{:x}",
        hkey,
        sec_info
    );
    if cb_sec_desc != 0 {
        emu.maps.write_dword(cb_sec_desc, 20);
    }
    emu.regs_mut().rax = 122; // ERROR_INSUFFICIENT_BUFFER
}
