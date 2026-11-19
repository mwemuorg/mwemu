use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathFindSuffixArrayA(emu: &mut Emu) {
    invoke(emu, false, "PathFindSuffixArrayA");
}

pub fn PathFindSuffixArrayW(emu: &mut Emu) {
    invoke(emu, true, "PathFindSuffixArrayW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let path = strings::read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let array = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_i32(emu, 2));
    log_red!(
        emu,
        "shlwapi!{} path:'{}' array:0x{:x} count:{}",
        api,
        path,
        array,
        count
    );
    call.set_return(emu, 0);
    if count <= 0
        || array == 0
        || !emu
            .maps
            .can_access_range(array, count as usize * size_of::<u64>(), Permission::READ)
    {
        return;
    }
    for index in 0..count as u64 {
        let Some(address) = emu.maps.read_qword(array + index * size_of::<u64>() as u64) else {
            return;
        };
        if address == 0 {
            continue;
        }
        let suffix = strings::read_text(emu, address, wide);
        if path.ends_with(&suffix) {
            call.set_return(emu, address);
            return;
        }
    }
}
