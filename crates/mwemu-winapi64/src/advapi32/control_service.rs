use crate::emu;

pub fn ControlService(emu: &mut emu::Emu) {
    let hsvc = emu.regs().rcx;
    let control = emu.regs().rdx;
    let status_ptr = emu.regs().r8;
    if status_ptr != 0 {
        emu.maps.write_dword(status_ptr, 0x10);
        emu.maps.write_dword(status_ptr + 4, 1);
    }
    log_red!(
        emu,
        "advapi32!ControlService svc:0x{:x} ctrl:{}",
        hsvc,
        control
    );
    emu.regs_mut().rax = 1;
}
