use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::winapi::libraries::kernel32::file_time::guest_file_time;
use crate::winapi::policy::api_try;
use crate::windows::constants::time::{FILETIME_TICKS_PER_SECOND, UNIX_EPOCH_AS_FILETIME};

pub fn clock(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let ticks = i32::try_from(emu.tick).unwrap_or(-1);
    call.set_return(emu, u64::from(ticks as u32));
}

pub fn _time32(emu: &mut Emu) {
    seconds(emu, true);
}
pub fn _time64(emu: &mut Emu) {
    seconds(emu, false);
}
pub fn time(emu: &mut Emu) {
    seconds(emu, !emu.cfg.arch.is_64bits());
}

fn seconds(emu: &mut Emu, narrow: bool) {
    let call = WinApi(Cdecl);
    let out = api_try!(emu, call.arg(emu, 0));
    let seconds = (guest_file_time(emu) - UNIX_EPOCH_AS_FILETIME) / FILETIME_TICKS_PER_SECOND;
    if narrow {
        let value = i32::try_from(seconds).unwrap_or(-1);
        if out != 0 && !emu.maps.write_bytes(out, &value.to_le_bytes()) {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        call.set_return(emu, u64::from(value as u32));
    } else {
        if out != 0 && !emu.maps.write_bytes(out, &seconds.to_le_bytes()) {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        call.set_return_u64(emu, seconds);
    }
}
