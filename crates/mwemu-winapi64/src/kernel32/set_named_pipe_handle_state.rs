use crate::emu;

pub fn SetNamedPipeHandleState(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let mode_ptr = emu.regs().rdx;
    let _max_collection_count = emu.regs().r8;
    let _collect_data_timeout = emu.regs().r9;

    let mode = if mode_ptr != 0 {
        emu.maps.read_dword(mode_ptr).unwrap_or(0)
    } else {
        0
    };

    log_red!(
        emu,
        "kernel32!SetNamedPipeHandleState handle:0x{:x} mode:0x{:x}",
        handle,
        mode
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
