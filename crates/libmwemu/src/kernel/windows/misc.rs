//! Process/thread (`Ps`), object manager (`Ob`), security (`Se`), native
//! system services (`Zw`), registry callbacks (`Cm`), filter manager (`Flt`),
//! the WDF loader, ETW/WMI and power (`Po`).
//!
//! None of these hold state a driver bug lives in, so they are modelled as
//! what a driver needs to keep going: a handle that is remembered, a fake
//! System process to point at, a registry that answers "not found" so the
//! driver's defaults are used, and a `ZwQuerySystemInformation` that reports
//! a one-module kernel so a driver resolving ntoskrnl's base succeeds.
//! `\Device\PhysicalMemory` sections are the exception: opening and mapping
//! one is a BYOVD primitive and is logged as such.

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::policy::api_try;
use crate::kernel::{HandlerRegistry, kernel_handlers, return_one, return_zero};

use super::structs::*;
use super::*;
use crate::kernel::DeferredCall;
use crate::kernel::heap::Region;

fn object_name_of(emu: &Emu, object_attributes: u64) -> String {
    if object_attributes == 0 {
        return String::new();
    }
    let us = emu
        .maps
        .read_qword(object_attributes + OBJATTR_OBJECT_NAME)
        .unwrap_or(0);
    read_unicode_string(emu, us)
}

fn system_process(emu: &Emu) -> u64 {
    emu.kernel
        .as_ref()
        .map(|k| k.win.system_process)
        .unwrap_or(0)
}

fn register_callback(emu: &mut Emu, api: &str, func: u64, ctx: u64) {
    log::info!("{}: callback 0x{:x} (ctx 0x{:x})", api, func, ctx);
    if let Some(k) = emu.kernel.as_mut() {
        k.win.callbacks.push(RegisteredCallback {
            api: api.to_string(),
            func,
            context: ctx,
        });
    }
}

/// Write an `IO_STATUS_BLOCK`.
fn write_iosb(emu: &mut Emu, iosb: u64, status: u64, information: u64) {
    if iosb != 0 {
        emu.maps.write_dword(iosb, status as u32);
        emu.maps.write_qword(iosb + 8, information);
    }
}

