use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ToAscii(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let virt_key = api_try!(emu, call.arg_u32(emu, 0));
    let scan_code = api_try!(emu, call.arg_u32(emu, 1));
    let _lp_key_state = api_try!(emu, call.arg(emu, 2));
    let lp_char = api_try!(emu, call.arg(emu, 3));
    let _flags = api_try!(emu, call.arg(emu, 4));
    let ch = if (0x20..=0x7e).contains(&virt_key) {
        virt_key as u8
    } else {
        0
    };
    if lp_char != 0 {
        emu.maps.write_word(lp_char, ch as u16);
    }
    log_red!(
        emu,
        "user32!ToAscii vk:0x{:x} scan:0x{:x} => 0x{:x}",
        virt_key,
        scan_code,
        ch
    );
    call.set_return(emu, if ch != 0 { 1 } else { 0 });
}
