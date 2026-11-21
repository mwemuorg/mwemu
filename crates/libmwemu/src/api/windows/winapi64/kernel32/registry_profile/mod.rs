use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

mod profile_string;
pub use profile_string::{
    GetPrivateProfileIntA, GetPrivateProfileIntW, GetPrivateProfileStringA,
    GetPrivateProfileStringW, GetProfileIntA, GetProfileIntW, GetProfileStringA, GetProfileStringW,
    WritePrivateProfileStringA, WritePrivateProfileStringW, WriteProfileStringA,
    WriteProfileStringW,
};

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
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
        _ => return false,
    }
    true
}
