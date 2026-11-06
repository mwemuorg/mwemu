use crate::emu;

pub fn ZombifyActCtxWorker(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;

    log_red!(emu, "kernel32!ZombifyActCtxWorker param1:0x{:x}", param1);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
