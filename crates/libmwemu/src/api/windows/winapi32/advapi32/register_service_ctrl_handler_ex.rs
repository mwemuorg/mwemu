use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn RegisterServiceCtrlHandlerExA(emu: &mut emu::Emu) {
    register_service_ctrl_handler_ex(emu, "RegisterServiceCtrlHandlerExA");
}

fn register_service_ctrl_handler_ex(emu: &mut emu::Emu, api: &str) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    let arg3 = WinApiAbi::arg(emu, 2) as u32;
    log_red!(
        emu,
        "advapi32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create(&format!("advapi32://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn RegisterServiceCtrlHandlerExW(emu: &mut emu::Emu) {
    register_service_ctrl_handler_ex(emu, "RegisterServiceCtrlHandlerExW");
}
