use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetCurrentObject(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let obj_type = arg(emu, 1);
    log_red!(
        emu,
        "gdi32!GetCurrentObject hdc:0x{:x} type:{}",
        hdc,
        obj_type
    );
    let hndl = helper::handler_create("gdi32://current_object");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 2);
}
