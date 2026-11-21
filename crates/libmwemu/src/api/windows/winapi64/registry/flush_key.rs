use super::finish;
use super::key_path;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn flush_key(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let err = key_path(hkey).err().unwrap_or(ERROR_SUCCESS);
    finish(emu, "RegFlushKey", &format!("0x{:x}", hkey), err);
}
