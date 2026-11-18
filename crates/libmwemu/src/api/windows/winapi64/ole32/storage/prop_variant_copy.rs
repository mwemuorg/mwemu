use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PropVariantCopy(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    if dst != 0 && src != 0 {
        for i in 0..4u64 {
            let dw = emu.maps.read_dword(src + i * 4).unwrap_or(0);
            emu.maps.write_dword(dst + i * 4, dw);
        }
    }
    log_red!(emu, "ole32!PropVariantCopy dst:0x{:x} src:0x{:x}", dst, src);
    call.set_return(emu, 0);
}
