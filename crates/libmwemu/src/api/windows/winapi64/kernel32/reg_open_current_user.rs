use crate::api::windows::helper;
use crate::emu;

pub fn RegOpenCurrentUser(emu: &mut emu::Emu) {
    let sam_desired = emu.regs().rcx;
    let ph_key = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!RegOpenCurrentUser sam:0x{:x} phKey:0x{:x}",
        sam_desired,
        ph_key
    );

    let handle = helper::handler_create("regkey://HKCU");
    if ph_key != 0 {
        emu.maps.write_qword(ph_key, handle);
    }
    emu.regs_mut().rax = 0;
}
