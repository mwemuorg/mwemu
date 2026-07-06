use crate::emu;

pub fn SetHandleCount(emu: &mut emu::Emu) {
    let num = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!SetHandleCount error getting num param");

    log_red!(emu, "kernel32!SetHandleCount {}", num);

    emu.stack_pop32(false);
    emu.regs_mut().rax = num as u64;
}
