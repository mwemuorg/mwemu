use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetComputerNameExA(emu: &mut emu::Emu) {
    get_computer_name_ex(emu, "GetComputerNameExA", false);
}

pub fn GetComputerNameExW(emu: &mut emu::Emu) {
    get_computer_name_ex(emu, "GetComputerNameExW", true);
}

fn get_computer_name_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _name_type = api_try!(emu, call.arg_u32(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let size_ptr = api_try!(emu, call.arg(emu, 2));
    let name = "DESKTOP";
    log_red!(emu, "kernelbase!{api} buf:0x{:x}", buf);
    if wide {
        if buf != 0 {
            for (i, ch) in name.encode_utf16().enumerate() {
                emu.maps.write_word(buf + (i as u64) * 2, ch);
            }
            emu.maps.write_word(buf + (name.len() as u64) * 2, 0);
        }
    } else {
        if buf != 0 {
            strings::write_string(emu, buf, name, wide);
        }
    }

    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, name.len() as u32);
    }
    call.set_return(emu, 1);
}
