use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn EqualRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rect1 = api_try!(emu, call.arg(emu, 0));
    let rect2 = api_try!(emu, call.arg(emu, 1));
    let eq = if rect1 != 0 && rect2 != 0 {
        let l1 = emu.maps.read_dword(rect1).unwrap_or(0);
        let l2 = emu.maps.read_dword(rect2).unwrap_or(0);
        let t1 = emu.maps.read_dword(rect1 + 4).unwrap_or(0);
        let t2 = emu.maps.read_dword(rect2 + 4).unwrap_or(0);
        let r1 = emu.maps.read_dword(rect1 + 8).unwrap_or(0);
        let r2 = emu.maps.read_dword(rect2 + 8).unwrap_or(0);
        let b1 = emu.maps.read_dword(rect1 + 12).unwrap_or(0);
        let b2 = emu.maps.read_dword(rect2 + 12).unwrap_or(0);
        l1 == l2 && t1 == t2 && r1 == r2 && b1 == b2
    } else {
        false
    };
    log_red!(
        emu,
        "user32!EqualRect 0x{:x} 0x{:x} => {}",
        rect1,
        rect2,
        eq
    );
    call.set_return(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, if eq { 1 } else { 0 });
}
