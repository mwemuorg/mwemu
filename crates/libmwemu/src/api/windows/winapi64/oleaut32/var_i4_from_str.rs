use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn VarI4FromStr(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_in = api_try!(emu, call.arg(emu, 0));
    let _lcid = api_try!(emu, call.arg_u32(emu, 1));
    let _flags = api_try!(emu, call.arg_u32(emu, 2));
    let pl_out = api_try!(emu, call.arg(emu, 3));
    let s = if str_in != 0 {
        emu.maps.read_wide_string(str_in)
    } else {
        String::new()
    };
    let val = s.trim().parse::<i32>().unwrap_or(0);
    log_red!(
        emu,
        "oleaut32!VarI4FromStr '{}' => {} out:0x{:x}",
        s,
        val,
        pl_out
    );
    if pl_out != 0 {
        emu.maps.write_dword(pl_out, val as u32);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
