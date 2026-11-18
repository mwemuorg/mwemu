use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptGenRandom(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_prov = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg_u32(emu, 1));
    let buffer = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptGenRandom prov: 0x{:x} len: {} buf: 0x{:x}",
        h_prov,
        len,
        buffer
    );
    // Fill buffer with deterministic pseudo-random bytes based on emu.pos
    if buffer != 0 {
        for i in 0..len {
            let byte = ((emu.pos as u32)
                .wrapping_mul(1103515245)
                .wrapping_add(i * 12345)) as u8;
            emu.maps.write_byte(buffer + i as u64, byte);
        }
    }
    call.set_return(emu, 1); // TRUE
}
