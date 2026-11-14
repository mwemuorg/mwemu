use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn DialogBoxParamA(emu: &mut emu::Emu) {
    dialog_box_param(emu, "DialogBoxParamA", false);
}

fn dialog_box_param(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hinstance = emu.regs().rcx;
    let template_ptr = emu.regs().rdx;
    let hwnd_parent = emu.regs().r8;
    let dlg_proc = emu.regs().r9;
    let init_param = WinApiAbi::arg(emu, 4);
    let template_name = if template_ptr > 0xffff {
        strings::read_text(emu, template_ptr, wide)
    } else {
        format!("#{}", template_ptr)
    };
    log_red!(
        emu,
        "user32!{api} '{}' proc:0x{:x} param:0x{:x}",
        template_name,
        dlg_proc,
        init_param
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // IDOK
}

pub fn DialogBoxParamW(emu: &mut emu::Emu) {
    dialog_box_param(emu, "DialogBoxParamW", true);
}
