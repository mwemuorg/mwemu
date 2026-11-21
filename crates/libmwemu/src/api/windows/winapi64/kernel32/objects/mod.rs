use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::libraries::kernel32::memory::{GlobalHandle, LocalHandle};
use crate::winapi::winapi64::kernel32;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GlobalHandle" => GlobalHandle(emu),
        "LocalHandle" => LocalHandle(emu),
        "DuplicateHandle" => kernel32::DuplicateHandle(emu),
        "SetHandleCount" => kernel32::SetHandleCount(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
