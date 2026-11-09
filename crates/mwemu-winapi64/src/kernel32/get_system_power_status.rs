use crate::emu;

pub fn GetSystemPowerStatus(emu: &mut emu::Emu) {
    let status_ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!GetSystemPowerStatus ptr:0x{:x}", status_ptr);
    if status_ptr != 0 {
        emu.maps.write_byte(status_ptr, 1);
        emu.maps.write_byte(status_ptr + 1, 0);
        emu.maps.write_byte(status_ptr + 2, 100);
        emu.maps.write_byte(status_ptr + 3, 0);
        emu.maps.write_dword(status_ptr + 4, 0xFFFFFFFF);
        emu.maps.write_dword(status_ptr + 8, 0xFFFFFFFF);
    }
    emu.regs_mut().rax = 1;
}
