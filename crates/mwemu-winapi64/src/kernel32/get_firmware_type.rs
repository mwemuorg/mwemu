use crate::emu;

pub fn GetFirmwareType(emu: &mut emu::Emu) {
    let fw_type_ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!GetFirmwareType ptr:0x{:x}", fw_type_ptr);
    if fw_type_ptr != 0 {
        emu.maps.write_dword(fw_type_ptr, 2);
    }
    emu.regs_mut().rax = 1;
}
