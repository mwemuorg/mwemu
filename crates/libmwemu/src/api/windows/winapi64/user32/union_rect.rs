use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn UnionRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src1 = api_try!(emu, call.arg(emu, 1));
    let src2 = api_try!(emu, call.arg(emu, 2));
    if dst != 0 && src1 != 0 && src2 != 0 {
        let l1 = emu.maps.read_dword(src1).unwrap_or(0) as i32;
        let t1 = emu.maps.read_dword(src1 + 4).unwrap_or(0) as i32;
        let r1 = emu.maps.read_dword(src1 + 8).unwrap_or(0) as i32;
        let b1 = emu.maps.read_dword(src1 + 12).unwrap_or(0) as i32;
        let l2 = emu.maps.read_dword(src2).unwrap_or(0) as i32;
        let t2 = emu.maps.read_dword(src2 + 4).unwrap_or(0) as i32;
        let r2 = emu.maps.read_dword(src2 + 8).unwrap_or(0) as i32;
        let b2 = emu.maps.read_dword(src2 + 12).unwrap_or(0) as i32;
        emu.maps.write_dword(dst, std::cmp::min(l1, l2) as u32);
        emu.maps.write_dword(dst + 4, std::cmp::min(t1, t2) as u32);
        emu.maps.write_dword(dst + 8, std::cmp::max(r1, r2) as u32);
        emu.maps.write_dword(dst + 12, std::cmp::max(b1, b2) as u32);
    }
    log_red!(
        emu,
        "user32!UnionRect dst:0x{:x} s1:0x{:x} s2:0x{:x}",
        dst,
        src1,
        src2
    );
    call.set_return(emu, 1);
}
