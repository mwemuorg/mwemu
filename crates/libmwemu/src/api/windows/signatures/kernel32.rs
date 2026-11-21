use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "DeviceIoControl" => {
            signature!(Win32Bool, 8, 8; Pointer(h_device), Dword(dw_io_control_code), Pointer(lp_in_buffer), Dword(n_in_buffer_size), Pointer(lp_out_buffer), Dword(n_out_buffer_size), Pointer(lp_bytes_returned), Pointer(lp_overlapped))
        }
        "UnregisterTraceGuids" => super::kernelbase::lookup(name),
        "SetThreadAffinityMask" => signature!(Generic,2,2; Pointer(thread), Pointer(affinity_mask)),
        "SetProcessWorkingSetSize" => super::kernelbase::lookup(name),
        "SetProcessAffinityMask" => {
            signature!(Win32Bool,2,2; Pointer(process), Pointer(affinity_mask))
        }
        "CloseState" => signature!(Void,1,1; Pointer(state)),
        "WakeConditionVariable" => signature!(Void, 1, 1; Pointer(condition_variable)),
        "WakeAllConditionVariable" => signature!(Void, 1, 1; Pointer(condition_variable)),
        "TryAcquireSRWLockShared" => signature!(Generic, 1, 1; Pointer(srw_lock)),
        "TryAcquireSRWLockExclusive" => signature!(Generic, 1, 1; Pointer(srw_lock)),
        "SleepEx" => super::kernelbase::lookup(name),
        "Sleep" => signature!(Void, 1, 1; Dword(milliseconds)),
        "ReleaseSRWLockShared" => signature!(Void, 1, 1; Pointer(srw_lock)),
        "ReleaseSRWLockExclusive" => signature!(Void, 1, 1; Pointer(srw_lock)),
        "LoadLibraryExW" => {
            signature!(Handle, 3, 3; WideString(file_name), Pointer(file), Dword(flags))
        }
        "LoadLibraryExA" => {
            signature!(Handle, 3, 3; AnsiString(file_name), Pointer(file), Dword(flags))
        }
        "IsProcessCritical" => super::kernelbase::lookup(name),
        "InitializeSRWLock" => signature!(Void, 1, 1; Pointer(srw_lock)),
        "InitializeConditionVariable" => signature!(Void, 1, 1; Pointer(condition_variable)),
        "InitOnceInitialize" => signature!(Void, 1, 1; Pointer(init_once)),
        "CreateRemoteThreadEx" => {
            signature!(Handle, 8, 8; Pointer(process), Pointer(attributes), Pointer(stack_size), Pointer(start_address), Pointer(parameter), Dword(creation_flags), Pointer(attribute_list), Pointer(thread_id))
        }
        "CreateRemoteThread" => super::kernelbase::lookup(name),
        "AcquireSRWLockShared" => signature!(Void, 1, 1; Pointer(srw_lock)),
        "AcquireSRWLockExclusive" => signature!(Void, 1, 1; Pointer(srw_lock)),
        "_local_unwind" => signature!(Void, 2, 2; Pointer(target_frame), Pointer(target_ip)),
        "__C_specific_handler" => {
            signature!(Generic, 4, 4; Pointer(exception_record), Pointer(establisher_frame), Pointer(context_record), Pointer(dispatcher_context))
        }
        "WaitForThreadpoolWorkCallbacks" => super::kernelbase::lookup(name),
        "WaitForThreadpoolWaitCallbacks" => super::kernelbase::lookup(name),
        "WaitForThreadpoolTimerCallbacks" => super::kernelbase::lookup(name),
        "WaitForThreadpoolIoCallbacks" => super::kernelbase::lookup(name),
        "WaitForDebugEventEx" => super::kernelbase::lookup(name),
        "UpdateResourceW" => {
            signature!(Win32Bool, 6, 6; Pointer(update), Pointer(resource_type), Pointer(resource_name), Word(language), Pointer(data), Dword(size))
        }
        "UpdateResourceA" => {
            signature!(Win32Bool, 6, 6; Pointer(update), Pointer(resource_type), Pointer(resource_name), Word(language), Pointer(data), Dword(size))
        }
        "SleepConditionVariableSRW" => super::kernelbase::lookup(name),
        "SleepConditionVariableCS" => super::kernelbase::lookup(name),
        "SetIoRingCompletionEvent" => super::kernelbase::lookup(name),
        "SetEventWhenCallbackReturns" => super::kernelbase::lookup(name),
        "SetConsoleInputExeNameW" => signature!(Win32Bool, 1, 1; WideString(name)),
        "SetConsoleInputExeNameA" => signature!(Win32Bool, 1, 1; AnsiString(name)),
        "ReadConsoleInputExW" => {
            signature!(Win32Bool, 5, 5; Pointer(console_input), Pointer(buffer), Dword(length), Pointer(events_read), Word(flags))
        }
        "ReadConsoleInputExA" => {
            signature!(Win32Bool, 5, 5; Pointer(console_input), Pointer(buffer), Dword(length), Pointer(events_read), Word(flags))
        }
        "LeaveCriticalSectionWhenCallbackReturns" => super::kernelbase::lookup(name),
        "IsThreadpoolTimerSet" => signature!(Generic, 1, 1; Pointer(timer)),
        "InterlockedPushListSListEx" => super::kernelbase::lookup(name),
        "InterlockedPushListSList" => {
            signature!(Generic, 4, 4; Pointer(list_head), Pointer(list), Pointer(list_end), Dword(count))
        }
        "InterlockedPushEntrySList" => super::kernelbase::lookup(name),
        "InterlockedPopEntrySList" => signature!(Generic, 1, 1; Pointer(list_head)),
        "InterlockedFlushSList" => signature!(Generic, 1, 1; Pointer(list_head)),
        "GetConsoleInputExeNameW" => {
            signature!(Generic, 2, 2; Dword(buffer_length), Pointer(buffer))
        }
        "GetConsoleInputExeNameA" => {
            signature!(Generic, 2, 2; Dword(buffer_length), Pointer(buffer))
        }
        "FindActCtxSectionStringW" => super::kernelbase::lookup(name),
        "FindActCtxSectionStringA" => {
            signature!(Win32Bool, 5, 5; Dword(flags), Pointer(extension_guid), Dword(section_id), AnsiString(key), Pointer(returned_data))
        }
        "EndUpdateResourceW" => signature!(Win32Bool, 2, 2; Pointer(update), Dword(discard)),
        "EndUpdateResourceA" => signature!(Win32Bool, 2, 2; Pointer(update), Dword(discard)),
        "DuplicateHandle" => {
            signature!(Win32Bool, 7, 7; Pointer(source_process), Pointer(source_handle), Pointer(target_process), Pointer(target_handle), Dword(access), Dword(inherit), Dword(options))
        }
        "CreateProcessW" => {
            signature!(Win32Bool, 10, 10; WideString(application), WideString(command_line), Pointer(process_attributes), Pointer(thread_attributes), Dword(inherit_handles), Dword(creation_flags), Pointer(environment), WideString(current_directory), Pointer(startup_info), Pointer(process_information))
        }
        "CreateProcessA" => {
            signature!(Win32Bool, 10, 10; AnsiString(application), AnsiString(command_line), Pointer(process_attributes), Pointer(thread_attributes), Dword(inherit_handles), Dword(creation_flags), Pointer(environment), AnsiString(current_directory), Pointer(startup_info), Pointer(process_information))
        }
        "CreateIoRing" => {
            signature!(Hresult, 5, 6; Dword(version), Qword(flags), Dword(submission_queue_size), Dword(completion_queue_size), Pointer(ring))
        }
        "CreateEnclave" => {
            signature!(Handle, 8, 8; Pointer(process), Pointer(address), Pointer(size), Pointer(initial_commitment), Dword(enclave_type), Pointer(enclave_information), Dword(info_length), Pointer(enclave_error))
        }
        "CloseThreadpoolWait" => signature!(Void, 1, 1; Pointer(wait)),
        "CloseThreadpoolTimer" => signature!(Void, 1, 1; Pointer(timer)),
        "BeginUpdateResourceW" => {
            signature!(Handle, 2, 2; WideString(filename), Dword(delete_existing))
        }
        "BeginUpdateResourceA" => {
            signature!(Handle, 2, 2; AnsiString(filename), Dword(delete_existing))
        }
        "HeapSetInformation" => {
            signature!(Win32Bool,4,4; Pointer(heap_handle), Dword(heap_information_class), Pointer(heap_information), Pointer(heap_information_length))
        }
        "MultiByteToWideChar" => {
            signature!(Generic,6,6; Dword(code_page), Dword(flags), Pointer(source), Dword(length), Pointer(output), Dword(capacity))
        }
        "SetThreadpoolTimer"
        | "SetThreadpoolTimerEx"
        | "SetThreadpoolWait"
        | "SetThreadpoolWaitEx" => super::kernelbase::lookup(name),
        "FlsAlloc" => signature!(Generic, 1, 1; Pointer(lp_callback)),
        "CreateActCtxA" | "CreateActCtxW" => signature!(Generic, 1, 1; Pointer(act_ctx)),
        "ActivateActCtx" => signature!(Generic, 2, 2; Pointer(h_act_ctx), Pointer(lp_cookie)),
        "AddDllDirectory" => signature!(Generic, 1, 1; WideString(new_directory)),
        "AddVectoredContinueHandler" => signature!(Generic, 2, 2; Dword(first), Pointer(handler)),
        "AppPolicyGetClrCompat"
        | "AppPolicyGetCreateFileAccess"
        | "AppPolicyGetLifecycleManagement"
        | "AppPolicyGetMediaFoundationCodecLoading"
        | "AppPolicyGetProcessTerminationMethod"
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
        "CancelThreadpoolIo" | "CloseThreadpoolIo" | "StartThreadpoolIo" => {
            signature!(Void, 1, 1; Pointer(pio))
        }
        "CeipIsOptedIn" | "GetCurrentProcessorNumber" => signature!(Generic, 0, 0; ),
        "CheckRemoteDebuggerPresent" => {
            signature!(Generic, 2, 2; Pointer(h_process), Pointer(pb_debugger_present))
        }
        "CloseIoRing" => signature!(Hresult, 1, 1; Pointer(io_ring)),
        "ClosePackageInfo" => signature!(Win32Status, 1, 1; Pointer(package_info_reference)),
        "CloseThreadpool" => signature!(Void, 1, 1; Pointer(ptpp)),
        "CloseThreadpoolCleanupGroup" => signature!(Void, 1, 1; Pointer(ptpcg)),
        "CloseThreadpoolCleanupGroupMembers" => {
            signature!(Void, 3, 3; Pointer(ptpcg), Dword(f_cancel_pending_callbacks), Pointer(pv_cleanup_context))
        }
        "CloseThreadpoolWork" | "SubmitThreadpoolWork" => signature!(Void, 1, 1; Pointer(pwk)),
        "CompareStringA" | "CompareStringW" => {
            signature!(Generic, 6, 6; Dword(locale), Dword(dw_cmp_flags), Pointer(lp_string1), Dword(cch_count1), Pointer(lp_string2), Dword(cch_count2))
        }
        "CompareStringOrdinal" => {
            signature!(Generic, 5, 5; Pointer(lp_string1), Dword(cch_count1), Pointer(lp_string2), Dword(cch_count2), Dword(b_ignore_case))
        }
        "DecodeSystemPointer" | "EncodeSystemPointer" => signature!(Generic, 1, 1; Pointer(ptr)),
        "DeleteProcThreadAttributeList" => signature!(Void, 1, 1; Pointer(lp_attribute_list)),
        "DisassociateCurrentThreadFromCallback" => signature!(Void, 1, 1; Pointer(pci)),
        "DiscardVirtualMemory" | "ReclaimVirtualMemory" => {
            signature!(Generic, 2, 2; Pointer(virtual_address), Pointer(size))
        }
        "FindPackagesByPackageFamily" => {
            signature!(Win32Status, 7, 7; WideString(package_family_name), Dword(package_filters), Pointer(count), Pointer(package_full_names), Pointer(buffer_length), Pointer(buffer), Pointer(package_properties))
        }
        "FlsFree" => signature!(Generic, 1, 1; Dword(dw_fls_index)),
        "FlushProcessWriteBuffers" => signature!(Void, 0, 0; ),
        "FormatApplicationUserModelId" => {
            signature!(Win32Status, 4, 4; WideString(package_family_name), WideString(package_relative_application_id), Pointer(application_user_model_id_length), Pointer(application_user_model_id))
        }
        "FreeLibraryWhenCallbackReturns" => signature!(Void, 2, 2; Pointer(pci), Pointer(mod)),
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
        "GetLocaleInfoA" | "GetLocaleInfoW" => {
            signature!(Generic, 4, 4; Dword(locale), Dword(lc_type), Pointer(lp_lc_data), Dword(cch_data))
        }
        "GetLocaleInfoEx" => {
            signature!(Generic, 4, 4; WideString(lp_locale_name), Dword(lc_type), Pointer(lp_lc_data), Dword(cch_data))
        }
        "GetLogicalProcessorInformationEx" => {
            signature!(Generic, 3, 3; Dword(relationship_type), Pointer(buffer), Pointer(returned_length))
        }
        "GetMachineTypeAttributes" => {
            signature!(Hresult, 2, 2; Word(machine), Pointer(machine_type_attributes))
        }
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
        "GetStringTypeW" => {
            signature!(Generic, 4, 4; Dword(dw_info_type), Pointer(lp_src_str), Dword(cch_src), Pointer(lp_char_type))
        }
        "GetStringTypeA" | "GetStringTypeExA" | "GetStringTypeExW" => {
            signature!(Generic, 5, 5; Dword(locale), Dword(dw_info_type), Pointer(lp_src_str), Dword(cch_src), Pointer(lp_char_type))
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
        "InitOnceBeginInitialize" => {
            signature!(Generic, 4, 4; Pointer(lp_init_once), Dword(dw_flags), Pointer(f_pending), Pointer(lp_context))
        }
        "InitOnceComplete" => {
            signature!(Generic, 3, 3; Pointer(lp_init_once), Dword(dw_flags), Pointer(lp_context))
        }
        "InitOnceExecuteOnce" => {
            signature!(Generic, 4, 4; Pointer(init_once), Pointer(init_fn), Pointer(parameter), Pointer(context))
        }
        "InitializeEnclave" => {
            signature!(Generic, 5, 5; Pointer(h_process), Pointer(lp_address), Pointer(lp_enclave_information), Dword(dw_info_length), Pointer(lp_enclave_error))
        }
        "InitializeProcThreadAttributeList" => {
            signature!(Generic, 4, 4; Pointer(lp_attribute_list), Dword(dw_attribute_count), Dword(dw_flags), Pointer(lp_size))
        }
        "InitializeSListHead" => signature!(Void, 1, 1; Pointer(list_head)),
        "InstallELAMCertificateInfo" => signature!(Generic, 1, 1; Pointer(elam_file)),
        "IsEnclaveTypeSupported" => signature!(Generic, 1, 1; Dword(fl_enclave_type)),
        "IsIoRingOpSupported" => signature!(Generic, 2, 2; Pointer(io_ring), Dword(op)),
        "IsUserCetAvailableInEnvironment" => signature!(Generic, 1, 1; Dword(user_cet_environment)),
        "IsWow64GuestMachineSupported" => {
            signature!(Hresult, 2, 2; Word(wow_guest_machine), Pointer(machine_is_supported))
        }
        "IsWow64Process2" => {
            signature!(Generic, 3, 3; Pointer(h_process), Pointer(p_process_machine), Pointer(p_native_machine))
        }
        "LCMapStringA" | "LCMapStringW" => {
            signature!(Generic, 6, 6; Dword(locale), Dword(dw_map_flags), Pointer(lp_src_str), Dword(cch_src), Pointer(lp_dest_str), Dword(cch_dest))
        }
        "LoadEnclaveData" => {
            signature!(Generic, 9, 9; Pointer(h_process), Pointer(lp_address), Pointer(lp_buffer), Pointer(n_size), Dword(fl_protect), Pointer(lp_page_information), Dword(dw_info_length), Pointer(lp_number_of_bytes_written), Pointer(lp_enclave_error))
        }
        "LocalFileTimeToLocalSystemTime" => {
            signature!(Generic, 3, 3; Pointer(time_zone_information), Pointer(local_file_time), Pointer(local_system_time))
        }
        "LocalReAlloc" => {
            signature!(Generic, 3, 3; Pointer(h_mem), Pointer(u_bytes), Dword(u_flags))
        }
        "LocalSystemTimeToLocalFileTime" => {
            signature!(Generic, 3, 3; Pointer(time_zone_information), Pointer(local_system_time), Pointer(local_file_time))
        }
        "MapViewOfFileEx" => {
            signature!(Generic, 6, 6; Pointer(h_file_mapping_object), Dword(dw_desired_access), Dword(dw_file_offset_high), Dword(dw_file_offset_low), Pointer(dw_number_of_bytes_to_map), Pointer(lp_base_address))
        }
        "OfferVirtualMemory" => {
            signature!(Generic, 3, 3; Pointer(virtual_address), Pointer(size), Dword(priority))
        }
        "OpenPackageInfoByFullName" => {
            signature!(Win32Status, 3, 3; WideString(package_full_name), Dword(reserved), Pointer(package_info_reference))
        }
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
        "RaiseFailFastException" => {
            signature!(Void, 3, 3; Pointer(p_exception_record), Pointer(p_context_record), Dword(dw_flags))
        }
        "RemoveDllDirectory" => signature!(Generic, 1, 1; Pointer(cookie)),
        "RemoveVectoredContinueHandler" => signature!(Generic, 1, 1; Pointer(handle)),
        "RtlCaptureStackBackTrace" => {
            signature!(Generic, 4, 4; Dword(frames_to_skip), Dword(frames_to_capture), Pointer(back_trace), Pointer(back_trace_hash))
        }
        "RtlLookupFunctionEntry" => {
            signature!(Generic, 3, 3; Pointer(control_pc), Pointer(image_base), Pointer(history_table))
        }
        "RtlVirtualUnwind" => {
            signature!(Generic, 8, 8; Dword(handler_type), Pointer(image_base), Pointer(control_pc), Pointer(function_entry), Pointer(context_record), Pointer(handler_data), Pointer(establisher_frame), Pointer(context_pointers))
        }
        "SetDefaultDllDirectories" => signature!(Generic, 1, 1; Dword(directory_flags)),
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
        "SetProtectedPolicy" => {
            signature!(Generic, 3, 3; Pointer(policy_guid), Pointer(policy_value), Pointer(old_policy_value))
        }
        "SetThreadDescription" => {
            signature!(Hresult, 2, 2; Pointer(h_thread), WideString(lp_thread_description))
        }
        "SetThreadSelectedCpuSetMasks" => {
            signature!(Generic, 3, 3; Pointer(thread), Pointer(cpu_set_masks), Word(cpu_set_mask_count))
        }
        "SetThreadSelectedCpuSets" => {
            signature!(Generic, 3, 3; Pointer(thread), Pointer(cpu_set_ids), Dword(cpu_set_id_count))
        }
        "SetThreadStackGuarantee" => signature!(Generic, 1, 1; Pointer(stack_size_in_bytes)),
        "SetThreadToken" => signature!(Generic, 2, 2; Pointer(thread), Pointer(token)),
        "SetThreadpoolThreadMaximum" => signature!(Void, 2, 2; Pointer(ptpp), Dword(cthrd_most)),
        "SubmitIoRing" => {
            signature!(Hresult, 4, 4; Pointer(io_ring), Dword(wait_operations), Dword(milliseconds), Pointer(submitted_entries))
        }
        "SystemTimeToTzSpecificLocalTimeEx" => {
            signature!(Generic, 3, 3; Pointer(lp_time_zone_information), Pointer(lp_universal_time), Pointer(lp_local_time))
        }
        "TzSpecificLocalTimeToSystemTimeEx" => {
            signature!(Generic, 3, 3; Pointer(lp_time_zone_information), Pointer(lp_local_time), Pointer(lp_universal_time))
        }
        "UnmapViewOfFileEx" => signature!(Generic, 2, 2),
        "UpdateProcThreadAttribute" => {
            signature!(Generic, 7, 7; Pointer(lp_attribute_list), Dword(dw_flags), Pointer(attribute), Pointer(lp_value), Pointer(cb_size), Pointer(lp_previous_value), Pointer(lp_return_size))
        }
        "VirtualFreeEx" => {
            signature!(Generic, 4, 4; Pointer(h_process), Pointer(lp_address), Pointer(dw_size), Dword(dw_free_type))
        }
        "VirtualLock" => signature!(Generic, 2, 2; Pointer(lp_address), Pointer(dw_size)),
        "WriteConsoleA" => {
            signature!(Generic, 5, 5; Pointer(h_console_output), Pointer(lp_buffer), Dword(n_number_of_chars_to_write), Pointer(lp_number_of_chars_written), Pointer(lp_reserved))
        }
        _ => None,
    }
}