/// `SystemModuleInformation`: one module, ntoskrnl, at the stub base.
fn system_module_information(emu: &mut Emu, buf: u64, len: u64, ret_len: u64) -> u64 {
    let need = 8 + RTL_PROCESS_MODULE_INFO_SIZE;
    write_out_dword(emu, ret_len, need as u32);
    if buf == 0 || len < need {
        return STATUS_INFO_LENGTH_MISMATCH;
    }
    let (base, size) = emu
        .kernel
        .as_ref()
        .map(|k| (k.layout.stub_base, k.layout.stub_size))
        .unwrap_or((0, 0));
    emu.maps.write_bytes(buf, &vec![0u8; need as usize]);
    emu.maps.write_dword(buf, 1);
    let m = buf + 8;
    emu.maps.write_qword(m + RTL_PM_IMAGE_BASE, base);
    emu.maps.write_dword(m + RTL_PM_IMAGE_SIZE, size as u32);
    emu.maps.write_word(m + RTL_PM_LOAD_COUNT, 1);
    let path = "\\SystemRoot\\system32\\ntoskrnl.exe";
    emu.maps.write_word(
        m + RTL_PM_OFFSET_TO_FILE_NAME,
        (path.len() - "ntoskrnl.exe".len()) as u16,
    );
    emu.maps.write_string(m + RTL_PM_FULL_PATH_NAME, path);
    STATUS_SUCCESS
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- Ps ------------------------------------------------------------------------
        "PsGetCurrentProcess"
        | "PsGetThreadProcess"
        | "PsGetCurrentThreadProcess"
        | "PsGetProcessInheritedFromUniqueProcessId" => process_stub,
        "PsGetCurrentProcessId"
        | "PsGetProcessId"
        | "PsGetThreadProcessId"
        | "PsGetCurrentThreadProcessId"
        | "PsGetCurrentProcessSessionId" => process_id_stub,
        "PsGetCurrentThreadId" | "PsGetThreadId" => thread_id_stub,
        "PsGetProcessImageFileName" => PsGetProcessImageFileName,
        "PsGetProcessPeb"
        | "PsGetProcessWow64Process"
        | "PsGetProcessSectionBaseAddress"
        | "PsGetProcessDebugPort"
        | "PsGetProcessJob"
        | "PsGetProcessWin32Process"
        | "PsGetThreadWin32Thread"
        | "PsGetThreadTeb"
        | "PsGetProcessCreateTimeQuadPart"
        | "PsGetProcessStartKey"
        | "PsGetProcessSecurityPort"
        | "PsGetProcessProtection" => return_zero,
        "PsGetProcessExitStatus" => return_pending,
        "PsGetProcessExitProcessCalled"
        | "PsIsThreadTerminating"
        | "PsIsProcessBeingDebugged"
        | "PsIsProtectedProcess"
        | "PsIsProtectedProcessLight"
        | "PsIsSystemProcess"
        | "PsGetProcessWow64ProcessEx" => return_zero,
        "PsIsSystemThread" => return_one,
        "PsLookupProcessByProcessId"
        | "PsLookupThreadByThreadId"
        | "PsLookupProcessThreadByCid" => PsLookupProcessByProcessId,
        "PsSetCreateProcessNotifyRoutine"
        | "PsSetCreateProcessNotifyRoutineEx"
        | "PsSetCreateProcessNotifyRoutineEx2"
        | "PsSetCreateThreadNotifyRoutine"
        | "PsSetCreateThreadNotifyRoutineEx"
        | "PsSetLoadImageNotifyRoutine"
        | "PsSetLoadImageNotifyRoutineEx" => PsSetCreateProcessNotifyRoutine,
        "PsRemoveCreateThreadNotifyRoutine" | "PsRemoveLoadImageNotifyRoutine" => PsRemoveCreateThreadNotifyRoutine,
        // PsCreateSystemThread(*Handle, Access, ObjAttr, Process, ClientId, StartRoutine, StartContext)
        "PsCreateSystemThread" | "PsCreateSystemThreadEx" => PsCreateSystemThread,
        "PsTerminateSystemThread" => PsTerminateSystemThread,
        "PsGetVersion" => PsGetVersion,
        "PsReferencePrimaryToken" | "PsReferenceImpersonationToken" => token_stub,
        "PsDereferencePrimaryToken"
        | "PsDereferenceImpersonationToken"
        | "PsChargeProcessPoolQuota"
        | "PsReturnPoolQuota"
        | "PsSetThreadHardErrorsAreDisabled"
        | "PsRevertToSelf"
        | "PsReferenceProcessFilePointer"
        | "PsSetProcessWindowStation"
        | "PsImpersonateClient"
        | "PsAcquireProcessExitSynchronization"
        | "PsReleaseProcessExitSynchronization"
        | "PsRegisterSiloMonitor"
        | "PsUnregisterSiloMonitor"
        | "PsStartSiloMonitor"
        | "PsGetSiloContainerId"
        | "PsGetProcessServerSilo"
        | "PsGetThreadServerSilo"
        | "PsGetHostSilo"
        | "PsGetCurrentServerSilo"
        | "PsGetCurrentServerSiloName"
        | "PsAttachSiloToCurrentThread"
        | "PsDetachSiloFromCurrentThread"
        | "PsChargePoolQuota"
        | "PsEnterPriorityRegion"
        | "PsLeavePriorityRegion"
        | "PsWrapApcWow64Thread" => return_zero,
        "PsGetProcessImageFileNameEx"
        | "PsReferenceProcessImageFileName"
        | "SeLocateProcessImageName" => process_image_name_stub,

        // --- Ob ------------------------------------------------------------------------
        // ObReferenceObjectByHandle(Handle, Access, Type, Mode, *Object, *HandleInfo)
        "ObReferenceObjectByHandle" | "ObReferenceObjectByHandleWithTag" => ObReferenceObjectByHandle,
        "ObReferenceObjectByPointer"
        | "ObReferenceObjectByPointerWithTag"
        | "ObfReferenceObject"
        | "ObReferenceObject"
        | "ObfReferenceObjectWithTag"
        | "ObReferenceObjectWithTag"
        | "ObfDereferenceObject"
        | "ObDereferenceObject"
        | "ObfDereferenceObjectWithTag"
        | "ObDereferenceObjectWithTag"
        | "ObDereferenceObjectDeferDelete"
        | "ObDereferenceObjectDeferDeleteWithTag"
        | "ObReleaseObjectSecurity"
        | "ObMakeTemporaryObject"
        | "ObDeleteCapturedInsertInfo"
        | "ObSetHandleAttributes"
        | "ObUnRegisterCallbacks"
        | "ObSetSecurityObjectByPointer"
        | "ObDereferenceSecurityDescriptor"
        | "ObReferenceSecurityDescriptor"
        | "ObAssignSecurity" => return_zero,
        "ObOpenObjectByPointer" | "ObOpenObjectByPointerWithTag" => ObOpenObjectByPointer,
        "ObOpenObjectByName" => ObOpenObjectByName,
        // ObReferenceObjectByName(Name, Attributes, PassedAccessState, Access, Type,
        //                         Mode, ParseContext, *Object)
        "ObReferenceObjectByName" => ObReferenceObjectByName,
        "ObQueryNameString" => ObQueryNameString,
        "ObGetObjectSecurity" => ObGetObjectSecurity,
        "ObRegisterCallbacks" => ObRegisterCallbacks,
        "ObGetFilterVersion" => ObGetFilterVersion,
        "ObCloseHandle" | "ZwClose" | "NtClose" => ObCloseHandle,
        "ObIsKernelHandle" => ObIsKernelHandle,

        // --- Se ------------------------------------------------------------------------
        "SeCaptureSecurityDescriptor" => SeCaptureSecurityDescriptor,
        "SeReleaseSecurityDescriptor"
        | "SeDeassignSecurity"
        | "SeReleaseSubjectContext"
        | "SeLockSubjectContext"
        | "SeUnlockSubjectContext"
        | "SeCaptureSubjectContext"
        | "SeCaptureSubjectContextEx"
        | "SeAuditingFileEvents"
        | "SeAuditingFileOrGlobalEvents"
        | "SeImpersonateClientEx"
        | "SeDeleteClientSecurity"
        | "SeSetSecurityDescriptorInfo"
        | "SeSetSecurityDescriptorInfoEx"
        | "SeAssignSecurity"
        | "SeAssignSecurityEx"
        | "SeQueryAuthenticationIdToken"
        | "SeQuerySessionIdToken"
        | "SeFreePrivileges"
        | "SeAppendPrivileges"
        | "SeRegisterLogonSessionTerminatedRoutine"
        | "SeUnregisterLogonSessionTerminatedRoutine"
        | "SeMarkLogonSessionForTerminationNotification"
        | "SeCreateClientSecurity"
        | "SeCreateClientSecurityFromSubjectContext"
        | "SeImpersonateClient"
        | "SeSetAuditParameter"
        | "SeReportSecurityEvent" => return_zero,
        // SeAccessCheck(SD, SubjectContext, Locked, DesiredAccess, Previously, *Privileges,
        //               GenericMapping, Mode, *GrantedAccess, *AccessStatus)
        "SeAccessCheck" | "SeAccessCheckEx" => SeAccessCheck,
        "SeSinglePrivilegeCheck"
        | "SePrivilegeCheck"
        | "SeTokenIsAdmin"
        | "SeTokenIsRestricted"
        | "SeValidSecurityDescriptor"
        | "SeAuditingHardLinkEvents"
        | "SeCheckAuditPrivilege" => return_one,
        "SeTokenIsWriteRestricted"
        | "SeTokenType"
        | "SeTokenImpersonationLevel"
        | "SeFilterToken"
        | "SeQueryInformationToken"
        | "SeQuerySecurityDescriptorInfo" => security_stub,
        // SeConvertStringSecurityDescriptorToSecurityDescriptor(String, Revision, *SD, *Size)
        "SeConvertStringSecurityDescriptorToSecurityDescriptor" => SeConvertStringSecurityDescriptorToSecurityDescriptor,
        "SeCaptureSid" | "SeCopySid" => return_zero,

        // --- Zw --------------------------------------------------------------------------
        "ZwOpenKey"
        | "ZwOpenKeyEx"
        | "ZwCreateKey"
        | "ZwOpenKeyTransacted"
        | "ZwCreateKeyTransacted"
        | "ZwOpenKeyTransactedEx" => registry_key_stub,
        // ZwQueryValueKey(KeyHandle, ValueName, Class, Information, Length, *ResultLength)
        "ZwQueryValueKey"
        | "ZwQueryKey"
        | "ZwEnumerateKey"
        | "ZwEnumerateValueKey"
        | "ZwQueryMultipleValueKey" => registry_query_stub,
        "ZwSetValueKey" => ZwSetValueKey,
        "ZwDeleteKey"
        | "ZwDeleteValueKey"
        | "ZwFlushKey"
        | "ZwNotifyChangeKey"
        | "ZwSetInformationKey"
        | "ZwSetSecurityObject"
        | "ZwLoadDriver"
        | "ZwUnloadDriver"
        | "ZwYieldExecution"
        | "ZwSetInformationFile"
        | "ZwSetInformationThread"
        | "ZwSetInformationProcess"
        | "ZwFlushBuffersFile"
        | "ZwTerminateProcess"
        | "ZwSetEvent"
        | "ZwResetEvent"
        | "ZwWaitForSingleObject"
        | "ZwWaitForMultipleObjects"
        | "ZwUnmapViewOfSection"
        | "ZwFreeVirtualMemory"
        | "ZwProtectVirtualMemory"
        | "ZwSetSystemInformation"
        | "ZwMakeTemporaryObject"
        | "ZwSetTimer"
        | "ZwCancelTimer"
        | "ZwSetVolumeInformationFile"
        | "ZwDeleteFile"
        | "ZwFsControlFile"
        | "ZwDeviceIoControlFile"
        | "ZwLockFile"
        | "ZwUnlockFile"
        | "ZwSetValueKeyEx"
        | "ZwPowerInformation"
        | "ZwTerminateThread"
        | "ZwSuspendThread"
        | "ZwResumeThread"
        | "ZwAlertThread"
        | "ZwDuplicateToken"
        | "ZwAdjustPrivilegesToken"
        | "ZwSetInformationToken"
        | "ZwRequestWaitReplyPort"
        | "ZwConnectPort"
        | "ZwCreateEvent"
        | "ZwSetSystemTime"
        | "ZwQuerySystemTime"
        | "ZwDisplayString"
        | "ZwWriteVirtualMemory"
        | "ZwReadVirtualMemory"
        | "ZwFlushVirtualMemory"
        | "ZwFlushInstructionCache"
        | "ZwLockVirtualMemory"
        | "ZwUnlockVirtualMemory"
        | "ZwRenameKey"
        | "ZwSaveKey"
        | "ZwRestoreKey"
        | "ZwReplaceKey"
        | "ZwQueryObject"
        | "ZwDuplicateObject"
        | "ZwQueryInformationToken"
        | "ZwQueryInformationThread"
        | "ZwQueryInformationProcess"
        | "ZwQueryVolumeInformationFile"
        | "ZwQueryDirectoryObject"
        | "ZwQueryEaFile"
        | "ZwSetEaFile"
        | "ZwQueryFullAttributesFile"
        | "ZwQueryAttributesFile"
        | "ZwQuerySecurityObject"
        | "ZwQueryLicenseValue"
        | "ZwQuerySymbolicLinkObject"
        | "ZwQueryVirtualMemory"
        | "ZwQueryInformationJobObject"
        | "ZwQueryDefaultLocale"
        | "ZwQueryInstallUILanguage"
        | "ZwQueryDefaultUILanguage" => native_service_stub,
        "ZwQueryDirectoryFile" => ZwQueryDirectoryFile,
        // ZwCreateFile(*Handle, Access, ObjAttr, IoStatus, AllocSize, Attrs, Share,
        //              Disposition, Options, EaBuffer, EaLength) / ZwOpenFile(*Handle,
        //              Access, ObjAttr, IoStatus, Share, Options)
        "ZwCreateFile"
        | "ZwOpenFile"
        | "ZwOpenProcess"
        | "ZwOpenThread"
        | "ZwOpenEvent"
        | "ZwOpenDirectoryObject"
        | "ZwOpenSymbolicLinkObject"
        | "ZwCreateDirectoryObject"
        | "ZwOpenProcessTokenEx"
        | "ZwOpenThreadTokenEx"
        | "ZwOpenProcessToken"
        | "ZwOpenTimer"
        | "ZwCreateTimer"
        | "ZwOpenJobObject"
        | "ZwCreateSymbolicLinkObject" => object_open_stub,
        // ZwOpenSection(*Handle, Access, ObjAttr): \Device\PhysicalMemory is the
        // classic physical-memory primitive.
        "ZwOpenSection" | "ZwCreateSection" | "ZwCreateSectionEx" => ZwOpenSection,
        // ZwMapViewOfSection(Section, Process, *Base, ZeroBits, Commit, *Offset,
        //                    *ViewSize, Inherit, AllocType, Protect)
        "ZwMapViewOfSection" => ZwMapViewOfSection,
        // ZwAllocateVirtualMemory(Process, *Base, ZeroBits, *Size, Type, Protect)
        "ZwAllocateVirtualMemory" | "ZwAllocateVirtualMemoryEx" => ZwAllocateVirtualMemory,
        // ZwReadFile(Handle, Event, ApcRoutine, ApcCtx, IoStatus, Buffer, Length, Offset, Key)
        "ZwReadFile" => ZwReadFile,
        "ZwWriteFile" => ZwWriteFile,
        // ZwQueryInformationFile(Handle, IoStatus, Info, Length, Class)
        "ZwQueryInformationFile" => ZwQueryInformationFile,
        // ZwQuerySystemInformation(Class, Buffer, Length, *ReturnLength)
        "ZwQuerySystemInformation" | "NtQuerySystemInformation" | "ZwQuerySystemInformationEx" => ZwQuerySystemInformation,

        // --- Cm ---------------------------------------------------------------------------
        "CmRegisterCallback" | "CmRegisterCallbackEx" => CmRegisterCallback,
        "CmUnRegisterCallback"
        | "CmSetCallbackObjectContext"
        | "CmCallbackReleaseKeyObjectIDEx" => return_zero,
        "CmCallbackGetKeyObjectID" | "CmCallbackGetKeyObjectIDEx" | "CmGetCallbackVersion" => registry_callback_stub,

        // --- Ex callbacks, misc ------------------------------------------------------------
        "ExCreateCallback" => ExCreateCallback,
        "ExRegisterCallback" => ExRegisterCallback,
        "ExUnregisterCallback"
        | "ExNotifyCallback"
        | "ExSetTimerResolution"
        | "ExSystemExceptionFilter"
        | "ExEnumHandleTable"
        | "ExRegisterAttributeInformationCallback"
        | "ExUnregisterAttributeInformationCallback"
        | "ExDestroyHandleTable"
        | "ExSetResourceOwnerPointer"
        | "ExDeleteTimer"
        | "ExCancelTimer"
        | "ExSetTimer"
        | "ExDisableResourceBoostLite"
        | "ExSystemTimeToLocalTime"
        | "ExLocalTimeToSystemTime" => executive_stub,
        "ExGetPreviousMode" => return_one, // UserMode
        "ExUuidCreate" => ExUuidCreate,
        "ExAllocateTimer" => ExAllocateTimer,
        "ExVerifySuite" | "ExIsManufacturingModeEnabled" => system_mode_stub,
        "ExGetFirmwareEnvironmentVariable"
        | "ExSetFirmwareEnvironmentVariable"
        | "ExGetFirmwareType" => firmware_stub,
        "ExCreateHandleTable" | "ExCreateHandle" => return_zero,

        // --- Flt (filter manager) -----------------------------------------------------------
        "FltRegisterFilter" => FltRegisterFilter,
        "FltCreateCommunicationPort" => FltCreateCommunicationPort,
        "FltBuildDefaultSecurityDescriptor" => FltBuildDefaultSecurityDescriptor,
        "FltFreeSecurityDescriptor" => FltFreeSecurityDescriptor,
        "FltAllocateGenericWorkItem" => FltAllocateGenericWorkItem,
        "FltFreeGenericWorkItem" => FltFreeGenericWorkItem,
        // FltQueueGenericWorkItem(WorkItem, FltObject, Routine, QueueType, Context)
        "FltQueueGenericWorkItem" => FltQueueGenericWorkItem,
        "FltGetRequestorProcessId" => process_id_stub,
        "FltGetRequestorProcess" => FltGetRequestorProcess,
        "FltStartFiltering"
        | "FltUnregisterFilter"
        | "FltCloseCommunicationPort"
        | "FltCloseClientPort"
        | "FltSendMessage"
        | "FltReleaseFileNameInformation"
        | "FltObjectReference"
        | "FltObjectDereference"
        | "FltSetCallbackDataDirty"
        | "FltParseFileNameInformation"
        | "FltReleaseContext"
        | "FltDeleteContext"
        | "FltSetInformationFile"
        | "FltFlushBuffers"
        | "FltCancelFileOpen"
        | "FltIsOperationSynchronous"
        | "FltIs32bitProcess"
        | "FltReferenceContext"
        | "FltSetStreamHandleContext"
        | "FltSetFileContext"
        | "FltSetInstanceContext"
        | "FltSetVolumeContext"
        | "FltDeleteStreamHandleContext"
        | "FltDeleteFileContext"
        | "FltRegisterForDataScan"
        | "FltGetFilterFromName"
        | "FltEnumerateFilters"
        | "FltReleaseResource"
        | "FltAcquireResourceExclusive"
        | "FltAcquireResourceShared"
        | "FltCompletePendedPreOperation"
        | "FltCompletePendedPostOperation" => return_zero,
        "FltGetFileNameInformation"
        | "FltGetFileNameInformationUnsafe"
        | "FltGetDestinationFileNameInformation"
        | "FltGetVolumeName"
        | "FltGetVolumeGuidName"
        | "FltQueryInformationFile"
        | "FltGetStreamHandleContext"
        | "FltGetFileContext"
        | "FltGetInstanceContext"
        | "FltGetVolumeContext"
        | "FltAllocateContext"
        | "FltGetVolumeFromInstance"
        | "FltGetDiskDeviceObject"
        | "FltGetDeviceObject"
        | "FltQueryVolumeInformation"
        | "FltReadFile"
        | "FltWriteFile"
        | "FltCreateFile"
        | "FltCreateFileEx"
        | "FltCreateFileEx2"
        | "FltGetVolumeProperties"
        | "FltGetFilterInformation"
        | "FltGetInstanceInformation"
        | "FltAttachVolume"
        | "FltDetachVolume"
        | "FltEnumerateVolumes"
        | "FltEnumerateInstances"
        | "FltGetVolumeFromName"
        | "FltFsControlFile"
        | "FltDeviceIoControlFile"
        | "FltGetTunneledName"
        | "FltGetTopInstance"
        | "FltGetBottomInstance"
        | "FltGetLowerInstance"
        | "FltGetUpperInstance" => filter_manager_stub,
        "FltAllocateCallbackData"
        | "FltAllocatePoolAlignedWithTag"
        | "FltAllocateFileLock"
        | "FltAllocateExtraCreateParameterList" => filter_allocation_stub,
        "FltFreeCallbackData" | "FltFreePoolAlignedWithTag" | "FltFreeFileLock" => FltFreeGenericWorkItem,
        "FltGetRoutineAddress" => FltGetRoutineAddress,

        // --- WDF loader -------------------------------------------------------------------------
        // WdfVersionBind(DriverObject, RegistryPath, BindInfo, *ComponentGlobals):
        // fills the driver's WdfFunctions table. Each slot is a distinct stub
        // so a call through the table is reported by index; the KMDF object
        // model itself (WdfDriverCreate, WdfDeviceCreate, queues) is not
        // modelled, so a KMDF driver gets through DriverEntry with every
        // framework call succeeding and no EvtDeviceAdd callback.
        "WdfVersionBind" => WdfVersionBind,
        "WdfVersionBindClass" | "WdfVersionUnbind" | "WdfVersionUnbindClass" => wdf_version_stub,
        "WdfLdrQueryInterface" => filter_manager_stub,

        // --- ETW / WMI ------------------------------------------------------------------------
        "EtwRegister" | "EtwRegisterClassicProvider" => EtwRegister,
        "EtwUnregister"
        | "EtwWrite"
        | "EtwWriteEx"
        | "EtwWriteTransfer"
        | "EtwWriteString"
        | "EtwSetInformation"
        | "EtwSendTraceBuffer"
        | "EtwActivityIdControl"
        | "EtwWriteStartScenario"
        | "EtwWriteEndScenario"
        | "WmiQueryTraceInformation"
        | "WmiTraceMessage"
        | "WmiTraceMessageVa"
        | "WmiSystemControl"
        | "WmiFlushTrace"
        | "IoWMIQueryAllDataMultiple"
        | "EtwEnableTrace"
        | "EtwUnregisterClassicProvider" => return_zero,
        "EtwProviderEnabled" | "EtwEventEnabled" => return_zero,

        // --- Po -----------------------------------------------------------------------------------
        "PoStartNextPowerIrp"
        | "PoUnregisterPowerSettingCallback"
        | "PoSetSystemState"
        | "PoRegisterSystemState"
        | "PoUnregisterSystemState"
        | "PoSetDeviceBusyEx"
        | "PoStartDeviceBusy"
        | "PoEndDeviceBusy"
        | "PoRegisterDeviceForIdleDetection"
        | "PoFxRegisterDevice"
        | "PoFxUnregisterDevice"
        | "PoFxStartDevicePowerManagement"
        | "PoFxActivateComponent"
        | "PoFxIdleComponent"
        | "PoFxSetComponentLatency"
        | "PoFxSetComponentResidency"
        | "PoFxReportDevicePoweredOn"
        | "PoFxCompleteDevicePowerNotRequired"
        | "PoFxCompleteIdleCondition"
        | "PoFxCompleteIdleState"
        | "PoFxSetDeviceIdleTimeout"
        | "PoQueryWatchdogTime"
        | "PoGetSystemWake"
        | "PoSetSystemWake"
        | "PoClearPowerRequest"
        | "PoDeletePowerRequest"
        | "PoSetPowerRequest" => return_zero,
        "PoRegisterPowerSettingCallback" => PoRegisterPowerSettingCallback,
        "PoSetPowerState" => PoSetPowerState,
        "PoRequestPowerIrp" => return_pending,
        "PoCreatePowerRequest" => PoCreatePowerRequest,
        "KdDisableDebugger"
        | "KdEnableDebugger"
        | "KdRefreshDebuggerNotPresent"
        | "KdChangeOption"
        | "KdSystemDebugControl"
        | "KdRegisterPowerHandler"
        | "KdDeregisterPowerHandler"
        | "KdPowerTransition" => debugger_stub,
        "NtBuildNumber" => NtBuildNumber,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn process_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let p = system_process(emu);
    call.set_return(
        emu,
        if symbol == "PsGetProcessInheritedFromUniqueProcessId" {
            0
        } else {
            p
        },
    );
}

