use super::parse_format_specifiers;
use crate::emu;

pub(super) fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let options = emu.regs().rcx; // _In_ options
    let file = emu.regs().rdx; // _In_ FILE*
    let format = emu.regs().r8; // _In_ format string ptr
    let locale = emu.regs().r9; // _In_opt_ locale
    let va_list = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    // Just try to read the format string
    let fmt_str = emu.maps.read_string(format);
    let specs = parse_format_specifiers(&fmt_str);

    log_red!(
        emu,
        "wincrt!__stdio_common_vfprintf options: 0x{:x} file: 0x{:x} format: '{}' locale: 0x{:x} va_list: 0x{:x}",
        options,
        file,
        fmt_str,
        locale,
        va_list
    );

    let mut current_ptr = va_list;
    for spec in specs {
        match spec {
            "int" | "hex" | "ptr" => {
                let arg = emu.maps.read_qword(current_ptr).unwrap_or(0);
                current_ptr += 8; // Move to next arg
                log::trace!("arg: {:016x}", arg);
                emu.regs_mut().rax = 1;
            }
            "str" => {
                let str_ptr = emu.maps.read_qword(current_ptr).unwrap_or(0);
                let string = emu.maps.read_string(str_ptr);
                current_ptr += 8;
                log::trace!("string: {}", string);
            }
            _ => {
                log::warn!(
                    "wincrt!__stdio_common_vfprintf unknown format character: {}",
                    spec
                );
            }
        }
    }

    // Return success (1) - this is super basic
    emu.regs_mut().rax = 1;
}
