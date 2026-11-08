use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn EnterSynchronizationBarrier(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;
    let param3 = arg(emu, 2) as u64;
    let param4 = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!EnterSynchronizationBarrier handle:0x{:x} p2:0x{:x}",
        handle,
        param2
    );
    let handle = helper::handler_create("EnterSynchronizationBarrier");
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
