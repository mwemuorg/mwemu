use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetWorldTransform(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0) as u64;
    let lpxform = arg(emu, 1) as u64;
    log_red!(
        emu,
        "gdi32!SetWorldTransform hdc:0x{:x} xform:0x{:x}",
        hdc,
        lpxform
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
