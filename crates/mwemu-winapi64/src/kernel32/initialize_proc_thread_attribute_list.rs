use crate::emu;

pub fn InitializeProcThreadAttributeList(emu: &mut emu::Emu) {
    let attr_list = emu.regs().rcx;
    let attr_count = emu.regs().rdx as u32;
    let _reserved = emu.regs().r8;
    let size_ptr = emu.regs().r9;

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
}
