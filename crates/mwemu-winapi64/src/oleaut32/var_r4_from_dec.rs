use crate::emu;

pub fn VarR4FromDec(emu: &mut emu::Emu) {
    let src_val = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "oleaut32!VarR4FromDec src:0x{:x} dst:0x{:x}",
        src_val,
        dst_ptr
    );
    if dst_ptr != 0 {
        emu.maps.write_dword(dst_ptr, src_val as u32);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
