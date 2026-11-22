use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "CryptCreateHash" => kernel32::CryptCreateHash(emu),
        "OpenProcessToken" => kernel32::OpenProcessToken(emu),
        "OpenThreadToken" => kernel32::OpenThreadToken(emu),
        "SetThreadToken" => kernel32::SetThreadToken(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
