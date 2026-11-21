use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn CryptDecodeObjectEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let encoding_type = api_try!(emu, call.arg_u32(emu, 0));
    let struct_type = api_try!(emu, call.arg_u32(emu, 1));
    let _encoded = api_try!(emu, call.arg_u32(emu, 2));
    let cb_encoded = api_try!(emu, call.arg_u32(emu, 3));
    let flags = api_try!(emu, call.arg_u32(emu, 4));
    let _decode_para = api_try!(emu, call.arg_u32(emu, 5));
    let _pv_struct_info = api_try!(emu, call.arg_u32(emu, 6));
    let pcb_struct_info = api_try!(emu, call.arg_u32(emu, 7)) as u64;

    log_red!(
        emu,
        "crypt32!CryptDecodeObjectEx enc_type: 0x{:x} struct_type: 0x{:x} len: {} flags: 0x{:x}",
        encoding_type,
        struct_type,
        cb_encoded,
        flags
    );

    if pcb_struct_info != 0 {
        emu.maps.write_dword(pcb_struct_info, 0);
    }

    call.set_return(emu, 1);
    call.cleanup_args(emu, 8);
}
