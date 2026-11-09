use crate::emu;

pub fn GlobalAddAtomW_api(emu: &mut emu::Emu) {
    let name = emu.regs().rcx;
    let name_s = if name != 0 {
        emu.maps.read_wide_string(name)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!GlobalAddAtomW '{}' => 0xC001", name_s);
    emu.regs_mut().rax = 0xC001;
}
