use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "DuplicateHandle" => kernel32::DuplicateHandle(emu),
        "SetHandleCount" => kernel32::SetHandleCount(emu),
        _ => return false,
    }
    true
}
