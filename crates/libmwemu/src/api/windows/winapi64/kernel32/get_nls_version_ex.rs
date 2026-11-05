use crate::emu;

pub fn GetNLSVersionEx(emu: &mut emu::Emu) {
    let function = emu.regs().rcx as u32;
    let locale_ptr = emu.regs().rdx;
    let version_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetNLSVersionEx func:{} locale:0x{:x} ver:0x{:x}",
        function,
        locale_ptr,
        version_ptr
    );
    if version_ptr != 0 {
        emu.maps.write_dword(version_ptr, 32);
        emu.maps.write_dword(version_ptr + 4, 1);
    }
    emu.regs_mut().rax = 1;
}
