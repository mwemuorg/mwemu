use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CharToOemBuffA(emu: &mut emu::Emu) {
    char_to_oem_buff(emu, "CharToOemBuffA", false);
}

pub fn CharToOemBuffW(emu: &mut emu::Emu) {
    char_to_oem_buff(emu, "CharToOemBuffW", true);
}

fn char_to_oem_buff(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let src = api_try!(emu, call.arg(emu, 0));
    let dst = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "user32!{api} src: 0x{:x} dst: 0x{:x} len: {}",
        src,
        dst,
        len
    );
    let src = api_try!(emu, call.arg(emu, 0));
    if wide {
        if src != 0 && emu.maps.is_mapped(src) {
            let _input = emu.maps.read_string(src);
        }
        if src != 0 && dst != 0 {
            for i in 0..len as u64 {
                let ch = emu.maps.read_word(src + i * 2).unwrap_or(0);
                emu.maps.write_byte(dst + i, (ch & 0xFF) as u8);
            }
        }
    } else {
        if src != 0 && emu.maps.is_mapped(src) {
            let _input = strings::read_text(emu, src, wide);
        }
        if src != 0 && dst != 0 {
            for i in 0..len as u64 {
                let b = emu.maps.read_byte(src + i).unwrap_or(0);
                emu.maps.write_byte(dst + i, b);
            }
        }
    }

    call.set_return(emu, 1);
}
