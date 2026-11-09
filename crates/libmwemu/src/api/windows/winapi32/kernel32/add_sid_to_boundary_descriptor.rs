use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn AddSIDToBoundaryDescriptor(emu: &mut emu::Emu) {
    let boundary_descriptor = arg(emu, 0) as u64;
    let required_sid = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!AddSIDToBoundaryDescriptor desc:0x{:x} sid:0x{:x}",
        boundary_descriptor,
        required_sid
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
