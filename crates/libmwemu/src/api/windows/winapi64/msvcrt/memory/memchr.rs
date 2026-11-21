use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn memchr(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let val = api_try!(emu, call.arg_u8(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let mut found: u64 = 0;
    for i in 0..n {
        if emu.maps.read_byte(ptr + i as u64).unwrap_or(0) == val {
            found = ptr + i as u64;
            break;
        }
    }
    log_red!(
        emu,
        "msvcrt!memchr ptr:0x{:x} val:0x{:x} n:{} => 0x{:x}",
        ptr,
        val,
        n,
        found
    );
    emu.set_last_error(0);
    call.set_return(emu, found);
}
