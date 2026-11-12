use crate::emu;

pub fn OleGetPackageClipboardOwner(emu: &mut emu::Emu) {
    let ppunk = emu.regs().rcx;
    log_red!(emu, "ole32!OleGetPackageClipboardOwner ppUnk:0x{:x}", ppunk);
    if ppunk != 0 {
        emu.maps.write_qword(ppunk, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
