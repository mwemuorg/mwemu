use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCommProperties(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let prop_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetCommProperties h:0x{:x} ptr:0x{:x}",
        h_file,
        prop_ptr
    );
    if prop_ptr != 0 {
        emu.maps.write_word(prop_ptr, 44);
        emu.maps.write_word(prop_ptr + 2, 1);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
