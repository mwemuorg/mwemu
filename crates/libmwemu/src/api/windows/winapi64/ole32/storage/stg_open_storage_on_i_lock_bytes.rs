use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn StgOpenStorageOnILockBytes(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    let arg6 = WinApiAbi::arg(emu, 5);
    log_red!(emu, "ole32!StgOpenStorageOnILockBytes 0x{:x}", arg1);
    let handle = helper::handler_create("StgOpenStorageOnILockBytes");
    let out_ptr = WinApiAbi::arg(emu, 5);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
