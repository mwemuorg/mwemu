use super::alloc_bstr;
use crate::emu;

pub fn VarBstrFromDisp(emu: &mut emu::Emu) {
    let _pdisp = emu.regs().rcx;
    let _lcid = emu.regs().rdx as u32;
    let _flags = emu.regs().r8 as u32;
    let pbstr_out = emu.regs().r9;
    let s = "[object]";
    log_red!(emu, "oleaut32!VarBstrFromDisp out:0x{:x}", pbstr_out);
    let bstr = alloc_bstr(emu, s);
    if pbstr_out != 0 {
        emu.maps.write_qword(pbstr_out, bstr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
