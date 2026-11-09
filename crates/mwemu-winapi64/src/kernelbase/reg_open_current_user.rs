use crate::emu;
use crate::winapi::helper;

pub fn RegOpenCurrentUser(emu: &mut emu::Emu) {
    let sam_desired = emu.regs().rcx;
    let result_ptr = emu.regs().rdx;
    let hndl = helper::handler_create("regkey_HKCU");
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
    log_red!(
        emu,
        "kernelbase!RegOpenCurrentUser sam:0x{:x} => 0x{:x}",
        sam_desired,
        hndl
    );
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}
