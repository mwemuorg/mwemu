use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "BuildIoRingRegisterFileHandles" => kernelbase::BuildIoRingRegisterFileHandles(emu),
        "DuplicateHandle" => kernelbase::DuplicateHandle(emu),
        "GetFinalPathNameByHandleA" => kernelbase::GetFinalPathNameByHandleA(emu),
        "GetFinalPathNameByHandleW" => kernelbase::GetFinalPathNameByHandleW(emu),
        "GetVolumeInformationByHandleW" => kernelbase::GetVolumeInformationByHandleW(emu),
        "GlobalHandle" => kernelbase::GlobalHandle(emu),
        "SetFileInformationByHandle" => kernelbase::SetFileInformationByHandle(emu),
        "SetHandleCount" => kernelbase::SetHandleCount(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
