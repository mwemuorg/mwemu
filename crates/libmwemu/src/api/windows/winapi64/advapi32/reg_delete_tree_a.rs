use crate::emu;

pub fn RegDeleteTreeA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "advapi32!RegDeleteTreeA hkey:0x{:x} '{}'", hkey, name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
