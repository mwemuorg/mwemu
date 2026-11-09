use crate::emu;
use crate::winapi::helper;

pub fn CreateDialogParamA(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let template_name = emu.regs().rdx;
    let hwnd_parent = emu.regs().r8;
    let dlg_proc = emu.regs().r9;
    let init_param = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let name = if template_name > 0xffff {
        emu.maps.read_string(template_name)
    } else {
        format!("#{}", template_name)
    };
    log_red!(
        emu,
        "user32!CreateDialogParamA hinst:0x{:x} tmpl:'{}' parent:0x{:x} proc:0x{:x}",
        hinstance,
        name,
        hwnd_parent,
        dlg_proc
    );
    let uri = format!("user32://CreateDialogParamA/{}", name);
    let hndl = helper::handler_create(&uri);
    emu.regs_mut().rax = hndl;
}
