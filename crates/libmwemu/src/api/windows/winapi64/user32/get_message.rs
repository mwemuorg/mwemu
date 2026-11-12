use crate::emu;

pub(super) fn GetMessageA(emu: &mut emu::Emu) {
    get_message(emu, "GetMessageA");
}

fn get_message(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    log_red!(
        emu,
        "user32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    emu.regs_mut().rax = 0;
}

pub(super) fn GetMessageW(emu: &mut emu::Emu) {
    get_message(emu, "GetMessageW");
}
