use crate::emu;

pub fn SetEnvironmentStringsA(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;

    let s = emu.maps.read_string(param1);

    log_red!(
        emu,
        "kernel32!SetEnvironmentStringsA str:'{}' param2:0x{:x}",
        s,
        param2
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
