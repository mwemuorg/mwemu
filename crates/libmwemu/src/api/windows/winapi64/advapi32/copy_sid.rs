use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CopySid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dest_len = api_try!(emu, call.arg(emu, 0)) as usize;
    let dest_sid = api_try!(emu, call.arg(emu, 1));
    let src_sid = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "advapi32!CopySid dest:0x{:x} src:0x{:x} len:{}",
        dest_sid,
        src_sid,
        dest_len
    );
    if dest_sid != 0 && src_sid != 0 && dest_len > 0 {
        let copy_len = dest_len.min(68);
        let data = emu.maps.read_bytes(src_sid, copy_len).to_vec();
        emu.maps.write_bytes(dest_sid, &data);
    }
    call.set_return(emu, 1);
}
