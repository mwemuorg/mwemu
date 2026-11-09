use crate::emu;

pub fn VariantCopyInd(emu: &mut emu::Emu) {
    let pvarg_dest = emu.regs().rcx;
    let pvarg_src = emu.regs().rdx;
    log_red!(
        emu,
        "oleaut32!VariantCopyInd dest: 0x{:x} src: 0x{:x}",
        pvarg_dest,
        pvarg_src
    );
    if pvarg_dest != 0 && pvarg_src != 0 {
        for i in 0..16u64 {
            let b = emu.maps.read_byte(pvarg_src + i).unwrap_or(0);
            emu.maps.write_byte(pvarg_dest + i, b);
        }
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
