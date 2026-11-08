use crate::emu;

pub fn SysStringByteLen(emu: &mut emu::Emu) {
    let bstr = emu.regs().rcx;

    if bstr == 0 {
        log_red!(emu, "oleaut32!SysStringByteLen NULL -> 0");
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }

    let len_bytes = emu.maps.read_dword(bstr - 4).unwrap_or(0);

    log_red!(
        emu,
        "oleaut32!SysStringByteLen 0x{:x} -> {}",
        bstr,
        len_bytes
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = len_bytes as u64;
}
