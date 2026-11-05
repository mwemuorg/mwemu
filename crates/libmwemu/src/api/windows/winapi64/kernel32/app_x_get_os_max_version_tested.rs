use crate::emu;

pub fn AppXGetOSMaxVersionTested(emu: &mut emu::Emu) {
    let package_full_name = emu.regs().rcx;
    let p_max_version = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!AppXGetOSMaxVersionTested name:0x{:x} pMaxVer:0x{:x}",
        package_full_name,
        p_max_version
    );

    if p_max_version != 0 {
        emu.maps.write_qword(p_max_version, 0x0A00_0000);
    }
    emu.regs_mut().rax = 0;
}
