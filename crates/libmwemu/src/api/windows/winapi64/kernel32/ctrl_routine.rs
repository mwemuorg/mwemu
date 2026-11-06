use crate::emu;

pub fn CtrlRoutine(emu: &mut emu::Emu) {
    let ctrl_type = emu.regs().rcx;
    log_red!(emu, "kernel32!CtrlRoutine ctrlType:{}", ctrl_type);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
}
