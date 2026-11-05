use crate::api::windows::helper;
use crate::emu;

pub fn RegOpenUserClassesRoot(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    let _options = emu.regs().rdx;
    let sam_desired = emu.regs().r8;
    let ph_key = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!RegOpenUserClassesRoot token:0x{:x} sam:0x{:x} phKey:0x{:x}",
        token,
        sam_desired,
        ph_key
    );

    let handle = helper::handler_create("regkey://HKCR");
    if ph_key != 0 {
        emu.maps.write_qword(ph_key, handle);
    }
    emu.regs_mut().rax = 0;
}
