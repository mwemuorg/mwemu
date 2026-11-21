use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ToUnicodeEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let virt_key = api_try!(emu, call.arg_u32(emu, 0));
    let scan_code = api_try!(emu, call.arg_u32(emu, 1));
    let _lp_key_state = api_try!(emu, call.arg(emu, 2));
    let pwsz_buff = api_try!(emu, call.arg(emu, 3));
    let cch_buff = api_try!(emu, call.arg_i32(emu, 4));
    let _flags = api_try!(emu, call.arg(emu, 5));
    let _hkl = api_try!(emu, call.arg(emu, 6));
    let ch = if (0x20..=0x7e).contains(&virt_key) {
        virt_key as u16
    } else {
        0
    };
    if pwsz_buff != 0 && cch_buff > 0 {
        emu.maps.write_word(pwsz_buff, ch);
        if cch_buff > 1 {
            emu.maps.write_word(pwsz_buff + 2, 0);
        }
    }
    log_red!(
        emu,
        "user32!ToUnicodeEx vk:0x{:x} scan:0x{:x} => 0x{:x}",
        virt_key,
        scan_code,
        ch
    );
    call.set_return(emu, if ch != 0 { 1 } else { 0 });
}
