use super::kernel32;
use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "DeviceIoControl" => kernel32::lookup(name),
        "SetThreadAffinityMask" => super::kernel32::lookup(name),
        "SetProcessAffinityMask" => super::kernel32::lookup(name),
        "FreeLibraryWhenCallbackReturns" => super::kernel32::lookup(name),
        "DeleteProcThreadAttributeList" => super::kernel32::lookup(name),
        "CloseThreadpoolWork" => super::kernel32::lookup(name),
        "CloseThreadpoolIo" => super::kernel32::lookup(name),
        "CloseThreadpoolCleanupGroupMembers" => super::kernel32::lookup(name),
        "CloseThreadpoolCleanupGroup" => super::kernel32::lookup(name),
        "CloseThreadpool" => super::kernel32::lookup(name),
        "CloseState" => super::kernel32::lookup(name),
        "ClosePackageInfo" => super::kernel32::lookup(name),
        "CloseIoRing" => super::kernel32::lookup(name),
        "WakeConditionVariable" => super::kernel32::lookup(name),
        "WakeAllConditionVariable" => super::kernel32::lookup(name),
        "TryAcquireSRWLockShared" => super::kernel32::lookup(name),
        "TryAcquireSRWLockExclusive" => super::kernel32::lookup(name),
        "Sleep" => super::kernel32::lookup(name),
        "ReleaseSRWLockShared" => super::kernel32::lookup(name),
        "ReleaseSRWLockExclusive" => super::kernel32::lookup(name),
        "LoadLibraryExW" => super::kernel32::lookup(name),
        "LoadLibraryExA" => super::kernel32::lookup(name),
        "InitializeSRWLock" => super::kernel32::lookup(name),
        "InitializeConditionVariable" => super::kernel32::lookup(name),
        "InitOnceInitialize" => super::kernel32::lookup(name),
        "CreateRemoteThreadEx" => super::kernel32::lookup(name),
        "AcquireSRWLockShared" => super::kernel32::lookup(name),
        "AcquireSRWLockExclusive" => super::kernel32::lookup(name),
        "_local_unwind" => super::kernel32::lookup(name),
        "__C_specific_handler" => super::kernel32::lookup(name),
        "UpdateResourceW" => super::kernel32::lookup(name),
        "UpdateResourceA" => super::kernel32::lookup(name),
        "SetConsoleInputExeNameW" => super::kernel32::lookup(name),
        "SetConsoleInputExeNameA" => super::kernel32::lookup(name),
        "ReadConsoleInputExW" => super::kernel32::lookup(name),
        "ReadConsoleInputExA" => super::kernel32::lookup(name),
        "OpenPackageInfoByFullName" => super::kernel32::lookup(name),
        "IsThreadpoolTimerSet" => super::kernel32::lookup(name),
        "InterlockedPushListSList" => super::kernel32::lookup(name),
        "InterlockedPopEntrySList" => super::kernel32::lookup(name),
        "InterlockedFlushSList" => super::kernel32::lookup(name),
        "GetConsoleInputExeNameW" => super::kernel32::lookup(name),
        "GetConsoleInputExeNameA" => super::kernel32::lookup(name),
        "FindActCtxSectionStringA" => super::kernel32::lookup(name),
        "EndUpdateResourceW" => super::kernel32::lookup(name),
        "EndUpdateResourceA" => super::kernel32::lookup(name),
        "DuplicateHandle" => super::kernel32::lookup(name),
        "CreateProcessW" => super::kernel32::lookup(name),
        "CreateProcessA" => super::kernel32::lookup(name),
        "CreateIoRing" => super::kernel32::lookup(name),
        "CreateEnclave" => super::kernel32::lookup(name),
        "CloseThreadpoolWait" => super::kernel32::lookup(name),
        "CloseThreadpoolTimer" => super::kernel32::lookup(name),
        "BeginUpdateResourceW" => super::kernel32::lookup(name),
        "BeginUpdateResourceA" => super::kernel32::lookup(name),
        "MultiByteToWideChar"
        | "CompareStringA"
        | "CompareStringOrdinal"
        | "CompareStringW"
        | "GetStringTypeA"
        | "GetStringTypeW"
        | "GetStringTypeExA"
        | "GetStringTypeExW"
        | "GetLocaleInfoA"
        | "GetLocaleInfoEx"
        | "GetLocaleInfoW"
        | "LCMapStringA"
        | "LCMapStringW"
        | "CreateActCtxA"
        | "CreateActCtxW" => super::kernel32::lookup(name),
        "ActivateActCtx" => signature!(Generic, 2, 2; Pointer(h_act_ctx), Pointer(lp_cookie)),
        "AddDllDirectory" => signature!(Generic, 1, 1; WideString(new_directory)),
        "AddVectoredContinueHandler" => signature!(Generic, 2, 2; Dword(first), Pointer(handler)),
        "AppPolicyGetClrCompat"
        | "AppPolicyGetCreateFileAccess"
        | "AppPolicyGetLifecycleManagement"
        | "AppPolicyGetMediaFoundationCodecLoading"
        | "AppPolicyGetShowDeveloperDiagnostic"
        | "AppPolicyGetThreadInitializationType"
        | "AppPolicyGetWindowingModel" => {
            signature!(Win32Status, 2, 2; Pointer(process_token), Pointer(policy))
        }
        "Beep" => signature!(Generic, 2, 2; Dword(dw_freq), Dword(dw_duration)),
        "BuildIoRingCancelRequest" => signature!(Hresult, 4, 5),
        "BuildIoRingFlushFile" => signature!(Hresult, 5, 6),
        "BuildIoRingReadFile" => signature!(Hresult, 7, 11),
        "BuildIoRingRegisterBuffers" => {
            signature!(Hresult, 4, 4; Pointer(io_ring), Dword(count), Pointer(buffers), Pointer(user_data))
        }
        "BuildIoRingRegisterFileHandles" => {
            signature!(Hresult, 4, 4; Pointer(io_ring), Dword(count), Pointer(handles), Pointer(user_data))
        }
        "BuildIoRingWriteFile" => signature!(Hresult, 8, 12),
        "CancelThreadpoolIo" | "StartThreadpoolIo" => signature!(Void, 1, 1; Pointer(pio)),
        "CeipIsOptedIn"
        | "GetConsoleCP"
        | "GetConsoleOutputCP"
        | "GetCurrentProcessorNumber"
        | "GetUserDefaultLangID"
        | "GetUserDefaultUILanguage"
        | "GetVersion" => signature!(Generic, 0, 0; ),
        "CheckRemoteDebuggerPresent" => {
            signature!(Generic, 2, 2; Pointer(h_process), Pointer(pb_debugger_present))
        }
        "CreateRemoteThread" => {
            signature!(Generic, 7, 7; Pointer(h_process), Pointer(lp_thread_attributes), Pointer(dw_stack_size), Pointer(lp_start_address), Pointer(lp_parameter), Dword(dw_creation_flags), Pointer(lp_thread_id))
        }
        "DecodeRemotePointer" => {
            signature!(Hresult, 3, 3; Pointer(process_handle), Pointer(ptr), Pointer(decoded_ptr))
        }
        "DecodeSystemPointer" | "EncodeSystemPointer" => signature!(Generic, 1, 1; Pointer(ptr)),

        "DisassociateCurrentThreadFromCallback" => signature!(Void, 1, 1; Pointer(pci)),
        "DiscardVirtualMemory" | "ReclaimVirtualMemory" => {
            signature!(Generic, 2, 2; Pointer(virtual_address), Pointer(size))
        }
        "EncodeRemotePointer" => {
            signature!(Hresult, 3, 3; Pointer(process_handle), Pointer(ptr), Pointer(encoded_ptr))
        }
        "EventActivityIdControl" => {
            signature!(Generic, 2, 2; Dword(control_code), Pointer(activity_id))
        }
        "EventProviderEnabled" => {
            signature!(Generic, 3, 5; Qword(reg_handle), Byte(level), Qword(keyword))
        }
        "EventWrite" => {
            signature!(Generic, 4, 5; Qword(reg_handle), Pointer(event_descriptor), Dword(user_data_count), Pointer(user_data))
        }
        "EventWriteEx" => {
            signature!(Generic, 8, 10; Qword(reg_handle), Pointer(event_descriptor), Qword(filter), Dword(flags), Pointer(activity_id), Pointer(related_activity_id), Dword(user_data_count), Pointer(user_data))
        }
        "EventWriteString" => {
            signature!(Generic, 4, 6; Qword(reg_handle), Byte(level), Qword(keyword), WideString(string))
        }
        "FindActCtxSectionStringW" => {
            signature!(Generic, 5, 5; Dword(dw_flags), Pointer(lp_extension_guid), Dword(ul_section_id), WideString(lp_string_to_find), Pointer(returned_data))
        }
        "FindPackagesByPackageFamily" => {
            signature!(Win32Status, 7, 7; WideString(package_family_name), Dword(package_filters), Pointer(count), Pointer(package_full_names), Pointer(buffer_length), Pointer(buffer), Pointer(package_properties))
        }
        "FlsAlloc" => signature!(Generic, 1, 1; Pointer(lp_callback)),
        "FlsFree" | "FlsGetValue" => signature!(Generic, 1, 1; Dword(dw_fls_index)),
        "FlsSetValue" => signature!(Generic, 2, 2; Dword(dw_fls_index), Pointer(lp_fls_data)),
        "FlushProcessWriteBuffers" => signature!(Void, 0, 0; ),
        "FormatApplicationUserModelId" => {
            signature!(Win32Status, 4, 4; WideString(package_family_name), WideString(package_relative_application_id), Pointer(application_user_model_id_length), Pointer(application_user_model_id))
        }
        "GetApplicationUserModelId" => {
            signature!(Win32Status, 3, 3; Pointer(h_process), Pointer(application_user_model_id_length), Pointer(application_user_model_id))
        }
        "GetCurrentApplicationUserModelId" => {
            signature!(Win32Status, 2, 2; Pointer(application_user_model_id_length), Pointer(application_user_model_id))
        }
        "GetCurrentPackageFamilyName" => {
            signature!(Win32Status, 2, 2; Pointer(package_family_name_length), Pointer(package_family_name))
        }
        "GetCurrentPackageFullName" => {
            signature!(Win32Status, 2, 2; Pointer(package_full_name_length), Pointer(package_full_name))
        }
        "GetCurrentPackageId" => {
            signature!(Win32Status, 2, 2; Pointer(buffer_length), Pointer(buffer))
        }
        "GetCurrentPackageInfo" => {
            signature!(Win32Status, 4, 4; Dword(flags), Pointer(buffer_length), Pointer(buffer), Pointer(count))
        }
        "GetCurrentPackagePath" => {
            signature!(Win32Status, 2, 2; Pointer(path_length), Pointer(path))
        }
        "GetCurrentProcessorNumberEx" => signature!(Void, 1, 1; Pointer(proc_number)),
        "GetCurrentThreadStackLimits" => {
            signature!(Void, 2, 2; Pointer(low_limit), Pointer(high_limit))
        }
        "GetIoRingInfo" => signature!(Hresult, 2, 2; Pointer(io_ring), Pointer(info)),
        "GetLogicalProcessorInformationEx" => {
            signature!(Generic, 3, 3; Dword(relationship_type), Pointer(buffer), Pointer(returned_length))
        }
        "GetMachineTypeAttributes" => {
            signature!(Hresult, 2, 2; Word(machine), Pointer(machine_type_attributes))
        }
        "GetNativeSystemInfo" | "GetSystemInfo" => signature!(Void, 1, 1; Pointer(lp_system_info)),
        "GetNumaNodeProcessorMask2" => {
            signature!(Generic, 4, 4; Word(node_number), Pointer(processor_masks), Word(processor_mask_count), Pointer(required_mask_count))
        }
        "GetPackageApplicationIds" => {
            signature!(Win32Status, 4, 4; Pointer(package_info_reference), Pointer(buffer_length), Pointer(buffer), Pointer(count))
        }
        "GetPackageFamilyName" => {
            signature!(Win32Status, 3, 3; Pointer(h_process), Pointer(package_family_name_length), Pointer(package_family_name))
        }
        "GetPackageFullName" => {
            signature!(Win32Status, 3, 3; Pointer(h_process), Pointer(package_full_name_length), Pointer(package_full_name))
        }
        "GetPackageId" => {
            signature!(Win32Status, 3, 3; Pointer(h_process), Pointer(buffer_length), Pointer(buffer))
        }
        "GetPackageInfo" => {
            signature!(Win32Status, 5, 5; Pointer(package_info_reference), Dword(flags), Pointer(buffer_length), Pointer(buffer), Pointer(count))
        }
        "GetPackagePath" => {
            signature!(Win32Status, 4, 4; Pointer(package_id), Dword(reserved), Pointer(path_length), Pointer(path))
        }
        "GetPackagePathByFullName" | "GetStagedPackagePathByFullName" => {
            signature!(Win32Status, 3, 3; WideString(package_full_name), Pointer(path_length), Pointer(path))
        }
        "GetPackagesByPackageFamily" => {
            signature!(Win32Status, 5, 5; WideString(package_family_name), Pointer(count), Pointer(package_full_names), Pointer(buffer_length), Pointer(buffer))
        }
        "GetProcessDefaultCpuSetMasks" => {
            signature!(Generic, 4, 4; Pointer(process), Pointer(cpu_set_masks), Word(cpu_set_mask_count), Pointer(required_mask_count))
        }
        "GetProcessDefaultCpuSets" => {
            signature!(Generic, 4, 4; Pointer(process), Pointer(cpu_set_ids), Dword(cpu_set_id_count), Pointer(required_id_count))
        }
        "GetProcessMitigationPolicy" => {
            signature!(Generic, 4, 4; Pointer(h_process), Dword(mitigation_policy), Pointer(lp_buffer), Pointer(dw_length))
        }
        "GetProcessorSystemCycleTime" => {
            signature!(Generic, 3, 3; Word(group), Pointer(buffer), Pointer(returned_length))
        }
        "GetSystemCpuSetInformation" => {
            signature!(Generic, 5, 5; Pointer(information), Dword(buffer_length), Pointer(returned_length), Pointer(process), Dword(flags))
        }
        "GetSystemFirmwareTable" => {
            signature!(Generic, 4, 4; Dword(firmware_table_provider_signature), Dword(firmware_table_id), Pointer(p_firmware_table_buffer), Dword(buffer_size))
        }
        "GetThreadDescription" => {
            signature!(Hresult, 2, 2; Pointer(h_thread), Pointer(ppsz_thread_description))
        }
        "GetThreadSelectedCpuSetMasks" => {
            signature!(Generic, 4, 4; Pointer(thread), Pointer(cpu_set_masks), Word(cpu_set_mask_count), Pointer(required_mask_count))
        }
        "GetThreadSelectedCpuSets" => {
            signature!(Generic, 4, 4; Pointer(thread), Pointer(cpu_set_ids), Dword(cpu_set_id_count), Pointer(required_id_count))
        }
        "GetTimeZoneInformationForYear" => {
            signature!(Generic, 3, 3; Word(w_year), Pointer(pdtzi), Pointer(ptzi))
        }
        "GetTraceEnableFlags" | "GetTraceEnableLevel" => {
            signature!(Generic, 1, 2; Qword(trace_handle))
        }
        "GetTraceLoggerHandle" => signature!(Generic, 1, 1; Pointer(buffer)),
        "HeapSetInformation" => {
            signature!(Generic, 4, 4; Pointer(heap_handle), Dword(heap_information_class), Pointer(heap_information), Pointer(heap_information_length))
        }
        "InitOnceBeginInitialize" => {
            signature!(Generic, 4, 4; Pointer(lp_init_once), Dword(dw_flags), Pointer(f_pending), Pointer(lp_context))
        }
        "InitOnceComplete" => {
            signature!(Generic, 3, 3; Pointer(lp_init_once), Dword(dw_flags), Pointer(lp_context))
        }
        "InitOnceExecuteOnce" => {
            signature!(Generic, 4, 4; Pointer(init_once), Pointer(init_fn), Pointer(parameter), Pointer(context))
        }
        "InitializeCriticalSectionAndSpinCount" | "SetCriticalSectionSpinCount" => {
            signature!(Generic, 2, 2; Pointer(lp_critical_section), Dword(dw_spin_count))
        }
        "InitializeEnclave" => {
            signature!(Generic, 5, 5; Pointer(h_process), Pointer(lp_address), Pointer(lp_enclave_information), Dword(dw_info_length), Pointer(lp_enclave_error))
        }
        "InitializeProcThreadAttributeList" => {
            signature!(Generic, 4, 4; Pointer(lp_attribute_list), Dword(dw_attribute_count), Dword(dw_flags), Pointer(lp_size))
        }
        "InitializeSListHead" => signature!(Void, 1, 1; Pointer(list_head)),
        "InstallELAMCertificateInfo" => signature!(Generic, 1, 1; Pointer(elam_file)),
        "InterlockedPushEntrySList" => {
            signature!(Generic, 2, 2; Pointer(list_head), Pointer(list_entry))
        }
        "InterlockedPushListSListEx" => {
            signature!(Generic, 4, 4; Pointer(list_head), Pointer(list), Pointer(list_end), Dword(count))
        }
        "IsIoRingOpSupported" => signature!(Generic, 2, 2; Pointer(io_ring), Dword(op)),
        "IsProcessCritical" => signature!(Generic, 2, 2; Pointer(h_process), Pointer(critical)),
        "IsWow64GuestMachineSupported" => {
            signature!(Hresult, 2, 2; Word(wow_guest_machine), Pointer(machine_is_supported))
        }
        "IsWow64Process2" => {
            signature!(Generic, 3, 3; Pointer(h_process), Pointer(p_process_machine), Pointer(p_native_machine))
        }
        "LeaveCriticalSectionWhenCallbackReturns" => {
            signature!(Void, 2, 2; Pointer(pci), Pointer(pcs))
        }
        "LoadEnclaveData" => {
            signature!(Generic, 9, 9; Pointer(h_process), Pointer(lp_address), Pointer(lp_buffer), Pointer(n_size), Dword(fl_protect), Pointer(lp_page_information), Dword(dw_info_length), Pointer(lp_number_of_bytes_written), Pointer(lp_enclave_error))
        }
        "LoadStringA" => {
            signature!(Generic, 4, 4; Pointer(h_instance), Dword(u_id), Pointer(lp_buffer), Dword(cch_buffer_max))
        }
        "LocalFileTimeToLocalSystemTime" => {
            signature!(Generic, 3, 3; Pointer(time_zone_information), Pointer(local_file_time), Pointer(local_system_time))
        }
        "LocalSystemTimeToLocalFileTime" => {
            signature!(Generic, 3, 3; Pointer(time_zone_information), Pointer(local_system_time), Pointer(local_file_time))
        }
        "OfferVirtualMemory" => {
            signature!(Generic, 3, 3; Pointer(virtual_address), Pointer(size), Dword(priority))
        }
        "OutputDebugStringA" => signature!(Void, 1, 1; AnsiString(lp_output_string)),
        "OutputDebugStringW" => signature!(Void, 1, 1; WideString(lp_output_string)),
        "PackageFamilyNameFromFullName" => {
            signature!(Win32Status, 3, 3; WideString(package_full_name), Pointer(package_family_name_length), Pointer(package_family_name))
        }
        "PackageFamilyNameFromId" => {
            signature!(Win32Status, 3, 3; Pointer(package_id), Pointer(package_family_name_length), Pointer(package_family_name))
        }
        "PackageFullNameFromId" => {
            signature!(Win32Status, 3, 3; Pointer(package_id), Pointer(package_full_name_length), Pointer(package_full_name))
        }
        "PackageIdFromFullName" => {
            signature!(Win32Status, 4, 4; WideString(package_full_name), Dword(flags), Pointer(buffer_length), Pointer(buffer))
        }
        "PackageNameAndPublisherIdFromFamilyName" => {
            signature!(Win32Status, 5, 5; WideString(package_family_name), Pointer(package_name_length), Pointer(package_name), Pointer(package_publisher_id_length), Pointer(package_publisher_id))
        }
        "ParseApplicationUserModelId" => {
            signature!(Win32Status, 5, 5; WideString(application_user_model_id), Pointer(package_family_name_length), Pointer(package_family_name), Pointer(package_relative_application_id_length), Pointer(package_relative_application_id))
        }
        "PopIoRingCompletion" => signature!(Hresult, 2, 2; Pointer(io_ring), Pointer(cqe)),
        "PrefetchVirtualMemory" => {
            signature!(Generic, 4, 4; Pointer(h_process), Pointer(number_of_entries), Pointer(virtual_addresses), Dword(flags))
        }
        "QueryDepthSList" => signature!(Generic, 1, 1; Pointer(list_head)),
        "QueryIoRingCapabilities" => signature!(Hresult, 1, 1; Pointer(capabilities)),
        "QueryProtectedPolicy" => {
            signature!(Generic, 2, 2; Pointer(policy_guid), Pointer(policy_value))
        }
        "QueueUserAPC2" => {
            signature!(Generic, 4, 4; Pointer(apc_routine), Pointer(thread), Pointer(data), Dword(flags))
        }
        "RaiseException" => {
            signature!(Void, 4, 4; Dword(dw_exception_code), Dword(dw_exception_flags), Dword(n_number_of_arguments), Pointer(lp_arguments))
        }
        "RaiseFailFastException" => {
            signature!(Void, 3, 3; Pointer(p_exception_record), Pointer(p_context_record), Dword(dw_flags))
        }
        "RegisterTraceGuidsW" => {
            signature!(Generic, 8, 8; Pointer(request_address), Pointer(request_context), Pointer(control_guid), Dword(guid_count), Pointer(trace_guid_reg), WideString(mof_image_path), WideString(mof_resource_name), Pointer(registration_handle))
        }
        "RemoveDllDirectory" => signature!(Generic, 1, 1; Pointer(cookie)),
        "RemoveVectoredContinueHandler" => signature!(Generic, 1, 1; Pointer(handle)),
        "SetDefaultDllDirectories" => signature!(Generic, 1, 1; Dword(directory_flags)),
        "SetEventWhenCallbackReturns" => signature!(Void, 2, 2; Pointer(pci), Pointer(evt)),
        "SetHandleCount" => signature!(Generic, 1, 1; Dword(u_number)),
        "SetIoRingCompletionEvent" => signature!(Hresult, 2, 2; Pointer(io_ring), Pointer(h_event)),
        "SetLocaleInfoW" => {
            signature!(Generic, 3, 3; Dword(locale), Dword(lc_type), WideString(lp_lc_data))
        }
        "SetProcessDefaultCpuSetMasks" => {
            signature!(Generic, 3, 3; Pointer(process), Pointer(cpu_set_masks), Word(cpu_set_mask_count))
        }
        "SetProcessDefaultCpuSets" => {
            signature!(Generic, 3, 3; Pointer(process), Pointer(cpu_set_ids), Dword(cpu_set_id_count))
        }
        "SetProcessDynamicEHContinuationTargets" => {
            signature!(Generic, 3, 3; Pointer(process), Word(number_of_targets), Pointer(targets))
        }
        "SetProcessDynamicEnforcedCetCompatibleRanges" => {
            signature!(Generic, 3, 3; Pointer(process), Word(number_of_ranges), Pointer(ranges))
        }
        "SetProcessMitigationPolicy" => {
            signature!(Generic, 3, 3; Dword(mitigation_policy), Pointer(lp_buffer), Pointer(dw_length))
        }
        "SetProcessWorkingSetSize" => {
            signature!(Generic, 3, 3; Pointer(h_process), Pointer(dw_minimum_working_set_size), Pointer(dw_maximum_working_set_size))
        }
        "SetProtectedPolicy" => {
            signature!(Generic, 3, 3; Pointer(policy_guid), Pointer(policy_value), Pointer(old_policy_value))
        }
        "SetThreadDescription" => {
            signature!(Hresult, 2, 2; Pointer(h_thread), WideString(lp_thread_description))
        }
        "SetThreadLocale" => signature!(Generic, 1, 1; Dword(locale)),
        "SetThreadSelectedCpuSetMasks" => {
            signature!(Generic, 3, 3; Pointer(thread), Pointer(cpu_set_masks), Word(cpu_set_mask_count))
        }
        "SetThreadSelectedCpuSets" => {
            signature!(Generic, 3, 3; Pointer(thread), Pointer(cpu_set_ids), Dword(cpu_set_id_count))
        }
        "SetThreadStackGuarantee" => signature!(Generic, 1, 1; Pointer(stack_size_in_bytes)),
        "SetThreadToken" => signature!(Generic, 2, 2; Pointer(thread), Pointer(token)),
        "SetThreadpoolThreadMaximum" => signature!(Void, 2, 2; Pointer(ptpp), Dword(cthrd_most)),
        "SetThreadpoolTimer" => {
            signature!(Void, 4, 4; Pointer(pti), Pointer(pft_due_time), Dword(ms_period), Dword(ms_window_length))
        }
        "SetThreadpoolTimerEx" => {
            signature!(Generic, 4, 4; Pointer(pti), Pointer(pft_due_time), Dword(ms_period), Dword(ms_window_length))
        }
        "SetThreadpoolWait" => {
            signature!(Void, 3, 3; Pointer(pwa), Pointer(h), Pointer(pft_timeout))
        }
        "SetThreadpoolWaitEx" => {
            signature!(Generic, 4, 4; Pointer(pwa), Pointer(h), Pointer(pft_timeout), Pointer(reserved))
        }
        "SleepConditionVariableCS" => {
            signature!(Generic, 3, 3; Pointer(condition_variable), Pointer(critical_section), Dword(dw_milliseconds))
        }
        "SleepConditionVariableSRW" => {
            signature!(Generic, 4, 4; Pointer(condition_variable), Pointer(srw_lock), Dword(dw_milliseconds), Dword(flags))
        }
        "SleepEx" => signature!(Generic, 2, 2; Dword(dw_milliseconds), Dword(b_alertable)),
        "SubmitIoRing" => {
            signature!(Hresult, 4, 4; Pointer(io_ring), Dword(wait_operations), Dword(milliseconds), Pointer(submitted_entries))
        }
        "SubmitThreadpoolWork" => signature!(Void, 1, 1; Pointer(pwk)),
        "SystemTimeToTzSpecificLocalTime" | "SystemTimeToTzSpecificLocalTimeEx" => {
            signature!(Generic, 3, 3; Pointer(lp_time_zone_information), Pointer(lp_universal_time), Pointer(lp_local_time))
        }
        "TerminateProcess" => signature!(Generic, 2, 2; Pointer(h_process), Dword(u_exit_code)),
        "TraceEvent" => signature!(Win32Status, 2, 3; Qword(trace_handle), Pointer(event_trace)),
        "TraceMessage" => {
            signature!(Win32Status, 4, 5; Qword(logger_handle), Dword(message_flags), Pointer(message_guid), Word(message_number))
        }
        "TraceMessageVa" => {
            signature!(Win32Status, 5, 6; Qword(logger_handle), Dword(message_flags), Pointer(message_guid), Word(message_number), Pointer(message_arg_list))
        }
        "TzSpecificLocalTimeToSystemTimeEx" => {
            signature!(Generic, 3, 3; Pointer(lp_time_zone_information), Pointer(lp_local_time), Pointer(lp_universal_time))
        }
        "UnhandledExceptionFilter" => signature!(Generic, 1, 1; Pointer(exception_info)),
        "UnregisterTraceGuids" => signature!(Generic, 1, 2; Qword(registration_handle)),
        "UpdateProcThreadAttribute" => {
            signature!(Generic, 7, 7; Pointer(lp_attribute_list), Dword(dw_flags), Pointer(attribute), Pointer(lp_value), Pointer(cb_size), Pointer(lp_previous_value), Pointer(lp_return_size))
        }
        "VirtualLock" => signature!(Generic, 2, 2; Pointer(lp_address), Pointer(dw_size)),
        "WaitForDebugEventEx" => {
            signature!(Generic, 2, 2; Pointer(lp_debug_event), Dword(dw_milliseconds))
        }
        "WaitForThreadpoolIoCallbacks" => {
            signature!(Void, 2, 2; Pointer(pio), Dword(f_cancel_pending_callbacks))
        }
        "WaitForThreadpoolTimerCallbacks" => {
            signature!(Void, 2, 2; Pointer(pti), Dword(f_cancel_pending_callbacks))
        }
        "WaitForThreadpoolWaitCallbacks" => {
            signature!(Void, 2, 2; Pointer(pwa), Dword(f_cancel_pending_callbacks))
        }
        "WaitForThreadpoolWorkCallbacks" => {
            signature!(Void, 2, 2; Pointer(pwk), Dword(f_cancel_pending_callbacks))
        }
        "WriteConsoleA" | "WriteConsoleW" => {
            signature!(Generic, 5, 5; Pointer(h_console_output), Pointer(lp_buffer), Dword(n_number_of_chars_to_write), Pointer(lp_number_of_chars_written), Pointer(lp_reserved))
        }
        _ => None,
    }
}
