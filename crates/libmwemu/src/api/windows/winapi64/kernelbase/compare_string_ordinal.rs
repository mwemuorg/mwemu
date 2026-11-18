use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CompareStringOrdinal(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str1_ptr = api_try!(emu, call.arg(emu, 0));
    let len1 = api_try!(emu, call.arg_i32(emu, 1));
    let str2_ptr = api_try!(emu, call.arg(emu, 2));
    let len2 = api_try!(emu, call.arg_i32(emu, 3));
    let ignore_case = api_try!(emu, call.arg(emu, 4));
    let s1 = emu.maps.read_wide_string(str1_ptr);
    let s2 = emu.maps.read_wide_string(str2_ptr);
    let result = if ignore_case != 0 {
        match s1.to_lowercase().cmp(&s2.to_lowercase()) {
            std::cmp::Ordering::Less => 1u64,
            std::cmp::Ordering::Equal => 2,
            std::cmp::Ordering::Greater => 3,
        }
    } else {
        match s1.cmp(&s2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 2,
            std::cmp::Ordering::Greater => 3,
        }
    };
    log_red!(
        emu,
        "kernelbase!CompareStringOrdinal '{}' vs '{}' => {}",
        s1,
        s2,
        result
    );
    emu.set_last_error(0);
    call.set_return(emu, result);
}
