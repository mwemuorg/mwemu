use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CompareStringOrdinal(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let s1 = api_try!(emu, call.arg(emu, 0));
    let len1 = api_try!(emu, call.arg_i64(emu, 1));
    let s2 = api_try!(emu, call.arg(emu, 2));
    let _len2 = api_try!(emu, call.arg_i64(emu, 3));
    let ignore_case = api_try!(emu, call.arg(emu, 4));
    let str1 = emu.maps.read_wide_string(s1);
    let str2 = emu.maps.read_wide_string(s2);
    let r = if ignore_case != 0 {
        str1.to_lowercase().cmp(&str2.to_lowercase()) as i32
    } else {
        str1.cmp(&str2) as i32
    };
    let result = match r.cmp(&0) {
        std::cmp::Ordering::Less => 1u64,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 3,
    };
    log_red!(
        emu,
        "kernel32!CompareStringOrdinal '{}' '{}' ic:{} => {}",
        str1,
        str2,
        ignore_case,
        result
    );
    call.set_return(emu, result);
}
