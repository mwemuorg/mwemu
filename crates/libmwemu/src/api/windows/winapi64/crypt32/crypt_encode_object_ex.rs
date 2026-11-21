use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptEncodeObjectEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let encoding = api_try!(emu, call.arg(emu, 0));
    let struct_type = api_try!(emu, call.arg(emu, 1));
    let struct_info = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CryptEncodeObjectEx enc:0x{:x} type:0x{:x} flags:0x{:x}",
        encoding,
        struct_type,
        flags
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