fn process_id_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, 4);
}

fn thread_id_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, 8);
}

fn PsGetProcessImageFileName(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    call.set_return(emu, p + EPROCESS_IMAGE_FILE_NAME);
}

fn return_pending(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, STATUS_PENDING);
}

fn PsLookupProcessByProcessId(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let id = api_try!(emu, call.arg(emu, 0));
    let out = if symbol == "PsLookupProcessThreadByCid" {
        api_try!(emu, call.arg(emu, 2))
    } else {
        api_try!(emu, call.arg(emu, 1))
    };
    let (p, t) = emu
        .kernel
        .as_ref()
        .map(|k| (k.win.system_process, k.win.current_thread))
        .unwrap_or((0, 0));
    let obj = if symbol == "PsLookupThreadByThreadId" {
        t
    } else {
        p
    };
    log::info!("{}: id {} -> 0x{:x}", symbol, id, obj);
    write_out_qword(emu, out, obj);
    if symbol == "PsLookupProcessThreadByCid" {
        let proc_out = api_try!(emu, call.arg(emu, 1));
        write_out_qword(emu, proc_out, p);
    }
    call.set_return(emu, STATUS_SUCCESS);
}

fn PsSetCreateProcessNotifyRoutine(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let func = api_try!(emu, call.arg(emu, 0));
    let remove =
        api_try!(emu, call.arg(emu, 1)) != 0 && symbol == "PsSetCreateProcessNotifyRoutine";
    if remove {
        if let Some(k) = emu.kernel.as_mut() {
            k.win.callbacks.retain(|c| c.func != func);
        }
    } else {
        register_callback(emu, symbol, func, 0);
    }
    call.set_return(emu, STATUS_SUCCESS);
}

