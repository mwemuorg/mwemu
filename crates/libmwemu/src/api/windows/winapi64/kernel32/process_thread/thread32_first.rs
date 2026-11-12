use crate::emu;

pub fn Thread32First(emu: &mut emu::Emu) {
    let h_snapshot = emu.regs().rcx;
    let p_entry = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!Thread32First snap:0x{:x} entry:0x{:x}",
        h_snapshot,
        p_entry
    );

    if p_entry == 0 {
        emu.set_last_error(87);
        emu.regs_mut().rax = 0;
        return;
    }

    // Fill THREADENTRY32 structure (28 bytes)
    // dwSize
    emu.maps.write_dword(p_entry, 28);
    // cntUsage (deprecated)
    emu.maps.write_dword(p_entry + 4, 0);
    // th32ThreadID
    let tid = emu.current_thread().id as u32;
    emu.maps.write_dword(p_entry + 8, tid);
    // th32OwnerProcessID
    emu.maps.write_dword(p_entry + 12, 0x123);
    // tpBasePri
    emu.maps.write_dword(p_entry + 16, 8); // THREAD_PRIORITY_NORMAL
    // tpDeltaPri (deprecated)
    emu.maps.write_dword(p_entry + 20, 0);
    // dwFlags (deprecated)
    emu.maps.write_dword(p_entry + 24, 0);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
