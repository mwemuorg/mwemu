use crate::emu;
use crate::winapi::helper;

pub fn CreateWindowStationW(emu: &mut emu::Emu) {
    let ex_style = emu.regs().rcx;
    let class_ptr = emu.regs().rdx;
    let title_ptr = emu.regs().r8;
    let style = emu.regs().r9;
    let class_name = if class_ptr > 0xffff {
        emu.maps.read_wide_string(class_ptr)
    } else {
        format!("atom_{}", class_ptr)
    };
    let title = if title_ptr != 0 {
        emu.maps.read_wide_string(title_ptr)
    } else {
        String::new()
    };
    let hndl = helper::handler_create(&format!("window_{}", class_name));
    log_red!(
        emu,
        "user32!CreateWindowStationW class:'{}' title:'{}' => 0x{:x}",
        class_name,
        title,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
