use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetXStateFeaturesMask(emu: &mut emu::Emu) {
    let context_ptr = arg(emu, 0) as u64;
    let mask_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetXStateFeaturesMask ctx:0x{:x} mask:0x{:x}",
        context_ptr,
        mask_ptr
    );
    if mask_ptr != 0 {
        emu.maps.write_qword(mask_ptr, 0x7);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
