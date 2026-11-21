use super::alloc_bstr;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn VarBstrFromCy(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cy = api_try!(emu, call.arg_i64(emu, 0));
    let _lcid = api_try!(emu, call.arg_u32(emu, 1));
    let _flags = api_try!(emu, call.arg_u32(emu, 2));
    let pbstr_out = api_try!(emu, call.arg(emu, 3));
    let s = format!("{}.{:04}", cy / 10000, (cy % 10000).unsigned_abs());
    log_red!(
        emu,
        "oleaut32!VarBstrFromCy val:{} out:0x{:x}",
        s,
        pbstr_out
    );
    let bstr = alloc_bstr(emu, &s);
    if pbstr_out != 0 {
        emu.maps.write_qword(pbstr_out, bstr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
