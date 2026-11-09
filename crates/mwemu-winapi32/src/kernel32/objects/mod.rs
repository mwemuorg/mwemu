use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "DuplicateHandle" => kernel32::DuplicateHandle(emu),
        "SetHandleCount" => kernel32::SetHandleCount(emu),
        "AddIntegrityLabelToBoundaryDescriptor" => {
            kernel32::AddIntegrityLabelToBoundaryDescriptor(emu)
        }
        "CloseConsoleHandle" => kernel32::CloseConsoleHandle(emu),
        "CreateBoundaryDescriptorA" => kernel32::CreateBoundaryDescriptorA(emu),
        "GetConsoleInputWaitHandle" => kernel32::GetConsoleInputWaitHandle(emu),
        "GetHandleContext" => kernel32::GetHandleContext(emu),
        "GetNamedPipeHandleStateA" => kernel32::GetNamedPipeHandleStateA(emu),
        "GetNumaNodeNumberFromHandle" => kernel32::GetNumaNodeNumberFromHandle(emu),
        "GlobalHandle" => kernel32::GlobalHandle(emu),
        "LocalHandle" => kernel32::LocalHandle(emu),
        "OpenPrivateNamespaceA" => kernel32::OpenPrivateNamespaceA(emu),
        "VerifyConsoleIoHandle" => kernel32::VerifyConsoleIoHandle(emu),
        "AddSIDToBoundaryDescriptor" => kernel32::AddSIDToBoundaryDescriptor(emu),
        "BaseFormatObjectAttributes" => kernel32::BaseFormatObjectAttributes(emu),
        "BasepMapModuleHandle" => kernel32::BasepMapModuleHandle(emu),
        "BuildIoRingRegisterFileHandles" => kernel32::BuildIoRingRegisterFileHandles(emu),
        "ClosePrivateNamespace" => kernel32::ClosePrivateNamespace(emu),
        "CreateBoundaryDescriptorW" => kernel32::CreateBoundaryDescriptorW(emu),
        "CreatePrivateNamespaceA" => kernel32::CreatePrivateNamespaceA(emu),
        "CreatePrivateNamespaceW" => kernel32::CreatePrivateNamespaceW(emu),
        "DeleteBoundaryDescriptor" => kernel32::DeleteBoundaryDescriptor(emu),
        "DuplicateConsoleHandle" => kernel32::DuplicateConsoleHandle(emu),
        "GetFileInformationByHandle" => kernel32::GetFileInformationByHandle(emu),
        "GetFileInformationByHandleEx" => kernel32::GetFileInformationByHandleEx(emu),
        "GetFinalPathNameByHandleA" => kernel32::GetFinalPathNameByHandleA(emu),
        "GetFinalPathNameByHandleW" => kernel32::GetFinalPathNameByHandleW(emu),
        "GetHandleInformation" => kernel32::GetHandleInformation(emu),
        "GetNamedPipeHandleStateW" => kernel32::GetNamedPipeHandleStateW(emu),
        "GetProcessHandleCount" => kernel32::GetProcessHandleCount(emu),
        "GetVolumeInformationByHandleW" => kernel32::GetVolumeInformationByHandleW(emu),
        "OpenPrivateNamespaceW" => kernel32::OpenPrivateNamespaceW(emu),
        "RegisterWowBaseHandlers" => kernel32::RegisterWowBaseHandlers(emu),
        "SetConsoleCtrlHandler" => kernel32::SetConsoleCtrlHandler(emu),
        "SetFileInformationByHandle" => kernel32::SetFileInformationByHandle(emu),
        "SetHandleContext" => kernel32::SetHandleContext(emu),
        "SetHandleInformation" => kernel32::SetHandleInformation(emu),
        "SetNamedPipeHandleState" => kernel32::SetNamedPipeHandleState(emu),
        "SetStdHandle" => kernel32::SetStdHandle(emu),
        "SetStdHandleEx" => kernel32::SetStdHandleEx(emu),
        "CreateSocketHandle" => kernel32::CreateSocketHandle(emu),
        _ => return false,
    }
    true
}
