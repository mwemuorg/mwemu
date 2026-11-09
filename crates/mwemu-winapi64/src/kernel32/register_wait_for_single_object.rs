use crate::emu;
use crate::winapi::helper;

pub fn RegisterWaitForSingleObject(emu: &mut emu::Emu) {
    let new_wait_ptr = emu.regs().rcx;
    let object = emu.regs().rdx;
    let callback = emu.regs().r8;
    let _context = emu.regs().r9;
    let millis = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

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
}
