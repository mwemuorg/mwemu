use crate::emu;

pub fn handler(emu: &mut emu::Emu) {
    let frame = emu.regs().rcx;
    let target = emu.regs().rdx;
    log_red!(emu, "kernel32!_local_unwind frame:0x{:x} target:0x{:x}", frame, target);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = 0;
}
