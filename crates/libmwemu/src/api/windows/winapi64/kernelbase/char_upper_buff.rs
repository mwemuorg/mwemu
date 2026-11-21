use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CharUpperBuffA(emu: &mut emu::Emu) {
    char_upper_buff(emu, "CharUpperBuffA", false);
}

pub fn CharUpperBuffW(emu: &mut emu::Emu) {
    char_upper_buff(emu, "CharUpperBuffW", true);
}

fn char_upper_buff(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1)) as usize;
    log_red!(emu, "kernelbase!{api} buf:0x{:x} len:{}", buf, len);
    if wide {
        if buf != 0 && len > 0 {
            let count = len.min(0x10000);
            for i in 0..count {
                let w = emu.maps.read_word(buf + (i as u64) * 2).unwrap_or(0);
                if w >= b'a' as u16 && w <= b'z' as u16 {
                    emu.maps.write_word(buf + (i as u64) * 2, w - 32);
                }
            }
            call.set_return(emu, count as u64);
        } else {
            call.set_return(emu, 0);
        }
    } else {
        let src = api_try!(emu, call.arg(emu, 0));
        if src != 0 && emu.maps.is_mapped(src) {
            let _input = strings::read_text(emu, src, wide);
        }
        if buf != 0 && len > 0 {
            let count = len.min(0x10000);
            for i in 0..count {
                let b = emu.maps.read_byte(buf + i as u64).unwrap_or(0);
                if b.is_ascii_lowercase() {
                    emu.maps.write_byte(buf + i as u64, b - 32);
                }
            }
            call.set_return(emu, count as u64);
        } else {
            call.set_return(emu, 0);
        }
    }
}
