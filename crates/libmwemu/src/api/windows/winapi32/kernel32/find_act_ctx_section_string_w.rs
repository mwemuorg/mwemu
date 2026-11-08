use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::{emu, windows::structures};

pub fn FindActCtxSectionStringW(emu: &mut emu::Emu) {
    let actctx = arg(emu, 0) as u64;
    let section_name_ptr = arg(emu, 1) as u64;
    let string_name_ptr = arg(emu, 2) as u64;
    let string_value_ptr = arg(emu, 3) as u64;
    let out_ptr = arg(emu, 4) as u64;

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

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
