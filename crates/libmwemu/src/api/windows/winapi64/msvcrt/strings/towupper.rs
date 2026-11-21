use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn towupper(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let c = api_try!(emu, call.arg_u32(emu, 0));
    let ch = char::from_u32(c).unwrap_or('\0');
    let upper = ch.to_uppercase().next().unwrap_or(ch);
    let result = upper as u32;
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, result);
    }
    log_red!(emu, "msvcrt!towupper '{}' => '{}'", ch, upper);
    emu.set_last_error(0);
    call.set_return(emu, result as u64);
}
