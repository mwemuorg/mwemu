use super::alloc_bstr;
use crate::emu;

pub fn VarBstrFromCy(emu: &mut emu::Emu) {
    let cy = emu.regs().rcx as i64;
    let _lcid = emu.regs().rdx as u32;
    let _flags = emu.regs().r8 as u32;
    let pbstr_out = emu.regs().r9;
    let s = format!("{}.{:04}", cy / 10000, (cy % 10000).unsigned_abs());
    log_red!(
        emu,
        "oleaut32!VarBstrFromCy val:{} out:0x{:x}",
        s,
        pbstr_out
    );
    let bstr = alloc_bstr(emu, &s);
    if pbstr_out != 0 {
        emu.maps.write_qword(pbstr_out, bstr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
