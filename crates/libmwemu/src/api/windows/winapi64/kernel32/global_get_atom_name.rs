use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GlobalGetAtomNameA(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameA", false);
}

fn global_get_atom_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let atom = api_try!(emu, call.arg(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let buf_size = api_try!(emu, call.arg(emu, 2));
    let name = format!("Atom#{:04X}", atom);
    log_red!(emu, "kernel32!{api} atom:0x{:x} => '{}'", atom, name);
    if buf != 0 && buf_size > 0 {
        strings::write_string(emu, buf, &name, wide);
        call.set_return(emu, name.len() as u64);
    } else {
        call.set_return(emu, 0);
    }
    emu.set_last_error(0);
}

pub fn GlobalGetAtomNameW(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameW", true);
}
