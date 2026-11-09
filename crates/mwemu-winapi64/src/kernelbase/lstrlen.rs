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