fn PsRemoveCreateThreadNotifyRoutine(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let func = api_try!(emu, call.arg(emu, 0));
    if let Some(k) = emu.kernel.as_mut() {
        k.win.callbacks.retain(|c| c.func != func);
    }
    call.set_return(emu, STATUS_SUCCESS);
}

fn PsCreateSystemThread(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let routine = api_try!(emu, call.arg(emu, 5));
    let ctx = api_try!(emu, call.arg(emu, 6));
    let h = new_handle(emu, format!("thread 0x{:x}", routine));
    write_out_qword(emu, out, h);
    emu.kernel_defer(DeferredCall {
        kind: "system_thread".to_string(),
        func: routine,
        args: vec![ctx],
    });
    call.set_return(emu, STATUS_SUCCESS);
}

fn PsTerminateSystemThread(emu: &mut Emu, _symbol: &str) {
    log::info!("PsTerminateSystemThread: the system thread finished");
    emu.stop();
}

fn PsGetVersion(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let (a, b, c, d) = (
        api_try!(emu, call.arg(emu, 0)),
        api_try!(emu, call.arg(emu, 1)),
        api_try!(emu, call.arg(emu, 2)),
        api_try!(emu, call.arg(emu, 3)),
    );
    write_out_dword(emu, a, OS_MAJOR);
    write_out_dword(emu, b, OS_MINOR);
    write_out_dword(emu, c, NT_BUILD_NUMBER as u32);
    if d != 0 {
        write_unicode_header(emu, d, 0, 0, 0);
    }
    call.set_return(emu, 0); // not a checked build
}

