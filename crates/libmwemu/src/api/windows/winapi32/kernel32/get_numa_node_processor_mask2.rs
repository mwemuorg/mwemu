use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNumaNodeProcessorMask2(emu: &mut emu::Emu) {
    let node = arg(emu, 0) as u64 as u16;
    let masks_ptr = arg(emu, 1) as u64;
    let mask_count = arg(emu, 2);
    let req_count_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetNumaNodeProcessorMask2 node:{} masks:0x{:x} count:{} req:0x{:x}",
        node,
        masks_ptr,
        mask_count,
        req_count_ptr
    );
    if req_count_ptr != 0 {
        emu.maps.write_dword(req_count_ptr, 1);
    }
    if masks_ptr != 0 && mask_count >= 1 {
        emu.maps.write_qword(masks_ptr, 0xF);
        emu.maps.write_word(masks_ptr + 8, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
