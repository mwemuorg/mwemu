use crate::emu;
use crate::winapi::helper;

pub fn DuplicateHandle(emu: &mut emu::Emu) {
    let src_process = emu.regs().rcx;
    let src_handle = emu.regs().rdx;
    let dst_process = emu.regs().r8;
    let dst_handle_ptr = emu.regs().r9;
    let hndl = helper::handler_create("dup_handle");
    if dst_handle_ptr != 0 {
        emu.maps.write_qword(dst_handle_ptr, hndl);
    }
    log_red!(
        emu,
        "kernelbase!DuplicateHandle src:0x{:x} => 0x{:x}",
        src_handle,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
