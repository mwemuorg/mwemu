use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn InitializeProcThreadAttributeList(emu: &mut emu::Emu) {
    let attr_list = arg(emu, 0) as u64;
    let attr_count = arg(emu, 1);
    let _reserved = arg(emu, 2) as u64;
    let size_ptr = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!InitializeProcThreadAttributeList list:0x{:x} count:{} size_ptr:0x{:x}",
        attr_list,
        attr_count,
        size_ptr
    );

    if attr_list == 0 {
        // Query mode: return required size
        let needed = 48 + (attr_count as u64) * 32;
        if size_ptr != 0 {
            emu.maps.write_qword(size_ptr, needed);
        }
        emu.set_last_error(122); // ERROR_INSUFFICIENT_BUFFER
        emu.regs_mut().rax = 0;
        return;
    }

    // Initialize the attribute list
    if attr_list != 0 {
        emu.maps.write_dword(attr_list, attr_count); // count at offset 0
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
