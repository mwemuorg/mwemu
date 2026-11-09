use crate::emu;

pub fn IsNativeVhdBoot(emu: &mut emu::Emu) {
    let p_native_vhd_boot = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!IsNativeVhdBoot pNativeVhdBoot:0x{:x}",
        p_native_vhd_boot
    );
    if p_native_vhd_boot != 0 {
        emu.maps.write_dword(p_native_vhd_boot, 0);
    }
    emu.regs_mut().rax = 1;
}
