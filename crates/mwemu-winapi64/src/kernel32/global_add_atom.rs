use crate::emu;

pub fn GlobalAddAtomA(emu: &mut emu::Emu) {
    let lp_string = emu.regs().rcx;

    let name = emu.maps.read_string(lp_string);
    log_red!(emu, "kernel32!GlobalAddAtomA '{}'", name);

    let mut hash: u16 = 0xC000;
    for b in name.bytes() {
        hash = hash.wrapping_add(b as u16);
    }
    if hash < 0xC000 {
        hash = hash.wrapping_add(0xC000);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = hash as u64;
}

pub fn GlobalAddAtomW(emu: &mut emu::Emu) {
    let name = emu.regs().rcx;
    let name_s = if name != 0 {
        emu.maps.read_wide_string(name)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!GlobalAddAtomW '{}' => 0xC001", name_s);
    emu.regs_mut().rax = 0xC001;
}
