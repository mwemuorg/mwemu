use crate::emu;

pub fn RaiseException(emu: &mut emu::Emu) {
    let code = emu.regs().rcx as u32;
    let flags = emu.regs().rdx as u32;
    let num_args = emu.regs().r8 as u32;
    let args_ptr = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!RaiseException code:0x{:x} flags:0x{:x} nargs:{} args:0x{:x}",
        code,
        flags,
        num_args,
        args_ptr,
    );

    emu.regs_mut().rax = 0;
}
