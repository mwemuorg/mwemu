use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::ole32::read_guid_string;

pub fn StringFromGUID2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rguid = api_try!(emu, call.arg(emu, 0));
    let lpsz = api_try!(emu, call.arg(emu, 1));
    let cch_max = api_try!(emu, call.arg_i32(emu, 2));

    let guid_str = read_guid_string(emu, rguid);
    log_red!(
        emu,
        "ole32!StringFromGUID2 {} buf: 0x{:x} max: {}",
        guid_str,
        lpsz,
        cch_max
    );

    let needed = 39; // "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" = 38 + null = 39
    if cch_max < needed {
        emu.set_last_error(0);
        call.set_return(emu, 0);
        return;
    }

    if lpsz != 0 {
        emu.maps.write_wide_string(lpsz, &guid_str);
    }

    emu.set_last_error(0);
    call.set_return(emu, needed as u64);
}
