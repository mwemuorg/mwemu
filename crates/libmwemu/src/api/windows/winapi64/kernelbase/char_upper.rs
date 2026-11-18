use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CharUpperA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    if str_ptr > 0xffff {
        let s = emu.maps.read_string(str_ptr);
        let upper = s.to_uppercase();
        emu.maps.write_string(str_ptr, &upper);
        log_red!(emu, "kernelbase!CharUpperA '{}' => '{}'", s, upper);
        call.set_return(emu, str_ptr);
    } else {
        let ch = str_ptr as u8;
        let uppered = (ch as char).to_uppercase().next().unwrap_or(ch as char) as u64;
        log_red!(emu, "kernelbase!CharUpperA char:0x{:x}", str_ptr);
        call.set_return(emu, uppered);
    }
}

pub fn CharUpperW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    if str_ptr > 0xffff {
        let s = emu.maps.read_wide_string(str_ptr);
        let upper = s.to_uppercase();
        emu.maps.write_wide_string(str_ptr, &upper);
        log_red!(emu, "kernelbase!CharUpperW '{}' => '{}'", s, upper);
        call.set_return(emu, str_ptr);
    } else {
        let ch = (str_ptr as u16) as u32;
        let uppered = char::from_u32(ch)
            .map(|c| c.to_uppercase().next().unwrap_or(c) as u64)
            .unwrap_or(str_ptr);
        log_red!(emu, "kernelbase!CharUpperW char:0x{:x}", str_ptr);
        call.set_return(emu, uppered);
    }
}
