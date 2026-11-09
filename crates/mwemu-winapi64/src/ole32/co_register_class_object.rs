use crate::emu;
use crate::winapi::helper;

pub fn CoRegisterClassObject(emu: &mut emu::Emu) {
    let rclsid = emu.regs().rcx;
    let punk = emu.regs().rdx;
    let cls_ctx = emu.regs().r8;
    let flags = emu.regs().r9;
    let pdw_register = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let hndl = helper::handler_create("com_class_registration");
    if pdw_register != 0 {
        emu.maps.write_dword(pdw_register, hndl as u32);
    }
    log_red!(
        emu,
        "ole32!CoRegisterClassObject clsid:0x{:x} ctx:{} => 0x{:x}",
        rclsid,
        cls_ctx,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
