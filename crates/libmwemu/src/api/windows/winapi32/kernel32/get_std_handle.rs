use crate::emu;

pub fn GetStdHandle(emu: &mut emu::Emu) {
    let nstd = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!GetStdHandle error reading nstd param");

    log_red!(emu, "kernel32!GetStdHandle {}", nstd);

    emu.stack_pop32(false);
    emu.regs_mut().rax = nstd as u64;
}
