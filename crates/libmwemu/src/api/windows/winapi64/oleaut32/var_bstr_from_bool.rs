use super::alloc_bstr;
use crate::emu;

pub fn VarBstrFromBool(emu: &mut emu::Emu) {
    let val = emu.regs().rcx as i16;
    let _lcid = emu.regs().rdx as u32;
    let _flags = emu.regs().r8 as u32;
    let pbstr_out = emu.regs().r9;
    let s = if val == 0 { "False" } else { "True" };
    log_red!(
        emu,
        "oleaut32!VarBstrFromBool val:{} out:0x{:x}",
        val,
        pbstr_out
    );
    let bstr = alloc_bstr(emu, s);
    if pbstr_out != 0 {
        emu.maps.write_qword(pbstr_out, bstr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
