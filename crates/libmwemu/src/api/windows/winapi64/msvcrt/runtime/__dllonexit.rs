use crate::emu;

pub fn __dllonexit(emu: &mut emu::Emu) {
    let func = emu.regs().rcx;
    let p_begin = emu.regs().rdx;
    let p_end = emu.regs().r8;
    if p_begin != 0 {
        let begin = emu.maps.read_qword(p_begin).unwrap_or(0);
        let end = if p_end != 0 {
            emu.maps.read_qword(p_end).unwrap_or(0)
        } else {
            0
        };
        log_red!(
            emu,
            "msvcrt!__dllonexit func:0x{:x} begin:0x{:x}(0x{:x}) end:0x{:x}(0x{:x})",
            func,
            p_begin,
            begin,
            p_end,
            end
        );
        let first_param = emu.regs().rcx;
        if first_param != 0 && emu.maps.is_mapped(first_param) {
            let _ = emu.maps.read_dword(first_param).unwrap_or(0);
        }
    } else {
        log_red!(
            emu,
            "msvcrt!__dllonexit func:0x{:x} begin:0x{:x} end:0x{:x}",
            func,
            p_begin,
            p_end
        );
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = func;
}
