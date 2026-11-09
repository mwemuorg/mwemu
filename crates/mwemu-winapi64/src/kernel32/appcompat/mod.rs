use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

// Intentionally mirrored across 32-bit and 64-bit trees.

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "CompareStringW" => kernel32::CompareStringW(emu),
        "GetCurrentPackagePath" => kernel32::GetCurrentPackagePath(emu),
        "CompareStringA" => kernel32::CompareStringA(emu),
        "CompareStringOrdinal" => kernel32::CompareStringOrdinal(emu),
        _ => return false,
    }
    true
}