fn token_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let t = emu.kernel_data_for("win.system_token", 0x100).unwrap_or(0);
    call.set_return(emu, t);
}

fn process_image_name_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 1));
    // A pool-allocated UNICODE_STRING the caller frees with ExFreePool.
    let us = emu.kernel_alloc(
        Region::Slab,
        UNICODE_STRING_SIZE + 0x40,
        "image-name",
        symbol,
        true,
    );
    if us != 0 {
        let buf = us + UNICODE_STRING_SIZE;
        emu.maps
            .write_wide_string(buf, "\\SystemRoot\\System32\\ntoskrnl.exe");
        write_unicode_header(emu, us, buf, 34 * 2, 0x40);
    }
    if symbol == "SeLocateProcessImageName" {
        write_out_qword(emu, out, us);
        call.set_return(emu, STATUS_SUCCESS);
    } else {
        call.set_return(emu, us);
    }
}

fn ObReferenceObjectByHandle(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let h = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, call.arg(emu, 4));
    let what = emu
        .kernel
        .as_ref()
        .and_then(|k| k.win.handles.get(&h).cloned())
        .unwrap_or_default();
    // A handle to an object this kernel created resolves to that
    // object; anything else stands for the System process.
    let obj = what
        .split_whitespace()
        .last()
        .and_then(|s| s.strip_prefix("0x"))
        .and_then(|s| u64::from_str_radix(s, 16).ok())
        .filter(|a| emu.maps.is_mapped(*a))
        .unwrap_or_else(|| system_process(emu));
    log::info!("{}: handle 0x{:x} ({}) -> 0x{:x}", symbol, h, what, obj);
    write_out_qword(emu, out, obj);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ObOpenObjectByPointer(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let obj = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, call.arg(emu, 6));
    let h = new_handle(emu, format!("object 0x{:x}", obj));
    write_out_qword(emu, out, h);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ObOpenObjectByName(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let name = object_name_of(emu, api_try!(emu, call.arg(emu, 0)));
    let out = api_try!(emu, call.arg(emu, 6));
    log::info!("ObOpenObjectByName: '{}'", name);
    let h = new_handle(emu, format!("object {}", name));
    write_out_qword(emu, out, h);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ObReferenceObjectByName(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let name = read_unicode_string(emu, api_try!(emu, call.arg(emu, 0)));
    let out = api_try!(emu, call.arg(emu, 7));
    let obj = emu
        .kernel
        .as_ref()
        .map(|k| {
            k.win
                .devices
                .iter()
                .find(|d| d.name.eq_ignore_ascii_case(&name))
                .map(|d| d.addr)
                .unwrap_or(k.win.driver_object)
        })
        .unwrap_or(0);
    log::info!("ObReferenceObjectByName: '{}' -> 0x{:x}", name, obj);
    write_out_qword(emu, out, obj);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ObQueryNameString(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    // (Object, *OBJECT_NAME_INFORMATION, Length, *ReturnLength)
    let obj = api_try!(emu, call.arg(emu, 0));
    let info = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let ret = api_try!(emu, call.arg(emu, 3));
    let name = emu
        .kernel
        .as_ref()
        .and_then(|k| {
            k.win
                .devices
                .iter()
                .find(|d| d.addr == obj)
                .map(|d| d.name.clone())
        })
        .unwrap_or_default();
    let need = UNICODE_STRING_SIZE + (name.encode_utf16().count() as u64 + 1) * 2;
    write_out_dword(emu, ret, need as u32);
    if info == 0 || len < need {
        call.set_return(emu, STATUS_INFO_LENGTH_MISMATCH);
    } else {
        let buf = info + UNICODE_STRING_SIZE;
        emu.maps.write_wide_string(buf, &name);
        write_unicode_header(
            emu,
            info,
            buf,
            need - UNICODE_STRING_SIZE - 2,
            need - UNICODE_STRING_SIZE,
        );
        call.set_return(emu, STATUS_SUCCESS);
    }
}

fn ObGetObjectSecurity(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 1));
    write_out_qword(emu, out, 0);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ObRegisterCallbacks(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 1));
    let h = new_handle(emu, "ObRegisterCallbacks".to_string());
    register_callback(emu, symbol, api_try!(emu, call.arg(emu, 0)), 0);
    write_out_qword(emu, out, h);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ObGetFilterVersion(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, 0x100);
}

