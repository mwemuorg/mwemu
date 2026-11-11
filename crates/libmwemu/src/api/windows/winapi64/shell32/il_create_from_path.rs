use crate::emu;
use crate::winapi::helper;

pub fn ILCreateFromPath(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!ILCreateFromPath arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("ilcreatefrompath");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}

pub fn ILCreateFromPathA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!ILCreateFromPathA arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("ilcreatefrompatha");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}

pub fn ILCreateFromPathW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!ILCreateFromPathW arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("ilcreatefrompathw");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
