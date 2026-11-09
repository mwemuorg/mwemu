use crate::emu;

pub fn HeapQueryInformation(emu: &mut emu::Emu) {
    let heap_handle = emu.regs().rcx;
    let info_class = emu.regs().rdx as u32;
    let info_ptr = emu.regs().r8;
    let info_len = emu.regs().r9;
    let return_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

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
}
