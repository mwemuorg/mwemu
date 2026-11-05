use crate::emu;

pub fn CheckForReadOnlyResourceFilter(emu: &mut emu::Emu) {
    let p_exception = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!CheckForReadOnlyResourceFilter pException:0x{:x}",
        p_exception
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
