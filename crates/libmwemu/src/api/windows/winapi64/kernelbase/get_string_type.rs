use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetStringTypeA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, call.arg(emu, 0));
    let info_type = api_try!(emu, call.arg(emu, 1));
    let src_ptr = api_try!(emu, call.arg(emu, 2));
    let src_len = api_try!(emu, call.arg(emu, 3));
    let src = if src_ptr != 0 {
        emu.maps.read_string(src_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!GetStringTypeA type:0x{:x} '{}'",
        info_type,
        src
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn GetStringTypeW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let info_type = api_try!(emu, call.arg_u32(emu, 0));
    let src_ptr = api_try!(emu, call.arg(emu, 1));
    let src_len = api_try!(emu, call.arg_i32(emu, 2));
    let out_ptr = api_try!(emu, call.arg(emu, 3));
    let src = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    let char_count = if src_len < 0 {
        src.encode_utf16().count()
    } else {
        src_len as usize
    };
    if out_ptr != 0 {
        for (i, ch) in src.chars().take(char_count).enumerate() {
            let ct: u16 = if info_type == 1 {
                let mut t: u16 = 0;
                if ch.is_uppercase() {
                    t |= 0x0001;
                }
                if ch.is_lowercase() {
                    t |= 0x0002;
                }
                if ch.is_ascii_digit() {
                    t |= 0x0004;
                }
                if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                    t |= 0x0008;
                }
                if ch.is_ascii_punctuation() {
                    t |= 0x0010;
                }
                if ch.is_control() {
                    t |= 0x0020;
                }
                if ch == ' ' {
                    t |= 0x0048;
                }
                if ch.is_alphanumeric() {
                    t |= 0x0100;
                }
                t
            } else {
                0
            };
            emu.maps.write_word(out_ptr + (i as u64) * 2, ct);
        }
    }
    log_red!(
        emu,
        "kernelbase!GetStringTypeW type:0x{:x} '{}' len:{}",
        info_type,
        src,
        char_count
    );
    call.set_return(emu, 1);
}
