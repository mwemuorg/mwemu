use crate::emu;

pub fn SetHandleCount(emu: &mut emu::Emu) {
    let num = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);

    log_red!(emu, "kernel32!SetHandleCount {}", num);

    emu.stack_pop32(false);
    emu.regs_mut().rax = num as u64;
}
