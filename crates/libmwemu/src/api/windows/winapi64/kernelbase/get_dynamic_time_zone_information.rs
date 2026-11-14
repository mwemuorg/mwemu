use crate::emu;

pub fn GetDynamicTimeZoneInformation(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(
        emu,
        "kernelbase!GetDynamicTimeZoneInformation ptr:0x{:x}",
        p0
    );
    if p0 != 0 && emu.maps.is_mapped(p0) {
        for i in 0..22u64 {
            emu.maps.write_dword(p0 + i * 4, 0);
        }
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
