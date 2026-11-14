use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateDialogIndirectParamA(emu: &mut emu::Emu) {
    create_dialog_indirect_param(emu, "CreateDialogIndirectParamA");
}

fn create_dialog_indirect_param(emu: &mut emu::Emu, api: &str) {
    let hinstance = emu.regs().rcx;
    let template_ptr = emu.regs().rdx;
    let hwnd_parent = emu.regs().r8;
    let dlg_proc = emu.regs().r9;
    let init_param = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "user32!{api} hinst:0x{:x} parent:0x{:x} proc:0x{:x}",
        hinstance,
        hwnd_parent,
        dlg_proc
    );
    let hndl = helper::handler_create(&format!("user32://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn CreateDialogIndirectParamW(emu: &mut emu::Emu) {
    create_dialog_indirect_param(emu, "CreateDialogIndirectParamW");
}
