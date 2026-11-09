use crate::emu;
pub fn IsEnclaveTypeSupported(emu: &mut emu::Emu) {
    let enclave_type = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!IsEnclaveTypeSupported type:0x{:x} => FALSE",
        enclave_type
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