fn ObCloseHandle(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let h = api_try!(emu, call.arg(emu, 0));
    if let Some(k) = emu.kernel.as_mut() {
        k.win.handles.remove(&h);
    }
    call.set_return(emu, STATUS_SUCCESS);
}

fn ObIsKernelHandle(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let h = api_try!(emu, call.arg(emu, 0));
    call.set_return(emu, h >> 63);
}

fn SeCaptureSecurityDescriptor(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, call.arg(emu, 4));
    write_out_qword(emu, out, input);
    call.set_return(emu, STATUS_SUCCESS);
}

fn SeAccessCheck(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let desired = api_try!(emu, call.arg_u32(emu, 3));
    let granted = api_try!(emu, call.arg(emu, 8));
    let status = api_try!(emu, call.arg(emu, 9));
    write_out_dword(emu, granted, desired);
    write_out_dword(emu, status, 0);
    call.set_return(emu, 1);
}

fn security_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(
        emu,
        if symbol.starts_with("SeQuery") || symbol == "SeFilterToken" {
            STATUS_NOT_SUPPORTED
        } else {
            0
        },
    );
}

fn SeConvertStringSecurityDescriptorToSecurityDescriptor(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let s = emu.maps.read_wide_string(api_try!(emu, call.arg(emu, 0)));
    let out = api_try!(emu, call.arg(emu, 2));
    let size_out = api_try!(emu, call.arg(emu, 3));
    log::info!("{}: '{}'", symbol, s);
    let sd = emu.kernel_alloc(Region::Slab, 0x40, "SECURITY_DESCRIPTOR", symbol, true);
    if sd != 0 {
        emu.maps.write_byte(sd, 1);
        emu.maps.write_word(sd + 2, SE_SELF_RELATIVE);
    }
    write_out_qword(emu, out, sd);
    write_out_dword(emu, size_out, 0x40);
    call.set_return(emu, STATUS_SUCCESS);
}

fn registry_key_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let name = object_name_of(emu, api_try!(emu, call.arg(emu, 2)));
    let disposition = if symbol.starts_with("ZwCreateKey") {
        Some(api_try!(emu, call.arg(emu, 6)))
    } else {
        None
    };
    log::info!("{}: '{}'", symbol, name);
    let h = new_handle(emu, format!("key {}", name));
    write_out_qword(emu, out, h);
    if let Some(disposition) = disposition {
        write_out_dword(emu, disposition, REG_CREATED_NEW_KEY);
    }
    call.set_return(emu, STATUS_SUCCESS);
}

fn registry_query_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let ret = api_try!(emu, call.arg(emu, 5));
    if symbol == "ZwQueryValueKey" {
        let name = read_unicode_string(emu, api_try!(emu, call.arg(emu, 1)));
        log::info!("ZwQueryValueKey: '{}' -> not found", name);
    }
    write_out_dword(emu, ret, 0);
    call.set_return(
        emu,
        match symbol {
            "ZwQueryValueKey" => STATUS_OBJECT_NAME_NOT_FOUND,
            "ZwEnumerateKey" | "ZwEnumerateValueKey" => STATUS_NO_MORE_ENTRIES,
            _ => STATUS_NOT_SUPPORTED,
        },
    );
}

fn ZwSetValueKey(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let name = read_unicode_string(emu, api_try!(emu, call.arg(emu, 1)));
    log::info!("ZwSetValueKey: '{}'", name);
    call.set_return(emu, STATUS_SUCCESS);
}

fn native_service_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let st = if symbol.starts_with("ZwQuery") && symbol != "ZwQuerySystemTime" {
        STATUS_NOT_SUPPORTED
    } else {
        STATUS_SUCCESS
    };
    if symbol == "ZwQuerySystemTime" {
        let out = api_try!(emu, call.arg(emu, 0));
        write_out_qword(emu, out, system_time_now());
    }
    call.set_return(emu, st);
}

fn ZwQueryDirectoryFile(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, STATUS_NO_MORE_FILES);
}

fn object_open_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let attr_idx = match symbol {
        "ZwOpenProcessTokenEx" | "ZwOpenProcessToken" | "ZwOpenThreadTokenEx" => 99,
        _ => 2,
    };
    let name = if attr_idx == 99 {
        String::new()
    } else {
        object_name_of(emu, api_try!(emu, call.arg(emu, attr_idx)))
    };
    log::info!("{}: '{}'", symbol, name);
    emu.kernel_log_line(format!("{}({})", symbol, name));
    let h = new_handle(emu, format!("{} {}", symbol.trim_start_matches("Zw"), name));
    write_out_qword(emu, out, h);
    if symbol == "ZwCreateFile" || symbol == "ZwOpenFile" {
        let iosb = api_try!(emu, call.arg(emu, 3));
        write_iosb(emu, iosb, 0, 1); // FILE_OPENED
    }
    call.set_return(emu, STATUS_SUCCESS);
}

