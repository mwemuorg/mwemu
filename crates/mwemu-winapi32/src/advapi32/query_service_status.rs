use crate::emu;

pub fn QueryServiceStatus(emu: &mut emu::Emu) {
    let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let status_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if status_ptr != 0 {
        emu.maps.write_dword(status_ptr, 0x10);
        emu.maps.write_dword(status_ptr + 4, 4);
        emu.maps.write_dword(status_ptr + 8, 0x1f);
        emu.maps.write_dword(status_ptr + 12, 0);
        emu.maps.write_dword(status_ptr + 16, 0);
        emu.maps.write_dword(status_ptr + 20, 0);
        emu.maps.write_dword(status_ptr + 24, 0);
    }
    log_red!(emu, "advapi32!QueryServiceStatus svc:0x{:x}", hsvc);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
