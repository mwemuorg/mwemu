use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptImportKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_prov = api_try!(emu, call.arg(emu, 0));
    let data_ptr = api_try!(emu, call.arg(emu, 1));
    let data_len = api_try!(emu, call.arg_u32(emu, 2));
    let h_pub_key = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CryptImportKey prov: 0x{:x} data: 0x{:x} len: {} pubkey: 0x{:x}",
        h_prov,
        data_ptr,
        data_len,
        h_pub_key
    );
    // 5th arg: flags, 6th arg: phKey
    let h_key_ptr = api_try!(emu, call.arg(emu, 5));
    let h_key = helper::handler_create("importedkey");
    if h_key_ptr != 0 {
        emu.maps.write_qword(h_key_ptr, h_key);
    }
    call.set_return(emu, 1); // TRUE
}
