use crate::emu;

pub fn CheckElevationEnabled(emu: &mut emu::Emu) {
    let p_result = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!CheckElevationEnabled pResult:0x{:x}",
        p_result
    );
    if p_result != 0 {
        emu.maps.write_dword(p_result, 0);
    }
    emu.regs_mut().rax = 0;
}
