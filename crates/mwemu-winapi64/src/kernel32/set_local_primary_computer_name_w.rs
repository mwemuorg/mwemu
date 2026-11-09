use crate::emu;

pub fn SetLocalPrimaryComputerNameW(emu: &mut emu::Emu) {
    let String = emu.regs().rcx as u32;
    let _a2 = emu.regs().rdx as u32;

    let String = emu.maps.read_wide_string(String as u64);

    log_red!(emu, "kernel32!SetLocalPrimaryComputerNameW {}", String);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
