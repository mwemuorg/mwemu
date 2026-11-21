use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminCalcHashFromFileHandle(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file_handle = api_try!(emu, call.arg(emu, 0));
    let hash_size_ptr = api_try!(emu, call.arg(emu, 1));
    let hash_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptCATAdminCalcHashFromFileHandle file:0x{:x}",
        file_handle
    );
    if hash_size_ptr != 0 {
        emu.maps.write_dword(hash_size_ptr, 20);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
