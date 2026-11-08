use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn HeapQueryInformation(emu: &mut emu::Emu) {
    let heap_handle = arg(emu, 0) as u64;
    let info_class = arg(emu, 1);
    let info_ptr = arg(emu, 2) as u64;
    let info_len = arg(emu, 3) as u64;
    let return_len_ptr = arg(emu, 4) as u64;

    log_red!(
        emu,
        "kernel32!HeapQueryInformation heap:0x{:x} class:{} buf:0x{:x} len:{}",
        heap_handle,
        info_class,
        info_ptr,
        info_len
    );

    match info_class {
        0 => {
            // HeapCompatibilityInformation — returns ULONG
            if info_ptr != 0 && info_len >= 4 {
                emu.maps.write_dword(info_ptr, 0); // standard heap
            }
            if return_len_ptr != 0 {
                emu.maps.write_qword(return_len_ptr, 4);
            }
            emu.regs_mut().rax = 1;
        }
        _ => {
            emu.regs_mut().rax = 1;
        }
    }
    stdcall_ret(emu, 5);
}
