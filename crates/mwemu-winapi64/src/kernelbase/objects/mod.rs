use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "AccessCheckByTypeResultListAndAuditAlarmByHandleW" => {
            kernelbase::AccessCheckByTypeResultListAndAuditAlarmByHandleW(emu)
        }
        "BasepAdjustObjectAttributesForPrivateNamespace" => {
            kernelbase::BasepAdjustObjectAttributesForPrivateNamespace(emu)
        }
        "CompareObjectHandles" => kernelbase::CompareObjectHandles(emu),
        "DisablePredefinedHandleTableInternal" => {
            kernelbase::DisablePredefinedHandleTableInternal(emu)
        }
        "DuplicateStateContainerHandle" => kernelbase::DuplicateStateContainerHandle(emu),
        "GetAdjustObjectAttributesForPrivateNamespaceRoutine" => {
            kernelbase::GetAdjustObjectAttributesForPrivateNamespaceRoutine(emu)
        }
        "GetFileVersionInfoByHandle" => kernelbase::GetFileVersionInfoByHandle(emu),
        "MapPredefinedHandleInternal" => kernelbase::MapPredefinedHandleInternal(emu),
        "RemapPredefinedHandleInternal" => kernelbase::RemapPredefinedHandleInternal(emu),
        "AddSIDToBoundaryDescriptor" => kernelbase::AddSIDToBoundaryDescriptor(emu),
        "BaseFormatObjectAttributes" => kernelbase::BaseFormatObjectAttributes(emu),
        "BuildIoRingRegisterFileHandles" => kernelbase::BuildIoRingRegisterFileHandles(emu),
        "ClosePrivateNamespace" => kernelbase::ClosePrivateNamespace(emu),
        "CreateBoundaryDescriptorW" => kernelbase::CreateBoundaryDescriptorW(emu),
        "CreatePrivateNamespaceW" => kernelbase::CreatePrivateNamespaceW(emu),
        "DeleteBoundaryDescriptor" => kernelbase::DeleteBoundaryDescriptor(emu),
        "DuplicateHandle" => kernelbase::DuplicateHandle(emu),
        "GetFileInformationByHandle" => kernelbase::GetFileInformationByHandle(emu),
        "GetFileInformationByHandleEx" => kernelbase::GetFileInformationByHandleEx(emu),
        "GetFinalPathNameByHandleA" => kernelbase::GetFinalPathNameByHandleA(emu),
        "GetFinalPathNameByHandleW" => kernelbase::GetFinalPathNameByHandleW(emu),
        "GetHandleInformation" => kernelbase::GetHandleInformation(emu),
        "GetNamedPipeHandleStateW" => kernelbase::GetNamedPipeHandleStateW(emu),
        "GetProcessHandleCount" => kernelbase::GetProcessHandleCount(emu),
        "GetVolumeInformationByHandleW" => kernelbase::GetVolumeInformationByHandleW(emu),
        "GlobalHandle" => kernelbase::GlobalHandle(emu),
        "OpenPrivateNamespaceW" => kernelbase::OpenPrivateNamespaceW(emu),
        "SetConsoleCtrlHandler" => kernelbase::SetConsoleCtrlHandler(emu),
        "SetFileInformationByHandle" => kernelbase::SetFileInformationByHandle(emu),
        "SetHandleCount" => kernelbase::SetHandleCount(emu),
        "SetHandleInformation" => kernelbase::SetHandleInformation(emu),
        "SetNamedPipeHandleState" => kernelbase::SetNamedPipeHandleState(emu),
        "SetStdHandle" => kernelbase::SetStdHandle(emu),
        "SetStdHandleEx" => kernelbase::SetStdHandleEx(emu),
        _ => return false,
    }
    true
}
