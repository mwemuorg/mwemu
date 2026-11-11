use crate::emu;

pub fn lstrlen(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!lstrlen '{}' => {}", s, s.len());
    emu.set_last_error(0);
    emu.regs_mut().rax = s.len() as u64;
}

pub fn lstrlenA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!lstrlenA '{}' => {}", s, s.len());
    emu.set_last_error(0);
    emu.regs_mut().rax = s.len() as u64;
}

pub fn lstrlenW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!lstrlenW '{}' => {}", s, s.len());
    emu.set_last_error(0);
    emu.regs_mut().rax = s.len() as u64;
}
