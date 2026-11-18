use crate::api::abi::windows::{CallingConvention::Thiscall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __0exception__QAE_ABV0__Z(emu: &mut emu::Emu) {
    let call = WinApi(Thiscall);
    // exception::exception(const exception&) - thiscall copy ctor
    let this = api_try!(emu, call.arg(emu, 0));
    let other = api_try!(emu, call.arg(emu, 1));

    if other != 0 && this != 0 {
        let vtable = emu.maps.read_dword(other).unwrap_or(0);
        let msg_ptr = emu.maps.read_dword(other + 4).unwrap_or(0);
        emu.maps.write_dword(this, vtable);
        emu.maps.write_dword(this + 4, msg_ptr);
    }
    log_red!(
        emu,
        "msvcrt!exception::exception(copy) this:0x{:x} other:0x{:x}",
        this,
        other
    );
    call.set_return(emu, this);
    call.cleanup_args(emu, 1);
}
