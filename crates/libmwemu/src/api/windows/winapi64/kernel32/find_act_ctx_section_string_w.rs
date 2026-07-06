use crate::{emu, windows::structures};

pub fn FindActCtxSectionStringW(emu: &mut emu::Emu) {
    let actctx = emu.regs().rcx;
    let section_name_ptr = emu.regs().rdx;
    let string_name_ptr = emu.regs().r8;
    let string_value_ptr = emu.regs().r9;
    let out_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("error reading out_ptr");

    let mut section_name = String::new();
    let mut string_name = String::new();
    let mut string_value = String::new();

    if section_name_ptr > 0 {
        section_name = emu.maps.read_wide_string(section_name_ptr);
    }
    if string_name_ptr > 0 {
        string_name = emu.maps.read_wide_string(string_name_ptr);
    }
    if string_value_ptr > 0 {
        string_value = emu.maps.read_wide_string(string_value_ptr);
    }

    let actctx_section_keyed_data = structures::ActCtxSectionKeyedData64::new();
    actctx_section_keyed_data.save(out_ptr, &mut emu.maps);

    log_red!(
        emu,
        "kernel32!FindActCtxSectionStringW section_name: {} string_name: {} string_value: {}",
        section_name,
        string_name,
        string_value
    );

    emu.regs_mut().rax = 1;
}
