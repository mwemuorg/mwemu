use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_strnicmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s1 = api_try!(emu, call.arg(emu, 0));
    let s2 = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let str1 = emu.maps.read_string(s1).to_ascii_lowercase();
    let str2 = emu.maps.read_string(s2).to_ascii_lowercase();
    let a = &str1[..str1.len().min(n)];
    let b = &str2[..str2.len().min(n)];
    let res = a.cmp(b) as i32;
    call.set_return(emu, res as u64);
}
