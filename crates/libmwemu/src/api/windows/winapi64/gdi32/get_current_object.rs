use crate::emu;
use crate::winapi::helper;

pub fn GetCurrentObject(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let obj_type = emu.regs().rdx;
    let tag = match obj_type {
        1 => "gdi32://pen",
        2 => "gdi32://brush",
        6 => "gdi32://font",
        7 => "gdi32://bitmap",
        _ => "",
    };
    let result: u64 = if tag.is_empty() {
        0
    } else {
        helper::handler_create(tag)
    };
    log_red!(
        emu,
        "gdi32!GetCurrentObject hdc:0x{:x} type:{} = 0x{:x}",
        hdc,
        obj_type,
        result
    );
    emu.regs_mut().rax = result;
}
