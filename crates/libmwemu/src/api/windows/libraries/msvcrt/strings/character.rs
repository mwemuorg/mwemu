use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;

// This model uses the default C locale; locale-specific tables remain unsupported.
fn change_case(emu: &mut Emu, library: &str, name: &str, wide: bool, upper: bool) {
    let call = WinApi(Cdecl);
    let value = if wide {
        api_try!(emu, call.arg_u16(emu, 0)) as u32
    } else {
        api_try!(emu, call.arg_u32(emu, 0))
    };
    let result = match u8::try_from(value) {
        Ok(byte) if upper => byte.to_ascii_uppercase() as u32,
        Ok(byte) => byte.to_ascii_lowercase() as u32,
        Err(_) => value,
    };
    log_red!(emu, "{library}!{name} value={value:#x} => {result:#x}");
    call.set_return(emu, result as u64);
}

fn classify_character(emu: &mut Emu, library: &str, name: &str) {
    let call = WinApi(Cdecl);
    let value = api_try!(emu, call.arg_i32(emu, 0));
    let result = u8::try_from(value).is_ok_and(|byte| match name {
        "isalpha" => byte.is_ascii_alphabetic(),
        "isdigit" => byte.is_ascii_digit(),
        "isspace" => byte.is_ascii_whitespace() || byte == b'\x0b',
        "isalnum" => byte.is_ascii_alphanumeric(),
        "ispunct" => byte.is_ascii_punctuation(),
        "isupper" => byte.is_ascii_uppercase(),
        "islower" => byte.is_ascii_lowercase(),
        "isxdigit" => byte.is_ascii_hexdigit(),
        _ => false,
    });
    log_red!(emu, "{library}!{name} value={value:#x} => {result}");
    call.set_return(emu, result as u64);
}

pub(crate) fn dispatch_character(api: &str, library: &str, emu: &mut Emu) -> bool {
    match api {
        "tolower" => change_case(emu, library, api, false, false),
        "toupper" => change_case(emu, library, api, false, true),
        "towlower" => change_case(emu, library, api, true, false),
        "towupper" => change_case(emu, library, api, true, true),
        "isalpha" | "isdigit" | "isspace" | "isalnum" | "ispunct" | "isupper" | "islower"
        | "isxdigit" => classify_character(emu, library, api),
        _ => return false,
    }
    true
}
