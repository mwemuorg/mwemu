use crate::emu;
use crate::winapi::helper;

pub fn CoGetClassObject(emu: &mut emu::Emu) {
    let clsid_ptr = emu.regs().rcx;
    let ctx = emu.regs().rdx;
    let server_info = emu.regs().r8;
    let iid_ptr = emu.regs().r9;
    let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let hndl = helper::handler_create("com_class_factory");
    if ppv != 0 {
        emu.maps.write_qword(ppv, hndl);
    }
    log_red!(
        emu,
        "ole32!CoGetClassObject ctx:0x{:x} => 0x{:x}",
        ctx,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
