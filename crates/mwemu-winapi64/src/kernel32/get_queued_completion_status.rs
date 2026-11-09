use crate::emu;

pub fn GetQueuedCompletionStatus(emu: &mut emu::Emu) {
    let completion_port = emu.regs().rcx;
    let bytes_ptr = emu.regs().rdx;
    let comp_key_ptr = emu.regs().r8;
    let overlapped_ptr = emu.regs().r9;
    let timeout = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    log_red!(
        emu,
        "kernel32!GetQueuedCompletionStatus port:0x{:x} timeout:{}",
        completion_port,
        timeout
    );
    if bytes_ptr != 0 {
        emu.maps.write_dword(bytes_ptr, 0);
    }
    if comp_key_ptr != 0 {
        emu.maps.write_qword(comp_key_ptr, 0);
    }
    if overlapped_ptr != 0 {
        emu.maps.write_qword(overlapped_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
