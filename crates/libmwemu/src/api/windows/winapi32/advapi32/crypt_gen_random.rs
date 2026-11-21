use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn CryptGenRandom(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let len = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let buf = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    log_red!(emu, "advapi32!CryptGenRandom len={} buf=0x{:x}", len, buf);
    let tick = emu.tick as u8;
    for i in 0..len {
        emu.maps.write_byte(buf + i, tick.wrapping_add(i as u8));
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
