use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CompareStringEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _locale_ptr = api_try!(emu, call.arg(emu, 0));
    let _flags = api_try!(emu, call.arg(emu, 1));
    let str1_ptr = api_try!(emu, call.arg(emu, 2));
    let _len1 = api_try!(emu, call.arg_i32(emu, 3));
    let str2_ptr = api_try!(emu, call.arg(emu, 4));
    let s1 = if str1_ptr != 0 {
        emu.maps.read_wide_string(str1_ptr)
    } else {
        String::new()
    };
    let s2 = if str2_ptr != 0 {
        emu.maps.read_wide_string(str2_ptr)
    } else {
        String::new()
    };
    let result = match s1.to_lowercase().cmp(&s2.to_lowercase()) {
        std::cmp::Ordering::Less => 1u64,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 3,
    };
    log_red!(
        emu,
        "kernelbase!CompareStringEx '{}' vs '{}' => {}",
        s1,
        s2,
        result
    );
    emu.set_last_error(0);
    call.set_return(emu, result);
}
