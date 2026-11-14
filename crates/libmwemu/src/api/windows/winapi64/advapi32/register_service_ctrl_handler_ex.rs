use crate::emu;
use crate::winapi::helper;

pub fn RegisterServiceCtrlHandlerExA(emu: &mut emu::Emu) {
    register_service_ctrl_handler_ex(emu, "RegisterServiceCtrlHandlerExA");
}

fn register_service_ctrl_handler_ex(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "advapi32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let hndl = helper::handler_create(&format!("advapi32://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn RegisterServiceCtrlHandlerExW(emu: &mut emu::Emu) {
    register_service_ctrl_handler_ex(emu, "RegisterServiceCtrlHandlerExW");
}
