use crate::emu;
use crate::emu::object_handle::MappingHandle;
use crate::windows::constants::INVALID_HANDLE_VALUE_32;

pub fn CreateFileMappingA(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let _attr = emu.regs().rdx;
    let protect = emu.regs().r8 as u32;
    let max_sz_high = emu.regs().r9;
    let max_sz_low = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!CreateFileMappingA cannot read max size low");
    let name_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("kernel32!CreateFileMappingA cannot read name pointer");

    let mut name: String = String::new();
    if name_ptr > 0 {
        name = emu.maps.read_string(name_ptr);
    }

    let max_size = (max_sz_high << 32) | max_sz_low;
    let file_handle_opt = if h_file == INVALID_HANDLE_VALUE_32 || h_file == 0xFFFFFFFFFFFFFFFF {
        None
    } else {
        Some(h_file as u32)
    };

    let mapping_handle = MappingHandle::new(name.clone(), file_handle_opt, protect, max_size);

    let handle_key = emu.handle_management.insert_mapping_handle(mapping_handle);

    emu.regs_mut().rax = handle_key as u64;

    log_red!(
        emu,
        "kernel32!CreateFileMappingA '{}' h_file: 0x{:x} protect: 0x{:x} size: 0x{:x} = 0x{:x}",
        name,
        h_file,
        protect,
        max_size,
        handle_key
    );
}
