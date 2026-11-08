use crate::emu;
use crate::winapi::helper;

pub fn CreateDialogIndirectParamA(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let template_ptr = emu.regs().rdx;
    let hwnd_parent = emu.regs().r8;
    let dlg_proc = emu.regs().r9;
    let init_param = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "user32!CreateDialogIndirectParamA hinst:0x{:x} parent:0x{:x} proc:0x{:x}",
        hinstance,
        hwnd_parent,
        dlg_proc
    );
    let hndl = helper::handler_create("user32://CreateDialogIndirectParamA");
    emu.regs_mut().rax = hndl;
}
