use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn IsEnclaveTypeSupported(emu: &mut emu::Emu) {
    let enclave_type = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!IsEnclaveTypeSupported type:0x{:x} => FALSE",
        enclave_type
    );
    emu.set_last_error(0);
    let h = arg(emu, 0) as u64;
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
