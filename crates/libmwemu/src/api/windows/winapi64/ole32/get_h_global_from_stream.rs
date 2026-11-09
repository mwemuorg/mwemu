use crate::emu;
use crate::winapi::helper;

pub fn GetHGlobalFromStream(emu: &mut emu::Emu) {
    let pstm = emu.regs().rcx;
    let phglobal = emu.regs().rdx;
    let hndl = helper::handler_create("ole32://HGlobal");
    if phglobal != 0 {
        emu.maps.write_dword(phglobal, hndl as u32);
    }
    log_red!(
        emu,
        "ole32!GetHGlobalFromStream pstm:0x{:x} => 0x{:x}",
        pstm,
        hndl
    );
    emu.regs_mut().rax = 0; // S_OK
}