fn ZwOpenSection(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let name = object_name_of(emu, api_try!(emu, call.arg(emu, 2)));
    log::info!("{}: '{}'", symbol, name);
    if name.to_ascii_lowercase().contains("physicalmemory") {
        emu.kernel_privop("physical_memory_section", 0, 0, 0);
    }
    let h = new_handle(emu, format!("section {}", name));
    write_out_qword(emu, out, h);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ZwMapViewOfSection(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let h = api_try!(emu, call.arg(emu, 0));
    let base_ptr = api_try!(emu, call.arg(emu, 2));
    let offset_ptr = api_try!(emu, call.arg(emu, 5));
    let size_ptr = api_try!(emu, call.arg(emu, 6));
    let offset = if offset_ptr != 0 {
        emu.maps.read_qword(offset_ptr).unwrap_or(0)
    } else {
        0
    };
    let size = emu.maps.read_qword(size_ptr).unwrap_or(0x1000).max(0x1000);
    let what = emu
        .kernel
        .as_ref()
        .and_then(|k| k.win.handles.get(&h).cloned())
        .unwrap_or_default();
    let va = emu.kernel_alloc(Region::Pages, size, "section-view", symbol, true);
    if what.to_ascii_lowercase().contains("physicalmemory") {
        emu.kernel_privop("map_io_space", offset, va, size);
        if let Some(k) = emu.kernel.as_mut() {
            k.win.io_mappings.insert(va, (offset, size));
        }
    }
    write_out_qword(emu, base_ptr, va);
    emu.maps.write_qword(size_ptr, size);
    call.set_return(
        emu,
        if va != 0 {
            STATUS_SUCCESS
        } else {
            STATUS_NO_MEMORY
        },
    );
}

fn ZwAllocateVirtualMemory(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let base_ptr = api_try!(emu, call.arg(emu, 1));
    let size_ptr = if symbol == "ZwAllocateVirtualMemory" {
        api_try!(emu, call.arg(emu, 3))
    } else {
        api_try!(emu, call.arg(emu, 2))
    };
    let size = emu
        .maps
        .read_qword(size_ptr)
        .unwrap_or(0x1000)
        .max(0x1000)
        .next_multiple_of(0x1000);
    let va = alloc_user_buffer(emu, size, "zwalloc");
    write_out_qword(emu, base_ptr, va);
    emu.maps.write_qword(size_ptr, size);
    call.set_return(
        emu,
        if va != 0 {
            STATUS_SUCCESS
        } else {
            STATUS_NO_MEMORY
        },
    );
}

fn ZwReadFile(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let iosb = api_try!(emu, call.arg(emu, 4));
    write_iosb(emu, iosb, STATUS_END_OF_FILE, 0);
    call.set_return(emu, STATUS_END_OF_FILE);
}

fn ZwWriteFile(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let iosb = api_try!(emu, call.arg(emu, 4));
    let len = api_try!(emu, call.arg(emu, 6));
    write_iosb(emu, iosb, 0, len);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ZwQueryInformationFile(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let iosb = api_try!(emu, call.arg(emu, 1));
    let info = api_try!(emu, call.arg(emu, 2));
    let len = api_try!(emu, call.arg(emu, 3));
    if info != 0 && len != 0 {
        emu.maps
            .write_bytes(info, &vec![0u8; len.min(0x1000) as usize]);
    }
    write_iosb(emu, iosb, 0, len);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ZwQuerySystemInformation(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let class = api_try!(emu, call.arg(emu, 0));
    let (buf, len, ret) = if symbol == "ZwQuerySystemInformationEx" {
        (
            api_try!(emu, call.arg(emu, 3)),
            api_try!(emu, call.arg(emu, 4)),
            api_try!(emu, call.arg(emu, 5)),
        )
    } else {
        (
            api_try!(emu, call.arg(emu, 1)),
            api_try!(emu, call.arg(emu, 2)),
            api_try!(emu, call.arg(emu, 3)),
        )
    };
    emu.kernel_log_line(format!("{}(class {})", symbol, class));
    let st = match class {
        11 => system_module_information(emu, buf, len, ret), // SystemModuleInformation
        _ => {
            write_out_dword(emu, ret, 0);
            STATUS_INVALID_INFO_CLASS
        }
    };
    call.set_return(emu, st);
}

fn CmRegisterCallback(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let func = api_try!(emu, call.arg(emu, 0));
    let (ctx, cookie) = if symbol == "CmRegisterCallback" {
        (
            api_try!(emu, call.arg(emu, 1)),
            api_try!(emu, call.arg(emu, 2)),
        )
    } else {
        (
            api_try!(emu, call.arg(emu, 3)),
            api_try!(emu, call.arg(emu, 4)),
        )
    };
    register_callback(emu, symbol, func, ctx);
    let h = new_handle(emu, "registry callback".to_string());
    write_out_qword(emu, cookie, h);
    call.set_return(emu, STATUS_SUCCESS);
}

fn registry_callback_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(
        emu,
        if symbol == "CmGetCallbackVersion" {
            0x10000
        } else {
            STATUS_NOT_SUPPORTED
        },
    );
}

fn ExCreateCallback(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let name = object_name_of(emu, api_try!(emu, call.arg(emu, 1)));
    let obj = emu.kernel_alloc(Region::Slab, 0x40, "CALLBACK_OBJECT", symbol, true);
    log::info!("ExCreateCallback: '{}' -> 0x{:x}", name, obj);
    write_out_qword(emu, out, obj);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ExRegisterCallback(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let func = api_try!(emu, call.arg(emu, 1));
    let ctx = api_try!(emu, call.arg(emu, 2));
    register_callback(emu, symbol, func, ctx);
    let h = new_handle(emu, "ExRegisterCallback".to_string());
    call.set_return(emu, h);
}

fn executive_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    if symbol == "ExSystemTimeToLocalTime" || symbol == "ExLocalTimeToSystemTime" {
        let src = api_try!(emu, call.arg(emu, 0));
        let dst = api_try!(emu, call.arg(emu, 1));
        let t = emu.maps.read_qword(src).unwrap_or(0);
        write_out_qword(emu, dst, t);
    }
    call.set_return(emu, 0);
}

fn ExUuidCreate(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let seed = emu.pos.wrapping_mul(0x9E3779B97F4A7C15);
    emu.maps.write_qword(out, seed);
    emu.maps
        .write_qword(out + 8, seed.rotate_left(32) ^ 0x4000_0000_0000_8000);
    call.set_return(emu, STATUS_SUCCESS);
}

fn ExAllocateTimer(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let t = emu.kernel_alloc(Region::Slab, 0x80, "EX_TIMER", symbol, true);
    call.set_return(emu, t);
}

fn system_mode_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, (symbol == "ExVerifySuite") as u64);
}

