use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn RegisterWaitForSingleObject(emu: &mut emu::Emu) {
    let new_wait_ptr = arg(emu, 0) as u64;
    let object = arg(emu, 1) as u64;
    let callback = arg(emu, 2) as u64;
    let _context = arg(emu, 3) as u64;
    let millis = arg(emu, 4) as u64;
    let flags = arg(emu, 5) as u64;

    log_red!(
        emu,
        "kernel32!RegisterWaitForSingleObject obj:0x{:x} cb:0x{:x} ms:{} flags:0x{:x}",
        object,
        callback,
        millis,
        flags
    );

    if new_wait_ptr != 0 {
        let h = helper::handler_create("wait_object");
        emu.maps.write_qword(new_wait_ptr, h);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
