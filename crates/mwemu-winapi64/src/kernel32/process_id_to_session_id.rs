use crate::emu;

pub fn ProcessIdToSessionId(emu: &mut emu::Emu) {
    let process_id = emu.regs().rcx as u32;
    let session_id_ptr = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!ProcessIdToSessionId pid:{} out:0x{:x}",
        process_id,
        session_id_ptr
    );

    if session_id_ptr != 0 {
        emu.maps.write_dword(session_id_ptr, 0); // session 0
    }
    emu.regs_mut().rax = 1;
}
