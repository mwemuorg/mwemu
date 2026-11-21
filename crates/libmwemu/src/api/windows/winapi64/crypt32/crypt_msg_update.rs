use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptMsgUpdate(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hmsg = api_try!(emu, call.arg(emu, 0));
    let pb_data = api_try!(emu, call.arg(emu, 1));
    let cb_data = api_try!(emu, call.arg(emu, 2));
    let f_final = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CryptMsgUpdate msg:0x{:x} data:0x{:x} len:{} final:{}",
        hmsg,
        pb_data,
        cb_data,
        f_final
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    if hmsg != 0 && pb_data != 0 && cb_data > 0 {
        let mut buf = vec![0u8; cb_data as usize];
        for i in 0..cb_data {
            buf[i as usize] = emu.maps.read_byte(pb_data + i).unwrap_or(0);
        }
        helper::handler_put_bytes(hmsg, &buf);
    }
    call.set_return(emu, 1);
}
