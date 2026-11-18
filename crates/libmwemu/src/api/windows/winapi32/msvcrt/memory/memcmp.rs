use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn memcmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s1 = api_try!(emu, call.arg(emu, 0));
    let s2 = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let mut result: i32 = 0;
    for i in 0..n {
        let b1 = emu.maps.read_byte(s1 + i as u64).unwrap_or(0);
        let b2 = emu.maps.read_byte(s2 + i as u64).unwrap_or(0);
        if b1 != b2 {
            result = if b1 < b2 { -1 } else { 1 };
            break;
        }
    }
    log_red!(
        emu,
        "msvcrt!memcmp 0x{:x} 0x{:x} n:{} => {}",
        s1,
        s2,
        n,
        result
    );

    emu.set_last_error(0);
    call.set_return(emu, result as u32 as u64);
}
