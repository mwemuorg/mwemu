use crate::emu;

pub fn ParseApplicationUserModelId(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!ParseApplicationUserModelId param1:0x{:x} param2:0x{:x} param3:0x{:x}",
        param1,
        param2,
        param3
    );
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 15700;
}
