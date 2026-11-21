pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    GetPrivateProfileIntA, GetPrivateProfileIntW, GetPrivateProfileStringA,
    GetPrivateProfileStringW, GetProfileIntA, GetProfileIntW, GetProfileStringA, GetProfileStringW,
    WritePrivateProfileStringA, WritePrivateProfileStringW, WriteProfileStringA,
    WriteProfileStringW,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GetPrivateProfileStringA" => kernel32::GetPrivateProfileStringA(emu),
        "GetPrivateProfileStringW" => kernel32::GetPrivateProfileStringW(emu),
        "GetProfileStringA" => kernel32::GetProfileStringA(emu),
        "GetProfileStringW" => kernel32::GetProfileStringW(emu),
        "WritePrivateProfileStringA" => kernel32::WritePrivateProfileStringA(emu),
        "WritePrivateProfileStringW" => kernel32::WritePrivateProfileStringW(emu),
        "WriteProfileStringA" => kernel32::WriteProfileStringA(emu),
        "WriteProfileStringW" => kernel32::WriteProfileStringW(emu),
        "GetPrivateProfileIntA" => kernel32::GetPrivateProfileIntA(emu),
        "GetPrivateProfileIntW" => kernel32::GetPrivateProfileIntW(emu),
        "GetProfileIntA" => kernel32::GetProfileIntA(emu),
        "GetProfileIntW" => kernel32::GetProfileIntW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
