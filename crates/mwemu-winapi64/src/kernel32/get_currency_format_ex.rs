use crate::emu;

pub fn GetCurrencyFormatEx(emu: &mut emu::Emu) {
    let locale_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let value_ptr = emu.regs().r8;
    let format_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetCurrencyFormatEx locale:0x{:x} flags:0x{:x} val:0x{:x}",
        locale_ptr,
        flags,
        value_ptr
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
