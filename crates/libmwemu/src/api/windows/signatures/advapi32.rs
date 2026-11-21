use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "LsaClearAuditLog" => signature!(NtStatus, 1, 1; Pointer(policy_handle)),
        "AbortSystemShutdownA" => signature!(Win32Bool, 1, 1; AnsiString(lp_machine_name)),
        "AbortSystemShutdownW" => signature!(Win32Bool, 1, 1; WideString(lp_machine_name)),
        "AccessCheckAndAuditAlarmA" => {
            signature!(Win32Bool, 11, 11; AnsiString(subsystem_name), Pointer(handle_id), AnsiString(object_type_name), AnsiString(object_name), Pointer(security_descriptor), Dword(desired_access), Pointer(generic_mapping), Dword(object_creation), Pointer(granted_access), Pointer(access_status), Pointer(pf_generate_on_close))
        }
        "AccessCheckAndAuditAlarmW" => {
            signature!(Win32Bool, 11, 11; WideString(subsystem_name), Pointer(handle_id), WideString(object_type_name), WideString(object_name), Pointer(security_descriptor), Dword(desired_access), Pointer(generic_mapping), Dword(object_creation), Pointer(granted_access), Pointer(access_status), Pointer(pf_generate_on_close))
        }
        "AccessCheckByTypeAndAuditAlarmA" => {
            signature!(Win32Bool, 16, 16; AnsiString(subsystem_name), Pointer(handle_id), AnsiString(object_type_name), AnsiString(object_name), Pointer(security_descriptor), Pointer(principal_self_sid), Dword(desired_access), Dword(audit_type), Dword(flags), Pointer(object_type_list), Dword(object_type_list_length), Pointer(generic_mapping), Dword(object_creation), Pointer(granted_access), Pointer(access_status), Pointer(pf_generate_on_close))
        }
        "AccessCheckByTypeAndAuditAlarmW" => {
            signature!(Win32Bool, 16, 16; WideString(subsystem_name), Pointer(handle_id), WideString(object_type_name), WideString(object_name), Pointer(security_descriptor), Pointer(principal_self_sid), Dword(desired_access), Dword(audit_type), Dword(flags), Pointer(object_type_list), Dword(object_type_list_length), Pointer(generic_mapping), Dword(object_creation), Pointer(granted_access), Pointer(access_status), Pointer(pf_generate_on_close))
        }
        "AccessCheckByTypeResultListAndAuditAlarmA" => {
            signature!(Win32Bool, 16, 16; AnsiString(subsystem_name), Pointer(handle_id), AnsiString(object_type_name), AnsiString(object_name), Pointer(security_descriptor), Pointer(principal_self_sid), Dword(desired_access), Dword(audit_type), Dword(flags), Pointer(object_type_list), Dword(object_type_list_length), Pointer(generic_mapping), Dword(object_creation), Pointer(granted_access), Pointer(access_status_list), Pointer(pf_generate_on_close))
        }
        "AccessCheckByTypeResultListAndAuditAlarmW" => {
            signature!(Win32Bool, 16, 16; WideString(subsystem_name), Pointer(handle_id), WideString(object_type_name), WideString(object_name), Pointer(security_descriptor), Pointer(principal_self_sid), Dword(desired_access), Dword(audit_type), Dword(flags), Pointer(object_type_list), Dword(object_type_list_length), Pointer(generic_mapping), Dword(object_creation), Pointer(granted_access_list), Pointer(access_status_list), Pointer(pf_generate_on_close))
        }
        "AllocateLocallyUniqueId" => signature!(Generic, 1, 1; Pointer(luid)),
        "AuditLookupCategoryNameA" | "AuditLookupCategoryNameW" => {
            signature!(Generic, 2, 2; Pointer(p_audit_category_guid), Pointer(ppsz_category_name))
        }
        "AuditLookupSubCategoryNameA" | "AuditLookupSubCategoryNameW" => {
            signature!(Generic, 2, 2; Pointer(p_audit_sub_category_guid), Pointer(ppsz_sub_category_name))
        }
        "AuditQueryGlobalSaclA" | "AuditSetGlobalSaclA" => {
            signature!(Generic, 2, 2; AnsiString(object_type_name), Pointer(acl))
        }
        "AuditQueryGlobalSaclW" | "AuditSetGlobalSaclW" => {
            signature!(Generic, 2, 2; WideString(object_type_name), Pointer(acl))
        }
        "BackupEventLogA" | "ClearEventLogA" => {
            signature!(Generic, 2, 2; Pointer(h_event_log), AnsiString(lp_backup_file_name))
        }
        "BackupEventLogW" | "ClearEventLogW" => {
            signature!(Generic, 2, 2; Pointer(h_event_log), WideString(lp_backup_file_name))
        }
        "BuildExplicitAccessWithNameA" => {
            signature!(Void, 5, 5; Pointer(p_explicit_access), AnsiString(p_trustee_name), Dword(access_permissions), Dword(access_mode), Dword(inheritance))
        }
        "BuildExplicitAccessWithNameW" => {
            signature!(Void, 5, 5; Pointer(p_explicit_access), WideString(p_trustee_name), Dword(access_permissions), Dword(access_mode), Dword(inheritance))
        }
        "BuildImpersonateExplicitAccessWithNameA" => {
            signature!(Void, 6, 6; Pointer(p_explicit_access), AnsiString(p_trustee_name), Pointer(p_trustee), Dword(access_permissions), Dword(access_mode), Dword(inheritance))
        }
        "BuildImpersonateExplicitAccessWithNameW" => {
            signature!(Void, 6, 6; Pointer(p_explicit_access), WideString(p_trustee_name), Pointer(p_trustee), Dword(access_permissions), Dword(access_mode), Dword(inheritance))
        }
        "BuildImpersonateTrusteeA" | "BuildImpersonateTrusteeW" => {
            signature!(Void, 2, 2; Pointer(p_trustee), Pointer(p_impersonate_trustee))
        }
        "BuildSecurityDescriptorA" | "BuildSecurityDescriptorW" => {
            signature!(Win32Status, 9, 9; Pointer(p_owner), Pointer(p_group), Dword(c_count_of_access_entries), Pointer(p_list_of_access_entries), Dword(c_count_of_audit_entries), Pointer(p_list_of_audit_entries), Pointer(p_old_sd), Pointer(p_size_new_sd), Pointer(p_new_sd))
        }
        "BuildTrusteeWithNameA" => signature!(Void, 2, 2; Pointer(p_trustee), AnsiString(p_name)),
        "BuildTrusteeWithNameW" => signature!(Void, 2, 2; Pointer(p_trustee), WideString(p_name)),
        "BuildTrusteeWithObjectsAndNameA" => {
            signature!(Void, 6, 6; Pointer(p_trustee), Pointer(p_obj_name), Dword(object_type), AnsiString(object_type_name), AnsiString(inherited_object_type_name), AnsiString(name))
        }
        "BuildTrusteeWithObjectsAndNameW" => {
            signature!(Void, 6, 6; Pointer(p_trustee), Pointer(p_obj_name), Dword(object_type), WideString(object_type_name), WideString(inherited_object_type_name), WideString(name))
        }
        "BuildTrusteeWithObjectsAndSidA" | "BuildTrusteeWithObjectsAndSidW" => {
            signature!(Void, 5, 5; Pointer(p_trustee), Pointer(p_obj_sid), Pointer(p_object_guid), Pointer(p_inherited_object_guid), Pointer(p_sid))
        }
        "BuildTrusteeWithSidA" | "BuildTrusteeWithSidW" => {
            signature!(Void, 2, 2; Pointer(p_trustee), Pointer(p_sid))
        }
        "ChangeServiceConfig2A" | "ChangeServiceConfig2W" => {
            signature!(Win32Bool, 3, 3; Pointer(h_service), Dword(dw_info_level), Pointer(lp_info))
        }
        "ChangeServiceConfigA" => {
            signature!(Win32Bool, 11, 11; Pointer(h_service), Dword(dw_service_type), Dword(dw_start_type), Dword(dw_error_control), AnsiString(lp_binary_path_name), AnsiString(lp_load_order_group), Pointer(lpdw_tag_id), AnsiString(lp_dependencies), AnsiString(lp_service_start_name), Pointer(lp_password), AnsiString(lp_display_name))
        }
        "ChangeServiceConfigW" => {
            signature!(Win32Bool, 11, 11; Pointer(h_service), Dword(dw_service_type), Dword(dw_start_type), Dword(dw_error_control), WideString(lp_binary_path_name), WideString(lp_load_order_group), Pointer(lpdw_tag_id), WideString(lp_dependencies), WideString(lp_service_start_name), Pointer(lp_password), WideString(lp_display_name))
        }
        "ControlServiceExA" | "ControlServiceExW" => {
            signature!(Generic, 4, 4; Pointer(h_service), Dword(dw_control), Dword(dw_info_level), Pointer(p_control_params))
        }
        "ControlTraceA" => {
            signature!(Win32Status, 4, 5; Qword(trace_id), AnsiString(instance_name), Pointer(properties), Dword(control_code))
        }
        "ControlTraceW" => {
            signature!(Win32Status, 4, 5; Qword(trace_id), WideString(instance_name), Pointer(properties), Dword(control_code))
        }
        "ConvertSecurityDescriptorToStringSecurityDescriptorA"
        | "ConvertSecurityDescriptorToStringSecurityDescriptorW" => {
            signature!(Generic, 5, 5; Pointer(security_descriptor), Dword(requested_string_sd_revision), Dword(security_information), Pointer(string_security_descriptor), Pointer(string_security_descriptor_len))
        }
        "ConvertStringSecurityDescriptorToSecurityDescriptorA" => {
            signature!(Generic, 4, 4; AnsiString(string_security_descriptor), Dword(string_sd_revision), Pointer(security_descriptor), Pointer(security_descriptor_size))
        }
        "ConvertStringSecurityDescriptorToSecurityDescriptorW" => {
            signature!(Generic, 4, 4; WideString(string_security_descriptor), Dword(string_sd_revision), Pointer(security_descriptor), Pointer(security_descriptor_size))
        }
        "CreateProcessAsUserA" => {
            signature!(Win32Bool, 11, 11; Pointer(h_token), AnsiString(lp_application_name), Pointer(lp_command_line), Pointer(lp_process_attributes), Pointer(lp_thread_attributes), Dword(b_inherit_handles), Dword(dw_creation_flags), Pointer(lp_environment), AnsiString(lp_current_directory), Pointer(lp_startup_info), Pointer(lp_process_information))
        }
        "CreateProcessAsUserW" => {
            signature!(Win32Bool, 11, 11; Pointer(h_token), WideString(lp_application_name), Pointer(lp_command_line), Pointer(lp_process_attributes), Pointer(lp_thread_attributes), Dword(b_inherit_handles), Dword(dw_creation_flags), Pointer(lp_environment), WideString(lp_current_directory), Pointer(lp_startup_info), Pointer(lp_process_information))
        }
        "CreateProcessWithLogonW" => {
            signature!(Generic, 11, 11; WideString(lp_username), WideString(lp_domain), Pointer(lp_password), Dword(dw_logon_flags), WideString(lp_application_name), Pointer(lp_command_line), Dword(dw_creation_flags), Pointer(lp_environment), WideString(lp_current_directory), Pointer(lp_startup_info), Pointer(lp_process_information))
        }
        "CreateProcessWithTokenW" => {
            signature!(Generic, 9, 9; Pointer(h_token), Dword(dw_logon_flags), WideString(lp_application_name), Pointer(lp_command_line), Dword(dw_creation_flags), Pointer(lp_environment), WideString(lp_current_directory), Pointer(lp_startup_info), Pointer(lp_process_information))
        }
        "CredDeleteA" => {
            signature!(Generic, 3, 3; AnsiString(target_name), Dword(type), Dword(flags))
        }
        "CredDeleteW" => {
            signature!(Generic, 3, 3; WideString(target_name), Dword(type), Dword(flags))
        }
        "CredEnumerateA" => {
            signature!(Generic, 4, 4; AnsiString(filter), Dword(flags), Pointer(count), Pointer(credential))
        }
        "CredEnumerateW" => {
            signature!(Generic, 4, 4; WideString(filter), Dword(flags), Pointer(count), Pointer(credential))
        }
        "CredFindBestCredentialA" | "CredReadA" => {
            signature!(Generic, 4, 4; AnsiString(target_name), Dword(type), Dword(flags), Pointer(credential))
        }
        "CredFindBestCredentialW" | "CredReadW" => {
            signature!(Generic, 4, 4; WideString(target_name), Dword(type), Dword(flags), Pointer(credential))
        }
        "CredGetTargetInfoA" => {
            signature!(Generic, 3, 3; AnsiString(target_name), Dword(flags), Pointer(target_info))
        }
        "CredGetTargetInfoW" => {
            signature!(Generic, 3, 3; WideString(target_name), Dword(flags), Pointer(target_info))
        }
        "CredIsMarshaledCredentialA" | "CredIsMarshaledCredentialW" => {
            signature!(Generic, 1, 1; Pointer(marshaled_credential))
        }
        "CredIsProtectedA" | "CredIsProtectedW" => {
            signature!(Generic, 2, 2; Pointer(psz_protected_credentials), Pointer(p_protection_type))
        }
        "CredMarshalCredentialA" | "CredMarshalCredentialW" => {
            signature!(Generic, 3, 3; Dword(cred_type), Pointer(credential), Pointer(marshaled_credential))
        }
        "CredProtectA" | "CredProtectW" => {
            signature!(Generic, 6, 6; Dword(f_as_self), Pointer(psz_credentials), Dword(cch_credentials), Pointer(psz_protected_credentials), Pointer(pcch_max_chars), Pointer(protection_type))
        }
        "CredReadDomainCredentialsA" | "CredReadDomainCredentialsW" => {
            signature!(Generic, 4, 4; Pointer(target_info), Dword(flags), Pointer(count), Pointer(credential))
        }
        "CredRenameA" => {
            signature!(Generic, 4, 4; AnsiString(old_target_name), AnsiString(new_target_name), Dword(type), Dword(flags))
        }
        "CredRenameW" => {
            signature!(Generic, 4, 4; WideString(old_target_name), WideString(new_target_name), Dword(type), Dword(flags))
        }
        "CredUnmarshalCredentialA" | "CredUnmarshalCredentialW" => {
            signature!(Generic, 3, 3; Pointer(marshaled_credential), Pointer(cred_type), Pointer(credential))
        }
        "CredUnprotectA" | "CredUnprotectW" => {
            signature!(Generic, 5, 5; Dword(f_as_self), Pointer(psz_protected_credentials), Dword(cch_protected_credentials), Pointer(psz_credentials), Pointer(pcch_max_chars))
        }
        "CredWriteA" | "CredWriteW" => signature!(Generic, 2, 2; Pointer(credential), Dword(flags)),
        "CredWriteDomainCredentialsA" | "CredWriteDomainCredentialsW" => {
            signature!(Generic, 3, 3; Pointer(target_info), Pointer(credential), Dword(flags))
        }
        "DecryptFileA" => signature!(Generic, 2, 2; AnsiString(lp_file_name), Dword(dw_reserved)),
        "DecryptFileW" => signature!(Generic, 2, 2; WideString(lp_file_name), Dword(dw_reserved)),
        "EnableTraceEx2" => {
            signature!(Win32Status, 8, 11; Qword(trace_id), Pointer(provider_id), Dword(control_code), Byte(level), Qword(match_any_keyword), Qword(match_all_keyword), Dword(timeout), Pointer(enable_parameters))
        }
        "EncryptFileA" => signature!(Generic, 1, 1; AnsiString(lp_file_name)),
        "EncryptFileW" => signature!(Generic, 1, 1; WideString(lp_file_name)),
        "EnumDependentServicesA" | "EnumDependentServicesW" => {
            signature!(Generic, 6, 6; Pointer(h_service), Dword(dw_service_state), Pointer(lp_services), Dword(cb_buf_size), Pointer(pcb_bytes_needed), Pointer(lp_services_returned))
        }
        "EnumServicesStatusA" | "EnumServicesStatusW" => {
            signature!(Win32Bool, 8, 8; Pointer(h_sc_manager), Dword(dw_service_type), Dword(dw_service_state), Pointer(lp_services), Dword(cb_buf_size), Pointer(pcb_bytes_needed), Pointer(lp_services_returned), Pointer(lp_resume_handle))
        }
        "EnumServicesStatusExA" => {
            signature!(Win32Bool, 10, 10; Pointer(h_sc_manager), Dword(info_level), Dword(dw_service_type), Dword(dw_service_state), Pointer(lp_services), Dword(cb_buf_size), Pointer(pcb_bytes_needed), Pointer(lp_services_returned), Pointer(lp_resume_handle), AnsiString(psz_group_name))
        }
        "EnumServicesStatusExW" => {
            signature!(Win32Bool, 10, 10; Pointer(h_sc_manager), Dword(info_level), Dword(dw_service_type), Dword(dw_service_state), Pointer(lp_services), Dword(cb_buf_size), Pointer(pcb_bytes_needed), Pointer(lp_services_returned), Pointer(lp_resume_handle), WideString(psz_group_name))
        }
        "FileEncryptionStatusA" => {
            signature!(Generic, 2, 2; AnsiString(lp_file_name), Pointer(lp_status))
        }
        "FileEncryptionStatusW" => {
            signature!(Generic, 2, 2; WideString(lp_file_name), Pointer(lp_status))
        }
        "FlushTraceA" | "QueryTraceA" | "StopTraceA" | "UpdateTraceA" => {
            signature!(Win32Status, 3, 4; Qword(trace_id), AnsiString(instance_name), Pointer(properties))
        }
        "FlushTraceW" | "QueryTraceW" | "StopTraceW" | "UpdateTraceW" => {
            signature!(Win32Status, 3, 4; Qword(trace_id), WideString(instance_name), Pointer(properties))
        }
        "GetAuditedPermissionsFromAclA" | "GetAuditedPermissionsFromAclW" => {
            signature!(Win32Status, 4, 4; Pointer(pacl), Pointer(p_trustee), Pointer(p_successful_audited_rights), Pointer(p_failed_audit_rights))
        }
        "GetCurrentHwProfileA" | "GetCurrentHwProfileW" => {
            signature!(Generic, 1, 1; Pointer(lp_hw_profile_info))
        }
        "GetEffectiveRightsFromAclA" | "GetEffectiveRightsFromAclW" => {
            signature!(Win32Status, 3, 3; Pointer(pacl), Pointer(p_trustee), Pointer(p_access_rights))
        }
        "GetExplicitEntriesFromAclA" | "GetExplicitEntriesFromAclW" => {
            signature!(Win32Status, 3, 3; Pointer(pacl), Pointer(pc_count_of_explicit_entries), Pointer(p_list_of_explicit_entries))
        }
        "GetFileSecurityA" => {
            signature!(Win32Bool, 5, 5; AnsiString(lp_file_name), Dword(requested_information), Pointer(p_security_descriptor), Dword(n_length), Pointer(lpn_length_needed))
        }
        "GetFileSecurityW" => {
            signature!(Win32Bool, 5, 5; WideString(lp_file_name), Dword(requested_information), Pointer(p_security_descriptor), Dword(n_length), Pointer(lpn_length_needed))
        }
        "GetInheritanceSourceA" => {
            signature!(Win32Status, 10, 10; AnsiString(p_object_name), Dword(object_type), Dword(security_info), Dword(container), Pointer(p_object_class_guids), Dword(guid_count), Pointer(p_acl), Pointer(pfn_array), Pointer(p_generic_mapping), Pointer(p_inherit_array))
        }
        "GetInheritanceSourceW" => {
            signature!(Win32Status, 10, 10; WideString(p_object_name), Dword(object_type), Dword(security_info), Dword(container), Pointer(p_object_class_guids), Dword(guid_count), Pointer(p_acl), Pointer(pfn_array), Pointer(p_generic_mapping), Pointer(p_inherit_array))
        }
        "GetMultipleTrusteeA"
        | "GetMultipleTrusteeOperationA"
        | "GetMultipleTrusteeOperationW"
        | "GetMultipleTrusteeW"
        | "GetTrusteeFormA"
        | "GetTrusteeFormW"
        | "GetTrusteeNameA"
        | "GetTrusteeNameW"
        | "GetTrusteeTypeA"
        | "GetTrusteeTypeW" => signature!(Generic, 1, 1; Pointer(p_trustee)),
        "GetNamedSecurityInfoA" => {
            signature!(Win32Status, 8, 8; AnsiString(p_object_name), Dword(object_type), Dword(security_info), Pointer(ppsid_owner), Pointer(ppsid_group), Pointer(pp_dacl), Pointer(pp_sacl), Pointer(pp_security_descriptor))
        }
        "GetNamedSecurityInfoW" => {
            signature!(Win32Status, 8, 8; WideString(p_object_name), Dword(object_type), Dword(security_info), Pointer(ppsid_owner), Pointer(ppsid_group), Pointer(pp_dacl), Pointer(pp_sacl), Pointer(pp_security_descriptor))
        }
        "GetSecurityDescriptorDacl" => {
            signature!(Generic, 4, 4; Pointer(p_security_descriptor), Pointer(lpb_dacl_present), Pointer(p_dacl), Pointer(lpb_dacl_defaulted))
        }
        "GetSecurityInfo" => {
            signature!(Win32Status, 8, 8; Pointer(handle), Dword(object_type), Dword(security_info), Pointer(ppsid_owner), Pointer(ppsid_group), Pointer(pp_dacl), Pointer(pp_sacl), Pointer(pp_security_descriptor))
        }
        "GetServiceDisplayNameA" => {
            signature!(Generic, 4, 4; Pointer(h_sc_manager), AnsiString(lp_service_name), Pointer(lp_display_name), Pointer(lpcch_buffer))
        }
        "GetServiceDisplayNameW" => {
            signature!(Generic, 4, 4; Pointer(h_sc_manager), WideString(lp_service_name), Pointer(lp_display_name), Pointer(lpcch_buffer))
        }
        "GetServiceKeyNameA" => {
            signature!(Generic, 4, 4; Pointer(h_sc_manager), AnsiString(lp_display_name), Pointer(lp_service_name), Pointer(lpcch_buffer))
        }
        "GetServiceKeyNameW" => {
            signature!(Generic, 4, 4; Pointer(h_sc_manager), WideString(lp_display_name), Pointer(lp_service_name), Pointer(lpcch_buffer))
        }
        "EventAccessQuery" => {
            signature!(Win32Status, 3, 3; Pointer(guid), Pointer(buffer), Pointer(buffer_size))
        }
        "GetThreadWaitChain" => {
            signature!(Win32Bool, 7, 7; Pointer(session), Pointer(context), Dword(flags), Dword(thread_id), Pointer(node_count), Pointer(nodes), Pointer(is_cycle))
        }
        "EventProviderEnabled" => {
            signature!(Generic, 3, 5; Qword(reg_handle), Byte(level), Qword(keyword))
        }
        "GetTraceLoggerHandle" => signature!(Generic, 1, 1; Pointer(buffer)),
        "InitializeSid" => {
            signature!(Generic, 3, 3; Pointer(sid), Pointer(p_identifier_authority), Byte(n_sub_authority_count))
        }
        "InitiateShutdownA" => {
            signature!(Generic, 5, 5; AnsiString(lp_machine_name), AnsiString(lp_message), Dword(dw_grace_period), Dword(dw_shutdown_flags), Dword(dw_reason))
        }
        "InitiateShutdownW" => {
            signature!(Generic, 5, 5; WideString(lp_machine_name), WideString(lp_message), Dword(dw_grace_period), Dword(dw_shutdown_flags), Dword(dw_reason))
        }
        "InitiateSystemShutdownA" => {
            signature!(Generic, 5, 5; AnsiString(lp_machine_name), AnsiString(lp_message), Dword(dw_timeout), Dword(b_force_apps_closed), Dword(b_reboot_after_shutdown))
        }
        "InitiateSystemShutdownExA" => {
            signature!(Generic, 6, 6; AnsiString(lp_machine_name), AnsiString(lp_message), Dword(dw_timeout), Dword(b_force_apps_closed), Dword(b_reboot_after_shutdown), Dword(dw_reason))
        }
        "InitiateSystemShutdownExW" => {
            signature!(Generic, 6, 6; WideString(lp_machine_name), WideString(lp_message), Dword(dw_timeout), Dword(b_force_apps_closed), Dword(b_reboot_after_shutdown), Dword(dw_reason))
        }
        "InitiateSystemShutdownW" => {
            signature!(Generic, 5, 5; WideString(lp_machine_name), WideString(lp_message), Dword(dw_timeout), Dword(b_force_apps_closed), Dword(b_reboot_after_shutdown))
        }
        "LogonUserA" => {
            signature!(Win32Bool, 6, 6; AnsiString(lpsz_username), AnsiString(lpsz_domain), Pointer(lpsz_password), Dword(dw_logon_type), Dword(dw_logon_provider), Pointer(ph_token))
        }
        "LogonUserExA" => {
            signature!(Generic, 10, 10; AnsiString(lpsz_username), AnsiString(lpsz_domain), Pointer(lpsz_password), Dword(dw_logon_type), Dword(dw_logon_provider), Pointer(ph_token), Pointer(pp_logon_sid), Pointer(pp_profile_buffer), Pointer(pdw_profile_length), Pointer(p_quota_limits))
        }
        "LogonUserExW" => {
            signature!(Generic, 10, 10; WideString(lpsz_username), WideString(lpsz_domain), Pointer(lpsz_password), Dword(dw_logon_type), Dword(dw_logon_provider), Pointer(ph_token), Pointer(pp_logon_sid), Pointer(pp_profile_buffer), Pointer(pdw_profile_length), Pointer(p_quota_limits))
        }
        "LogonUserW" => {
            signature!(Win32Bool, 6, 6; WideString(lpsz_username), WideString(lpsz_domain), Pointer(lpsz_password), Dword(dw_logon_type), Dword(dw_logon_provider), Pointer(ph_token))
        }
        "NotifyServiceStatusChangeA" | "NotifyServiceStatusChangeW" => {
            signature!(Generic, 3, 3; Pointer(h_service), Dword(dw_notify_mask), Pointer(p_notify_buffer))
        }
        "ObjectCloseAuditAlarmA" | "ObjectDeleteAuditAlarmA" => {
            signature!(Generic, 3, 3; AnsiString(subsystem_name), Pointer(handle_id), Dword(generate_on_close))
        }
        "ObjectCloseAuditAlarmW" | "ObjectDeleteAuditAlarmW" => {
            signature!(Generic, 3, 3; WideString(subsystem_name), Pointer(handle_id), Dword(generate_on_close))
        }
        "ObjectOpenAuditAlarmA" => {
            signature!(Generic, 12, 12; AnsiString(subsystem_name), Pointer(handle_id), AnsiString(object_type_name), AnsiString(object_name), Pointer(p_security_descriptor), Pointer(client_token), Dword(desired_access), Dword(granted_access), Pointer(privileges), Dword(object_creation), Dword(access_granted), Pointer(generate_on_close))
        }
        "ObjectOpenAuditAlarmW" => {
            signature!(Generic, 12, 12; WideString(subsystem_name), Pointer(handle_id), WideString(object_type_name), WideString(object_name), Pointer(p_security_descriptor), Pointer(client_token), Dword(desired_access), Dword(granted_access), Pointer(privileges), Dword(object_creation), Dword(access_granted), Pointer(generate_on_close))
        }
        "ObjectPrivilegeAuditAlarmA" => {
            signature!(Generic, 6, 6; AnsiString(subsystem_name), Pointer(handle_id), Pointer(client_token), Dword(desired_access), Pointer(privileges), Dword(access_granted))
        }
        "ObjectPrivilegeAuditAlarmW" => {
            signature!(Generic, 6, 6; WideString(subsystem_name), Pointer(handle_id), Pointer(client_token), Dword(desired_access), Pointer(privileges), Dword(access_granted))
        }
        "OpenBackupEventLogA" => {
            signature!(Generic, 2, 2; AnsiString(lp_unc_server_name), AnsiString(lp_file_name))
        }
        "OpenBackupEventLogW" => {
            signature!(Generic, 2, 2; WideString(lp_unc_server_name), WideString(lp_file_name))
        }
        "OpenEncryptedFileRawA" => {
            signature!(Generic, 3, 3; AnsiString(lp_file_name), Dword(ul_flags), Pointer(pv_context))
        }
        "OpenEncryptedFileRawW" => {
            signature!(Generic, 3, 3; WideString(lp_file_name), Dword(ul_flags), Pointer(pv_context))
        }
        "OpenEventLogA" | "RegisterEventSourceA" => {
            signature!(Generic, 2, 2; AnsiString(lp_unc_server_name), AnsiString(lp_source_name))
        }
        "OpenEventLogW" | "RegisterEventSourceW" => {
            signature!(Generic, 2, 2; WideString(lp_unc_server_name), WideString(lp_source_name))
        }
        "OpenTraceA" | "OpenTraceW" => signature!(Generic, 1, 1; Pointer(logfile)),
        "PrivilegedServiceAuditAlarmA" => {
            signature!(Generic, 5, 5; AnsiString(subsystem_name), AnsiString(service_name), Pointer(client_token), Pointer(privileges), Dword(access_granted))
        }
        "PrivilegedServiceAuditAlarmW" => {
            signature!(Generic, 5, 5; WideString(subsystem_name), WideString(service_name), Pointer(client_token), Pointer(privileges), Dword(access_granted))
        }
        "QueryAllTracesA" | "QueryAllTracesW" => {
            signature!(Win32Status, 3, 3; Pointer(property_array), Dword(property_array_count), Pointer(logger_count))
        }
        "QueryServiceConfig2A" | "QueryServiceConfig2W" => {
            signature!(Win32Bool, 5, 5; Pointer(h_service), Dword(dw_info_level), Pointer(lp_buffer), Dword(cb_buf_size), Pointer(pcb_bytes_needed))
        }
        "QueryServiceConfigA" | "QueryServiceConfigW" => {
            signature!(Win32Bool, 4, 4; Pointer(h_service), Pointer(lp_service_config), Dword(cb_buf_size), Pointer(pcb_bytes_needed))
        }
        "QueryServiceLockStatusA" | "QueryServiceLockStatusW" => {
            signature!(Win32Bool, 4, 4; Pointer(h_sc_manager), Pointer(lp_lock_status), Dword(cb_buf_size), Pointer(pcb_bytes_needed))
        }
        "QueryTraceProcessingHandle" => signature!(Win32Status, 7, 8),
        "ReadEventLogA" | "ReadEventLogW" => {
            signature!(Generic, 7, 7; Pointer(h_event_log), Dword(dw_read_flags), Dword(dw_record_offset), Pointer(lp_buffer), Dword(n_number_of_bytes_to_read), Pointer(pn_bytes_read), Pointer(pn_min_number_of_bytes_needed))
        }
        "RegConnectRegistryExA" => {
            signature!(Generic, 4, 4; AnsiString(lp_machine_name), Pointer(h_key), Dword(flags), Pointer(phk_result))
        }
        "RegConnectRegistryExW" => {
            signature!(Generic, 4, 4; WideString(lp_machine_name), Pointer(h_key), Dword(flags), Pointer(phk_result))
        }
        "RegCopyTreeA" => {
            signature!(Win32Status, 3, 3; Pointer(h_key_src), AnsiString(lp_sub_key), Pointer(h_key_dest))
        }
        "RegCopyTreeW" => {
            signature!(Win32Status, 3, 3; Pointer(h_key_src), WideString(lp_sub_key), Pointer(h_key_dest))
        }
        "RegLoadAppKeyA" => {
            signature!(Win32Status, 5, 5; AnsiString(lp_file), Pointer(phk_result), Dword(sam_desired), Dword(dw_options), Dword(reserved))
        }
        "RegLoadAppKeyW" => {
            signature!(Win32Status, 5, 5; WideString(lp_file), Pointer(phk_result), Dword(sam_desired), Dword(dw_options), Dword(reserved))
        }
        "RegLoadMUIStringA" => {
            signature!(Win32Status, 7, 7; Pointer(h_key), AnsiString(psz_value), Pointer(psz_out_buf), Dword(cb_out_buf), Pointer(pcb_data), Dword(flags), AnsiString(psz_directory))
        }
        "RegLoadMUIStringW" => {
            signature!(Win32Status, 7, 7; Pointer(h_key), WideString(psz_value), Pointer(psz_out_buf), Dword(cb_out_buf), Pointer(pcb_data), Dword(flags), WideString(psz_directory))
        }
        "RegSaveKeyExA" => {
            signature!(Win32Status, 4, 4; Pointer(h_key), AnsiString(lp_file), Pointer(lp_security_attributes), Dword(flags))
        }
        "RegSaveKeyExW" => {
            signature!(Win32Status, 4, 4; Pointer(h_key), WideString(lp_file), Pointer(lp_security_attributes), Dword(flags))
        }
        "RegisterServiceCtrlHandlerA" => {
            signature!(Generic, 2, 2; AnsiString(lp_service_name), Pointer(lp_handler_proc))
        }
        "RegisterServiceCtrlHandlerExA" => {
            signature!(Generic, 3, 3; AnsiString(lp_service_name), Pointer(lp_handler_proc), Pointer(lp_context))
        }
        "RegisterServiceCtrlHandlerExW" => {
            signature!(Generic, 3, 3; WideString(lp_service_name), Pointer(lp_handler_proc), Pointer(lp_context))
        }
        "RegisterServiceCtrlHandlerW" => {
            signature!(Generic, 2, 2; WideString(lp_service_name), Pointer(lp_handler_proc))
        }
        "RegisterTraceGuidsA" => {
            signature!(Generic, 8, 8; Pointer(request_address), Pointer(request_context), Pointer(control_guid), Dword(guid_count), Pointer(trace_guid_reg), AnsiString(mof_image_path), AnsiString(mof_resource_name), Pointer(registration_handle))
        }
        "RegisterTraceGuidsW" => {
            signature!(Generic, 8, 8; Pointer(request_address), Pointer(request_context), Pointer(control_guid), Dword(guid_count), Pointer(trace_guid_reg), WideString(mof_image_path), WideString(mof_resource_name), Pointer(registration_handle))
        }
        "ReportEventA" | "ReportEventW" => {
            signature!(Generic, 9, 9; Pointer(h_event_log), Word(w_type), Word(w_category), Dword(dw_event_id), Pointer(lp_user_sid), Word(w_num_strings), Dword(dw_data_size), Pointer(lp_strings), Pointer(lp_raw_data))
        }
        "RevertToSelf" => signature!(Generic, 0, 0; ),
        "SetEntriesInAclA" | "SetEntriesInAclW" => {
            signature!(Win32Status, 4, 4; Dword(c_count_of_explicit_entries), Pointer(p_list_of_explicit_entries), Pointer(old_acl), Pointer(new_acl))
        }
        "SetFileSecurityA" => {
            signature!(Win32Bool, 3, 3; AnsiString(lp_file_name), Dword(security_information), Pointer(p_security_descriptor))
        }
        "SetFileSecurityW" => {
            signature!(Win32Bool, 3, 3; WideString(lp_file_name), Dword(security_information), Pointer(p_security_descriptor))
        }
        "SetNamedSecurityInfoA" => {
            signature!(Win32Status, 7, 7; AnsiString(p_object_name), Dword(object_type), Dword(security_info), Pointer(psid_owner), Pointer(psid_group), Pointer(p_dacl), Pointer(p_sacl))
        }
        "SetNamedSecurityInfoW" => {
            signature!(Win32Status, 7, 7; WideString(p_object_name), Dword(object_type), Dword(security_info), Pointer(psid_owner), Pointer(psid_group), Pointer(p_dacl), Pointer(p_sacl))
        }
        "StartTraceA" => {
            signature!(Win32Status, 3, 3; Pointer(trace_id), AnsiString(instance_name), Pointer(properties))
        }
        "StartTraceW" => {
            signature!(Win32Status, 3, 3; Pointer(trace_id), WideString(instance_name), Pointer(properties))
        }
        "TraceMessage" => {
            signature!(Win32Status, 4, 5; Qword(logger_handle), Dword(message_flags), Pointer(message_guid), Word(message_number))
        }
        "TraceMessageVa" => {
            signature!(Win32Status, 5, 6; Qword(logger_handle), Dword(message_flags), Pointer(message_guid), Word(message_number), Pointer(message_arg_list))
        }
        "TraceQueryInformation" => {
            signature!(Win32Status, 5, 6; Qword(trace_id), Dword(information_class), Pointer(trace_information), Dword(information_length), Pointer(return_length))
        }
        "TraceSetInformation" => {
            signature!(Win32Status, 4, 5; Qword(trace_id), Dword(information_class), Pointer(trace_information), Dword(information_length))
        }
        "TreeResetNamedSecurityInfoA" => {
            signature!(Win32Status, 11, 11; AnsiString(p_object_name), Dword(object_type), Dword(security_info), Pointer(p_owner), Pointer(p_group), Pointer(p_dacl), Pointer(p_sacl), Dword(keep_explicit), Pointer(fn_progress), Dword(progress_invoke_setting), Pointer(args))
        }
        "TreeResetNamedSecurityInfoW" => {
            signature!(Win32Status, 11, 11; WideString(p_object_name), Dword(object_type), Dword(security_info), Pointer(p_owner), Pointer(p_group), Pointer(p_dacl), Pointer(p_sacl), Dword(keep_explicit), Pointer(fn_progress), Dword(progress_invoke_setting), Pointer(args))
        }
        "TreeSetNamedSecurityInfoA" => {
            signature!(Win32Status, 11, 11; AnsiString(p_object_name), Dword(object_type), Dword(security_info), Pointer(p_owner), Pointer(p_group), Pointer(p_dacl), Pointer(p_sacl), Dword(dw_action), Pointer(fn_progress), Dword(progress_invoke_setting), Pointer(args))
        }
        "TreeSetNamedSecurityInfoW" => {
            signature!(Win32Status, 11, 11; WideString(p_object_name), Dword(object_type), Dword(security_info), Pointer(p_owner), Pointer(p_group), Pointer(p_dacl), Pointer(p_sacl), Dword(dw_action), Pointer(fn_progress), Dword(progress_invoke_setting), Pointer(args))
        }
        "AccessCheckByTypeResultListAndAuditAlarmByHandleA" => {
            signature!(Win32Bool, 17, 17; AnsiString(subsystem_name), Pointer(handle_id), Pointer(client_token), AnsiString(object_type_name), AnsiString(object_name), Pointer(security_descriptor), Pointer(principal_self_sid), Dword(desired_access), Dword(audit_type), Dword(flags), Pointer(object_type_list), Dword(object_type_list_length), Pointer(generic_mapping), Dword(object_creation), Pointer(granted_access), Pointer(access_status_list), Pointer(pf_generate_on_close))
        }
        "AccessCheckByTypeResultListAndAuditAlarmByHandleW" => {
            signature!(Win32Bool, 17, 17; WideString(subsystem_name), Pointer(handle_id), Pointer(client_token), WideString(object_type_name), WideString(object_name), Pointer(security_descriptor), Pointer(principal_self_sid), Dword(desired_access), Dword(audit_type), Dword(flags), Pointer(object_type_list), Dword(object_type_list_length), Pointer(generic_mapping), Dword(object_creation), Pointer(granted_access_list), Pointer(access_status_list), Pointer(pf_generate_on_close))
        }
        "AdjustTokenPrivileges" => {
            signature!(Win32Bool, 6, 6; Pointer(token_handle), Dword(disable_all_privileges), Pointer(new_state), Dword(buffer_length), Pointer(previous_state), Pointer(return_length))
        }
        "AllocateAndInitializeSid" => {
            signature!(Win32Bool, 11, 11; Pointer(p_identifier_authority), Byte(n_sub_authority_count), Dword(n_sub_authority0), Dword(n_sub_authority1), Dword(n_sub_authority2), Dword(n_sub_authority3), Dword(n_sub_authority4), Dword(n_sub_authority5), Dword(n_sub_authority6), Dword(n_sub_authority7), Pointer(p_sid))
        }
        "CheckTokenMembership" => {
            signature!(Win32Bool, 3, 3; Pointer(token_handle), Pointer(sid_to_check), Pointer(is_member))
        }
        "ControlService" => {
            signature!(Win32Bool, 3, 3; Pointer(h_service), Dword(dw_control), Pointer(lp_service_status))
        }
        "ConvertStringSidToSidA" => {
            signature!(Win32Bool, 2, 2; AnsiString(string_sid), Pointer(sid))
        }
        "ConvertStringSidToSidW" => {
            signature!(Win32Bool, 2, 2; WideString(string_sid), Pointer(sid))
        }
        "CreatePrivateObjectSecurity" => {
            signature!(Win32Bool, 6, 6; Pointer(parent_descriptor), Pointer(creator_descriptor), Pointer(new_descriptor), Dword(is_directory_object), Pointer(token), Pointer(generic_mapping))
        }
        "CreatePrivateObjectSecurityEx" => {
            signature!(Win32Bool, 8, 8; Pointer(parent_descriptor), Pointer(creator_descriptor), Pointer(new_descriptor), Pointer(object_type), Dword(is_container_object), Dword(auto_inherit_flags), Pointer(token), Pointer(generic_mapping))
        }
        "CreateRestrictedToken" => {
            signature!(Win32Bool, 9, 9; Pointer(existing_token_handle), Dword(flags), Dword(disable_sid_count), Pointer(sids_to_disable), Dword(delete_privilege_count), Pointer(privileges_to_delete), Dword(restricted_sid_count), Pointer(sids_to_restrict), Pointer(new_token_handle))
        }
        "CreateTraceInstanceId" => {
            signature!(Win32Status, 2, 2; Pointer(reg_handle), Pointer(inst_info))
        }
        "CreateWellKnownSid" => {
            signature!(Win32Bool, 4, 4; Dword(well_known_sid_type), Pointer(domain_sid), Pointer(p_sid), Pointer(cb_sid))
        }
        "CryptDeriveKey" => {
            signature!(Win32Bool, 5, 5; Pointer(h_prov), Dword(algid), Pointer(h_base_data), Dword(dw_flags), Pointer(ph_key))
        }
        "CryptDuplicateHash" => {
            signature!(Win32Bool, 4, 4; Pointer(h_hash), Pointer(pdw_reserved), Dword(dw_flags), Pointer(ph_hash))
        }
        "CryptDuplicateKey" => {
            signature!(Win32Bool, 4, 4; Pointer(h_key), Pointer(pdw_reserved), Dword(dw_flags), Pointer(ph_key))
        }
        "CryptEnumProviderTypesA" => {
            signature!(Win32Bool, 6, 6; Dword(dw_index), Pointer(pdw_reserved), Dword(dw_flags), Pointer(pdw_prov_type), Pointer(sz_type_name), Pointer(pcb_type_name))
        }
        "CryptEnumProviderTypesW" => {
            signature!(Win32Bool, 6, 6; Dword(dw_index), Pointer(pdw_reserved), Dword(dw_flags), Pointer(pdw_prov_type), Pointer(sz_type_name), Pointer(pcb_type_name))
        }
        "CryptEnumProvidersA" => {
            signature!(Win32Bool, 6, 6; Dword(dw_index), Pointer(pdw_reserved), Dword(dw_flags), Pointer(pdw_prov_type), Pointer(sz_prov_name), Pointer(pcb_prov_name))
        }
        "CryptEnumProvidersW" => {
            signature!(Win32Bool, 6, 6; Dword(dw_index), Pointer(pdw_reserved), Dword(dw_flags), Pointer(pdw_prov_type), Pointer(sz_prov_name), Pointer(pcb_prov_name))
        }
        "CryptExportKey" => {
            signature!(Win32Bool, 6, 6; Pointer(h_key), Pointer(h_exp_key), Dword(dw_blob_type), Dword(dw_flags), Pointer(pb_data), Pointer(pdw_data_len))
        }
        "CryptGenKey" => {
            signature!(Win32Bool, 4, 4; Pointer(h_prov), Dword(algid), Dword(dw_flags), Pointer(ph_key))
        }
        "CryptGetDefaultProviderA" => {
            signature!(Win32Bool, 5, 5; Dword(dw_prov_type), Pointer(pdw_reserved), Dword(dw_flags), Pointer(psz_prov_name), Pointer(pcb_prov_name))
        }
        "CryptGetDefaultProviderW" => {
            signature!(Win32Bool, 5, 5; Dword(dw_prov_type), Pointer(pdw_reserved), Dword(dw_flags), Pointer(psz_prov_name), Pointer(pcb_prov_name))
        }
        "CryptGetHashParam" => {
            signature!(Win32Bool, 5, 5; Pointer(h_hash), Dword(dw_param), Pointer(pb_data), Pointer(pdw_data_len), Dword(dw_flags))
        }
        "CryptGetKeyParam" => {
            signature!(Win32Bool, 5, 5; Pointer(h_key), Dword(dw_param), Pointer(pb_data), Pointer(pdw_data_len), Dword(dw_flags))
        }
        "CryptGetProvParam" => {
            signature!(Win32Bool, 5, 5; Pointer(h_prov), Dword(dw_param), Pointer(pb_data), Pointer(pdw_data_len), Dword(dw_flags))
        }
        "CryptGetUserKey" => {
            signature!(Win32Bool, 3, 3; Pointer(h_prov), Dword(dw_key_spec), Pointer(ph_user_key))
        }
        "CryptHashSessionKey" => {
            signature!(Win32Bool, 3, 3; Pointer(h_hash), Pointer(h_key), Dword(dw_flags))
        }
        "CryptImportKey" => {
            signature!(Win32Bool, 6, 6; Pointer(h_prov), Pointer(pb_data), Dword(dw_data_len), Pointer(h_pub_key), Dword(dw_flags), Pointer(ph_key))
        }
        "CryptSetKeyParam" => {
            signature!(Win32Bool, 4, 4; Pointer(h_key), Dword(dw_param), Pointer(pb_data), Dword(dw_flags))
        }
        "CryptSetProviderA" => {
            signature!(Win32Bool, 2, 2; AnsiString(psz_prov_name), Dword(dw_prov_type))
        }
        "CryptSetProviderExA" => {
            signature!(Win32Bool, 4, 4; AnsiString(psz_prov_name), Dword(dw_prov_type), Pointer(pdw_reserved), Dword(dw_flags))
        }
        "CryptSetProviderExW" => {
            signature!(Win32Bool, 4, 4; WideString(psz_prov_name), Dword(dw_prov_type), Pointer(pdw_reserved), Dword(dw_flags))
        }
        "CryptSetProviderW" => {
            signature!(Win32Bool, 2, 2; WideString(psz_prov_name), Dword(dw_prov_type))
        }
        "CryptVerifySignatureA" => {
            signature!(Win32Bool, 6, 6; Pointer(h_hash), Pointer(pb_signature), Dword(dw_sig_len), Pointer(h_pub_key), AnsiString(sz_description), Dword(dw_flags))
        }
        "CryptVerifySignatureW" => {
            signature!(Win32Bool, 6, 6; Pointer(h_hash), Pointer(pb_signature), Dword(dw_sig_len), Pointer(h_pub_key), WideString(sz_description), Dword(dw_flags))
        }
        "CveEventWrite" => {
            signature!(Win32Status, 2, 2; WideString(cve_id), WideString(additional_details))
        }
        "DuplicateToken" => {
            signature!(Win32Bool, 3, 3; Pointer(existing_token_handle), Dword(impersonation_level), Pointer(duplicate_token_handle))
        }
        "DuplicateTokenEx" => {
            signature!(Win32Bool, 6, 6; Pointer(h_existing_token), Dword(dw_desired_access), Pointer(lp_token_attributes), Dword(impersonation_level), Dword(token_type), Pointer(ph_new_token))
        }
        "EventAccessControl" => {
            signature!(Win32Status, 5, 5; Pointer(guid), Dword(operation), Pointer(sid), Dword(rights), Byte(allow_or_deny))
        }
        "EventActivityIdControl" => {
            signature!(Win32Status, 2, 2; Dword(control_code), Pointer(activity_id))
        }
        "EventRegister" => {
            signature!(Win32Status, 4, 4; Pointer(provider_id), Pointer(enable_callback), Pointer(callback_context), Pointer(reg_handle))
        }
        "EventSetInformation" => {
            signature!(Win32Status, 4, 5; Qword(reg_handle), Dword(information_class), Pointer(event_information), Dword(information_length))
        }
        "EventUnregister" => signature!(Win32Status, 1, 2; Qword(reg_handle)),
        "EventWrite" => {
            signature!(Win32Status, 4, 5; Qword(reg_handle), Pointer(event_descriptor), Dword(user_data_count), Pointer(user_data))
        }
        "EventWriteEx" => {
            signature!(Win32Status, 8, 10; Qword(reg_handle), Pointer(event_descriptor), Qword(filter), Dword(flags), Pointer(activity_id), Pointer(related_activity_id), Dword(user_data_count), Pointer(user_data))
        }
        "EventWriteString" => {
            signature!(Win32Status, 4, 6; Qword(reg_handle), Byte(level), Qword(keyword), WideString(string))
        }
        "EventWriteTransfer" => {
            signature!(Win32Status, 6, 7; Qword(reg_handle), Pointer(event_descriptor), Pointer(activity_id), Pointer(related_activity_id), Dword(user_data_count), Pointer(user_data))
        }
        "GetNumberOfEventLogRecords" => {
            signature!(Win32Bool, 2, 2; Pointer(h_event_log), Pointer(number_of_records))
        }
        "GetOldestEventLogRecord" => {
            signature!(Win32Bool, 2, 2; Pointer(h_event_log), Pointer(oldest_record))
        }
        "GetUserNameA" => signature!(Win32Bool, 2, 2; Pointer(lp_buffer), Pointer(pcb_buffer)),
        "GetUserNameW" => signature!(Win32Bool, 2, 2; Pointer(lp_buffer), Pointer(pcb_buffer)),
        "ImpersonateLoggedOnUser" => signature!(Win32Bool, 1, 1; Pointer(h_token)),
        "InitializeSecurityDescriptor" => {
            signature!(Win32Bool, 2, 2; Pointer(p_security_descriptor), Dword(dw_revision))
        }
        "LookupAccountNameA" => {
            signature!(Win32Bool, 7, 7; AnsiString(lp_system_name), AnsiString(lp_account_name), Pointer(sid), Pointer(cb_sid), Pointer(referenced_domain_name), Pointer(cch_referenced_domain_name), Pointer(pe_use))
        }
        "LookupAccountNameW" => {
            signature!(Win32Bool, 7, 7; WideString(lp_system_name), WideString(lp_account_name), Pointer(sid), Pointer(cb_sid), Pointer(referenced_domain_name), Pointer(cch_referenced_domain_name), Pointer(pe_use))
        }
        "LookupPrivilegeDisplayNameA" => {
            signature!(Win32Bool, 5, 5; AnsiString(lp_system_name), AnsiString(lp_name), Pointer(lp_display_name), Pointer(cch_display_name), Pointer(lp_language_id))
        }
        "LookupPrivilegeDisplayNameW" => {
            signature!(Win32Bool, 5, 5; WideString(lp_system_name), WideString(lp_name), Pointer(lp_display_name), Pointer(cch_display_name), Pointer(lp_language_id))
        }
        "LookupPrivilegeNameA" => {
            signature!(Win32Bool, 4, 4; AnsiString(lp_system_name), Pointer(lp_luid), Pointer(lp_name), Pointer(cch_name))
        }
        "LookupPrivilegeNameW" => {
            signature!(Win32Bool, 4, 4; WideString(lp_system_name), Pointer(lp_luid), Pointer(lp_name), Pointer(cch_name))
        }
        "LookupPrivilegeValueA" => {
            signature!(Win32Bool, 3, 3; AnsiString(lp_system_name), AnsiString(lp_name), Pointer(lp_luid))
        }
        "LookupPrivilegeValueW" => {
            signature!(Win32Bool, 3, 3; WideString(lp_system_name), WideString(lp_name), Pointer(lp_luid))
        }
        "LookupSecurityDescriptorPartsA" => {
            signature!(Win32Status, 7, 7; Pointer(pp_owner), Pointer(pp_group), Pointer(pc_count_of_access_entries), Pointer(pp_list_of_access_entries), Pointer(pc_count_of_audit_entries), Pointer(pp_list_of_audit_entries), Pointer(p_sd))
        }
        "LookupSecurityDescriptorPartsW" => {
            signature!(Win32Status, 7, 7; Pointer(pp_owner), Pointer(pp_group), Pointer(pc_count_of_access_entries), Pointer(pp_list_of_access_entries), Pointer(pc_count_of_audit_entries), Pointer(pp_list_of_audit_entries), Pointer(p_sd))
        }
        "NotifyBootConfigStatus" => signature!(Win32Bool, 1, 1; Dword(boot_acceptable)),
        "NotifyChangeEventLog" => {
            signature!(Win32Bool, 2, 2; Pointer(h_event_log), Pointer(h_event))
        }
        "OpenThreadToken" => {
            signature!(Win32Bool, 4, 4; Pointer(thread_handle), Dword(desired_access), Dword(open_as_self), Pointer(token_handle))
        }
        "OpenThreadWaitChainSession" => signature!(Handle, 2, 2; Dword(flags), Pointer(callback)),
        "QueryServiceStatus" => {
            signature!(Win32Bool, 2, 2; Pointer(h_service), Pointer(lp_service_status))
        }
        "QueryServiceStatusEx" => {
            signature!(Win32Bool, 5, 5; Pointer(h_service), Dword(info_level), Pointer(lp_buffer), Dword(cb_buf_size), Pointer(pcb_bytes_needed))
        }
        "RegCreateKeyTransactedA" => {
            signature!(Win32Status, 11, 11; Pointer(h_key), AnsiString(lp_sub_key), Dword(reserved), AnsiString(lp_class), Dword(dw_options), Dword(sam_desired), Pointer(lp_security_attributes), Pointer(phk_result), Pointer(lpdw_disposition), Pointer(h_transaction), Pointer(p_extended_paremeter))
        }
        "RegCreateKeyTransactedW" => {
            signature!(Win32Status, 11, 11; Pointer(h_key), WideString(lp_sub_key), Dword(reserved), WideString(lp_class), Dword(dw_options), Dword(sam_desired), Pointer(lp_security_attributes), Pointer(phk_result), Pointer(lpdw_disposition), Pointer(h_transaction), Pointer(p_extended_paremeter))
        }
        "RegDeleteKeyTransactedA" => {
            signature!(Win32Status, 6, 6; Pointer(h_key), AnsiString(lp_sub_key), Dword(sam_desired), Dword(reserved), Pointer(h_transaction), Pointer(p_extended_parameter))
        }
        "RegDeleteKeyTransactedW" => {
            signature!(Win32Status, 6, 6; Pointer(h_key), WideString(lp_sub_key), Dword(sam_desired), Dword(reserved), Pointer(h_transaction), Pointer(p_extended_parameter))
        }
        "RegDeleteKeyValueA" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), AnsiString(lp_sub_key), AnsiString(lp_value_name))
        }
        "RegDeleteKeyValueW" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), WideString(lp_sub_key), WideString(lp_value_name))
        }
        "RegNotifyChangeKeyValue" => {
            signature!(Win32Status, 5, 5; Pointer(h_key), Dword(b_watch_subtree), Dword(dw_notify_filter), Pointer(h_event), Dword(f_asynchronous))
        }
        "RegOpenKeyTransactedA" => {
            signature!(Win32Status, 7, 7; Pointer(h_key), AnsiString(lp_sub_key), Dword(ul_options), Dword(sam_desired), Pointer(phk_result), Pointer(h_transaction), Pointer(p_extended_paremeter))
        }
        "RegOpenKeyTransactedW" => {
            signature!(Win32Status, 7, 7; Pointer(h_key), WideString(lp_sub_key), Dword(ul_options), Dword(sam_desired), Pointer(phk_result), Pointer(h_transaction), Pointer(p_extended_paremeter))
        }
        "RegSetKeyValueA" => {
            signature!(Win32Status, 6, 6; Pointer(h_key), AnsiString(lp_sub_key), AnsiString(lp_value_name), Dword(dw_type), Pointer(lp_data), Dword(cb_data))
        }
        "RegSetKeyValueW" => {
            signature!(Win32Status, 6, 6; Pointer(h_key), WideString(lp_sub_key), WideString(lp_value_name), Dword(dw_type), Pointer(lp_data), Dword(cb_data))
        }
        "RegSetValueA" => {
            signature!(Win32Status, 5, 5; Pointer(h_key), AnsiString(lp_sub_key), Dword(dw_type), Pointer(lp_data), Dword(cb_data))
        }
        "RegSetValueW" => {
            signature!(Win32Status, 5, 5; Pointer(h_key), WideString(lp_sub_key), Dword(dw_type), Pointer(lp_data), Dword(cb_data))
        }
        "SaferRecordEventLogEntry" => {
            signature!(Win32Bool, 3, 3; Pointer(h_level), WideString(sz_target_path), Pointer(lp_reserved))
        }
        "StartServiceA" => {
            signature!(Win32Bool, 3, 3; Pointer(h_service), Dword(dw_num_service_args), Pointer(lp_service_arg_vectors))
        }
        "StartServiceCtrlDispatcherA" => {
            signature!(Win32Bool, 1, 1; Pointer(lp_service_start_table))
        }
        "StartServiceCtrlDispatcherW" => {
            signature!(Win32Bool, 1, 1; Pointer(lp_service_start_table))
        }
        "StartServiceW" => {
            signature!(Win32Bool, 3, 3; Pointer(h_service), Dword(dw_num_service_args), Pointer(lp_service_arg_vectors))
        }
        "TraceEvent" => signature!(Win32Status, 2, 3; Qword(trace_handle), Pointer(event_trace)),
        "TraceEventInstance" => {
            signature!(Win32Status, 4, 5; Qword(trace_handle), Pointer(event_trace), Pointer(inst_info), Pointer(parent_inst_info))
        }
        "WaitServiceState" => {
            signature!(Generic, 4, 4; Pointer(h_service), Dword(dw_notify), Dword(dw_timeout), Pointer(h_cancel_event))
        }
        "A_SHAFinal" => signature!(Void, 2, 2; Pointer(context), Pointer(digest)),
        "A_SHAInit" => signature!(Void, 1, 1; Pointer(context)),
        "A_SHAUpdate" => signature!(Void, 3, 3; Pointer(context), Pointer(buffer), Dword(length)),
        "AddAccessAllowedObjectAce" => {
            signature!(Win32Bool, 7, 7; Pointer(p_acl), Dword(dw_ace_revision), Dword(ace_flags), Dword(access_mask), Pointer(object_type_guid), Pointer(inherited_object_type_guid), Pointer(p_sid))
        }
        "AddAccessDeniedObjectAce" => {
            signature!(Win32Bool, 7, 7; Pointer(p_acl), Dword(dw_ace_revision), Dword(ace_flags), Dword(access_mask), Pointer(object_type_guid), Pointer(inherited_object_type_guid), Pointer(p_sid))
        }
        "AddAce" => {
            signature!(Win32Bool, 5, 5; Pointer(p_acl), Dword(dw_ace_revision), Dword(dw_starting_ace_index), Pointer(p_ace_list), Dword(n_ace_list_length))
        }
        "AddAuditAccessAce" => {
            signature!(Win32Bool, 6, 6; Pointer(p_acl), Dword(dw_ace_revision), Dword(dw_access_mask), Pointer(p_sid), Dword(b_audit_success), Dword(b_audit_failure))
        }
        "AddAuditAccessAceEx" => {
            signature!(Win32Bool, 7, 7; Pointer(p_acl), Dword(dw_ace_revision), Dword(ace_flags), Dword(dw_access_mask), Pointer(p_sid), Dword(b_audit_success), Dword(b_audit_failure))
        }
        "AddAuditAccessObjectAce" => {
            signature!(Win32Bool, 9, 9; Pointer(p_acl), Dword(dw_ace_revision), Dword(ace_flags), Dword(access_mask), Pointer(object_type_guid), Pointer(inherited_object_type_guid), Pointer(p_sid), Dword(b_audit_success), Dword(b_audit_failure))
        }
        "AddConditionalAce" => {
            signature!(Win32Bool, 8, 8; Pointer(p_acl), Dword(dw_ace_revision), Dword(ace_flags), Byte(ace_type), Dword(access_mask), Pointer(p_sid), WideString(condition_str), Pointer(return_length))
        }
        "AddMandatoryAce" => {
            signature!(Win32Bool, 5, 5; Pointer(p_acl), Dword(dw_ace_revision), Dword(ace_flags), Dword(mandatory_policy), Pointer(p_label_sid))
        }
        "AddUsersToEncryptedFile" => {
            signature!(Generic, 2, 2; WideString(lp_file_name), Pointer(p_encryption_certificates))
        }
        "AdjustTokenGroups" => {
            signature!(Win32Bool, 6, 6; Pointer(token_handle), Dword(reset_to_default), Pointer(new_state), Dword(buffer_length), Pointer(previous_state), Pointer(return_length))
        }
        "AuditFree" => signature!(Void, 1, 1; Pointer(buffer)),
        "AuditSetPerUserPolicy" => {
            signature!(Generic, 3, 3; Pointer(p_sid), Pointer(p_audit_policy), Dword(dw_policy_count))
        }
        "AuditSetSecurity" => {
            signature!(Generic, 2, 2; Dword(security_information), Pointer(p_security_descriptor))
        }
        "AuditSetSystemPolicy" => {
            signature!(Generic, 2, 2; Pointer(p_audit_policy), Dword(dw_policy_count))
        }
        "CheckForHiberboot" => signature!(Generic, 2, 2; Pointer(p_hiberboot), Byte(b_clear_flag)),
        "CloseEncryptedFileRaw" => signature!(Void, 1, 1; Pointer(pv_context)),
        "CloseThreadWaitChainSession" => signature!(Void, 1, 1; Pointer(wct_handle)),
        "CloseTrace" => signature!(Win32Status, 1, 2),
        "CommandLineFromMsiDescriptor" => {
            signature!(Generic, 3, 3; Pointer(descriptor), Pointer(commandline), Pointer(commandlinelength))
        }
        "ConvertToAutoInheritPrivateObjectSecurity" => {
            signature!(Win32Bool, 6, 6; Pointer(parent_descriptor), Pointer(current_security_descriptor), Pointer(new_security_descriptor), Pointer(object_type), Byte(is_directory_object), Pointer(generic_mapping))
        }
        "CreatePrivateObjectSecurityWithMultipleInheritance" => {
            signature!(Win32Bool, 9, 9; Pointer(parent_descriptor), Pointer(creator_descriptor), Pointer(new_descriptor), Pointer(object_types), Dword(guid_count), Dword(is_container_object), Dword(auto_inherit_flags), Pointer(token), Pointer(generic_mapping))
        }
        "CredFree" => signature!(Void, 1, 1; Pointer(buffer)),
        "CryptContextAddRef" => {
            signature!(Win32Bool, 3, 3; Pointer(h_prov), Pointer(pdw_reserved), Dword(dw_flags))
        }
        "CryptSetHashParam" => {
            signature!(Win32Bool, 4, 4; Pointer(h_hash), Dword(dw_param), Pointer(pb_data), Dword(dw_flags))
        }
        "CryptSetProvParam" => {
            signature!(Win32Bool, 4, 4; Pointer(h_prov), Dword(dw_param), Pointer(pb_data), Dword(dw_flags))
        }
        "CryptSignHashA" => {
            signature!(Win32Bool, 6, 6; Pointer(h_hash), Dword(dw_key_spec), AnsiString(sz_description), Dword(dw_flags), Pointer(pb_signature), Pointer(pdw_sig_len))
        }
        "CryptSignHashW" => {
            signature!(Win32Bool, 6, 6; Pointer(h_hash), Dword(dw_key_spec), WideString(sz_description), Dword(dw_flags), Pointer(pb_signature), Pointer(pdw_sig_len))
        }
        "DeleteService" => signature!(Win32Bool, 1, 1; Pointer(h_service)),
        "DestroyPrivateObjectSecurity" => signature!(Win32Bool, 1, 1; Pointer(object_descriptor)),
        "DuplicateEncryptionInfoFile" => {
            signature!(Generic, 5, 5; WideString(src_file_name), WideString(dst_file_name), Dword(dw_creation_distribution), Dword(dw_attributes), Pointer(lp_security_attributes))
        }
        "EnableTrace" => {
            signature!(Win32Status, 5, 6; Dword(enable), Dword(enable_flag), Dword(enable_level), Pointer(control_guid), Qword(trace_id))
        }
        "EnableTraceEx" => {
            signature!(Win32Status, 9, 12; Pointer(provider_id), Pointer(source_id), Qword(trace_id), Dword(is_enabled), Byte(level), Qword(match_any_keyword), Qword(match_all_keyword), Dword(enable_property), Pointer(enable_filter_desc))
        }
        "EnumDynamicTimeZoneInformation" => {
            signature!(Generic, 2, 2; Dword(dw_index), Pointer(lp_time_zone_information))
        }
        "EnumerateTraceGuids" => {
            signature!(Win32Status, 3, 3; Pointer(guid_properties_array), Dword(property_array_count), Pointer(guid_count))
        }
        "EnumerateTraceGuidsEx" => {
            signature!(Win32Status, 6, 6; Dword(trace_query_info_class), Pointer(in_buffer), Dword(in_buffer_size), Pointer(out_buffer), Dword(out_buffer_size), Pointer(return_length))
        }
        "EqualDomainSid" => {
            signature!(Win32Bool, 3, 3; Pointer(p_sid1), Pointer(p_sid2), Pointer(pf_equal))
        }
        "FreeEncryptedFileMetadata" => signature!(Void, 1, 1; Pointer(pb_metadata)),
        "FreeEncryptionCertificateHashList" => signature!(Void, 1, 1; Pointer(p_users)),
        "FreeInheritedFromArray" => {
            signature!(Win32Status, 3, 3; Pointer(p_inherit_array), Word(ace_cnt), Pointer(optional))
        }
        "GetDynamicTimeZoneInformationEffectiveYears" => {
            signature!(Generic, 3, 3; Pointer(lp_time_zone_information), Pointer(first_year), Pointer(last_year))
        }
        "GetEncryptedFileMetadata" => {
            signature!(Generic, 3, 3; WideString(lp_file_name), Pointer(pcb_metadata), Pointer(ppb_metadata))
        }
        "GetKernelObjectSecurity" => {
            signature!(Win32Bool, 5, 5; Pointer(handle), Dword(requested_information), Pointer(p_security_descriptor), Dword(n_length), Pointer(lpn_length_needed))
        }
        "GetLocalManagedApplicationData" => {
            signature!(Void, 3, 3; Pointer(productcode), Pointer(displayname), Pointer(supporturl))
        }
        "GetLocalManagedApplications" => {
            signature!(Generic, 3, 3; Dword(buserapps), Pointer(pdwapps), Pointer(prglocalapps))
        }
        "GetManagedApplicationCategories" => {
            signature!(Generic, 2, 2; Dword(dwreserved), Pointer(pappcategory))
        }
        "GetManagedApplications" => {
            signature!(Generic, 5, 5; Pointer(pcategory), Dword(dwqueryflags), Dword(dwinfolevel), Pointer(pdwapps), Pointer(prgmanagedapps))
        }
        "GetPrivateObjectSecurity" => {
            signature!(Win32Bool, 5, 5; Pointer(object_descriptor), Dword(security_information), Pointer(resultant_descriptor), Dword(descriptor_length), Pointer(return_length))
        }
        "GetSecurityDescriptorRMControl" => {
            signature!(Generic, 2, 2; Pointer(security_descriptor), Pointer(rm_control))
        }
        "GetTraceEnableFlags" => signature!(Generic, 1, 2; Qword(trace_handle)),
        "GetTraceEnableLevel" => signature!(Generic, 1, 2; Qword(trace_handle)),
        "GetWindowsAccountDomainSid" => {
            signature!(Win32Bool, 3, 3; Pointer(p_sid), Pointer(p_domain_sid), Pointer(cb_domain_sid))
        }
        "ImpersonateAnonymousToken" => signature!(Win32Bool, 1, 1; Pointer(thread_handle)),
        "ImpersonateNamedPipeClient" => signature!(Win32Bool, 1, 1; Pointer(h_named_pipe)),
        "ImpersonateSelf" => signature!(Win32Bool, 1, 1; Dword(impersonation_level)),
        "InstallApplication" => signature!(Generic, 1, 1; Pointer(pinstallinfo)),
        "IsTextUnicode" => {
            signature!(Win32Bool, 3, 3; Pointer(lpv), Dword(i_size), Pointer(lpi_result))
        }
        "IsTokenRestricted" => signature!(Win32Bool, 1, 1; Pointer(token_handle)),
        "IsTokenUntrusted" => signature!(Win32Bool, 1, 1; Pointer(token_handle)),
        "IsValidSecurityDescriptor" => signature!(Win32Bool, 1, 1; Pointer(p_security_descriptor)),
        "LsaAddAccountRights" => {
            signature!(NtStatus, 4, 4; Pointer(policy_handle), Pointer(account_sid), Pointer(user_rights), Dword(count_of_rights))
        }
        "LsaClose" => signature!(NtStatus, 1, 1; Pointer(object_handle)),
        "LsaFreeMemory" => signature!(NtStatus, 1, 1; Pointer(buffer)),
        "LsaSetCAPs" => {
            signature!(NtStatus, 3, 3; Pointer(capd_ns), Dword(capdn_count), Dword(flags))
        }
        "LsaSetDomainInformationPolicy" => {
            signature!(NtStatus, 3, 3; Pointer(policy_handle), Dword(information_class), Pointer(buffer))
        }
        "LsaSetForestTrustInformation" => {
            signature!(NtStatus, 5, 5; Pointer(policy_handle), Pointer(trusted_domain_name), Pointer(forest_trust_info), Byte(check_only), Pointer(collision_info))
        }
        "LsaSetForestTrustInformation2" => {
            signature!(NtStatus, 6, 6; Pointer(policy_handle), Pointer(trusted_domain_name), Dword(highest_record_type), Pointer(forest_trust_info), Byte(check_only), Pointer(collision_info))
        }
        "LsaSetInformationPolicy" => {
            signature!(NtStatus, 3, 3; Pointer(policy_handle), Dword(information_class), Pointer(buffer))
        }
        "LsaSetTrustedDomainInfoByName" => {
            signature!(NtStatus, 4, 4; Pointer(policy_handle), Pointer(trusted_domain_name), Dword(information_class), Pointer(buffer))
        }
        "LsaSetTrustedDomainInformation" => {
            signature!(NtStatus, 4, 4; Pointer(policy_handle), Pointer(trusted_domain_sid), Dword(information_class), Pointer(buffer))
        }
        "LsaStorePrivateData" => {
            signature!(NtStatus, 3, 3; Pointer(policy_handle), Pointer(key_name), Pointer(private_data))
        }
        "MSChapSrvChangePassword" => {
            signature!(Generic, 7, 7; WideString(server_name), WideString(user_name), Byte(lm_old_present), Pointer(lm_old_owf_password), Pointer(lm_new_owf_password), Pointer(nt_old_owf_password), Pointer(nt_new_owf_password))
        }
        "MSChapSrvChangePassword2" => {
            signature!(Generic, 7, 7; WideString(server_name), WideString(user_name), Pointer(new_password_encrypted_with_old_nt), Pointer(old_nt_owf_password_encrypted_with_new_nt), Byte(lm_present), Pointer(new_password_encrypted_with_old_lm), Pointer(old_lm_owf_password_encrypted_with_new_lm_or_nt))
        }
        "MakeAbsoluteSD" => {
            signature!(Win32Bool, 11, 11; Pointer(p_self_relative_security_descriptor), Pointer(p_absolute_security_descriptor), Pointer(lpdw_absolute_security_descriptor_size), Pointer(p_dacl), Pointer(lpdw_dacl_size), Pointer(p_sacl), Pointer(lpdw_sacl_size), Pointer(p_owner), Pointer(lpdw_owner_size), Pointer(p_primary_group), Pointer(lpdw_primary_group_size))
        }
        "MakeSelfRelativeSD" => {
            signature!(Win32Bool, 3, 3; Pointer(p_absolute_security_descriptor), Pointer(p_self_relative_security_descriptor), Pointer(lpdw_buffer_length))
        }
        "OperationEnd" => signature!(Win32Bool, 1, 1; Pointer(operation_end_params)),
        "OperationStart" => signature!(Win32Bool, 1, 1; Pointer(operation_start_params)),
        "PerfAddCounters" => {
            signature!(Win32Status, 3, 3; Pointer(h_query), Pointer(p_counters), Dword(cb_counters))
        }
        "PerfCloseQueryHandle" => signature!(Win32Status, 1, 1; Pointer(h_query)),
        "PerfCreateInstance" => {
            signature!(Generic, 4, 4; Pointer(provider_handle), Pointer(counter_set_guid), WideString(name), Dword(id))
        }
        "PerfDecrementULongCounterValue" => {
            signature!(Win32Status, 4, 4; Pointer(provider), Pointer(instance), Dword(counter_id), Dword(value))
        }
        "PerfDecrementULongLongCounterValue" => {
            signature!(Win32Status, 4, 5; Pointer(provider), Pointer(instance), Dword(counter_id), Qword(value))
        }
        "PerfEnumerateCounterSet" => {
            signature!(Win32Status, 4, 4; WideString(sz_machine), Pointer(p_counter_set_ids), Dword(c_counter_set_ids), Pointer(pc_counter_set_ids_actual))
        }
        "PerfEnumerateCounterSetInstances" => {
            signature!(Win32Status, 5, 5; WideString(sz_machine), Pointer(p_counter_set_id), Pointer(p_instances), Dword(cb_instances), Pointer(pcb_instances_actual))
        }
        "PerfIncrementULongCounterValue" => {
            signature!(Win32Status, 4, 4; Pointer(provider), Pointer(instance), Dword(counter_id), Dword(value))
        }
        "PerfIncrementULongLongCounterValue" => {
            signature!(Win32Status, 4, 5; Pointer(provider), Pointer(instance), Dword(counter_id), Qword(value))
        }
        "PerfOpenQueryHandle" => {
            signature!(Win32Status, 2, 2; WideString(sz_machine), Pointer(ph_query))
        }
        "PerfQueryCounterData" => {
            signature!(Win32Status, 4, 4; Pointer(h_query), Pointer(p_counter_block), Dword(cb_counter_block), Pointer(pcb_counter_block_actual))
        }
        "PerfQueryCounterInfo" => {
            signature!(Win32Status, 4, 4; Pointer(h_query), Pointer(p_counters), Dword(cb_counters), Pointer(pcb_counters_actual))
        }
        "PerfQueryCounterSetRegistrationInfo" => {
            signature!(Win32Status, 7, 7; WideString(sz_machine), Pointer(p_counter_set_id), Dword(request_code), Dword(request_lang_id), Pointer(pb_reg_info), Dword(cb_reg_info), Pointer(pcb_reg_info_actual))
        }
        "PerfQueryInstance" => {
            signature!(Generic, 4, 4; Pointer(provider_handle), Pointer(counter_set_guid), WideString(name), Dword(id))
        }
        "PerfSetCounterRefValue" => {
            signature!(Win32Status, 4, 4; Pointer(provider), Pointer(instance), Dword(counter_id), Pointer(address))
        }
        "PerfSetCounterSetInfo" => {
            signature!(Win32Status, 3, 3; Pointer(provider_handle), Pointer(template), Dword(template_size))
        }
        "PerfSetULongCounterValue" => {
            signature!(Win32Status, 4, 4; Pointer(provider), Pointer(instance), Dword(counter_id), Dword(value))
        }
        "PerfSetULongLongCounterValue" => {
            signature!(Win32Status, 4, 5; Pointer(provider), Pointer(instance), Dword(counter_id), Qword(value))
        }
        "PerfStartProvider" => {
            signature!(Win32Status, 3, 3; Pointer(provider_guid), Pointer(control_callback), Pointer(ph_provider))
        }
        "PerfStartProviderEx" => {
            signature!(Win32Status, 3, 3; Pointer(provider_guid), Pointer(provider_context), Pointer(provider))
        }
        "PerfStopProvider" => signature!(Win32Status, 1, 1; Pointer(provider_handle)),
        "ProcessTrace" => {
            signature!(Win32Status, 4, 4; Pointer(handle_array), Dword(handle_count), Pointer(start_time), Pointer(end_time))
        }
        "QueryRecoveryAgentsOnEncryptedFile" => {
            signature!(Generic, 2, 2; WideString(lp_file_name), Pointer(p_recovery_agents))
        }
        "QuerySecurityAccessMask" => {
            signature!(Void, 2, 2; Dword(security_information), Pointer(desired_access))
        }
        "QueryUsersOnEncryptedFile" => {
            signature!(Generic, 2, 2; WideString(lp_file_name), Pointer(p_users))
        }
        "ReadEncryptedFileRaw" => {
            signature!(Generic, 3, 3; Pointer(pf_export_callback), Pointer(pv_callback_context), Pointer(pv_context))
        }
        "RegDisablePredefinedCache" => signature!(Win32Status, 0, 0; ),
        "RegDisablePredefinedCacheEx" => signature!(Win32Status, 0, 0; ),
        "RegDisableReflectionKey" => signature!(Win32Status, 1, 1; Pointer(h_base)),
        "RegEnableReflectionKey" => signature!(Win32Status, 1, 1; Pointer(h_base)),
        "RegGetKeySecurity" => {
            signature!(Win32Status, 4, 4; Pointer(h_key), Dword(security_information), Pointer(p_security_descriptor), Pointer(lpcb_security_descriptor))
        }
        "RegLoadKeyA" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), AnsiString(lp_sub_key), AnsiString(lp_file))
        }
        "RegLoadKeyW" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), WideString(lp_sub_key), WideString(lp_file))
        }
        "RegOpenUserClassesRoot" => {
            signature!(Win32Status, 4, 4; Pointer(h_token), Dword(dw_options), Dword(sam_desired), Pointer(phk_result))
        }
        "RegOverridePredefKey" => {
            signature!(Win32Status, 2, 2; Pointer(h_key), Pointer(h_new_h_key))
        }
        "RegQueryMultipleValuesA" => {
            signature!(Win32Status, 5, 5; Pointer(h_key), Pointer(val_list), Dword(num_vals), Pointer(lp_value_buf), Pointer(ldw_totsize))
        }
        "RegQueryMultipleValuesW" => {
            signature!(Win32Status, 5, 5; Pointer(h_key), Pointer(val_list), Dword(num_vals), Pointer(lp_value_buf), Pointer(ldw_totsize))
        }
        "RegQueryReflectionKey" => {
            signature!(Win32Status, 2, 2; Pointer(h_base), Pointer(b_is_reflection_disabled))
        }
        "RegRenameKey" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), WideString(lp_sub_key_name), WideString(lp_new_key_name))
        }
        "RegReplaceKeyA" => {
            signature!(Win32Status, 4, 4; Pointer(h_key), AnsiString(lp_sub_key), AnsiString(lp_new_file), AnsiString(lp_old_file))
        }
        "RegReplaceKeyW" => {
            signature!(Win32Status, 4, 4; Pointer(h_key), WideString(lp_sub_key), WideString(lp_new_file), WideString(lp_old_file))
        }
        "RegRestoreKeyA" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), AnsiString(lp_file), Dword(dw_flags))
        }
        "RegRestoreKeyW" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), WideString(lp_file), Dword(dw_flags))
        }
        "RegSaveKeyA" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), AnsiString(lp_file), Pointer(lp_security_attributes))
        }
        "RegSaveKeyW" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), WideString(lp_file), Pointer(lp_security_attributes))
        }
        "RegSetKeySecurity" => {
            signature!(Win32Status, 3, 3; Pointer(h_key), Dword(security_information), Pointer(p_security_descriptor))
        }
        "RegUnLoadKeyA" => signature!(Win32Status, 2, 2; Pointer(h_key), AnsiString(lp_sub_key)),
        "RegUnLoadKeyW" => signature!(Win32Status, 2, 2; Pointer(h_key), WideString(lp_sub_key)),
        "SaferCloseLevel" => signature!(Win32Bool, 1, 1; Pointer(h_level_handle)),
        "SaferSetLevelInformation" => {
            signature!(Win32Bool, 4, 4; Pointer(level_handle), Dword(dw_info_type), Pointer(lp_query_buffer), Dword(dw_in_buffer_size))
        }
        "SaferSetPolicyInformation" => {
            signature!(Win32Bool, 5, 5; Dword(dw_scope_id), Dword(safer_policy_info_class), Dword(info_buffer_size), Pointer(info_buffer), Pointer(lp_reserved))
        }
        "SaferiIsExecutableFileType" => {
            signature!(Win32Bool, 2, 2; WideString(sz_full_pathname), Byte(b_from_shell_execute))
        }
        "SetAclInformation" => {
            signature!(Win32Bool, 4, 4; Pointer(p_acl), Pointer(p_acl_information), Dword(n_acl_information_length), Dword(dw_acl_information_class))
        }
        "SetEncryptedFileMetadata" => {
            signature!(Generic, 6, 6; WideString(lp_file_name), Pointer(pb_old_metadata), Pointer(pb_new_metadata), Pointer(p_owner_hash), Dword(dw_operation), Pointer(p_certificates_added))
        }
        "SetKernelObjectSecurity" => {
            signature!(Win32Bool, 3, 3; Pointer(handle), Dword(security_information), Pointer(security_descriptor))
        }
        "SetPrivateObjectSecurity" => {
            signature!(Win32Bool, 5, 5; Dword(security_information), Pointer(modification_descriptor), Pointer(objects_security_descriptor), Pointer(generic_mapping), Pointer(token))
        }
        "SetPrivateObjectSecurityEx" => {
            signature!(Win32Bool, 6, 6; Dword(security_information), Pointer(modification_descriptor), Pointer(objects_security_descriptor), Dword(auto_inherit_flags), Pointer(generic_mapping), Pointer(token))
        }
        "SetSecurityAccessMask" => {
            signature!(Void, 2, 2; Dword(security_information), Pointer(desired_access))
        }
        "SetSecurityDescriptorControl" => {
            signature!(Win32Bool, 3, 3; Pointer(p_security_descriptor), Word(control_bits_of_interest), Word(control_bits_to_set))
        }
        "SetSecurityDescriptorRMControl" => {
            signature!(Generic, 2, 2; Pointer(security_descriptor), Pointer(rm_control))
        }
        "SetSecurityInfo" => {
            signature!(Win32Status, 7, 7; Pointer(handle), Dword(object_type), Dword(security_info), Pointer(psid_owner), Pointer(psid_group), Pointer(p_dacl), Pointer(p_sacl))
        }
        "SetServiceBits" => {
            signature!(Win32Bool, 4, 4; Pointer(h_service_status), Dword(dw_service_bits), Dword(b_set_bits_on), Dword(b_update_immediately))
        }
        "SetServiceObjectSecurity" => {
            signature!(Win32Bool, 3, 3; Pointer(h_service), Dword(dw_security_information), Pointer(lp_security_descriptor))
        }
        "SetServiceStatus" => {
            signature!(Win32Bool, 2, 2; Pointer(h_service_status), Pointer(lp_service_status))
        }
        "SetThreadToken" => signature!(Win32Bool, 2, 2; Pointer(thread), Pointer(token)),
        "SetTokenInformation" => {
            signature!(Win32Bool, 4, 4; Pointer(token_handle), Dword(token_information_class), Pointer(token_information), Dword(token_information_length))
        }
        "SetTraceCallback" => {
            signature!(Win32Status, 2, 2; Pointer(p_guid), Pointer(event_callback))
        }
        "SetUserFileEncryptionKey" => signature!(Generic, 1, 1; Pointer(p_encryption_certificate)),
        "SetUserFileEncryptionKeyEx" => {
            signature!(Generic, 4, 4; Pointer(p_encryption_certificate), Dword(dw_capabilities), Dword(dw_flags), Pointer(pv_reserved))
        }
        "WriteEncryptedFileRaw" => {
            signature!(Generic, 3, 3; Pointer(pf_import_callback), Pointer(pv_callback_context), Pointer(pv_context))
        }
        _ => None,
    }
}
