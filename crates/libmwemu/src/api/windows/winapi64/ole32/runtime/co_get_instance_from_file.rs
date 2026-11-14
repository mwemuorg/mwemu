use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CoGetInstanceFromFile(emu: &mut emu::Emu) {
    let clsid_ptr = emu.regs().rcx;
    let outer = emu.regs().rdx;
    let ctx = emu.regs().r8;
    let iid_ptr = emu.regs().r9;
    let ppv = WinApiAbi::arg(emu, 5);
    let hndl = helper::handler_create("com_object");
    let ppv_addr = ppv;
    if ppv_addr != 0 {
        emu.maps.write_qword(ppv_addr, hndl);
    }
    log_red!(
        emu,
        "ole32!CoGetInstanceFromFile ctx:0x{:x} => 0x{:x}",
        ctx,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
