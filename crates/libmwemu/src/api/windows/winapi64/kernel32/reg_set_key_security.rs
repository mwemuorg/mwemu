use crate::emu;

pub fn RegSetKeySecurity(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!RegSetKeySecurity hkey:0x{:x} param2:0x{:x} param3:0x{:x}",
        hkey,
        param2,
        param3
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
