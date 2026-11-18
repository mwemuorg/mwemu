use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CopySid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dest_len = api_try!(emu, call.arg(emu, 0)) as usize;
    let dest_ptr = api_try!(emu, call.arg(emu, 1));
    let src_ptr = api_try!(emu, call.arg(emu, 2));
    if dest_ptr != 0 && src_ptr != 0 && dest_len > 0 {
        let sub_auth_count = emu.maps.read_byte(src_ptr + 1).unwrap_or(0) as usize;
        let sid_len = 8 + sub_auth_count * 4;
        let copy_len = sid_len.min(dest_len);
        for i in 0..copy_len {
            let b = emu.maps.read_byte(src_ptr + i as u64).unwrap_or(0);
            emu.maps.write_byte(dest_ptr + i as u64, b);
        }
    }
    log_red!(
        emu,
        "kernelbase!CopySid len:{} dst:0x{:x} src:0x{:x}",
        dest_len,
        dest_ptr,
        src_ptr
    );

    call.set_return(emu, 1);
}
