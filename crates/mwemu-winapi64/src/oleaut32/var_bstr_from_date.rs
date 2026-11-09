use super::alloc_bstr;
use crate::emu;

pub fn VarBstrFromDate(emu: &mut emu::Emu) {
    let _date_bits = emu.regs().rcx;
    let _lcid = emu.regs().rdx as u32;
    let _flags = emu.regs().r8 as u32;
    let pbstr_out = emu.regs().r9;
    let s = "1/1/2024";
    log_red!(emu, "oleaut32!VarBstrFromDate out:0x{:x}", pbstr_out);
    let bstr = alloc_bstr(emu, s);
    if pbstr_out != 0 {
        emu.maps.write_qword(pbstr_out, bstr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
