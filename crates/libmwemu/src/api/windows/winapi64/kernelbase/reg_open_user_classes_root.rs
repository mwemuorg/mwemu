use crate::emu;
use crate::winapi::helper;

pub fn RegOpenUserClassesRoot(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let sam = emu.regs().r8;
    let result_ptr = emu.regs().r9;
    let hndl = helper::handler_create("regkey_HKCR_user");
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
    log_red!(
        emu,
        "kernelbase!RegOpenUserClassesRoot token:0x{:x} => 0x{:x}",
        token,
        hndl
    );
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}
