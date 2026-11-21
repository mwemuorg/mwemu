use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn LoadStringA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let p3 = api_try!(emu, call.arg(emu, 3));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernelbase!LoadStringA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );

    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn LoadStringW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let id = api_try!(emu, call.arg_u32(emu, 1));
    let buffer_ptr = api_try!(emu, call.arg(emu, 2));
    let buffer_len = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "kernelbase!LoadStringW inst:0x{:x} id:{} buf:0x{:x} len:{}",
        hinstance,
        id,
        buffer_ptr,
        buffer_len
    );

    if buffer_ptr != 0 && buffer_len > 0 {
        let fake = format!("String_{}", id);
        emu.maps.write_wide_string(buffer_ptr, &fake);
        call.set_return(emu, fake.len() as u64);
    } else {
        call.set_return(emu, 0);
    }
}
