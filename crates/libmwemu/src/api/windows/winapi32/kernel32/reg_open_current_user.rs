use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn RegOpenCurrentUser(emu: &mut emu::Emu) {
    let sam_desired = arg(emu, 0) as u64;
    let ph_key = arg(emu, 1) as u64;
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
    stdcall_ret(emu, 2);
}
