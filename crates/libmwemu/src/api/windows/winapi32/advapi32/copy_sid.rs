use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CopySid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dest_len = api_try!(emu, call.arg(emu, 0));
    let dest_sid = api_try!(emu, call.arg(emu, 1));
    let src_sid = api_try!(emu, call.arg(emu, 2));
    if src_sid != 0 && dest_sid != 0 {
        let sub_count = emu.maps.read_byte(src_sid + 1).unwrap_or(1) as u64;
        let sid_len = 8 + sub_count * 4;
        let copy_len = std::cmp::min(sid_len, dest_len);
        for i in 0..copy_len {
            let b = emu.maps.read_byte(src_sid + i).unwrap_or(0);
            emu.maps.write_byte(dest_sid + i, b);
        }
    }
    log_red!(
        emu,
        "advapi32!CopySid dst:0x{:x} src:0x{:x} len:{}",
        dest_sid,
        src_sid,
        dest_len
    );
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}
