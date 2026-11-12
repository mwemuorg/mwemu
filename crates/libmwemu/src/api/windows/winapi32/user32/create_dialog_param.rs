use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateDialogParamA(emu: &mut emu::Emu) {
    create_dialog_param(emu, "CreateDialogParamA", false);
}

pub fn CreateDialogParamW(emu: &mut emu::Emu) {
    create_dialog_param(emu, "CreateDialogParamW", true);
}

fn create_dialog_param(emu: &mut emu::Emu, api: &str, wide: bool) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "user32!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    if wide {
        emu.regs_mut().rax = helper::handler_create("createdialogparamw");
    } else {
        emu.regs_mut().rax = helper::handler_create("createdialogparama");
    }

    stdcall_ret(emu, 5);
}
