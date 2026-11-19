use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsEqualGUID(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let guid1 = api_try!(emu, call.arg(emu, 0));
    let guid2 = api_try!(emu, call.arg(emu, 1));
    let equal = if guid1 != 0 && guid2 != 0 {
        let mut eq = true;
        for i in 0..4u64 {
            let a = emu.maps.read_dword(guid1 + i * 4).unwrap_or(0);
            let b = emu.maps.read_dword(guid2 + i * 4).unwrap_or(0);
            if a != b {
                eq = false;
                break;
            }
        }
        eq
    } else {
        false
    };
    log_red!(
        emu,
        "ole32!IsEqualGUID 0x{:x} 0x{:x} => {}",
        guid1,
        guid2,
        equal
    );
    call.set_return(emu, 1);
    call.set_return(emu, equal as u64);
}
