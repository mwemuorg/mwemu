use crate::emu;

pub fn OleQueryLinkFromData(emu: &mut emu::Emu) {
    let pdata_src = emu.regs().rcx;
    log_red!(
        emu,
        "ole32!OleQueryLinkFromData pSrcDataObj:0x{:x}",
        pdata_src
    );
    if pdata_src != 0 && emu.maps.is_mapped(pdata_src) {
        let _ = emu.maps.read_qword(pdata_src).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
