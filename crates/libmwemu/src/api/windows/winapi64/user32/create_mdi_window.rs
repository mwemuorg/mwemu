use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CreateMDIWindowA(emu: &mut emu::Emu) {
    create_mdi_window(emu, "CreateMDIWindowA", false);
}

fn create_mdi_window(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ex_style = emu.regs().rcx;
    let class_ptr = emu.regs().rdx;
    let title_ptr = emu.regs().r8;
    let style = emu.regs().r9;
    let class_name = if class_ptr > 0xffff {
        strings::read_text(emu, class_ptr, wide)
    } else {
        format!("atom_{}", class_ptr)
    };
    let title = if title_ptr != 0 {
        strings::read_text(emu, title_ptr, wide)
    } else {
        String::new()
    };
    let hndl = helper::handler_create(&format!("window_{}", class_name));
    log_red!(
        emu,
        "user32!{api} class:'{}' title:'{}' => 0x{:x}",
        class_name,
        title,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}

pub fn CreateMDIWindowW(emu: &mut emu::Emu) {
    create_mdi_window(emu, "CreateMDIWindowW", true);
}
