use crate::emu;

pub fn GetStdHandle(emu: &mut emu::Emu) {
    let nstd = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);

    log_red!(emu, "kernel32!GetStdHandle {}", nstd);

    emu.stack_pop32(false);
    emu.regs_mut().rax = nstd as u64;
}
