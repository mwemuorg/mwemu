use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GlobalAddAtomA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lp_string = api_try!(emu, call.arg(emu, 0));

    let name = emu.maps.read_string(lp_string);
    log_red!(emu, "kernel32!GlobalAddAtomA '{}'", name);

    let mut hash: u16 = 0xC000;
    for b in name.bytes() {
        hash = hash.wrapping_add(b as u16);
    }
    if hash < 0xC000 {
        hash = hash.wrapping_add(0xC000);
    }
    emu.set_last_error(0);
    call.set_return(emu, hash as u64);
}

pub fn GlobalAddAtomW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let name = api_try!(emu, call.arg(emu, 0));
    let name_s = if name != 0 {
        emu.maps.read_wide_string(name)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!GlobalAddAtomW '{}' => 0xC001", name_s);
    call.set_return(emu, 0xC001);
}
