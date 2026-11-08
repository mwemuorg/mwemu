use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetNamedPipeHandleState(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let mode_ptr = arg(emu, 1) as u64;
    let _max_collection_count = arg(emu, 2) as u64;
    let _collect_data_timeout = arg(emu, 3) as u64;

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
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
