use crate::emu;

pub fn GetQueuedCompletionStatusEx(emu: &mut emu::Emu) {
    let completion_port = emu.regs().rcx;
    let entries_ptr = emu.regs().rdx;
    let count = emu.regs().r8 as u32;
    let removed_ptr = emu.regs().r9;
    let timeout = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    log_red!(
        emu,
        "kernel32!GetQueuedCompletionStatusEx port:0x{:x} count:{} timeout:{}",
        completion_port,
        count,
        timeout
    );
    if removed_ptr != 0 {
        emu.maps.write_dword(removed_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
