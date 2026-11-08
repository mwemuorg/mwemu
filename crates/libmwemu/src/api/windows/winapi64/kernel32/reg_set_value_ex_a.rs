use crate::emu;

pub fn RegSetValueExA(emu: &mut emu::Emu) {
    let _hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let _type_ = emu.regs().r9;

    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(emu, "kernel32!RegSetValueExA '{}'", name);
    emu.regs_mut().rax = 0;
}
