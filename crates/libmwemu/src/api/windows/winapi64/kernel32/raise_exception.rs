use crate::emu;

pub fn RaiseException(emu: &mut emu::Emu) {
    let code = emu.regs().rcx as u32;
    let flags = emu.regs().rdx as u32;
    let num_args = emu.regs().r8 as u32;
    let args_ptr = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!RaiseException code:0x{:08x} flags:0x{:x} nArgs:{} pArgs:0x{:x}",
        code,
        flags,
        num_args,
        args_ptr
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
}
