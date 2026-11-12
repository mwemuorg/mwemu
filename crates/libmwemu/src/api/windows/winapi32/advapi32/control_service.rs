use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn ControlService(emu: &mut emu::Emu) {
    let hsvc = WinApiAbi::arg(emu, 0) as u32;
    let control = WinApiAbi::arg(emu, 1) as u32;
    let status_ptr = WinApiAbi::arg(emu, 2);
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
