use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _wcsnicmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s1_ptr = api_try!(emu, call.arg(emu, 0));
    let s2_ptr = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let s1 = emu.maps.read_wide_string(s1_ptr).to_ascii_lowercase();
    let s2 = emu.maps.read_wide_string(s2_ptr).to_ascii_lowercase();
    let result = s1.get(..n).unwrap_or(&s1).cmp(s2.get(..n).unwrap_or(&s2)) as i32;
    log_red!(
        emu,
        "msvcrt!_wcsnicmp '{}' '{}' {} => {}",
        s1,
        s2,
        n,
        result
    );
    call.set_return(emu, result as u64);
}
