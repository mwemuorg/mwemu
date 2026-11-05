use crate::emu;

pub fn QueueUserWorkItem(emu: &mut emu::Emu) {
    let function = emu.regs().rcx;
    let context = emu.regs().rdx;
    let flags = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!QueueUserWorkItem func:0x{:x} ctx:0x{:x} flags:0x{:x}",
        function,
        context,
        flags
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
