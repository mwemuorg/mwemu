use crate::emu;

pub fn DialogBoxParamA(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let template_ptr = emu.regs().rdx;
    let hwnd_parent = emu.regs().r8;
    let dlg_proc = emu.regs().r9;
    let init_param = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let template_name = if template_ptr > 0xffff {
        emu.maps.read_string(template_ptr)
    } else {
        format!("#{}", template_ptr)
    };
    log_red!(
        emu,
        "user32!DialogBoxParamA '{}' proc:0x{:x} param:0x{:x}",
        template_name,
        dlg_proc,
        init_param
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // IDOK
}
