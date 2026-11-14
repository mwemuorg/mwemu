use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn isalpha(emu: &mut emu::Emu, api: &str) {
    let c = WinApiAbi::arg(emu, 0) as u32;
    let ch = c as u8 as char;
    let r: u64 = match api {
        "isalpha" => ch.is_ascii_alphabetic() as u64,
        "isdigit" => ch.is_ascii_digit() as u64,
        "isspace" => ch.is_ascii_whitespace() as u64,
        "isalnum" => ch.is_ascii_alphanumeric() as u64,
        "ispunct" => ch.is_ascii_punctuation() as u64,
        "isupper" => ch.is_ascii_uppercase() as u64,
        "islower" => ch.is_ascii_lowercase() as u64,
        "isxdigit" => ch.is_ascii_hexdigit() as u64,
        _ => 0,
    };
    let name = format!("'{}'", ch);
    log_red!(emu, "msvcrt!{} {} => {}", api, name, r);
    emu.set_last_error(0);
    emu.regs_mut().rax = r;
}
