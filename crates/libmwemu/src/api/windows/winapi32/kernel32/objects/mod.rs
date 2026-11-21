use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "DuplicateHandle" => kernel32::DuplicateHandle(emu),
        "SetHandleCount" => kernel32::SetHandleCount(emu),
        "AddIntegrityLabelToBoundaryDescriptor" => kernel32::AddIntegrityLabelToBoundaryDescriptor(emu),
        "CloseConsoleHandle" => kernel32::CloseConsoleHandle(emu),
        "GetConsoleInputWaitHandle" => kernel32::GetConsoleInputWaitHandle(emu),
        "GetHandleContext" => kernel32::GetHandleContext(emu),
        "GetNumaNodeNumberFromHandle" => kernel32::GetNumaNodeNumberFromHandle(emu),
        "GlobalHandle" => kernel32::GlobalHandle(emu),
        "LocalHandle" => kernel32::LocalHandle(emu),
        "OpenPrivateNamespaceA" => kernel32::OpenPrivateNamespaceA(emu),
        "VerifyConsoleIoHandle" => kernel32::VerifyConsoleIoHandle(emu),
        "BuildIoRingRegisterFileHandles" => kernel32::BuildIoRingRegisterFileHandles(emu),
        "GetFinalPathNameByHandleA" => kernel32::GetFinalPathNameByHandleA(emu),
        "GetFinalPathNameByHandleW" => kernel32::GetFinalPathNameByHandleW(emu),
        "GetVolumeInformationByHandleW" => kernel32::GetVolumeInformationByHandleW(emu),
        "SetFileInformationByHandle" => kernel32::SetFileInformationByHandle(emu),
        "SetHandleContext" => kernel32::SetHandleContext(emu),
        "CreateSocketHandle" => kernel32::CreateSocketHandle(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
