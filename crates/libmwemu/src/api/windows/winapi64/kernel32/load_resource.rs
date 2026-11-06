use crate::emu;
use crate::winapi::helper;
pub fn LoadResource(emu: &mut emu::Emu) {
    let hModule = emu.regs().rcx;
    let hResInfo = emu.regs().rdx;
    log_red!(
        emu,
        "** {} kernel32!LoadResource {:x} {:x}",
        emu.pos,
        hModule,
        hResInfo
    );
    let handle = helper::handler_create("LoadResource");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = hResInfo;
}
