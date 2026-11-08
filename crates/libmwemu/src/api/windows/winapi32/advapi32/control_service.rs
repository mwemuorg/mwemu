use crate::emu;

pub fn ControlService(emu: &mut emu::Emu) {
    let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let control = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let status_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
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
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
