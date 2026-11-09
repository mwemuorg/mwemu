use crate::emu;

pub fn VariantChangeTypeEx(emu: &mut emu::Emu) {
    let pvarg_dest = emu.regs().rcx;
    let pvarg_src = emu.regs().rdx;
    let vt = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as u16;
    log_red!(
        emu,
        "oleaut32!VariantChangeTypeEx dest: 0x{:x} src: 0x{:x} vt: {}",
        pvarg_dest,
        pvarg_src,
        vt
    );
    if pvarg_dest != 0 && pvarg_src != 0 {
        for i in 0..16u64 {
            let b = emu.maps.read_byte(pvarg_src + i).unwrap_or(0);
            emu.maps.write_byte(pvarg_dest + i, b);
        }
        emu.maps.write_word(pvarg_dest, vt);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
