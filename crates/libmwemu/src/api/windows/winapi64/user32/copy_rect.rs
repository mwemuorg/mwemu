use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CopyRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    if dst != 0 && src != 0 {
        let l = emu.maps.read_dword(src).unwrap_or(0);
        let t = emu.maps.read_dword(src + 4).unwrap_or(0);
        let r = emu.maps.read_dword(src + 8).unwrap_or(0);
        let b = emu.maps.read_dword(src + 12).unwrap_or(0);
        emu.maps.write_dword(dst, l);
        emu.maps.write_dword(dst + 4, t);
        emu.maps.write_dword(dst + 8, r);
        emu.maps.write_dword(dst + 12, b);
    }
    log_red!(emu, "user32!CopyRect dst:0x{:x} src:0x{:x}", dst, src);
    call.set_return(emu, 1);
}
