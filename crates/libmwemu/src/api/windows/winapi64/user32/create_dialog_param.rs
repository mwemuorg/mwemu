use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateDialogParamA(emu: &mut emu::Emu) {
    create_dialog_param(emu, "CreateDialogParamA", false);
}

fn create_dialog_param(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hinstance = emu.regs().rcx;
    let template_name = emu.regs().rdx;
    let hwnd_parent = emu.regs().r8;
    let dlg_proc = emu.regs().r9;
    let init_param = WinApiAbi::arg(emu, 4);
    let name = if template_name > 0xffff {
        strings::read_text(emu, template_name, wide)
    } else {
        format!("#{}", template_name)
    };
    log_red!(
        emu,
        "user32!{api} hinst:0x{:x} tmpl:'{}' parent:0x{:x} proc:0x{:x}",
        hinstance,
        name,
        hwnd_parent,
        dlg_proc
    );
    let uri = format!("user32://{api}/{}", name);
    let hndl = helper::handler_create(&uri);
    emu.regs_mut().rax = hndl;
}

pub fn CreateDialogParamW(emu: &mut emu::Emu) {
    create_dialog_param(emu, "CreateDialogParamW", true);
}
