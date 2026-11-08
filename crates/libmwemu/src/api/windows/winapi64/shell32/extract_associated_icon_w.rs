use crate::emu;
use crate::winapi::helper;

pub fn ExtractAssociatedIconW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!ExtractAssociatedIconW '{}' arg2:0x{:x}",
        s,
        arg2
    );
    let handle = helper::handler_create("extractassociatediconw");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
