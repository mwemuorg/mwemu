use crate::emu;

pub fn GlobalHandle(emu: &mut emu::Emu) {
    let p_mem = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalHandle pMem:0x{:x}", p_mem);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = p_mem;
}
