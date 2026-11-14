use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateDialogIndirectParamA(emu: &mut emu::Emu) {
    create_dialog_indirect_param(emu, "CreateDialogIndirectParamA", false);
}

pub fn CreateDialogIndirectParamW(emu: &mut emu::Emu) {
    create_dialog_indirect_param(emu, "CreateDialogIndirectParamW", true);
}

fn create_dialog_indirect_param(emu: &mut emu::Emu, api: &str, wide: bool) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "user32!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    if wide {
        emu.regs_mut().rax = helper::handler_create("createdialogindirectparamw");
    } else {
        emu.regs_mut().rax = helper::handler_create("createdialogindirectparama");
    }

    stdcall_ret(emu, 5);
}
