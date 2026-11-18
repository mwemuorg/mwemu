use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _wcsnicmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s1_ptr = api_try!(emu, call.arg(emu, 0));
    let s2_ptr = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let s1: String = emu
        .maps
        .read_wide_string(s1_ptr)
        .to_lowercase()
        .chars()
        .take(n)
        .collect();
    let s2: String = emu
        .maps
        .read_wide_string(s2_ptr)
        .to_lowercase()
        .chars()
        .take(n)
        .collect();
    let result = s1.cmp(&s2) as i32;
    log_red!(
        emu,
        "wincrt!_wcsnicmp '{}' '{}' n:{} => {}",
        s1,
        s2,
        n,
        result
    );
    call.set_return(emu, result as u32 as u64);
}
