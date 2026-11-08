use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNumaProximityNodeEx(emu: &mut emu::Emu) {
    let proximity_id = arg(emu, 0);
    let node_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetNumaProximityNodeEx id:{} ptr:0x{:x}",
        proximity_id,
        node_ptr
    );
    if node_ptr != 0 {
        emu.maps.write_word(node_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
