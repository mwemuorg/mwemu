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
    match symbol {
        // --- Ps ------------------------------------------------------------------------
        "PsGetCurrentProcess"
        | "PsGetThreadProcess"
        | "PsGetCurrentThreadProcess"
        | "PsGetProcessInheritedFromUniqueProcessId" => {
            let p = system_process(emu);
            emu.set_kernel_ret(if symbol == "PsGetProcessInheritedFromUniqueProcessId" {
                0
            } else {
                p
            });
        }
        "PsGetCurrentProcessId"
        | "PsGetProcessId"
        | "PsGetThreadProcessId"
        | "PsGetCurrentThreadProcessId"
        | "PsGetCurrentProcessSessionId" => emu.set_kernel_ret(4),
        "PsGetCurrentThreadId" | "PsGetThreadId" => emu.set_kernel_ret(8),
        "PsGetProcessImageFileName" => {
            let p = emu.kernel_arg(0);
            emu.set_kernel_ret(p + EPROCESS_IMAGE_FILE_NAME);
        }
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
        | "PsGetProcessProtection" => {
            emu.set_kernel_ret(0);
        }
        "PsGetProcessExitStatus" => emu.set_kernel_ret(STATUS_PENDING),
        "PsGetProcessExitProcessCalled"
        | "PsIsThreadTerminating"
        | "PsIsProcessBeingDebugged"
        | "PsIsProtectedProcess"
        | "PsIsProtectedProcessLight"
        | "PsIsSystemProcess"
        | "PsGetProcessWow64ProcessEx" => {
            emu.set_kernel_ret(0);
        }
        "PsIsSystemThread" => emu.set_kernel_ret(1),
        "PsLookupProcessByProcessId"
        | "PsLookupThreadByThreadId"
        | "PsLookupProcessThreadByCid" => {
            let id = emu.kernel_arg(0);
            let out = if symbol == "PsLookupProcessThreadByCid" {
                emu.kernel_arg(2)
            } else {
                emu.kernel_arg(1)
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
                let proc_out = emu.kernel_arg(1);
                write_out_qword(emu, proc_out, p);
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "PsSetCreateProcessNotifyRoutine"
        | "PsSetCreateProcessNotifyRoutineEx"
        | "PsSetCreateProcessNotifyRoutineEx2"
        | "PsSetCreateThreadNotifyRoutine"
        | "PsSetCreateThreadNotifyRoutineEx"
        | "PsSetLoadImageNotifyRoutine"
        | "PsSetLoadImageNotifyRoutineEx" => {
            let func = emu.kernel_arg(0);
            let remove = emu.kernel_arg(1) != 0 && symbol == "PsSetCreateProcessNotifyRoutine";
            if remove {
                if let Some(k) = emu.kernel.as_mut() {
                    k.win.callbacks.retain(|c| c.func != func);
                }
            } else {
                register_callback(emu, symbol, func, 0);
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "PsRemoveCreateThreadNotifyRoutine" | "PsRemoveLoadImageNotifyRoutine" => {
            let func = emu.kernel_arg(0);
            if let Some(k) = emu.kernel.as_mut() {
                k.win.callbacks.retain(|c| c.func != func);
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        // PsCreateSystemThread(*Handle, Access, ObjAttr, Process, ClientId, StartRoutine, StartContext)
        "PsCreateSystemThread" | "PsCreateSystemThreadEx" => {
            let out = emu.kernel_arg(0);
            let routine = emu.kernel_arg(5);
            let ctx = emu.kernel_arg(6);
            let h = new_handle(emu, format!("thread 0x{:x}", routine));
            write_out_qword(emu, out, h);
            emu.kernel_defer(DeferredCall {
                kind: "system_thread".to_string(),
                func: routine,
                args: vec![ctx],
            });
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "PsTerminateSystemThread" => {
            log::info!("PsTerminateSystemThread: the system thread finished");
            emu.stop();
        }
        "PsGetVersion" => {
            let (a, b, c, d) = (
                emu.kernel_arg(0),
                emu.kernel_arg(1),
                emu.kernel_arg(2),
                emu.kernel_arg(3),
            );
            write_out_dword(emu, a, OS_MAJOR);
            write_out_dword(emu, b, OS_MINOR);
            write_out_dword(emu, c, NT_BUILD_NUMBER as u32);
            if d != 0 {
                write_unicode_header(emu, d, 0, 0, 0);
            }
            emu.set_kernel_ret(0); // not a checked build
        }
        "PsReferencePrimaryToken" | "PsReferenceImpersonationToken" => {
            let t = emu.kernel_data_for("win.system_token", 0x100).unwrap_or(0);
            emu.set_kernel_ret(t);
        }
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
        | "PsWrapApcWow64Thread" => {
            emu.set_kernel_ret(0);
        }
        "PsGetProcessImageFileNameEx"
        | "PsReferenceProcessImageFileName"
        | "SeLocateProcessImageName" => {
            let out = emu.kernel_arg(1);
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
                emu.set_kernel_ret(STATUS_SUCCESS);
            } else {
                emu.set_kernel_ret(us);
            }
        }

        // --- Ob ------------------------------------------------------------------------
        // ObReferenceObjectByHandle(Handle, Access, Type, Mode, *Object, *HandleInfo)
        "ObReferenceObjectByHandle" | "ObReferenceObjectByHandleWithTag" => {
            let h = emu.kernel_arg(0);
            let out = emu.kernel_arg(4);
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
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
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
        | "ObAssignSecurity" => {
            emu.set_kernel_ret(0);
        }
        "ObOpenObjectByPointer" | "ObOpenObjectByPointerWithTag" => {
            let obj = emu.kernel_arg(0);
            let out = emu.kernel_arg(6);
            let h = new_handle(emu, format!("object 0x{:x}", obj));
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "ObOpenObjectByName" => {
            let name = object_name_of(emu, emu.kernel_arg(0));
            let out = emu.kernel_arg(6);
            log::info!("ObOpenObjectByName: '{}'", name);
            let h = new_handle(emu, format!("object {}", name));
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        // ObReferenceObjectByName(Name, Attributes, PassedAccessState, Access, Type,
        //                         Mode, ParseContext, *Object)
        "ObReferenceObjectByName" => {
            let name = read_unicode_string(emu, emu.kernel_arg(0));
            let out = emu.kernel_arg(7);
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
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "ObQueryNameString" => {
            // (Object, *OBJECT_NAME_INFORMATION, Length, *ReturnLength)
            let obj = emu.kernel_arg(0);
            let info = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let ret = emu.kernel_arg(3);
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
                emu.set_kernel_ret(STATUS_INFO_LENGTH_MISMATCH);
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
                emu.set_kernel_ret(STATUS_SUCCESS);
            }
        }
        "ObGetObjectSecurity" => {
            let out = emu.kernel_arg(1);
            write_out_qword(emu, out, 0);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "ObRegisterCallbacks" => {
            let out = emu.kernel_arg(1);
            let h = new_handle(emu, "ObRegisterCallbacks".to_string());
            register_callback(emu, symbol, emu.kernel_arg(0), 0);
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "ObGetFilterVersion" => emu.set_kernel_ret(0x100),
        "ObCloseHandle" | "ZwClose" | "NtClose" => {
            let h = emu.kernel_arg(0);
            if let Some(k) = emu.kernel.as_mut() {
                k.win.handles.remove(&h);
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "ObIsKernelHandle" => {
            let h = emu.kernel_arg(0);
            emu.set_kernel_ret(h >> 63);
        }

        // --- Se ------------------------------------------------------------------------
        "SeCaptureSecurityDescriptor" => {
            let input = emu.kernel_arg(0);
            let out = emu.kernel_arg(4);
            write_out_qword(emu, out, input);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
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
        | "SeReportSecurityEvent" => {
            emu.set_kernel_ret(0);
        }
        // SeAccessCheck(SD, SubjectContext, Locked, DesiredAccess, Previously, *Privileges,
        //               GenericMapping, Mode, *GrantedAccess, *AccessStatus)
        "SeAccessCheck" | "SeAccessCheckEx" => {
            let desired = emu.kernel_arg(3) as u32;
            let granted = emu.kernel_arg(8);
            let status = emu.kernel_arg(9);
            write_out_dword(emu, granted, desired);
            write_out_dword(emu, status, 0);
            emu.set_kernel_ret(1);
        }
        "SeSinglePrivilegeCheck"
        | "SePrivilegeCheck"
        | "SeTokenIsAdmin"
        | "SeTokenIsRestricted"
        | "SeValidSecurityDescriptor"
        | "SeAuditingHardLinkEvents"
        | "SeCheckAuditPrivilege" => {
            emu.set_kernel_ret(1);
        }
        "SeTokenIsWriteRestricted"
        | "SeTokenType"
        | "SeTokenImpersonationLevel"
        | "SeFilterToken"
        | "SeQueryInformationToken"
        | "SeQuerySecurityDescriptorInfo" => {
            emu.set_kernel_ret(
                if symbol.starts_with("SeQuery") || symbol == "SeFilterToken" {
                    STATUS_NOT_SUPPORTED
                } else {
                    0
                },
            );
        }
        // SeConvertStringSecurityDescriptorToSecurityDescriptor(String, Revision, *SD, *Size)
        "SeConvertStringSecurityDescriptorToSecurityDescriptor" => {
            let s = emu.maps.read_wide_string(emu.kernel_arg(0));
            let out = emu.kernel_arg(2);
            let size_out = emu.kernel_arg(3);
            log::info!("{}: '{}'", symbol, s);
            let sd = emu.kernel_alloc(Region::Slab, 0x40, "SECURITY_DESCRIPTOR", symbol, true);
            if sd != 0 {
                emu.maps.write_byte(sd, 1);
                emu.maps.write_word(sd + 2, SE_SELF_RELATIVE);
            }
            write_out_qword(emu, out, sd);
            write_out_dword(emu, size_out, 0x40);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "SeCaptureSid" | "SeCopySid" => emu.set_kernel_ret(0),

        // --- Zw --------------------------------------------------------------------------
        "ZwOpenKey"
        | "ZwOpenKeyEx"
        | "ZwCreateKey"
        | "ZwOpenKeyTransacted"
        | "ZwCreateKeyTransacted"
        | "ZwOpenKeyTransactedEx" => {
            let out = emu.kernel_arg(0);
            let name = object_name_of(emu, emu.kernel_arg(2));
            log::info!("{}: '{}'", symbol, name);
            let h = new_handle(emu, format!("key {}", name));
            write_out_qword(emu, out, h);
            if symbol.starts_with("ZwCreateKey") {
                let disposition = emu.kernel_arg(6);
                write_out_dword(emu, disposition, 1); // REG_CREATED_NEW_KEY
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        // ZwQueryValueKey(KeyHandle, ValueName, Class, Information, Length, *ResultLength)
        "ZwQueryValueKey"
        | "ZwQueryKey"
        | "ZwEnumerateKey"
        | "ZwEnumerateValueKey"
        | "ZwQueryMultipleValueKey" => {
            let ret = emu.kernel_arg(5);
            if symbol == "ZwQueryValueKey" {
                let name = read_unicode_string(emu, emu.kernel_arg(1));
                log::info!("ZwQueryValueKey: '{}' -> not found", name);
            }
            write_out_dword(emu, ret, 0);
            emu.set_kernel_ret(match symbol {
                "ZwQueryValueKey" => STATUS_OBJECT_NAME_NOT_FOUND,
                "ZwEnumerateKey" | "ZwEnumerateValueKey" => STATUS_NO_MORE_ENTRIES,
                _ => STATUS_NOT_SUPPORTED,
            });
        }
        "ZwSetValueKey" => {
            let name = read_unicode_string(emu, emu.kernel_arg(1));
            log::info!("ZwSetValueKey: '{}'", name);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
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
        | "ZwQueryDefaultUILanguage" => {
            let st = if symbol.starts_with("ZwQuery") && symbol != "ZwQuerySystemTime" {
                STATUS_NOT_SUPPORTED
            } else {
                STATUS_SUCCESS
            };
            if symbol == "ZwQuerySystemTime" {
                let out = emu.kernel_arg(0);
                write_out_qword(emu, out, system_time_now());
            }
            emu.set_kernel_ret(st);
        }
        "ZwQueryDirectoryFile" => emu.set_kernel_ret(STATUS_NO_MORE_FILES),
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
        | "ZwCreateSymbolicLinkObject" => {
            let out = emu.kernel_arg(0);
            let attr_idx = match symbol {
                "ZwOpenProcessTokenEx" | "ZwOpenProcessToken" | "ZwOpenThreadTokenEx" => 99,
                _ => 2,
            };
            let name = if attr_idx == 99 {
                String::new()
            } else {
                object_name_of(emu, emu.kernel_arg(attr_idx))
            };
            log::info!("{}: '{}'", symbol, name);
            emu.kernel_log_line(format!("{}({})", symbol, name));
            let h = new_handle(emu, format!("{} {}", symbol.trim_start_matches("Zw"), name));
            write_out_qword(emu, out, h);
            if symbol == "ZwCreateFile" || symbol == "ZwOpenFile" {
                let iosb = emu.kernel_arg(3);
                write_iosb(emu, iosb, 0, 1); // FILE_OPENED
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        // ZwOpenSection(*Handle, Access, ObjAttr): \Device\PhysicalMemory is the
        // classic physical-memory primitive.
        "ZwOpenSection" | "ZwCreateSection" | "ZwCreateSectionEx" => {
            let out = emu.kernel_arg(0);
            let name = object_name_of(emu, emu.kernel_arg(2));
            log::info!("{}: '{}'", symbol, name);
            if name.to_ascii_lowercase().contains("physicalmemory") {
                emu.kernel_privop("physical_memory_section", 0, 0, 0);
            }
            let h = new_handle(emu, format!("section {}", name));
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        // ZwMapViewOfSection(Section, Process, *Base, ZeroBits, Commit, *Offset,
        //                    *ViewSize, Inherit, AllocType, Protect)
        "ZwMapViewOfSection" => {
            let h = emu.kernel_arg(0);
            let base_ptr = emu.kernel_arg(2);
            let offset_ptr = emu.kernel_arg(5);
            let size_ptr = emu.kernel_arg(6);
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
            emu.set_kernel_ret(if va != 0 {
                STATUS_SUCCESS
            } else {
                STATUS_NO_MEMORY
            });
        }
        // ZwAllocateVirtualMemory(Process, *Base, ZeroBits, *Size, Type, Protect)
        "ZwAllocateVirtualMemory" | "ZwAllocateVirtualMemoryEx" => {
            let base_ptr = emu.kernel_arg(1);
            let size_ptr = if symbol == "ZwAllocateVirtualMemory" {
                emu.kernel_arg(3)
            } else {
                emu.kernel_arg(2)
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
            emu.set_kernel_ret(if va != 0 {
                STATUS_SUCCESS
            } else {
                STATUS_NO_MEMORY
            });
        }
        // ZwReadFile(Handle, Event, ApcRoutine, ApcCtx, IoStatus, Buffer, Length, Offset, Key)
        "ZwReadFile" => {
            let iosb = emu.kernel_arg(4);
            write_iosb(emu, iosb, STATUS_END_OF_FILE, 0);
            emu.set_kernel_ret(STATUS_END_OF_FILE);
        }
        "ZwWriteFile" => {
            let iosb = emu.kernel_arg(4);
            let len = emu.kernel_arg(6);
            write_iosb(emu, iosb, 0, len);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        // ZwQueryInformationFile(Handle, IoStatus, Info, Length, Class)
        "ZwQueryInformationFile" => {
            let iosb = emu.kernel_arg(1);
            let info = emu.kernel_arg(2);
            let len = emu.kernel_arg(3);
            if info != 0 && len != 0 {
                emu.maps
                    .write_bytes(info, &vec![0u8; len.min(0x1000) as usize]);
            }
            write_iosb(emu, iosb, 0, len);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        // ZwQuerySystemInformation(Class, Buffer, Length, *ReturnLength)
        "ZwQuerySystemInformation" | "NtQuerySystemInformation" | "ZwQuerySystemInformationEx" => {
            let class = emu.kernel_arg(0);
            let (buf, len, ret) = if symbol == "ZwQuerySystemInformationEx" {
                (emu.kernel_arg(3), emu.kernel_arg(4), emu.kernel_arg(5))
            } else {
                (emu.kernel_arg(1), emu.kernel_arg(2), emu.kernel_arg(3))
            };
            emu.kernel_log_line(format!("{}(class {})", symbol, class));
            let st = match class {
                11 => system_module_information(emu, buf, len, ret), // SystemModuleInformation
                _ => {
                    write_out_dword(emu, ret, 0);
                    STATUS_INVALID_INFO_CLASS
                }
            };
            emu.set_kernel_ret(st);
        }

        // --- Cm ---------------------------------------------------------------------------
        "CmRegisterCallback" | "CmRegisterCallbackEx" => {
            let func = emu.kernel_arg(0);
            let (ctx, cookie) = if symbol == "CmRegisterCallback" {
                (emu.kernel_arg(1), emu.kernel_arg(2))
            } else {
                (emu.kernel_arg(3), emu.kernel_arg(4))
            };
            register_callback(emu, symbol, func, ctx);
            let h = new_handle(emu, "registry callback".to_string());
            write_out_qword(emu, cookie, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "CmUnRegisterCallback"
        | "CmSetCallbackObjectContext"
        | "CmCallbackReleaseKeyObjectIDEx" => {
            emu.set_kernel_ret(0);
        }
        "CmCallbackGetKeyObjectID" | "CmCallbackGetKeyObjectIDEx" | "CmGetCallbackVersion" => {
            emu.set_kernel_ret(if symbol == "CmGetCallbackVersion" {
                0x10000
            } else {
                STATUS_NOT_SUPPORTED
            });
        }

        // --- Ex callbacks, misc ------------------------------------------------------------
        "ExCreateCallback" => {
            let out = emu.kernel_arg(0);
            let name = object_name_of(emu, emu.kernel_arg(1));
            let obj = emu.kernel_alloc(Region::Slab, 0x40, "CALLBACK_OBJECT", symbol, true);
            log::info!("ExCreateCallback: '{}' -> 0x{:x}", name, obj);
            write_out_qword(emu, out, obj);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "ExRegisterCallback" => {
            let func = emu.kernel_arg(1);
            let ctx = emu.kernel_arg(2);
            register_callback(emu, symbol, func, ctx);
            let h = new_handle(emu, "ExRegisterCallback".to_string());
            emu.set_kernel_ret(h);
        }
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
        | "ExLocalTimeToSystemTime" => {
            if symbol == "ExSystemTimeToLocalTime" || symbol == "ExLocalTimeToSystemTime" {
                let src = emu.kernel_arg(0);
                let dst = emu.kernel_arg(1);
                let t = emu.maps.read_qword(src).unwrap_or(0);
                write_out_qword(emu, dst, t);
            }
            emu.set_kernel_ret(0);
        }
        "ExGetPreviousMode" => emu.set_kernel_ret(1), // UserMode
        "ExUuidCreate" => {
            let out = emu.kernel_arg(0);
            let seed = emu.pos.wrapping_mul(0x9E3779B97F4A7C15);
            emu.maps.write_qword(out, seed);
            emu.maps
                .write_qword(out + 8, seed.rotate_left(32) ^ 0x4000_0000_0000_8000);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "ExAllocateTimer" => {
            let t = emu.kernel_alloc(Region::Slab, 0x80, "EX_TIMER", symbol, true);
            emu.set_kernel_ret(t);
        }
        "ExVerifySuite" | "ExIsManufacturingModeEnabled" => {
            emu.set_kernel_ret((symbol == "ExVerifySuite") as u64)
        }
        "ExGetFirmwareEnvironmentVariable"
        | "ExSetFirmwareEnvironmentVariable"
        | "ExGetFirmwareType" => emu.set_kernel_ret(if symbol == "ExGetFirmwareType" {
            2
        } else {
            STATUS_NOT_SUPPORTED
        }),
        "ExCreateHandleTable" | "ExCreateHandle" => emu.set_kernel_ret(0),

        // --- Flt (filter manager) -----------------------------------------------------------
        "FltRegisterFilter" => {
            let out = emu.kernel_arg(2);
            let h = new_handle(emu, "filter".to_string());
            log::info!("FltRegisterFilter -> 0x{:x}", h);
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "FltCreateCommunicationPort" => {
            // (Filter, *ServerPort, ObjAttr, Cookie, Connect, Disconnect, Message, MaxConnections)
            let out = emu.kernel_arg(1);
            let name = object_name_of(emu, emu.kernel_arg(2));
            let connect = emu.kernel_arg(4);
            let message = emu.kernel_arg(6);
            register_callback(emu, "FltCreateCommunicationPort.Connect", connect, 0);
            register_callback(emu, "FltCreateCommunicationPort.Message", message, 0);
            log::info!("FltCreateCommunicationPort: '{}'", name);
            let h = new_handle(emu, format!("port {}", name));
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "FltBuildDefaultSecurityDescriptor" => {
            let out = emu.kernel_arg(0);
            let sd = emu.kernel_alloc(Region::Slab, 0x40, "SECURITY_DESCRIPTOR", symbol, true);
            if sd != 0 {
                emu.maps.write_byte(sd, 1);
                emu.maps.write_word(sd + 2, SE_SELF_RELATIVE);
            }
            write_out_qword(emu, out, sd);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "FltFreeSecurityDescriptor" => {
            let sd = emu.kernel_arg(0);
            emu.kernel_free(sd, symbol);
            emu.set_kernel_ret(0);
        }
        "FltAllocateGenericWorkItem" => {
            let p = emu.kernel_alloc(Region::Slab, 0x40, "FLT_GENERIC_WORKITEM", symbol, true);
            emu.set_kernel_ret(p);
        }
        "FltFreeGenericWorkItem" => {
            let p = emu.kernel_arg(0);
            emu.kernel_free(p, symbol);
            emu.set_kernel_ret(0);
        }
        // FltQueueGenericWorkItem(WorkItem, FltObject, Routine, QueueType, Context)
        "FltQueueGenericWorkItem" => {
            let item = emu.kernel_arg(0);
            let obj = emu.kernel_arg(1);
            let routine = emu.kernel_arg(2);
            let ctx = emu.kernel_arg(4);
            emu.kernel_defer(DeferredCall {
                kind: "flt_workitem".to_string(),
                func: routine,
                args: vec![item, obj, ctx],
            });
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "FltGetRequestorProcessId" => emu.set_kernel_ret(4),
        "FltGetRequestorProcess" => {
            let p = system_process(emu);
            emu.set_kernel_ret(p);
        }
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
        | "FltCompletePendedPostOperation" => {
            emu.set_kernel_ret(0);
        }
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
        | "FltGetUpperInstance" => emu.set_kernel_ret(STATUS_NOT_SUPPORTED),
        "FltAllocateCallbackData"
        | "FltAllocatePoolAlignedWithTag"
        | "FltAllocateFileLock"
        | "FltAllocateExtraCreateParameterList" => {
            let size = if symbol == "FltAllocatePoolAlignedWithTag" {
                emu.kernel_arg(2)
            } else {
                0x100
            };
            let p = emu.kernel_alloc(Region::Slab, size.max(1), "flt", symbol, true);
            if symbol == "FltAllocateCallbackData" {
                let out = emu.kernel_arg(2);
                write_out_qword(emu, out, p);
                emu.set_kernel_ret(STATUS_SUCCESS);
            } else {
                emu.set_kernel_ret(p);
            }
        }
        "FltFreeCallbackData" | "FltFreePoolAlignedWithTag" | "FltFreeFileLock" => {
            let p = emu.kernel_arg(0);
            emu.kernel_free(p, symbol);
            emu.set_kernel_ret(0);
        }
        "FltGetRoutineAddress" => {
            let name = emu.maps.read_string(emu.kernel_arg(0));
            let addr = emu.kernel_resolve_import(&name).unwrap_or(0);
            emu.set_kernel_ret(addr);
        }

        // --- WDF loader -------------------------------------------------------------------------
        // WdfVersionBind(DriverObject, RegistryPath, BindInfo, *ComponentGlobals):
        // fills the driver's WdfFunctions table. Each slot is a distinct stub
        // so a call through the table is reported by index; the KMDF object
        // model itself (WdfDriverCreate, WdfDeviceCreate, queues) is not
        // modelled, so a KMDF driver gets through DriverEntry with every
        // framework call succeeding and no EvtDeviceAdd callback.
        "WdfVersionBind" => {
            let bind = emu.kernel_arg(2);
            let globals_out = emu.kernel_arg(3);
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
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "WdfVersionBindClass" | "WdfVersionUnbind" | "WdfVersionUnbindClass" => {
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "WdfLdrQueryInterface" => emu.set_kernel_ret(STATUS_NOT_SUPPORTED),

        // --- ETW / WMI ------------------------------------------------------------------------
        "EtwRegister" | "EtwRegisterClassicProvider" => {
            let out = emu.kernel_arg(3);
            let h = new_handle(emu, "etw provider".to_string());
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
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
        | "EtwUnregisterClassicProvider" => emu.set_kernel_ret(0),
        "EtwProviderEnabled" | "EtwEventEnabled" => emu.set_kernel_ret(0),

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
        | "PoSetPowerRequest" => emu.set_kernel_ret(0),
        "PoRegisterPowerSettingCallback" => {
            let func = emu.kernel_arg(2);
            let ctx = emu.kernel_arg(3);
            let out = emu.kernel_arg(4);
            register_callback(emu, symbol, func, ctx);
            let h = new_handle(emu, "power setting callback".to_string());
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "PoSetPowerState" => {
            let state = emu.kernel_arg(2);
            emu.set_kernel_ret(state);
        }
        "PoRequestPowerIrp" => emu.set_kernel_ret(STATUS_PENDING),
        "PoCreatePowerRequest" => {
            let out = emu.kernel_arg(0);
            let p = emu.kernel_alloc(Region::Slab, 0x40, "POWER_REQUEST", symbol, true);
            write_out_qword(emu, out, p);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "KdDisableDebugger"
        | "KdEnableDebugger"
        | "KdRefreshDebuggerNotPresent"
        | "KdChangeOption"
        | "KdSystemDebugControl"
        | "KdRegisterPowerHandler"
        | "KdDeregisterPowerHandler"
        | "KdPowerTransition" => emu.set_kernel_ret(if symbol == "KdRefreshDebuggerNotPresent" {
            1
        } else {
            0
        }),
        "NtBuildNumber" => emu.set_kernel_ret(NT_BUILD_NUMBER as u64),
        _ => return false,
    }
    true
}

pub const SURFACE: &[&str] = &[
    "PsGetCurrentProcess",
    "PsGetCurrentProcessId",
    "PsGetCurrentThreadId",
    "PsGetProcessId",
    "PsGetThreadId",
    "PsGetThreadProcessId",
    "PsGetThreadProcess",
    "PsGetProcessImageFileName",
    "PsGetProcessPeb",
    "PsGetProcessWow64Process",
    "PsGetProcessSectionBaseAddress",
    "PsGetProcessDebugPort",
    "PsGetProcessExitStatus",
    "PsGetProcessCreateTimeQuadPart",
    "PsGetProcessInheritedFromUniqueProcessId",
    "PsIsSystemThread",
    "PsIsThreadTerminating",
    "PsIsProtectedProcess",
    "PsLookupProcessByProcessId",
    "PsLookupThreadByThreadId",
    "PsLookupProcessThreadByCid",
    "PsSetCreateProcessNotifyRoutine",
    "PsSetCreateProcessNotifyRoutineEx",
    "PsSetCreateProcessNotifyRoutineEx2",
    "PsSetCreateThreadNotifyRoutine",
    "PsSetCreateThreadNotifyRoutineEx",
    "PsSetLoadImageNotifyRoutine",
    "PsSetLoadImageNotifyRoutineEx",
    "PsRemoveCreateThreadNotifyRoutine",
    "PsRemoveLoadImageNotifyRoutine",
    "PsCreateSystemThread",
    "PsTerminateSystemThread",
    "PsGetVersion",
    "PsReferencePrimaryToken",
    "PsDereferencePrimaryToken",
    "PsChargeProcessPoolQuota",
    "PsReturnPoolQuota",
    "PsRegisterSiloMonitor",
    "PsUnregisterSiloMonitor",
    "PsStartSiloMonitor",
    "PsGetSiloContainerId",
    "PsGetProcessServerSilo",
    "PsGetThreadServerSilo",
    "SeLocateProcessImageName",
    "ObReferenceObjectByHandle",
    "ObReferenceObjectByHandleWithTag",
    "ObReferenceObjectByPointer",
    "ObfReferenceObject",
    "ObReferenceObject",
    "ObfDereferenceObject",
    "ObDereferenceObject",
    "ObfReferenceObjectWithTag",
    "ObfDereferenceObjectWithTag",
    "ObDereferenceObjectDeferDelete",
    "ObOpenObjectByPointer",
    "ObOpenObjectByName",
    "ObReferenceObjectByName",
    "ObQueryNameString",
    "ObGetObjectSecurity",
    "ObReleaseObjectSecurity",
    "ObRegisterCallbacks",
    "ObUnRegisterCallbacks",
    "ObGetFilterVersion",
    "ObCloseHandle",
    "ObIsKernelHandle",
    "ZwClose",
    "NtClose",
    "SeCaptureSecurityDescriptor",
    "SeReleaseSecurityDescriptor",
    "SeAccessCheck",
    "SeSinglePrivilegeCheck",
    "SePrivilegeCheck",
    "SeTokenIsAdmin",
    "SeTokenIsRestricted",
    "SeCaptureSubjectContext",
    "SeCaptureSubjectContextEx",
    "SeReleaseSubjectContext",
    "SeLockSubjectContext",
    "SeUnlockSubjectContext",
    "SeAssignSecurity",
    "SeAssignSecurityEx",
    "SeDeassignSecurity",
    "SeQueryInformationToken",
    "SeQueryAuthenticationIdToken",
    "SeConvertStringSecurityDescriptorToSecurityDescriptor",
    "SeValidSecurityDescriptor",
    "ZwOpenKey",
    "ZwOpenKeyEx",
    "ZwCreateKey",
    "ZwQueryValueKey",
    "ZwQueryKey",
    "ZwEnumerateKey",
    "ZwEnumerateValueKey",
    "ZwSetValueKey",
    "ZwDeleteKey",
    "ZwDeleteValueKey",
    "ZwFlushKey",
    "ZwNotifyChangeKey",
    "ZwSetSecurityObject",
    "ZwQuerySecurityObject",
    "ZwLoadDriver",
    "ZwUnloadDriver",
    "ZwCreateFile",
    "ZwOpenFile",
    "ZwReadFile",
    "ZwWriteFile",
    "ZwDeleteFile",
    "ZwQueryInformationFile",
    "ZwSetInformationFile",
    "ZwQueryDirectoryFile",
    "ZwQueryVolumeInformationFile",
    "ZwFsControlFile",
    "ZwDeviceIoControlFile",
    "ZwOpenProcess",
    "ZwOpenThread",
    "ZwTerminateProcess",
    "ZwQueryInformationProcess",
    "ZwQueryInformationThread",
    "ZwQuerySystemInformation",
    "ZwQuerySystemInformationEx",
    "NtQuerySystemInformation",
    "ZwSetSystemInformation",
    "ZwAllocateVirtualMemory",
    "ZwFreeVirtualMemory",
    "ZwProtectVirtualMemory",
    "ZwQueryVirtualMemory",
    "ZwReadVirtualMemory",
    "ZwWriteVirtualMemory",
    "ZwOpenSection",
    "ZwCreateSection",
    "ZwMapViewOfSection",
    "ZwUnmapViewOfSection",
    "ZwOpenDirectoryObject",
    "ZwCreateDirectoryObject",
    "ZwOpenSymbolicLinkObject",
    "ZwCreateSymbolicLinkObject",
    "ZwQuerySymbolicLinkObject",
    "ZwOpenEvent",
    "ZwCreateEvent",
    "ZwSetEvent",
    "ZwWaitForSingleObject",
    "ZwWaitForMultipleObjects",
    "ZwOpenProcessTokenEx",
    "ZwOpenThreadTokenEx",
    "ZwQueryInformationToken",
    "ZwDuplicateObject",
    "ZwQueryObject",
    "ZwPowerInformation",
    "ZwYieldExecution",
    "ZwQuerySystemTime",
    "ZwSetSystemTime",
    "ZwMakeTemporaryObject",
    "CmRegisterCallback",
    "CmRegisterCallbackEx",
    "CmUnRegisterCallback",
    "CmCallbackGetKeyObjectID",
    "CmCallbackGetKeyObjectIDEx",
    "CmGetCallbackVersion",
    "ExCreateCallback",
    "ExRegisterCallback",
    "ExUnregisterCallback",
    "ExNotifyCallback",
    "ExGetPreviousMode",
    "ExUuidCreate",
    "ExSystemTimeToLocalTime",
    "ExLocalTimeToSystemTime",
    "ExSetTimerResolution",
    "ExEnumHandleTable",
    "ExAllocateTimer",
    "ExSetTimer",
    "ExCancelTimer",
    "ExDeleteTimer",
    "ExVerifySuite",
    "ExGetFirmwareEnvironmentVariable",
    "ExSetFirmwareEnvironmentVariable",
    "ExGetFirmwareType",
    "ExSystemExceptionFilter",
    "FltRegisterFilter",
    "FltUnregisterFilter",
    "FltStartFiltering",
    "FltCreateCommunicationPort",
    "FltCloseCommunicationPort",
    "FltCloseClientPort",
    "FltSendMessage",
    "FltBuildDefaultSecurityDescriptor",
    "FltFreeSecurityDescriptor",
    "FltAllocateGenericWorkItem",
    "FltFreeGenericWorkItem",
    "FltQueueGenericWorkItem",
    "FltGetRequestorProcessId",
    "FltGetRequestorProcess",
    "FltGetFileNameInformation",
    "FltGetFileNameInformationUnsafe",
    "FltGetDestinationFileNameInformation",
    "FltReleaseFileNameInformation",
    "FltParseFileNameInformation",
    "FltGetVolumeName",
    "FltObjectReference",
    "FltObjectDereference",
    "FltSetCallbackDataDirty",
    "FltAllocateCallbackData",
    "FltFreeCallbackData",
    "FltAllocatePoolAlignedWithTag",
    "FltFreePoolAlignedWithTag",
    "FltGetRoutineAddress",
    "FltQueryInformationFile",
    "FltReadFile",
    "FltWriteFile",
    "FltCreateFile",
    "FltCreateFileEx",
    "FltGetVolumeFromInstance",
    "FltGetDiskDeviceObject",
    "WdfVersionBind",
    "WdfVersionBindClass",
    "WdfVersionUnbind",
    "WdfVersionUnbindClass",
    "WdfLdrQueryInterface",
    "EtwRegister",
    "EtwUnregister",
    "EtwRegisterClassicProvider",
    "EtwUnregisterClassicProvider",
    "EtwWrite",
    "EtwWriteEx",
    "EtwWriteTransfer",
    "EtwWriteString",
    "EtwSetInformation",
    "EtwSendTraceBuffer",
    "EtwProviderEnabled",
    "EtwEventEnabled",
    "EtwActivityIdControl",
    "WmiQueryTraceInformation",
    "WmiTraceMessage",
    "WmiTraceMessageVa",
    "WmiSystemControl",
    "PoCallDriver",
    "PoStartNextPowerIrp",
    "PoSetPowerState",
    "PoRequestPowerIrp",
    "PoRegisterPowerSettingCallback",
    "PoUnregisterPowerSettingCallback",
    "PoSetSystemState",
    "PoRegisterSystemState",
    "PoUnregisterSystemState",
    "PoRegisterDeviceForIdleDetection",
    "PoSetDeviceBusyEx",
    "PoStartDeviceBusy",
    "PoEndDeviceBusy",
    "PoCreatePowerRequest",
    "PoSetPowerRequest",
    "PoClearPowerRequest",
    "PoDeletePowerRequest",
    "PoFxRegisterDevice",
    "PoFxUnregisterDevice",
    "PoFxStartDevicePowerManagement",
    "PoFxActivateComponent",
    "PoFxIdleComponent",
    "KdDisableDebugger",
    "KdEnableDebugger",
    "KdRefreshDebuggerNotPresent",
    "KdChangeOption",
    "KdSystemDebugControl",
    "KdRegisterPowerHandler",
    "KdDeregisterPowerHandler",
    "FltAttachVolume",
    "FltDetachVolume",
    "FltEnumerateInstances",
    "FltEnumerateVolumes",
    "FltGetVolumeFromName",
    "FltGetFilterFromName",
    "FltGetVolumeProperties",
    "FltGetFilterInformation",
    "FltGetInstanceInformation",
    "FltFsControlFile",
    "FltDeviceIoControlFile",
    "FltGetTunneledName",
    "FltGetTopInstance",
    "FltGetBottomInstance",
    "FltGetLowerInstance",
    "FltGetUpperInstance",
    "FltReleaseContext",
    "FltDeleteContext",
    "FltSetInformationFile",
    "FltFlushBuffers",
    "FltCancelFileOpen",
    "FltIsOperationSynchronous",
    "FltIs32bitProcess",
    "FltReferenceContext",
    "FltSetStreamHandleContext",
    "FltSetFileContext",
    "FltSetInstanceContext",
    "FltSetVolumeContext",
    "FltDeleteStreamHandleContext",
    "FltDeleteFileContext",
    "FltRegisterForDataScan",
    "FltEnumerateFilters",
    "FltReleaseResource",
    "FltAcquireResourceExclusive",
    "FltAcquireResourceShared",
    "FltCompletePendedPreOperation",
    "FltCompletePendedPostOperation",
    "FltGetVolumeGuidName",
    "FltQueryVolumeInformation",
    "FltGetVolumeFromInstance",
    "FltGetDiskDeviceObject",
    "FltGetDeviceObject",
    "FltGetStreamHandleContext",
    "FltGetFileContext",
    "FltGetInstanceContext",
    "FltGetVolumeContext",
    "FltAllocateContext",
    "FltCreateFileEx2",
];
