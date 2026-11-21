use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptGenRandom(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hprov = api_try!(emu, call.arg(emu, 0));
    let dw_len = api_try!(emu, call.arg(emu, 1));
    let pb_buffer = api_try!(emu, call.arg(emu, 2));

    log_red!(
        emu,
        "advapi32!CryptGenRandom len={} buf=0x{:x}",
        dw_len,
        pb_buffer
    );

    if pb_buffer != 0 && dw_len > 0 {
        let tick = emu.tick as u8;
        for i in 0..dw_len {
            let byte = tick.wrapping_add(i as u8);
            emu.maps.write_byte(pb_buffer + i, byte);
        }
    }

    call.set_return(emu, 1);
}