fn firmware_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(
        emu,
        if symbol == "ExGetFirmwareType" {
            2
        } else {
            STATUS_NOT_SUPPORTED
        },
    );
}

fn FltRegisterFilter(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 2));
    let h = new_handle(emu, "filter".to_string());
    log::info!("FltRegisterFilter -> 0x{:x}", h);
    write_out_qword(emu, out, h);
    call.set_return(emu, STATUS_SUCCESS);
}

fn FltCreateCommunicationPort(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    // (Filter, *ServerPort, ObjAttr, Cookie, Connect, Disconnect, Message, MaxConnections)
    let out = api_try!(emu, call.arg(emu, 1));
    let name = object_name_of(emu, api_try!(emu, call.arg(emu, 2)));
    let connect = api_try!(emu, call.arg(emu, 4));
    let message = api_try!(emu, call.arg(emu, 6));
    register_callback(emu, "FltCreateCommunicationPort.Connect", connect, 0);
    register_callback(emu, "FltCreateCommunicationPort.Message", message, 0);
    log::info!("FltCreateCommunicationPort: '{}'", name);
    let h = new_handle(emu, format!("port {}", name));
    write_out_qword(emu, out, h);
    call.set_return(emu, STATUS_SUCCESS);
}

fn FltBuildDefaultSecurityDescriptor(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let sd = emu.kernel_alloc(Region::Slab, 0x40, "SECURITY_DESCRIPTOR", symbol, true);
    if sd != 0 {
        emu.maps.write_byte(sd, 1);
        emu.maps.write_word(sd + 2, SE_SELF_RELATIVE);
    }
    write_out_qword(emu, out, sd);
    call.set_return(emu, STATUS_SUCCESS);
}

fn FltFreeSecurityDescriptor(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let sd = api_try!(emu, call.arg(emu, 0));
    emu.kernel_free(sd, symbol);
    call.set_return(emu, 0);
}

fn FltAllocateGenericWorkItem(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let p = emu.kernel_alloc(Region::Slab, 0x40, "FLT_GENERIC_WORKITEM", symbol, true);
    call.set_return(emu, p);
}

fn FltFreeGenericWorkItem(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    emu.kernel_free(p, symbol);
    call.set_return(emu, 0);
}

fn FltQueueGenericWorkItem(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let item = api_try!(emu, call.arg(emu, 0));
    let obj = api_try!(emu, call.arg(emu, 1));
    let routine = api_try!(emu, call.arg(emu, 2));
    let ctx = api_try!(emu, call.arg(emu, 4));
    emu.kernel_defer(DeferredCall {
        kind: "flt_workitem".to_string(),
        func: routine,
        args: vec![item, obj, ctx],
    });
    call.set_return(emu, STATUS_SUCCESS);
}

fn FltGetRequestorProcess(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = system_process(emu);
    call.set_return(emu, p);
}

fn filter_manager_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, STATUS_NOT_SUPPORTED);
}

fn filter_allocation_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let size = if symbol == "FltAllocatePoolAlignedWithTag" {
        api_try!(emu, call.arg(emu, 2))
    } else {
        0x100
    };
    let p = emu.kernel_alloc(Region::Slab, size.max(1), "flt", symbol, true);
    if symbol == "FltAllocateCallbackData" {
        let out = api_try!(emu, call.arg(emu, 2));
        write_out_qword(emu, out, p);
        call.set_return(emu, STATUS_SUCCESS);
    } else {
        call.set_return(emu, p);
    }
}

fn FltGetRoutineAddress(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let name = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let addr = emu.kernel_resolve_import(&name).unwrap_or(0);
    call.set_return(emu, addr);
}

fn WdfVersionBind(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let bind = api_try!(emu, call.arg(emu, 2));
    let globals_out = api_try!(emu, call.arg(emu, 3));
    let count = emu
        .maps
        .read_dword(bind + WDF_BIND_INFO_FUNC_COUNT)
        .unwrap_or(0);
    let table = emu
        .maps
        .read_qword(bind + WDF_BIND_INFO_FUNC_TABLE)
        .unwrap_or(0);
    let major = emu.maps.read_dword(bind + 0x10).unwrap_or(0);
    let minor = emu.maps.read_dword(bind + 0x14).unwrap_or(0);
    log::info!(
        "WdfVersionBind: KMDF {}.{} with {} functions (table 0x{:x})",
        major,
        minor,
        count,
        table
    );
    emu.kernel_log_line(format!(
        "WdfVersionBind KMDF {}.{} ({} functions)",
        major, minor, count
    ));
    if table != 0 {
        for i in 0..count.min(1024) {
            let stub = emu
                .kernel_stub_for(&format!("WdfFunctions[{}]", i))
                .unwrap_or(0);
            emu.maps.write_qword(table + i as u64 * 8, stub);
        }
    }
    let globals = emu.kernel_data_for("win.wdf_globals", 0x200).unwrap_or(0);
    write_out_qword(emu, globals_out, globals);
    call.set_return(emu, STATUS_SUCCESS);
}

fn wdf_version_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, STATUS_SUCCESS);
}

fn EtwRegister(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 3));
    let h = new_handle(emu, "etw provider".to_string());
    write_out_qword(emu, out, h);
    call.set_return(emu, STATUS_SUCCESS);
}

fn PoRegisterPowerSettingCallback(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let func = api_try!(emu, call.arg(emu, 2));
    let ctx = api_try!(emu, call.arg(emu, 3));
    let out = api_try!(emu, call.arg(emu, 4));
    register_callback(emu, symbol, func, ctx);
    let h = new_handle(emu, "power setting callback".to_string());
    write_out_qword(emu, out, h);
    call.set_return(emu, STATUS_SUCCESS);
}

fn PoSetPowerState(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let state = api_try!(emu, call.arg(emu, 2));
    call.set_return(emu, state);
}

fn PoCreatePowerRequest(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let p = emu.kernel_alloc(Region::Slab, 0x40, "POWER_REQUEST", symbol, true);
    write_out_qword(emu, out, p);
    call.set_return(emu, STATUS_SUCCESS);
}

fn debugger_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(
        emu,
        if symbol == "KdRefreshDebuggerNotPresent" {
            1
        } else {
            0
        },
    );
}

fn NtBuildNumber(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, NT_BUILD_NUMBER as u64);
}
