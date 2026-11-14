use crate::emu;
use crate::winapi::helper;

pub fn RegisterServiceCtrlHandlerA(emu: &mut emu::Emu) {
    register_service_ctrl_handler(emu, "RegisterServiceCtrlHandlerA");
}

fn register_service_ctrl_handler(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "advapi32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let hndl = helper::handler_create(&format!("advapi32://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn RegisterServiceCtrlHandlerW(emu: &mut emu::Emu) {
    register_service_ctrl_handler(emu, "RegisterServiceCtrlHandlerW");
}
