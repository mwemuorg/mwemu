use crate::emu;

pub fn K32EnumDeviceDrivers(emu: &mut emu::Emu) {
    let p_image_base = emu.regs().rcx;
    let cb = emu.regs().rdx;
    let p_needed = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!K32EnumDeviceDrivers pImageBase:0x{:x} cb:{} pNeeded:0x{:x}",
        p_image_base,
        cb,
        p_needed
    );

    if p_needed != 0 {
        emu.maps.write_dword(p_needed, 0);
    }
    emu.regs_mut().rax = 1;
}
