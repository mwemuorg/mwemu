use crate::emu;

pub fn GetNLSVersion(emu: &mut emu::Emu) {
    let function = emu.regs().rcx as u32;
    let locale = emu.regs().rdx as u32;
    let version_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetNLSVersion func:{} locale:0x{:x} ver:0x{:x}",
        function,
        locale,
        version_ptr
    );
    if version_ptr != 0 {
        emu.maps.write_dword(version_ptr, 32);
        emu.maps.write_dword(version_ptr + 4, 1);
    }
    emu.regs_mut().rax = 1;
}
