use crate::emu;

pub fn RegDeleteKeyExA(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "advapi32!RegDeleteKeyExA hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 0;
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
