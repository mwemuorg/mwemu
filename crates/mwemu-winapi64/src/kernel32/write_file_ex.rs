use crate::emu;
use crate::winapi::helper;

pub fn WriteFileEx(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let lp_buffer = emu.regs().rdx;
    let n_bytes = emu.regs().r8 as u32;
    let lp_overlapped = emu.regs().r9;
    let _completion = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(
        emu,
        "kernel32!WriteFileEx h:0x{:x} buf:0x{:x} len:{} ovl:0x{:x}",
        h_file,
        lp_buffer,
        n_bytes,
        lp_overlapped
    );

    if !helper::handler_exist(h_file) {
        emu.set_last_error(6);
        emu.regs_mut().rax = 0;
        return;
    }

    emu.regs_mut().rax = 1;
}
