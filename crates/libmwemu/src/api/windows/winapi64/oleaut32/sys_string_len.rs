use crate::emu;

pub fn SysStringLen(emu: &mut emu::Emu) {
    let bstr = emu.regs().rcx;

    if bstr == 0 {
        log_red!(emu, "oleaut32!SysStringLen NULL -> 0");
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }

    let len_bytes = emu.maps.read_dword(bstr - 4).unwrap_or(0);
    let len_chars = len_bytes / 2;

    log_red!(emu, "oleaut32!SysStringLen 0x{:x} -> {}", bstr, len_chars);
    emu.set_last_error(0);
    emu.regs_mut().rax = len_chars as u64;
}
