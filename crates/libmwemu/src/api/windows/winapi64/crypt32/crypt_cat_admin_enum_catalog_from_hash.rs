use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminEnumCatalogFromHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let admin = api_try!(emu, call.arg(emu, 0));
    let hash_ptr = api_try!(emu, call.arg(emu, 1));
    let hash_len = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptCATAdminEnumCatalogFromHash admin:0x{:x}",
        admin
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("cryptcatenumcatalog");
    call.set_return(emu, handle);
}
