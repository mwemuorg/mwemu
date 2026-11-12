use crate::emu;

pub(super) fn GetClassInfoExA(emu: &mut emu::Emu) {
    get_class_info_ex(emu, "GetClassInfoExA", false);
}

pub(super) fn GetClassInfoExW(emu: &mut emu::Emu) {
    get_class_info_ex(emu, "GetClassInfoExW", true);
}

fn get_class_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "user32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    if wide {
        emu.set_last_error(0);
    }

    emu.regs_mut().rax = 0;
}
