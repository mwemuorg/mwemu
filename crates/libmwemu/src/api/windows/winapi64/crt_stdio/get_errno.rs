use super::EINVAL;
use super::errno_cell;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _get_errno(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    if out == 0 {
        call.set_return(emu, EINVAL as u64);
        return;
    }
    let cell = errno_cell(emu);
    let value = emu.maps.read_dword(cell).unwrap_or(0);
    emu.maps.write_dword(out, value);
    call.set_return(emu, 0);
}
