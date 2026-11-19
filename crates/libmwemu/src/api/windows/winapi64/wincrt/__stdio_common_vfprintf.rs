use super::parse_format_specifiers;
use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let options = api_try!(emu, call.arg(emu, 0)); // _In_ options
    let file = api_try!(emu, call.arg(emu, 1)); // _In_ FILE*
    let format = api_try!(emu, call.arg(emu, 2)); // _In_ format string ptr
    let locale = api_try!(emu, call.arg(emu, 3)); // _In_opt_ locale
    let va_list = api_try!(emu, call.arg(emu, 4));

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
                call.set_return(emu, 1);
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
    call.set_return(emu, 1);
}
