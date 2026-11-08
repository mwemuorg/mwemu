use crate::emu;

pub fn GetNLSVersionEx(emu: &mut emu::Emu) {
    let func = emu.regs().rcx as u32;
    let locale_ptr = emu.regs().rdx;
    let info_ptr = emu.regs().r8;
    let locale = if locale_ptr != 0 && locale_ptr > 0xffff {
        emu.maps.read_wide_string(locale_ptr)
    } else {
        "en-US".to_string()
    };
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 32); // dwNLSVersionInfoSize
        emu.maps.write_dword(info_ptr + 4, 6); // dwNLSVersion
        emu.maps.write_dword(info_ptr + 8, 2); // dwDefinedVersion
        emu.maps.write_dword(info_ptr + 12, 0); // dwEffectiveId
    }
    log_red!(
        emu,
        "kernelbase!GetNLSVersionEx func:{} locale:'{}'",
        func,
        locale
    );
    emu.regs_mut().rax = 1; // TRUE
}
