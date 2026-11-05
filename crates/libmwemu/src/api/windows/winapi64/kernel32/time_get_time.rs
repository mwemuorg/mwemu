use crate::emu;

pub fn timeGetTime(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!timeGetTime tick:{}", emu.tick);

    emu.set_last_error(0);
    emu.regs_mut().rax = emu.tick as u64;
}
