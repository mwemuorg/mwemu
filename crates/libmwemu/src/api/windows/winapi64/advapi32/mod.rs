use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi64;
use crate::windows::constants;

mod a_sha_final;
pub use a_sha_final::A_SHAFinal;
mod a_sha_init;
pub use a_sha_init::A_SHAInit;
mod a_sha_update;
pub use a_sha_update::A_SHAUpdate;
mod abort_system_shutdown_a;
pub use abort_system_shutdown_a::AbortSystemShutdownA;
mod abort_system_shutdown_w;
pub use abort_system_shutdown_w::AbortSystemShutdownW;
mod access_check;
pub use access_check::AccessCheck;
mod access_check_and_audit_alarm_a;
pub use access_check_and_audit_alarm_a::AccessCheckAndAuditAlarmA;
mod access_check_and_audit_alarm_w;
pub use access_check_and_audit_alarm_w::AccessCheckAndAuditAlarmW;
mod access_check_by_type;
pub use access_check_by_type::AccessCheckByType;
mod access_check_by_type_and_audit_alarm_a;
pub use access_check_by_type_and_audit_alarm_a::AccessCheckByTypeAndAuditAlarmA;
mod access_check_by_type_and_audit_alarm_w;
pub use access_check_by_type_and_audit_alarm_w::AccessCheckByTypeAndAuditAlarmW;
mod access_check_by_type_result_list;
pub use access_check_by_type_result_list::AccessCheckByTypeResultList;
mod access_check_by_type_result_list_and_audit_alarm_a;
pub use access_check_by_type_result_list_and_audit_alarm_a::AccessCheckByTypeResultListAndAuditAlarmA;
mod access_check_by_type_result_list_and_audit_alarm_by_handle_a;
pub use access_check_by_type_result_list_and_audit_alarm_by_handle_a::AccessCheckByTypeResultListAndAuditAlarmByHandleA;
mod access_check_by_type_result_list_and_audit_alarm_by_handle_w;
pub use access_check_by_type_result_list_and_audit_alarm_by_handle_w::AccessCheckByTypeResultListAndAuditAlarmByHandleW;
mod access_check_by_type_result_list_and_audit_alarm_w;
pub use access_check_by_type_result_list_and_audit_alarm_w::AccessCheckByTypeResultListAndAuditAlarmW;
mod add_access_allowed_ace;
pub use add_access_allowed_ace::AddAccessAllowedAce;
mod add_access_allowed_ace_ex;
pub use add_access_allowed_ace_ex::AddAccessAllowedAceEx;
mod add_access_allowed_object_ace;
pub use add_access_allowed_object_ace::AddAccessAllowedObjectAce;
mod add_access_denied_ace;
pub use add_access_denied_ace::AddAccessDeniedAce;
mod add_access_denied_ace_ex;
pub use add_access_denied_ace_ex::AddAccessDeniedAceEx;
mod add_access_denied_object_ace;
pub use add_access_denied_object_ace::AddAccessDeniedObjectAce;
mod add_ace;
pub use add_ace::AddAce;
mod add_audit_access_ace;
pub use add_audit_access_ace::AddAuditAccessAce;
mod add_audit_access_ace_ex;
pub use add_audit_access_ace_ex::AddAuditAccessAceEx;
mod add_audit_access_object_ace;
pub use add_audit_access_object_ace::AddAuditAccessObjectAce;
mod add_conditional_ace;
pub use add_conditional_ace::AddConditionalAce;
mod add_mandatory_ace;
pub use add_mandatory_ace::AddMandatoryAce;
mod add_users_to_encrypted_file;
pub use add_users_to_encrypted_file::AddUsersToEncryptedFile;
mod add_users_to_encrypted_file_ex;
pub use add_users_to_encrypted_file_ex::AddUsersToEncryptedFileEx;
mod adjust_token_groups;
pub use adjust_token_groups::AdjustTokenGroups;
mod allocate_and_initialize_sid;
pub use allocate_and_initialize_sid::AllocateAndInitializeSid;
mod allocate_locally_unique_id;
pub use allocate_locally_unique_id::AllocateLocallyUniqueId;
mod are_all_accesses_granted;
pub use are_all_accesses_granted::AreAllAccessesGranted;
mod are_any_accesses_granted;
pub use are_any_accesses_granted::AreAnyAccessesGranted;
mod audit_compute_effective_policy_by_sid;
pub use audit_compute_effective_policy_by_sid::AuditComputeEffectivePolicyBySid;
mod audit_compute_effective_policy_by_token;
pub use audit_compute_effective_policy_by_token::AuditComputeEffectivePolicyByToken;
mod audit_enumerate_categories;
pub use audit_enumerate_categories::AuditEnumerateCategories;
mod audit_enumerate_per_user_policy;
pub use audit_enumerate_per_user_policy::AuditEnumeratePerUserPolicy;
mod audit_enumerate_sub_categories;
pub use audit_enumerate_sub_categories::AuditEnumerateSubCategories;
mod audit_free;
pub use audit_free::AuditFree;
mod audit_lookup_category_guid_from_category_id;
pub use audit_lookup_category_guid_from_category_id::AuditLookupCategoryGuidFromCategoryId;
mod audit_lookup_category_id_from_category_guid;
pub use audit_lookup_category_id_from_category_guid::AuditLookupCategoryIdFromCategoryGuid;
mod audit_lookup_category_name_a;
pub use audit_lookup_category_name_a::AuditLookupCategoryNameA;
mod audit_lookup_category_name_w;
pub use audit_lookup_category_name_w::AuditLookupCategoryNameW;
mod audit_lookup_sub_category_name_a;
pub use audit_lookup_sub_category_name_a::AuditLookupSubCategoryNameA;
mod audit_lookup_sub_category_name_w;
pub use audit_lookup_sub_category_name_w::AuditLookupSubCategoryNameW;
mod audit_query_global_sacl_a;
pub use audit_query_global_sacl_a::AuditQueryGlobalSaclA;
mod audit_query_global_sacl_w;
pub use audit_query_global_sacl_w::AuditQueryGlobalSaclW;
mod audit_query_per_user_policy;
pub use audit_query_per_user_policy::AuditQueryPerUserPolicy;
mod audit_query_security;
pub use audit_query_security::AuditQuerySecurity;
mod audit_query_system_policy;
pub use audit_query_system_policy::AuditQuerySystemPolicy;
mod audit_set_global_sacl_a;
pub use audit_set_global_sacl_a::AuditSetGlobalSaclA;
mod audit_set_global_sacl_w;
pub use audit_set_global_sacl_w::AuditSetGlobalSaclW;
mod audit_set_per_user_policy;
pub use audit_set_per_user_policy::AuditSetPerUserPolicy;
mod audit_set_security;
pub use audit_set_security::AuditSetSecurity;
mod audit_set_system_policy;
pub use audit_set_system_policy::AuditSetSystemPolicy;
mod backup_event_log_a;
pub use backup_event_log_a::BackupEventLogA;
mod backup_event_log_w;
pub use backup_event_log_w::BackupEventLogW;
mod base_reg_close_key;
pub use base_reg_close_key::BaseRegCloseKey;
mod base_reg_create_key;
pub use base_reg_create_key::BaseRegCreateKey;
mod base_reg_delete_key_ex;
pub use base_reg_delete_key_ex::BaseRegDeleteKeyEx;
mod base_reg_delete_value;
pub use base_reg_delete_value::BaseRegDeleteValue;
mod base_reg_flush_key;
pub use base_reg_flush_key::BaseRegFlushKey;
mod base_reg_get_version;
pub use base_reg_get_version::BaseRegGetVersion;
mod base_reg_load_key;
pub use base_reg_load_key::BaseRegLoadKey;
mod base_reg_open_key;
pub use base_reg_open_key::BaseRegOpenKey;
mod base_reg_restore_key;
pub use base_reg_restore_key::BaseRegRestoreKey;
mod base_reg_save_key_ex;
pub use base_reg_save_key_ex::BaseRegSaveKeyEx;
mod base_reg_set_key_security;
pub use base_reg_set_key_security::BaseRegSetKeySecurity;
mod base_reg_set_value;
pub use base_reg_set_value::BaseRegSetValue;
mod base_reg_un_load_key;
pub use base_reg_un_load_key::BaseRegUnLoadKey;
mod build_explicit_access_with_name_a;
pub use build_explicit_access_with_name_a::BuildExplicitAccessWithNameA;
mod build_explicit_access_with_name_w;
pub use build_explicit_access_with_name_w::BuildExplicitAccessWithNameW;
mod build_impersonate_explicit_access_with_name_a;
pub use build_impersonate_explicit_access_with_name_a::BuildImpersonateExplicitAccessWithNameA;
mod build_impersonate_explicit_access_with_name_w;
pub use build_impersonate_explicit_access_with_name_w::BuildImpersonateExplicitAccessWithNameW;
mod build_impersonate_trustee_a;
pub use build_impersonate_trustee_a::BuildImpersonateTrusteeA;
mod build_impersonate_trustee_w;
pub use build_impersonate_trustee_w::BuildImpersonateTrusteeW;
mod build_security_descriptor_a;
pub use build_security_descriptor_a::BuildSecurityDescriptorA;
mod build_security_descriptor_w;
pub use build_security_descriptor_w::BuildSecurityDescriptorW;
mod build_trustee_with_name_a;
pub use build_trustee_with_name_a::BuildTrusteeWithNameA;
mod build_trustee_with_name_w;
pub use build_trustee_with_name_w::BuildTrusteeWithNameW;
mod build_trustee_with_objects_and_name_a;
pub use build_trustee_with_objects_and_name_a::BuildTrusteeWithObjectsAndNameA;
mod build_trustee_with_objects_and_name_w;
pub use build_trustee_with_objects_and_name_w::BuildTrusteeWithObjectsAndNameW;
mod build_trustee_with_objects_and_sid_a;
pub use build_trustee_with_objects_and_sid_a::BuildTrusteeWithObjectsAndSidA;
mod build_trustee_with_objects_and_sid_w;
pub use build_trustee_with_objects_and_sid_w::BuildTrusteeWithObjectsAndSidW;
mod build_trustee_with_sid_a;
pub use build_trustee_with_sid_a::BuildTrusteeWithSidA;
mod build_trustee_with_sid_w;
pub use build_trustee_with_sid_w::BuildTrusteeWithSidW;
mod cancel_overlapped_access;
pub use cancel_overlapped_access::CancelOverlappedAccess;
mod change_service_config2_a;
pub use change_service_config2_a::ChangeServiceConfig2A;
mod change_service_config2_w;
pub use change_service_config2_w::ChangeServiceConfig2W;
mod change_service_config_a;
pub use change_service_config_a::ChangeServiceConfigA;
mod change_service_config_w;
pub use change_service_config_w::ChangeServiceConfigW;
mod check_for_hiberboot;
pub use check_for_hiberboot::CheckForHiberboot;
mod check_token_membership;
pub use check_token_membership::CheckTokenMembership;
mod clear_event_log_a;
pub use clear_event_log_a::ClearEventLogA;
mod clear_event_log_w;
pub use clear_event_log_w::ClearEventLogW;
mod close_code_authz_level;
pub use close_code_authz_level::CloseCodeAuthzLevel;
mod close_encrypted_file_raw;
pub use close_encrypted_file_raw::CloseEncryptedFileRaw;
mod close_event_log;
pub use close_event_log::CloseEventLog;
mod close_service_handle;
pub use close_service_handle::CloseServiceHandle;
mod close_thread_wait_chain_session;
pub use close_thread_wait_chain_session::CloseThreadWaitChainSession;
mod close_trace;
pub use close_trace::CloseTrace;
mod command_line_from_msi_descriptor;
pub use command_line_from_msi_descriptor::CommandLineFromMsiDescriptor;
mod compute_access_token_from_code_authz_level;
pub use compute_access_token_from_code_authz_level::ComputeAccessTokenFromCodeAuthzLevel;
mod control_service;
pub use control_service::ControlService;
mod control_service_ex_a;
pub use control_service_ex_a::ControlServiceExA;
mod control_service_ex_w;
pub use control_service_ex_w::ControlServiceExW;
mod control_trace_a;
pub use control_trace_a::ControlTraceA;
mod control_trace_w;
pub use control_trace_w::ControlTraceW;
mod convert_access_to_security_descriptor_a;
pub use convert_access_to_security_descriptor_a::ConvertAccessToSecurityDescriptorA;
mod convert_access_to_security_descriptor_w;
pub use convert_access_to_security_descriptor_w::ConvertAccessToSecurityDescriptorW;
mod convert_sd_to_string_sd_domain_w;
pub use convert_sd_to_string_sd_domain_w::ConvertSDToStringSDDomainW;
mod convert_sd_to_string_sd_root_domain_a;
pub use convert_sd_to_string_sd_root_domain_a::ConvertSDToStringSDRootDomainA;
mod convert_sd_to_string_sd_root_domain_w;
pub use convert_sd_to_string_sd_root_domain_w::ConvertSDToStringSDRootDomainW;
mod convert_security_descriptor_to_access_a;
pub use convert_security_descriptor_to_access_a::ConvertSecurityDescriptorToAccessA;
mod convert_security_descriptor_to_access_named_a;
pub use convert_security_descriptor_to_access_named_a::ConvertSecurityDescriptorToAccessNamedA;
mod convert_security_descriptor_to_access_named_w;
pub use convert_security_descriptor_to_access_named_w::ConvertSecurityDescriptorToAccessNamedW;
mod convert_security_descriptor_to_access_w;
pub use convert_security_descriptor_to_access_w::ConvertSecurityDescriptorToAccessW;
mod convert_security_descriptor_to_string_security_descriptor_a;
pub use convert_security_descriptor_to_string_security_descriptor_a::ConvertSecurityDescriptorToStringSecurityDescriptorA;
mod convert_security_descriptor_to_string_security_descriptor_w;
pub use convert_security_descriptor_to_string_security_descriptor_w::ConvertSecurityDescriptorToStringSecurityDescriptorW;
mod convert_sid_to_string_sid_a;
pub use convert_sid_to_string_sid_a::ConvertSidToStringSidA;
mod convert_sid_to_string_sid_w;
pub use convert_sid_to_string_sid_w::ConvertSidToStringSidW;
mod convert_string_sd_to_sd_domain_a;
pub use convert_string_sd_to_sd_domain_a::ConvertStringSDToSDDomainA;
mod convert_string_sd_to_sd_domain_w;
pub use convert_string_sd_to_sd_domain_w::ConvertStringSDToSDDomainW;
mod convert_string_sd_to_sd_root_domain_a;
pub use convert_string_sd_to_sd_root_domain_a::ConvertStringSDToSDRootDomainA;
mod convert_string_sd_to_sd_root_domain_w;
pub use convert_string_sd_to_sd_root_domain_w::ConvertStringSDToSDRootDomainW;
mod convert_string_security_descriptor_to_security_descriptor_a;
pub use convert_string_security_descriptor_to_security_descriptor_a::ConvertStringSecurityDescriptorToSecurityDescriptorA;
mod convert_string_security_descriptor_to_security_descriptor_w;
pub use convert_string_security_descriptor_to_security_descriptor_w::ConvertStringSecurityDescriptorToSecurityDescriptorW;
mod convert_string_sid_to_sid_a;
pub use convert_string_sid_to_sid_a::ConvertStringSidToSidA;
mod convert_string_sid_to_sid_w;
pub use convert_string_sid_to_sid_w::ConvertStringSidToSidW;
mod convert_to_auto_inherit_private_object_security;
pub use convert_to_auto_inherit_private_object_security::ConvertToAutoInheritPrivateObjectSecurity;
mod copy_sid;
pub use copy_sid::CopySid;
mod create_code_authz_level;
pub use create_code_authz_level::CreateCodeAuthzLevel;
mod create_private_object_security;
pub use create_private_object_security::CreatePrivateObjectSecurity;
mod create_private_object_security_ex;
pub use create_private_object_security_ex::CreatePrivateObjectSecurityEx;
mod create_private_object_security_with_multiple_inheritance;
pub use create_private_object_security_with_multiple_inheritance::CreatePrivateObjectSecurityWithMultipleInheritance;
mod create_process_as_user_a;
pub use create_process_as_user_a::CreateProcessAsUserA;
mod create_process_as_user_w;
pub use create_process_as_user_w::CreateProcessAsUserW;
mod create_process_with_logon_w;
pub use create_process_with_logon_w::CreateProcessWithLogonW;
mod create_process_with_token_w;
pub use create_process_with_token_w::CreateProcessWithTokenW;
mod create_restricted_token;
pub use create_restricted_token::CreateRestrictedToken;
mod create_service_a;
pub use create_service_a::CreateServiceA;
mod create_service_ex;
pub use create_service_ex::CreateServiceEx;
mod create_service_w;
pub use create_service_w::CreateServiceW;
mod create_trace_instance_id;
pub use create_trace_instance_id::CreateTraceInstanceId;
mod create_well_known_sid;
pub use create_well_known_sid::CreateWellKnownSid;
mod cred_backup_credentials;
pub use cred_backup_credentials::CredBackupCredentials;
mod cred_delete_a;
pub use cred_delete_a::CredDeleteA;
mod cred_delete_w;
pub use cred_delete_w::CredDeleteW;
mod cred_encrypt_and_marshal_binary_blob;
pub use cred_encrypt_and_marshal_binary_blob::CredEncryptAndMarshalBinaryBlob;
mod cred_enumerate_a;
pub use cred_enumerate_a::CredEnumerateA;
mod cred_enumerate_w;
pub use cred_enumerate_w::CredEnumerateW;
mod cred_find_best_credential_a;
pub use cred_find_best_credential_a::CredFindBestCredentialA;
mod cred_find_best_credential_w;
pub use cred_find_best_credential_w::CredFindBestCredentialW;
mod cred_free;
pub use cred_free::CredFree;
mod cred_get_session_types;
pub use cred_get_session_types::CredGetSessionTypes;
mod cred_get_target_info_a;
pub use cred_get_target_info_a::CredGetTargetInfoA;
mod cred_get_target_info_w;
pub use cred_get_target_info_w::CredGetTargetInfoW;
mod cred_is_marshaled_credential_a;
pub use cred_is_marshaled_credential_a::CredIsMarshaledCredentialA;
mod cred_is_marshaled_credential_w;
pub use cred_is_marshaled_credential_w::CredIsMarshaledCredentialW;
mod cred_is_protected_a;
pub use cred_is_protected_a::CredIsProtectedA;
mod cred_is_protected_w;
pub use cred_is_protected_w::CredIsProtectedW;
mod cred_marshal_credential_a;
pub use cred_marshal_credential_a::CredMarshalCredentialA;
mod cred_marshal_credential_w;
pub use cred_marshal_credential_w::CredMarshalCredentialW;
mod cred_profile_loaded;
pub use cred_profile_loaded::CredProfileLoaded;
mod cred_profile_loaded_ex;
pub use cred_profile_loaded_ex::CredProfileLoadedEx;
mod cred_profile_unloaded;
pub use cred_profile_unloaded::CredProfileUnloaded;
mod cred_protect_a;
pub use cred_protect_a::CredProtectA;
mod cred_protect_w;
pub use cred_protect_w::CredProtectW;
mod cred_read_a;
pub use cred_read_a::CredReadA;
mod cred_read_by_token_handle;
pub use cred_read_by_token_handle::CredReadByTokenHandle;
mod cred_read_domain_credentials_a;
pub use cred_read_domain_credentials_a::CredReadDomainCredentialsA;
mod cred_read_domain_credentials_w;
pub use cred_read_domain_credentials_w::CredReadDomainCredentialsW;
mod cred_read_w;
pub use cred_read_w::CredReadW;
mod cred_rename_a;
pub use cred_rename_a::CredRenameA;
mod cred_rename_w;
pub use cred_rename_w::CredRenameW;
mod cred_restore_credentials;
pub use cred_restore_credentials::CredRestoreCredentials;
mod cred_unmarshal_credential_a;
pub use cred_unmarshal_credential_a::CredUnmarshalCredentialA;
mod cred_unmarshal_credential_w;
pub use cred_unmarshal_credential_w::CredUnmarshalCredentialW;
mod cred_unprotect_a;
pub use cred_unprotect_a::CredUnprotectA;
mod cred_unprotect_w;
pub use cred_unprotect_w::CredUnprotectW;
mod cred_write_a;
pub use cred_write_a::CredWriteA;
mod cred_write_domain_credentials_a;
pub use cred_write_domain_credentials_a::CredWriteDomainCredentialsA;
mod cred_write_domain_credentials_w;
pub use cred_write_domain_credentials_w::CredWriteDomainCredentialsW;
mod cred_write_w;
pub use cred_write_w::CredWriteW;
mod credp_convert_credential;
pub use credp_convert_credential::CredpConvertCredential;
mod credp_convert_one_credential_size;
pub use credp_convert_one_credential_size::CredpConvertOneCredentialSize;
mod credp_convert_target_info;
pub use credp_convert_target_info::CredpConvertTargetInfo;
mod credp_decode_credential;
pub use credp_decode_credential::CredpDecodeCredential;
mod credp_encode_credential;
pub use credp_encode_credential::CredpEncodeCredential;
mod credp_encode_secret;
pub use credp_encode_secret::CredpEncodeSecret;
mod crypt_context_add_ref;
pub use crypt_context_add_ref::CryptContextAddRef;
mod crypt_destroy_hash;
pub use crypt_destroy_hash::CryptDestroyHash;
mod crypt_destroy_key;
pub use crypt_destroy_key::CryptDestroyKey;
mod crypt_duplicate_hash;
pub use crypt_duplicate_hash::CryptDuplicateHash;
mod crypt_duplicate_key;
pub use crypt_duplicate_key::CryptDuplicateKey;
mod crypt_enum_provider_types_a;
pub use crypt_enum_provider_types_a::CryptEnumProviderTypesA;
mod crypt_enum_provider_types_w;
pub use crypt_enum_provider_types_w::CryptEnumProviderTypesW;
mod crypt_enum_providers_a;
pub use crypt_enum_providers_a::CryptEnumProvidersA;
mod crypt_enum_providers_w;
pub use crypt_enum_providers_w::CryptEnumProvidersW;
mod crypt_export_key;
pub use crypt_export_key::CryptExportKey;
mod crypt_get_default_provider_a;
pub use crypt_get_default_provider_a::CryptGetDefaultProviderA;
mod crypt_get_default_provider_w;
pub use crypt_get_default_provider_w::CryptGetDefaultProviderW;
mod crypt_get_prov_param;
pub use crypt_get_prov_param::CryptGetProvParam;
mod crypt_get_user_key;
pub use crypt_get_user_key::CryptGetUserKey;
mod crypt_hash_session_key;
pub use crypt_hash_session_key::CryptHashSessionKey;
mod crypt_set_hash_param;
pub use crypt_set_hash_param::CryptSetHashParam;
mod crypt_set_key_param;
pub use crypt_set_key_param::CryptSetKeyParam;
mod crypt_set_prov_param;
pub use crypt_set_prov_param::CryptSetProvParam;
mod crypt_set_provider_a;
pub use crypt_set_provider_a::CryptSetProviderA;
mod crypt_set_provider_ex_a;
pub use crypt_set_provider_ex_a::CryptSetProviderExA;
mod crypt_set_provider_ex_w;
pub use crypt_set_provider_ex_w::CryptSetProviderExW;
mod crypt_set_provider_w;
pub use crypt_set_provider_w::CryptSetProviderW;
mod crypt_verify_signature_a;
pub use crypt_verify_signature_a::CryptVerifySignatureA;
mod crypt_verify_signature_w;
pub use crypt_verify_signature_w::CryptVerifySignatureW;
mod cve_event_write;
pub use cve_event_write::CveEventWrite;
mod decrypt_file_a;
pub use decrypt_file_a::DecryptFileA;
mod decrypt_file_w;
pub use decrypt_file_w::DecryptFileW;
mod delete_ace;
pub use delete_ace::DeleteAce;
mod delete_service;
pub use delete_service::DeleteService;
mod deregister_event_source;
pub use deregister_event_source::DeregisterEventSource;
mod destroy_private_object_security;
pub use destroy_private_object_security::DestroyPrivateObjectSecurity;
mod duplicate_encryption_info_file;
pub use duplicate_encryption_info_file::DuplicateEncryptionInfoFile;
mod duplicate_token;
pub use duplicate_token::DuplicateToken;
mod duplicate_token_ex;
pub use duplicate_token_ex::DuplicateTokenEx;
mod enable_trace;
pub use enable_trace::EnableTrace;
mod enable_trace_ex;
pub use enable_trace_ex::EnableTraceEx;
mod enable_trace_ex2;
pub use enable_trace_ex2::EnableTraceEx2;
mod encrypt_file_a;
pub use encrypt_file_a::EncryptFileA;
mod encrypt_file_w;
pub use encrypt_file_w::EncryptFileW;
mod encrypted_file_key_info;
pub use encrypted_file_key_info::EncryptedFileKeyInfo;
mod encryption_disable;
pub use encryption_disable::EncryptionDisable;
mod enum_dependent_services_a;
pub use enum_dependent_services_a::EnumDependentServicesA;
mod enum_dependent_services_w;
pub use enum_dependent_services_w::EnumDependentServicesW;
mod enum_dynamic_time_zone_information;
pub use enum_dynamic_time_zone_information::EnumDynamicTimeZoneInformation;
mod enum_service_group_w;
pub use enum_service_group_w::EnumServiceGroupW;
mod enum_services_status_a;
pub use enum_services_status_a::EnumServicesStatusA;
mod enum_services_status_ex_a;
pub use enum_services_status_ex_a::EnumServicesStatusExA;
mod enum_services_status_ex_w;
pub use enum_services_status_ex_w::EnumServicesStatusExW;
mod enum_services_status_w;
pub use enum_services_status_w::EnumServicesStatusW;
mod enumerate_trace_guids;
pub use enumerate_trace_guids::EnumerateTraceGuids;
mod enumerate_trace_guids_ex;
pub use enumerate_trace_guids_ex::EnumerateTraceGuidsEx;
mod equal_domain_sid;
pub use equal_domain_sid::EqualDomainSid;
mod equal_prefix_sid;
pub use equal_prefix_sid::EqualPrefixSid;
mod equal_sid;
pub use equal_sid::EqualSid;
mod event_access_control;
pub use event_access_control::EventAccessControl;
mod event_access_query;
pub use event_access_query::EventAccessQuery;
mod event_access_remove;
pub use event_access_remove::EventAccessRemove;
mod event_activity_id_control;
pub use event_activity_id_control::EventActivityIdControl;
mod event_enabled;
pub use event_enabled::EventEnabled;
mod event_provider_enabled;
pub use event_provider_enabled::EventProviderEnabled;
mod event_register;
pub use event_register::EventRegister;
mod event_set_information;
pub use event_set_information::EventSetInformation;
mod event_unregister;
pub use event_unregister::EventUnregister;
mod event_write;
pub use event_write::EventWrite;
mod event_write_end_scenario;
pub use event_write_end_scenario::EventWriteEndScenario;
mod event_write_ex;
pub use event_write_ex::EventWriteEx;
mod event_write_start_scenario;
pub use event_write_start_scenario::EventWriteStartScenario;
mod event_write_string;
pub use event_write_string::EventWriteString;
mod event_write_transfer;
pub use event_write_transfer::EventWriteTransfer;
mod file_encryption_status_a;
pub use file_encryption_status_a::FileEncryptionStatusA;
mod file_encryption_status_w;
pub use file_encryption_status_w::FileEncryptionStatusW;
mod find_first_free_ace;
pub use find_first_free_ace::FindFirstFreeAce;
mod flush_efs_cache;
pub use flush_efs_cache::FlushEfsCache;
mod flush_trace_a;
pub use flush_trace_a::FlushTraceA;
mod flush_trace_w;
pub use flush_trace_w::FlushTraceW;
mod free_encrypted_file_key_info;
pub use free_encrypted_file_key_info::FreeEncryptedFileKeyInfo;
mod free_encrypted_file_metadata;
pub use free_encrypted_file_metadata::FreeEncryptedFileMetadata;
mod free_encryption_certificate_hash_list;
pub use free_encryption_certificate_hash_list::FreeEncryptionCertificateHashList;
mod free_inherited_from_array;
pub use free_inherited_from_array::FreeInheritedFromArray;
mod free_sid;
pub use free_sid::FreeSid;
mod get_access_permissions_for_object_a;
pub use get_access_permissions_for_object_a::GetAccessPermissionsForObjectA;
mod get_access_permissions_for_object_w;
pub use get_access_permissions_for_object_w::GetAccessPermissionsForObjectW;
mod get_ace;
pub use get_ace::GetAce;
mod get_acl_information;
pub use get_acl_information::GetAclInformation;
mod get_audited_permissions_from_acl_a;
pub use get_audited_permissions_from_acl_a::GetAuditedPermissionsFromAclA;
mod get_audited_permissions_from_acl_w;
pub use get_audited_permissions_from_acl_w::GetAuditedPermissionsFromAclW;
mod get_current_hw_profile_a;
pub use get_current_hw_profile_a::GetCurrentHwProfileA;
mod get_current_hw_profile_w;
pub use get_current_hw_profile_w::GetCurrentHwProfileW;
mod get_dynamic_time_zone_information_effective_years;
pub use get_dynamic_time_zone_information_effective_years::GetDynamicTimeZoneInformationEffectiveYears;
mod get_effective_rights_from_acl_a;
pub use get_effective_rights_from_acl_a::GetEffectiveRightsFromAclA;
mod get_effective_rights_from_acl_w;
pub use get_effective_rights_from_acl_w::GetEffectiveRightsFromAclW;
mod get_encrypted_file_metadata;
pub use get_encrypted_file_metadata::GetEncryptedFileMetadata;
mod get_event_log_information;
pub use get_event_log_information::GetEventLogInformation;
mod get_explicit_entries_from_acl_a;
pub use get_explicit_entries_from_acl_a::GetExplicitEntriesFromAclA;
mod get_explicit_entries_from_acl_w;
pub use get_explicit_entries_from_acl_w::GetExplicitEntriesFromAclW;
mod get_file_security_a;
pub use get_file_security_a::GetFileSecurityA;
mod get_file_security_w;
pub use get_file_security_w::GetFileSecurityW;
mod get_information_code_authz_level_w;
pub use get_information_code_authz_level_w::GetInformationCodeAuthzLevelW;
mod get_information_code_authz_policy_w;
pub use get_information_code_authz_policy_w::GetInformationCodeAuthzPolicyW;
mod get_inheritance_source_a;
pub use get_inheritance_source_a::GetInheritanceSourceA;
mod get_inheritance_source_w;
pub use get_inheritance_source_w::GetInheritanceSourceW;
mod get_kernel_object_security;
pub use get_kernel_object_security::GetKernelObjectSecurity;
mod get_length_sid;
pub use get_length_sid::GetLengthSid;
mod get_local_managed_application_data;
pub use get_local_managed_application_data::GetLocalManagedApplicationData;
mod get_local_managed_applications;
pub use get_local_managed_applications::GetLocalManagedApplications;
mod get_managed_application_categories;
pub use get_managed_application_categories::GetManagedApplicationCategories;
mod get_managed_applications;
pub use get_managed_applications::GetManagedApplications;
mod get_multiple_trustee_a;
pub use get_multiple_trustee_a::GetMultipleTrusteeA;
mod get_multiple_trustee_operation_a;
pub use get_multiple_trustee_operation_a::GetMultipleTrusteeOperationA;
mod get_multiple_trustee_operation_w;
pub use get_multiple_trustee_operation_w::GetMultipleTrusteeOperationW;
mod get_multiple_trustee_w;
pub use get_multiple_trustee_w::GetMultipleTrusteeW;
mod get_named_security_info_a;
pub use get_named_security_info_a::GetNamedSecurityInfoA;
mod get_named_security_info_ex_a;
pub use get_named_security_info_ex_a::GetNamedSecurityInfoExA;
mod get_named_security_info_ex_w;
pub use get_named_security_info_ex_w::GetNamedSecurityInfoExW;
mod get_named_security_info_w;
pub use get_named_security_info_w::GetNamedSecurityInfoW;
mod get_number_of_event_log_records;
pub use get_number_of_event_log_records::GetNumberOfEventLogRecords;
mod get_oldest_event_log_record;
pub use get_oldest_event_log_record::GetOldestEventLogRecord;
mod get_overlapped_access_results;
pub use get_overlapped_access_results::GetOverlappedAccessResults;
mod get_private_object_security;
pub use get_private_object_security::GetPrivateObjectSecurity;
mod get_security_descriptor_control;
pub use get_security_descriptor_control::GetSecurityDescriptorControl;
mod get_security_descriptor_dacl;
pub use get_security_descriptor_dacl::GetSecurityDescriptorDacl;
mod get_security_descriptor_group;
pub use get_security_descriptor_group::GetSecurityDescriptorGroup;
mod get_security_descriptor_length;
pub use get_security_descriptor_length::GetSecurityDescriptorLength;
mod get_security_descriptor_owner;
pub use get_security_descriptor_owner::GetSecurityDescriptorOwner;
mod get_security_descriptor_rm_control;
pub use get_security_descriptor_rm_control::GetSecurityDescriptorRMControl;
mod get_security_descriptor_sacl;
pub use get_security_descriptor_sacl::GetSecurityDescriptorSacl;
mod get_security_info;
pub use get_security_info::GetSecurityInfo;
mod get_security_info_ex_a;
pub use get_security_info_ex_a::GetSecurityInfoExA;
mod get_security_info_ex_w;
pub use get_security_info_ex_w::GetSecurityInfoExW;
mod get_service_display_name_a;
pub use get_service_display_name_a::GetServiceDisplayNameA;
mod get_service_display_name_w;
pub use get_service_display_name_w::GetServiceDisplayNameW;
mod get_service_key_name_a;
pub use get_service_key_name_a::GetServiceKeyNameA;
mod get_service_key_name_w;
pub use get_service_key_name_w::GetServiceKeyNameW;
mod get_sid_identifier_authority;
pub use get_sid_identifier_authority::GetSidIdentifierAuthority;
mod get_sid_length_required;
pub use get_sid_length_required::GetSidLengthRequired;
mod get_sid_sub_authority;
pub use get_sid_sub_authority::GetSidSubAuthority;
mod get_sid_sub_authority_count;
pub use get_sid_sub_authority_count::GetSidSubAuthorityCount;
mod get_string_condition_from_binary;
pub use get_string_condition_from_binary::GetStringConditionFromBinary;
mod get_thread_wait_chain;
pub use get_thread_wait_chain::GetThreadWaitChain;
mod get_token_information;
pub use get_token_information::GetTokenInformation;
mod get_trace_enable_flags;
pub use get_trace_enable_flags::GetTraceEnableFlags;
mod get_trace_enable_level;
pub use get_trace_enable_level::GetTraceEnableLevel;
mod get_trace_logger_handle;
pub use get_trace_logger_handle::GetTraceLoggerHandle;
mod get_trustee_form_a;
pub use get_trustee_form_a::GetTrusteeFormA;
mod get_trustee_form_w;
pub use get_trustee_form_w::GetTrusteeFormW;
mod get_trustee_name_a;
pub use get_trustee_name_a::GetTrusteeNameA;
mod get_trustee_name_w;
pub use get_trustee_name_w::GetTrusteeNameW;
mod get_trustee_type_a;
pub use get_trustee_type_a::GetTrusteeTypeA;
mod get_trustee_type_w;
pub use get_trustee_type_w::GetTrusteeTypeW;
mod get_windows_account_domain_sid;
pub use get_windows_account_domain_sid::GetWindowsAccountDomainSid;
mod i_query_tag_information;
pub use i_query_tag_information::I_QueryTagInformation;
mod i_sc_get_current_group_state_w;
pub use i_sc_get_current_group_state_w::I_ScGetCurrentGroupStateW;
mod i_sc_is_security_process;
pub use i_sc_is_security_process::I_ScIsSecurityProcess;
mod i_sc_pn_p_get_service_name;
pub use i_sc_pn_p_get_service_name::I_ScPnPGetServiceName;
mod i_sc_query_service_config;
pub use i_sc_query_service_config::I_ScQueryServiceConfig;
mod i_sc_register_preshutdown_restart;
pub use i_sc_register_preshutdown_restart::I_ScRegisterPreshutdownRestart;
mod i_sc_reparse_service_database;
pub use i_sc_reparse_service_database::I_ScReparseServiceDatabase;
mod i_sc_send_pn_p_message;
pub use i_sc_send_pn_p_message::I_ScSendPnPMessage;
mod i_sc_send_ts_message;
pub use i_sc_send_ts_message::I_ScSendTSMessage;
mod i_sc_set_service_bits_a;
pub use i_sc_set_service_bits_a::I_ScSetServiceBitsA;
mod i_sc_set_service_bits_w;
pub use i_sc_set_service_bits_w::I_ScSetServiceBitsW;
mod i_sc_validate_pn_p_service;
pub use i_sc_validate_pn_p_service::I_ScValidatePnPService;
mod identify_code_authz_level_w;
pub use identify_code_authz_level_w::IdentifyCodeAuthzLevelW;
mod impersonate_anonymous_token;
pub use impersonate_anonymous_token::ImpersonateAnonymousToken;
mod impersonate_logged_on_user;
pub use impersonate_logged_on_user::ImpersonateLoggedOnUser;
mod impersonate_named_pipe_client;
pub use impersonate_named_pipe_client::ImpersonateNamedPipeClient;
mod impersonate_self;
pub use impersonate_self::ImpersonateSelf;
mod initialize_acl;
pub use initialize_acl::InitializeAcl;
mod initialize_security_descriptor;
pub use initialize_security_descriptor::InitializeSecurityDescriptor;
mod initialize_sid;
pub use initialize_sid::InitializeSid;
mod initiate_shutdown_a;
pub use initiate_shutdown_a::InitiateShutdownA;
mod initiate_shutdown_w;
pub use initiate_shutdown_w::InitiateShutdownW;
mod initiate_system_shutdown_a;
pub use initiate_system_shutdown_a::InitiateSystemShutdownA;
mod initiate_system_shutdown_ex_a;
pub use initiate_system_shutdown_ex_a::InitiateSystemShutdownExA;
mod initiate_system_shutdown_ex_w;
pub use initiate_system_shutdown_ex_w::InitiateSystemShutdownExW;
mod initiate_system_shutdown_w;
pub use initiate_system_shutdown_w::InitiateSystemShutdownW;
mod install_application;
pub use install_application::InstallApplication;
mod is_text_unicode;
pub use is_text_unicode::IsTextUnicode;
mod is_token_restricted;
pub use is_token_restricted::IsTokenRestricted;
mod is_token_untrusted;
pub use is_token_untrusted::IsTokenUntrusted;
mod is_valid_acl;
pub use is_valid_acl::IsValidAcl;
mod is_valid_relative_security_descriptor;
pub use is_valid_relative_security_descriptor::IsValidRelativeSecurityDescriptor;
mod is_valid_security_descriptor;
pub use is_valid_security_descriptor::IsValidSecurityDescriptor;
mod is_valid_sid;
pub use is_valid_sid::IsValidSid;
mod is_well_known_sid;
pub use is_well_known_sid::IsWellKnownSid;
mod lock_service_database;
pub use lock_service_database::LockServiceDatabase;
mod logon_secondary_user_into_session_w;
pub use logon_secondary_user_into_session_w::LogonSecondaryUserIntoSessionW;
mod logon_user_a;
pub use logon_user_a::LogonUserA;
mod logon_user_ex_a;
pub use logon_user_ex_a::LogonUserExA;
mod logon_user_ex_ex_w;
pub use logon_user_ex_ex_w::LogonUserExExW;
mod logon_user_ex_w;
pub use logon_user_ex_w::LogonUserExW;
mod logon_user_w;
pub use logon_user_w::LogonUserW;
mod lookup_account_name_a;
pub use lookup_account_name_a::LookupAccountNameA;
mod lookup_account_name_w;
pub use lookup_account_name_w::LookupAccountNameW;
mod lookup_account_sid_a;
pub use lookup_account_sid_a::LookupAccountSidA;
mod lookup_account_sid_w;
pub use lookup_account_sid_w::LookupAccountSidW;
mod lookup_privilege_display_name_a;
pub use lookup_privilege_display_name_a::LookupPrivilegeDisplayNameA;
mod lookup_privilege_display_name_w;
pub use lookup_privilege_display_name_w::LookupPrivilegeDisplayNameW;
mod lookup_privilege_name_a;
pub use lookup_privilege_name_a::LookupPrivilegeNameA;
mod lookup_privilege_name_w;
pub use lookup_privilege_name_w::LookupPrivilegeNameW;
mod lookup_privilege_value_a;
pub use lookup_privilege_value_a::LookupPrivilegeValueA;
mod lookup_security_descriptor_parts_a;
pub use lookup_security_descriptor_parts_a::LookupSecurityDescriptorPartsA;
mod lookup_security_descriptor_parts_w;
pub use lookup_security_descriptor_parts_w::LookupSecurityDescriptorPartsW;
mod lsa_add_account_rights;
pub use lsa_add_account_rights::LsaAddAccountRights;
mod lsa_add_privileges_to_account;
pub use lsa_add_privileges_to_account::LsaAddPrivilegesToAccount;
mod lsa_clear_audit_log;
pub use lsa_clear_audit_log::LsaClearAuditLog;
mod lsa_close;
pub use lsa_close::LsaClose;
mod lsa_configure_auto_logon_credentials;
pub use lsa_configure_auto_logon_credentials::LsaConfigureAutoLogonCredentials;
mod lsa_create_account;
pub use lsa_create_account::LsaCreateAccount;
mod lsa_create_secret;
pub use lsa_create_secret::LsaCreateSecret;
mod lsa_create_trusted_domain;
pub use lsa_create_trusted_domain::LsaCreateTrustedDomain;
mod lsa_create_trusted_domain_ex;
pub use lsa_create_trusted_domain_ex::LsaCreateTrustedDomainEx;
mod lsa_delete;
pub use lsa_delete::LsaDelete;
mod lsa_delete_trusted_domain;
pub use lsa_delete_trusted_domain::LsaDeleteTrustedDomain;
mod lsa_disable_user_arso;
pub use lsa_disable_user_arso::LsaDisableUserArso;
mod lsa_enable_user_arso;
pub use lsa_enable_user_arso::LsaEnableUserArso;
mod lsa_enumerate_account_rights;
pub use lsa_enumerate_account_rights::LsaEnumerateAccountRights;
mod lsa_enumerate_accounts;
pub use lsa_enumerate_accounts::LsaEnumerateAccounts;
mod lsa_enumerate_accounts_with_user_right;
pub use lsa_enumerate_accounts_with_user_right::LsaEnumerateAccountsWithUserRight;
mod lsa_enumerate_privileges;
pub use lsa_enumerate_privileges::LsaEnumeratePrivileges;
mod lsa_enumerate_privileges_of_account;
pub use lsa_enumerate_privileges_of_account::LsaEnumeratePrivilegesOfAccount;
mod lsa_enumerate_trusted_domains;
pub use lsa_enumerate_trusted_domains::LsaEnumerateTrustedDomains;
mod lsa_enumerate_trusted_domains_ex;
pub use lsa_enumerate_trusted_domains_ex::LsaEnumerateTrustedDomainsEx;
mod lsa_free_memory;
pub use lsa_free_memory::LsaFreeMemory;
mod lsa_get_applied_capi_ds;
pub use lsa_get_applied_capi_ds::LsaGetAppliedCAPIDs;
mod lsa_get_device_registration_info;
pub use lsa_get_device_registration_info::LsaGetDeviceRegistrationInfo;
mod lsa_get_quotas_for_account;
pub use lsa_get_quotas_for_account::LsaGetQuotasForAccount;
mod lsa_get_remote_user_name;
pub use lsa_get_remote_user_name::LsaGetRemoteUserName;
mod lsa_get_system_access_account;
pub use lsa_get_system_access_account::LsaGetSystemAccessAccount;
mod lsa_get_user_name;
pub use lsa_get_user_name::LsaGetUserName;
mod lsa_ic_lookup_names;
pub use lsa_ic_lookup_names::LsaICLookupNames;
mod lsa_ic_lookup_names_with_creds;
pub use lsa_ic_lookup_names_with_creds::LsaICLookupNamesWithCreds;
mod lsa_ic_lookup_sids;
pub use lsa_ic_lookup_sids::LsaICLookupSids;
mod lsa_ic_lookup_sids_with_creds;
pub use lsa_ic_lookup_sids_with_creds::LsaICLookupSidsWithCreds;
mod lsa_invoke_trust_scanner;
pub use lsa_invoke_trust_scanner::LsaInvokeTrustScanner;
mod lsa_is_user_arso_allowed;
pub use lsa_is_user_arso_allowed::LsaIsUserArsoAllowed;
mod lsa_is_user_arso_enabled;
pub use lsa_is_user_arso_enabled::LsaIsUserArsoEnabled;
mod lsa_lookup_names;
pub use lsa_lookup_names::LsaLookupNames;
mod lsa_lookup_names2;
pub use lsa_lookup_names2::LsaLookupNames2;
mod lsa_lookup_privilege_display_name;
pub use lsa_lookup_privilege_display_name::LsaLookupPrivilegeDisplayName;
mod lsa_lookup_privilege_name;
pub use lsa_lookup_privilege_name::LsaLookupPrivilegeName;
mod lsa_lookup_privilege_value;
pub use lsa_lookup_privilege_value::LsaLookupPrivilegeValue;
mod lsa_lookup_sids;
pub use lsa_lookup_sids::LsaLookupSids;
mod lsa_lookup_sids2;
pub use lsa_lookup_sids2::LsaLookupSids2;
mod lsa_manage_sid_name_mapping;
pub use lsa_manage_sid_name_mapping::LsaManageSidNameMapping;
mod lsa_nt_status_to_win_error;
pub use lsa_nt_status_to_win_error::LsaNtStatusToWinError;
mod lsa_open_account;
pub use lsa_open_account::LsaOpenAccount;
mod lsa_open_policy;
pub use lsa_open_policy::LsaOpenPolicy;
mod lsa_open_policy_sce;
pub use lsa_open_policy_sce::LsaOpenPolicySce;
mod lsa_open_secret;
pub use lsa_open_secret::LsaOpenSecret;
mod lsa_open_trusted_domain;
pub use lsa_open_trusted_domain::LsaOpenTrustedDomain;
mod lsa_open_trusted_domain_by_name;
pub use lsa_open_trusted_domain_by_name::LsaOpenTrustedDomainByName;
mod lsa_profile_deleted;
pub use lsa_profile_deleted::LsaProfileDeleted;
mod lsa_purge_local_system_access_table;
pub use lsa_purge_local_system_access_table::LsaPurgeLocalSystemAccessTable;
mod lsa_query_ca_ps;
pub use lsa_query_ca_ps::LsaQueryCAPs;
mod lsa_query_domain_information_policy;
pub use lsa_query_domain_information_policy::LsaQueryDomainInformationPolicy;
mod lsa_query_forest_trust_information;
pub use lsa_query_forest_trust_information::LsaQueryForestTrustInformation;
mod lsa_query_forest_trust_information2;
pub use lsa_query_forest_trust_information2::LsaQueryForestTrustInformation2;
mod lsa_query_info_trusted_domain;
pub use lsa_query_info_trusted_domain::LsaQueryInfoTrustedDomain;
mod lsa_query_information_policy;
pub use lsa_query_information_policy::LsaQueryInformationPolicy;
mod lsa_query_local_system_access;
pub use lsa_query_local_system_access::LsaQueryLocalSystemAccess;
mod lsa_query_local_system_access_all;
pub use lsa_query_local_system_access_all::LsaQueryLocalSystemAccessAll;
mod lsa_query_secret;
pub use lsa_query_secret::LsaQuerySecret;
mod lsa_query_security_object;
pub use lsa_query_security_object::LsaQuerySecurityObject;
mod lsa_query_trusted_domain_info;
pub use lsa_query_trusted_domain_info::LsaQueryTrustedDomainInfo;
mod lsa_query_trusted_domain_info_by_name;
pub use lsa_query_trusted_domain_info_by_name::LsaQueryTrustedDomainInfoByName;
mod lsa_remove_account_rights;
pub use lsa_remove_account_rights::LsaRemoveAccountRights;
mod lsa_remove_privileges_from_account;
pub use lsa_remove_privileges_from_account::LsaRemovePrivilegesFromAccount;
mod lsa_retrieve_private_data;
pub use lsa_retrieve_private_data::LsaRetrievePrivateData;
mod lsa_set_ca_ps;
pub use lsa_set_ca_ps::LsaSetCAPs;
mod lsa_set_domain_information_policy;
pub use lsa_set_domain_information_policy::LsaSetDomainInformationPolicy;
mod lsa_set_forest_trust_information;
pub use lsa_set_forest_trust_information::LsaSetForestTrustInformation;
mod lsa_set_forest_trust_information2;
pub use lsa_set_forest_trust_information2::LsaSetForestTrustInformation2;
mod lsa_set_information_policy;
pub use lsa_set_information_policy::LsaSetInformationPolicy;
mod lsa_set_information_trusted_domain;
pub use lsa_set_information_trusted_domain::LsaSetInformationTrustedDomain;
mod lsa_set_local_system_access;
pub use lsa_set_local_system_access::LsaSetLocalSystemAccess;
mod lsa_set_quotas_for_account;
pub use lsa_set_quotas_for_account::LsaSetQuotasForAccount;
mod lsa_set_secret;
pub use lsa_set_secret::LsaSetSecret;
mod lsa_set_security_object;
pub use lsa_set_security_object::LsaSetSecurityObject;
mod lsa_set_system_access_account;
pub use lsa_set_system_access_account::LsaSetSystemAccessAccount;
mod lsa_set_trusted_domain_info_by_name;
pub use lsa_set_trusted_domain_info_by_name::LsaSetTrustedDomainInfoByName;
mod lsa_set_trusted_domain_information;
pub use lsa_set_trusted_domain_information::LsaSetTrustedDomainInformation;
mod lsa_store_private_data;
pub use lsa_store_private_data::LsaStorePrivateData;
mod lsa_validate_proc_unique_luid;
pub use lsa_validate_proc_unique_luid::LsaValidateProcUniqueLuid;
mod md4_final;
pub use md4_final::MD4Final;
mod md4_init;
pub use md4_init::MD4Init;
mod md4_update;
pub use md4_update::MD4Update;
mod md5_final;
pub use md5_final::MD5Final;
mod md5_init;
pub use md5_init::MD5Init;
mod md5_update;
pub use md5_update::MD5Update;
mod midl_user_free_ext;
pub use midl_user_free_ext::MIDL_user_free_Ext;
mod ms_chap_srv_change_password;
pub use ms_chap_srv_change_password::MSChapSrvChangePassword;
mod ms_chap_srv_change_password2;
pub use ms_chap_srv_change_password2::MSChapSrvChangePassword2;
mod make_absolute_sd;
pub use make_absolute_sd::MakeAbsoluteSD;
mod make_absolute_sd2;
pub use make_absolute_sd2::MakeAbsoluteSD2;
mod make_self_relative_sd;
pub use make_self_relative_sd::MakeSelfRelativeSD;
mod map_generic_mask;
pub use map_generic_mask::MapGenericMask;
mod notify_boot_config_status;
pub use notify_boot_config_status::NotifyBootConfigStatus;
mod notify_change_event_log;
pub use notify_change_event_log::NotifyChangeEventLog;
mod notify_service_status_change;
pub use notify_service_status_change::NotifyServiceStatusChange;
mod notify_service_status_change_a;
pub use notify_service_status_change_a::NotifyServiceStatusChangeA;
mod notify_service_status_change_w;
pub use notify_service_status_change_w::NotifyServiceStatusChangeW;
mod np_get_user_name;
pub use np_get_user_name::NpGetUserName;
mod object_close_audit_alarm_a;
pub use object_close_audit_alarm_a::ObjectCloseAuditAlarmA;
mod object_close_audit_alarm_w;
pub use object_close_audit_alarm_w::ObjectCloseAuditAlarmW;
mod object_delete_audit_alarm_a;
pub use object_delete_audit_alarm_a::ObjectDeleteAuditAlarmA;
mod object_delete_audit_alarm_w;
pub use object_delete_audit_alarm_w::ObjectDeleteAuditAlarmW;
mod object_open_audit_alarm_a;
pub use object_open_audit_alarm_a::ObjectOpenAuditAlarmA;
mod object_open_audit_alarm_w;
pub use object_open_audit_alarm_w::ObjectOpenAuditAlarmW;
mod object_privilege_audit_alarm_a;
pub use object_privilege_audit_alarm_a::ObjectPrivilegeAuditAlarmA;
mod object_privilege_audit_alarm_w;
pub use object_privilege_audit_alarm_w::ObjectPrivilegeAuditAlarmW;
mod open_backup_event_log_a;
pub use open_backup_event_log_a::OpenBackupEventLogA;
mod open_backup_event_log_w;
pub use open_backup_event_log_w::OpenBackupEventLogW;
mod open_encrypted_file_raw_a;
pub use open_encrypted_file_raw_a::OpenEncryptedFileRawA;
mod open_encrypted_file_raw_w;
pub use open_encrypted_file_raw_w::OpenEncryptedFileRawW;
mod open_event_log_a;
pub use open_event_log_a::OpenEventLogA;
mod open_event_log_w;
pub use open_event_log_w::OpenEventLogW;
mod open_process_token;
pub use open_process_token::OpenProcessToken;
mod open_sc_manager_a;
pub use open_sc_manager_a::OpenSCManagerA;
mod open_sc_manager_w;
pub use open_sc_manager_w::OpenSCManagerW;
mod open_service_a;
pub use open_service_a::OpenServiceA;
mod open_service_w;
pub use open_service_w::OpenServiceW;
mod open_thread_token;
pub use open_thread_token::OpenThreadToken;
mod open_thread_wait_chain_session;
pub use open_thread_wait_chain_session::OpenThreadWaitChainSession;
mod open_trace_a;
pub use open_trace_a::OpenTraceA;
mod open_trace_w;
pub use open_trace_w::OpenTraceW;
mod operation_end;
pub use operation_end::OperationEnd;
mod operation_start;
pub use operation_start::OperationStart;
mod perf_add_counters;
pub use perf_add_counters::PerfAddCounters;
mod perf_close_query_handle;
pub use perf_close_query_handle::PerfCloseQueryHandle;
mod perf_create_instance;
pub use perf_create_instance::PerfCreateInstance;
mod perf_decrement_u_long_counter_value;
pub use perf_decrement_u_long_counter_value::PerfDecrementULongCounterValue;
mod perf_decrement_u_long_long_counter_value;
pub use perf_decrement_u_long_long_counter_value::PerfDecrementULongLongCounterValue;
mod perf_delete_counters;
pub use perf_delete_counters::PerfDeleteCounters;
mod perf_delete_instance;
pub use perf_delete_instance::PerfDeleteInstance;
mod perf_enumerate_counter_set;
pub use perf_enumerate_counter_set::PerfEnumerateCounterSet;
mod perf_enumerate_counter_set_instances;
pub use perf_enumerate_counter_set_instances::PerfEnumerateCounterSetInstances;
mod perf_increment_u_long_counter_value;
pub use perf_increment_u_long_counter_value::PerfIncrementULongCounterValue;
mod perf_increment_u_long_long_counter_value;
pub use perf_increment_u_long_long_counter_value::PerfIncrementULongLongCounterValue;
mod perf_open_query_handle;
pub use perf_open_query_handle::PerfOpenQueryHandle;
mod perf_query_counter_data;
pub use perf_query_counter_data::PerfQueryCounterData;
mod perf_query_counter_info;
pub use perf_query_counter_info::PerfQueryCounterInfo;
mod perf_query_counter_set_registration_info;
pub use perf_query_counter_set_registration_info::PerfQueryCounterSetRegistrationInfo;
mod perf_query_instance;
pub use perf_query_instance::PerfQueryInstance;
mod perf_reg_close_key;
pub use perf_reg_close_key::PerfRegCloseKey;
mod perf_reg_enum_key;
pub use perf_reg_enum_key::PerfRegEnumKey;
mod perf_reg_enum_value;
pub use perf_reg_enum_value::PerfRegEnumValue;
mod perf_reg_query_info_key;
pub use perf_reg_query_info_key::PerfRegQueryInfoKey;
mod perf_reg_query_value;
pub use perf_reg_query_value::PerfRegQueryValue;
mod perf_reg_set_value;
pub use perf_reg_set_value::PerfRegSetValue;
mod perf_set_counter_ref_value;
pub use perf_set_counter_ref_value::PerfSetCounterRefValue;
mod perf_set_counter_set_info;
pub use perf_set_counter_set_info::PerfSetCounterSetInfo;
mod perf_set_u_long_counter_value;
pub use perf_set_u_long_counter_value::PerfSetULongCounterValue;
mod perf_set_u_long_long_counter_value;
pub use perf_set_u_long_long_counter_value::PerfSetULongLongCounterValue;
mod perf_start_provider;
pub use perf_start_provider::PerfStartProvider;
mod perf_start_provider_ex;
pub use perf_start_provider_ex::PerfStartProviderEx;
mod perf_stop_provider;
pub use perf_stop_provider::PerfStopProvider;
mod privilege_check;
pub use privilege_check::PrivilegeCheck;
mod privileged_service_audit_alarm_a;
pub use privileged_service_audit_alarm_a::PrivilegedServiceAuditAlarmA;
mod privileged_service_audit_alarm_w;
pub use privileged_service_audit_alarm_w::PrivilegedServiceAuditAlarmW;
mod process_idle_tasks;
pub use process_idle_tasks::ProcessIdleTasks;
mod process_idle_tasks_w;
pub use process_idle_tasks_w::ProcessIdleTasksW;
mod process_trace;
pub use process_trace::ProcessTrace;
mod query_all_traces_a;
pub use query_all_traces_a::QueryAllTracesA;
mod query_all_traces_w;
pub use query_all_traces_w::QueryAllTracesW;
mod query_local_user_service_name;
pub use query_local_user_service_name::QueryLocalUserServiceName;
mod query_recovery_agents_on_encrypted_file;
pub use query_recovery_agents_on_encrypted_file::QueryRecoveryAgentsOnEncryptedFile;
mod query_security_access_mask;
pub use query_security_access_mask::QuerySecurityAccessMask;
mod query_service_config2_a;
pub use query_service_config2_a::QueryServiceConfig2A;
mod query_service_config2_w;
pub use query_service_config2_w::QueryServiceConfig2W;
mod query_service_config_a;
pub use query_service_config_a::QueryServiceConfigA;
mod query_service_config_w;
pub use query_service_config_w::QueryServiceConfigW;
mod query_service_dynamic_information;
pub use query_service_dynamic_information::QueryServiceDynamicInformation;
mod query_service_lock_status_a;
pub use query_service_lock_status_a::QueryServiceLockStatusA;
mod query_service_lock_status_w;
pub use query_service_lock_status_w::QueryServiceLockStatusW;
mod query_service_object_security;
pub use query_service_object_security::QueryServiceObjectSecurity;
mod query_service_status;
pub use query_service_status::QueryServiceStatus;
mod query_service_status_ex;
pub use query_service_status_ex::QueryServiceStatusEx;
mod query_trace_a;
pub use query_trace_a::QueryTraceA;
mod query_trace_processing_handle;
pub use query_trace_processing_handle::QueryTraceProcessingHandle;
mod query_trace_w;
pub use query_trace_w::QueryTraceW;
mod query_user_service_name;
pub use query_user_service_name::QueryUserServiceName;
mod query_user_service_name_for_context;
pub use query_user_service_name_for_context::QueryUserServiceNameForContext;
mod query_users_on_encrypted_file;
pub use query_users_on_encrypted_file::QueryUsersOnEncryptedFile;
mod read_encrypted_file_raw;
pub use read_encrypted_file_raw::ReadEncryptedFileRaw;
mod read_event_log_a;
pub use read_event_log_a::ReadEventLogA;
mod read_event_log_w;
pub use read_event_log_w::ReadEventLogW;
mod reg_close_key_api;
pub use reg_close_key_api::RegCloseKey_api;
mod reg_open_key_ex_a_api;
pub use reg_open_key_ex_a_api::RegOpenKeyExA_api;
mod reg_open_key_ex_w_api;
pub use reg_open_key_ex_w_api::RegOpenKeyExW_api;
mod reg_query_value_ex_a_api;
pub use reg_query_value_ex_a_api::RegQueryValueExA_api;
mod reg_query_value_ex_w_api;
pub use reg_query_value_ex_w_api::RegQueryValueExW_api;
mod reg_connect_registry_a;
pub use reg_connect_registry_a::RegConnectRegistryA;
mod reg_connect_registry_ex_a;
pub use reg_connect_registry_ex_a::RegConnectRegistryExA;
mod reg_connect_registry_ex_w;
pub use reg_connect_registry_ex_w::RegConnectRegistryExW;
mod reg_connect_registry_w;
pub use reg_connect_registry_w::RegConnectRegistryW;
mod reg_copy_tree_a;
pub use reg_copy_tree_a::RegCopyTreeA;
mod reg_copy_tree_w;
pub use reg_copy_tree_w::RegCopyTreeW;
mod reg_create_key_a;
pub use reg_create_key_a::RegCreateKeyA;
mod reg_create_key_ex_a;
pub use reg_create_key_ex_a::RegCreateKeyExA;
mod reg_create_key_ex_w;
pub use reg_create_key_ex_w::RegCreateKeyExW;
mod reg_create_key_transacted_a;
pub use reg_create_key_transacted_a::RegCreateKeyTransactedA;
mod reg_create_key_transacted_w;
pub use reg_create_key_transacted_w::RegCreateKeyTransactedW;
mod reg_create_key_w;
pub use reg_create_key_w::RegCreateKeyW;
mod reg_delete_key_a;
pub use reg_delete_key_a::RegDeleteKeyA;
mod reg_delete_key_ex_a;
pub use reg_delete_key_ex_a::RegDeleteKeyExA;
mod reg_delete_key_ex_w;
pub use reg_delete_key_ex_w::RegDeleteKeyExW;
mod reg_delete_key_transacted_a;
pub use reg_delete_key_transacted_a::RegDeleteKeyTransactedA;
mod reg_delete_key_transacted_w;
pub use reg_delete_key_transacted_w::RegDeleteKeyTransactedW;
mod reg_delete_key_value_a;
pub use reg_delete_key_value_a::RegDeleteKeyValueA;
mod reg_delete_key_value_w;
pub use reg_delete_key_value_w::RegDeleteKeyValueW;
mod reg_delete_key_w;
pub use reg_delete_key_w::RegDeleteKeyW;
mod reg_delete_tree_a;
pub use reg_delete_tree_a::RegDeleteTreeA;
mod reg_delete_tree_w;
pub use reg_delete_tree_w::RegDeleteTreeW;
mod reg_delete_value_a;
pub use reg_delete_value_a::RegDeleteValueA;
mod reg_delete_value_w;
pub use reg_delete_value_w::RegDeleteValueW;
mod reg_disable_predefined_cache;
pub use reg_disable_predefined_cache::RegDisablePredefinedCache;
mod reg_disable_predefined_cache_ex;
pub use reg_disable_predefined_cache_ex::RegDisablePredefinedCacheEx;
mod reg_disable_reflection_key;
pub use reg_disable_reflection_key::RegDisableReflectionKey;
mod reg_enable_reflection_key;
pub use reg_enable_reflection_key::RegEnableReflectionKey;
mod reg_enum_key_a;
pub use reg_enum_key_a::RegEnumKeyA;
mod reg_enum_key_ex_a;
pub use reg_enum_key_ex_a::RegEnumKeyExA;
mod reg_enum_key_ex_w;
pub use reg_enum_key_ex_w::RegEnumKeyExW;
mod reg_enum_key_w;
pub use reg_enum_key_w::RegEnumKeyW;
mod reg_enum_value_a;
pub use reg_enum_value_a::RegEnumValueA;
mod reg_enum_value_w;
pub use reg_enum_value_w::RegEnumValueW;
mod reg_flush_key;
pub use reg_flush_key::RegFlushKey;
mod reg_get_key_security;
pub use reg_get_key_security::RegGetKeySecurity;
mod reg_get_value_a;
pub use reg_get_value_a::RegGetValueA;
mod reg_get_value_w;
pub use reg_get_value_w::RegGetValueW;
mod reg_load_app_key_a;
pub use reg_load_app_key_a::RegLoadAppKeyA;
mod reg_load_app_key_w;
pub use reg_load_app_key_w::RegLoadAppKeyW;
mod reg_load_key_a;
pub use reg_load_key_a::RegLoadKeyA;
mod reg_load_key_w;
pub use reg_load_key_w::RegLoadKeyW;
mod reg_load_mui_string_a;
pub use reg_load_mui_string_a::RegLoadMUIStringA;
mod reg_load_mui_string_w;
pub use reg_load_mui_string_w::RegLoadMUIStringW;
mod reg_notify_change_key_value;
pub use reg_notify_change_key_value::RegNotifyChangeKeyValue;
mod reg_open_current_user;
pub use reg_open_current_user::RegOpenCurrentUser;
mod reg_open_key_a;
pub use reg_open_key_a::RegOpenKeyA;
mod reg_open_key_transacted_a;
pub use reg_open_key_transacted_a::RegOpenKeyTransactedA;
mod reg_open_key_transacted_w;
pub use reg_open_key_transacted_w::RegOpenKeyTransactedW;
mod reg_open_key_w;
pub use reg_open_key_w::RegOpenKeyW;
mod reg_open_user_classes_root;
pub use reg_open_user_classes_root::RegOpenUserClassesRoot;
mod reg_override_predef_key;
pub use reg_override_predef_key::RegOverridePredefKey;
mod reg_query_info_key_a;
pub use reg_query_info_key_a::RegQueryInfoKeyA;
mod reg_query_info_key_w;
pub use reg_query_info_key_w::RegQueryInfoKeyW;
mod reg_query_multiple_values_a;
pub use reg_query_multiple_values_a::RegQueryMultipleValuesA;
mod reg_query_multiple_values_w;
pub use reg_query_multiple_values_w::RegQueryMultipleValuesW;
mod reg_query_reflection_key;
pub use reg_query_reflection_key::RegQueryReflectionKey;
mod reg_query_value_a;
pub use reg_query_value_a::RegQueryValueA;
mod reg_query_value_w;
pub use reg_query_value_w::RegQueryValueW;
mod reg_rename_key;
pub use reg_rename_key::RegRenameKey;
mod reg_replace_key_a;
pub use reg_replace_key_a::RegReplaceKeyA;
mod reg_replace_key_w;
pub use reg_replace_key_w::RegReplaceKeyW;
mod reg_restore_key_a;
pub use reg_restore_key_a::RegRestoreKeyA;
mod reg_restore_key_w;
pub use reg_restore_key_w::RegRestoreKeyW;
mod reg_save_key_a;
pub use reg_save_key_a::RegSaveKeyA;
mod reg_save_key_ex_a;
pub use reg_save_key_ex_a::RegSaveKeyExA;
mod reg_save_key_ex_w;
pub use reg_save_key_ex_w::RegSaveKeyExW;
mod reg_save_key_w;
pub use reg_save_key_w::RegSaveKeyW;
mod reg_set_key_security;
pub use reg_set_key_security::RegSetKeySecurity;
mod reg_set_key_value_a;
pub use reg_set_key_value_a::RegSetKeyValueA;
mod reg_set_key_value_w;
pub use reg_set_key_value_w::RegSetKeyValueW;
mod reg_set_value_a;
pub use reg_set_value_a::RegSetValueA;
mod reg_set_value_ex_a;
pub use reg_set_value_ex_a::RegSetValueExA;
mod reg_set_value_ex_w;
pub use reg_set_value_ex_w::RegSetValueExW;
mod reg_set_value_w;
pub use reg_set_value_w::RegSetValueW;
mod reg_un_load_key_a;
pub use reg_un_load_key_a::RegUnLoadKeyA;
mod reg_un_load_key_w;
pub use reg_un_load_key_w::RegUnLoadKeyW;
mod register_event_source_a;
pub use register_event_source_a::RegisterEventSourceA;
mod register_event_source_w;
pub use register_event_source_w::RegisterEventSourceW;
mod register_idle_task;
pub use register_idle_task::RegisterIdleTask;
mod register_service_ctrl_handler_a;
pub use register_service_ctrl_handler_a::RegisterServiceCtrlHandlerA;
mod register_service_ctrl_handler_ex_a;
pub use register_service_ctrl_handler_ex_a::RegisterServiceCtrlHandlerExA;
mod register_service_ctrl_handler_ex_w;
pub use register_service_ctrl_handler_ex_w::RegisterServiceCtrlHandlerExW;
mod register_service_ctrl_handler_w;
pub use register_service_ctrl_handler_w::RegisterServiceCtrlHandlerW;
mod register_trace_guids_a;
pub use register_trace_guids_a::RegisterTraceGuidsA;
mod register_trace_guids_w;
pub use register_trace_guids_w::RegisterTraceGuidsW;
mod register_wait_chain_com_callback;
pub use register_wait_chain_com_callback::RegisterWaitChainCOMCallback;
mod remote_reg_enum_key_wrapper;
pub use remote_reg_enum_key_wrapper::RemoteRegEnumKeyWrapper;
mod remote_reg_enum_value_wrapper;
pub use remote_reg_enum_value_wrapper::RemoteRegEnumValueWrapper;
mod remote_reg_query_info_key_wrapper;
pub use remote_reg_query_info_key_wrapper::RemoteRegQueryInfoKeyWrapper;
mod remote_reg_query_multiple_values2_wrapper;
pub use remote_reg_query_multiple_values2_wrapper::RemoteRegQueryMultipleValues2Wrapper;
mod remote_reg_query_multiple_values_wrapper;
pub use remote_reg_query_multiple_values_wrapper::RemoteRegQueryMultipleValuesWrapper;
mod remote_reg_query_value_wrapper;
pub use remote_reg_query_value_wrapper::RemoteRegQueryValueWrapper;
mod remove_trace_callback;
pub use remove_trace_callback::RemoveTraceCallback;
mod remove_users_from_encrypted_file;
pub use remove_users_from_encrypted_file::RemoveUsersFromEncryptedFile;
mod report_event_a;
pub use report_event_a::ReportEventA;
mod report_event_w;
pub use report_event_w::ReportEventW;
mod revert_to_self;
pub use revert_to_self::RevertToSelf;
mod safe_base_reg_get_key_security;
pub use safe_base_reg_get_key_security::SafeBaseRegGetKeySecurity;
mod safer_close_level;
pub use safer_close_level::SaferCloseLevel;
mod safer_compute_token_from_level;
pub use safer_compute_token_from_level::SaferComputeTokenFromLevel;
mod safer_create_level;
pub use safer_create_level::SaferCreateLevel;
mod safer_get_level_information;
pub use safer_get_level_information::SaferGetLevelInformation;
mod safer_get_policy_information;
pub use safer_get_policy_information::SaferGetPolicyInformation;
mod safer_identify_level;
pub use safer_identify_level::SaferIdentifyLevel;
mod safer_record_event_log_entry;
pub use safer_record_event_log_entry::SaferRecordEventLogEntry;
mod safer_set_level_information;
pub use safer_set_level_information::SaferSetLevelInformation;
mod safer_set_policy_information;
pub use safer_set_policy_information::SaferSetPolicyInformation;
mod saferi_change_registry_scope;
pub use saferi_change_registry_scope::SaferiChangeRegistryScope;
mod saferi_compare_token_levels;
pub use saferi_compare_token_levels::SaferiCompareTokenLevels;
mod saferi_is_dll_allowed;
pub use saferi_is_dll_allowed::SaferiIsDllAllowed;
mod saferi_is_executable_file_type;
pub use saferi_is_executable_file_type::SaferiIsExecutableFileType;
mod saferi_populate_defaults_in_registry;
pub use saferi_populate_defaults_in_registry::SaferiPopulateDefaultsInRegistry;
mod saferi_record_event_log_entry;
pub use saferi_record_event_log_entry::SaferiRecordEventLogEntry;
mod saferi_search_matching_hash_rules;
pub use saferi_search_matching_hash_rules::SaferiSearchMatchingHashRules;
mod set_acl_information;
pub use set_acl_information::SetAclInformation;
mod set_encrypted_file_metadata;
pub use set_encrypted_file_metadata::SetEncryptedFileMetadata;
mod set_entries_in_access_list_a;
pub use set_entries_in_access_list_a::SetEntriesInAccessListA;
mod set_entries_in_access_list_w;
pub use set_entries_in_access_list_w::SetEntriesInAccessListW;
mod set_entries_in_acl_a;
pub use set_entries_in_acl_a::SetEntriesInAclA;
mod set_entries_in_acl_w;
pub use set_entries_in_acl_w::SetEntriesInAclW;
mod set_entries_in_audit_list_a;
pub use set_entries_in_audit_list_a::SetEntriesInAuditListA;
mod set_entries_in_audit_list_w;
pub use set_entries_in_audit_list_w::SetEntriesInAuditListW;
mod set_file_security_a;
pub use set_file_security_a::SetFileSecurityA;
mod set_file_security_w;
pub use set_file_security_w::SetFileSecurityW;
mod set_information_code_authz_level_w;
pub use set_information_code_authz_level_w::SetInformationCodeAuthzLevelW;
mod set_information_code_authz_policy_w;
pub use set_information_code_authz_policy_w::SetInformationCodeAuthzPolicyW;
mod set_kernel_object_security;
pub use set_kernel_object_security::SetKernelObjectSecurity;
mod set_named_security_info_a;
pub use set_named_security_info_a::SetNamedSecurityInfoA;
mod set_named_security_info_ex_a;
pub use set_named_security_info_ex_a::SetNamedSecurityInfoExA;
mod set_named_security_info_ex_w;
pub use set_named_security_info_ex_w::SetNamedSecurityInfoExW;
mod set_named_security_info_w;
pub use set_named_security_info_w::SetNamedSecurityInfoW;
mod set_private_object_security;
pub use set_private_object_security::SetPrivateObjectSecurity;
mod set_private_object_security_ex;
pub use set_private_object_security_ex::SetPrivateObjectSecurityEx;
mod set_security_access_mask;
pub use set_security_access_mask::SetSecurityAccessMask;
mod set_security_descriptor_control;
pub use set_security_descriptor_control::SetSecurityDescriptorControl;
mod set_security_descriptor_dacl;
pub use set_security_descriptor_dacl::SetSecurityDescriptorDacl;
mod set_security_descriptor_group;
pub use set_security_descriptor_group::SetSecurityDescriptorGroup;
mod set_security_descriptor_owner;
pub use set_security_descriptor_owner::SetSecurityDescriptorOwner;
mod set_security_descriptor_rm_control;
pub use set_security_descriptor_rm_control::SetSecurityDescriptorRMControl;
mod set_security_descriptor_sacl;
pub use set_security_descriptor_sacl::SetSecurityDescriptorSacl;
mod set_security_info;
pub use set_security_info::SetSecurityInfo;
mod set_security_info_ex_a;
pub use set_security_info_ex_a::SetSecurityInfoExA;
mod set_security_info_ex_w;
pub use set_security_info_ex_w::SetSecurityInfoExW;
mod set_service_bits;
pub use set_service_bits::SetServiceBits;
mod set_service_object_security;
pub use set_service_object_security::SetServiceObjectSecurity;
mod set_service_status;
pub use set_service_status::SetServiceStatus;
mod set_thread_token;
pub use set_thread_token::SetThreadToken;
mod set_token_information;
pub use set_token_information::SetTokenInformation;
mod set_trace_callback;
pub use set_trace_callback::SetTraceCallback;
mod set_user_file_encryption_key;
pub use set_user_file_encryption_key::SetUserFileEncryptionKey;
mod set_user_file_encryption_key_ex;
pub use set_user_file_encryption_key_ex::SetUserFileEncryptionKeyEx;
mod start_service_a;
pub use start_service_a::StartServiceA;
mod start_service_w;
pub use start_service_w::StartServiceW;
mod start_trace_a;
pub use start_trace_a::StartTraceA;
mod start_trace_w;
pub use start_trace_w::StartTraceW;
mod stop_trace_a;
pub use stop_trace_a::StopTraceA;
mod stop_trace_w;
pub use stop_trace_w::StopTraceW;
mod system_function001;
pub use system_function001::SystemFunction001;
mod system_function002;
pub use system_function002::SystemFunction002;
mod system_function003;
pub use system_function003::SystemFunction003;
mod system_function004;
pub use system_function004::SystemFunction004;
mod system_function005;
pub use system_function005::SystemFunction005;
mod system_function006;
pub use system_function006::SystemFunction006;
mod system_function007;
pub use system_function007::SystemFunction007;
mod system_function008;
pub use system_function008::SystemFunction008;
mod system_function009;
pub use system_function009::SystemFunction009;
mod system_function010;
pub use system_function010::SystemFunction010;
mod system_function011;
pub use system_function011::SystemFunction011;
mod system_function012;
pub use system_function012::SystemFunction012;
mod system_function013;
pub use system_function013::SystemFunction013;
mod system_function014;
pub use system_function014::SystemFunction014;
mod system_function015;
pub use system_function015::SystemFunction015;
mod system_function016;
pub use system_function016::SystemFunction016;
mod system_function017;
pub use system_function017::SystemFunction017;
mod system_function018;
pub use system_function018::SystemFunction018;
mod system_function019;
pub use system_function019::SystemFunction019;
mod system_function020;
pub use system_function020::SystemFunction020;
mod system_function021;
pub use system_function021::SystemFunction021;
mod system_function022;
pub use system_function022::SystemFunction022;
mod system_function023;
pub use system_function023::SystemFunction023;
mod system_function024;
pub use system_function024::SystemFunction024;
mod system_function025;
pub use system_function025::SystemFunction025;
mod system_function026;
pub use system_function026::SystemFunction026;
mod system_function027;
pub use system_function027::SystemFunction027;
mod system_function028;
pub use system_function028::SystemFunction028;
mod system_function029;
pub use system_function029::SystemFunction029;
mod system_function030;
pub use system_function030::SystemFunction030;
mod system_function031;
pub use system_function031::SystemFunction031;
mod system_function032;
pub use system_function032::SystemFunction032;
mod system_function033;
pub use system_function033::SystemFunction033;
mod system_function034;
pub use system_function034::SystemFunction034;
mod system_function035;
pub use system_function035::SystemFunction035;
mod system_function036;
pub use system_function036::SystemFunction036;
mod system_function040;
pub use system_function040::SystemFunction040;
mod system_function041;
pub use system_function041::SystemFunction041;
mod trace_event;
pub use trace_event::TraceEvent;
mod trace_event_instance;
pub use trace_event_instance::TraceEventInstance;
mod trace_message;
pub use trace_message::TraceMessage;
mod trace_message_va;
pub use trace_message_va::TraceMessageVa;
mod trace_query_information;
pub use trace_query_information::TraceQueryInformation;
mod trace_set_information;
pub use trace_set_information::TraceSetInformation;
mod tree_reset_named_security_info_a;
pub use tree_reset_named_security_info_a::TreeResetNamedSecurityInfoA;
mod tree_reset_named_security_info_w;
pub use tree_reset_named_security_info_w::TreeResetNamedSecurityInfoW;
mod tree_set_named_security_info_a;
pub use tree_set_named_security_info_a::TreeSetNamedSecurityInfoA;
mod tree_set_named_security_info_w;
pub use tree_set_named_security_info_w::TreeSetNamedSecurityInfoW;
mod trustee_access_to_object_a;
pub use trustee_access_to_object_a::TrusteeAccessToObjectA;
mod trustee_access_to_object_w;
pub use trustee_access_to_object_w::TrusteeAccessToObjectW;
mod uninstall_application;
pub use uninstall_application::UninstallApplication;
mod unlock_service_database;
pub use unlock_service_database::UnlockServiceDatabase;
mod unregister_idle_task;
pub use unregister_idle_task::UnregisterIdleTask;
mod unregister_trace_guids;
pub use unregister_trace_guids::UnregisterTraceGuids;
mod update_trace_a;
pub use update_trace_a::UpdateTraceA;
mod update_trace_w;
pub use update_trace_w::UpdateTraceW;
mod use_pin_for_encrypted_files_a;
pub use use_pin_for_encrypted_files_a::UsePinForEncryptedFilesA;
mod use_pin_for_encrypted_files_w;
pub use use_pin_for_encrypted_files_w::UsePinForEncryptedFilesW;
mod wait_service_state;
pub use wait_service_state::WaitServiceState;
mod wmi_close_block;
pub use wmi_close_block::WmiCloseBlock;
mod wmi_dev_inst_to_instance_name_a;
pub use wmi_dev_inst_to_instance_name_a::WmiDevInstToInstanceNameA;
mod wmi_dev_inst_to_instance_name_w;
pub use wmi_dev_inst_to_instance_name_w::WmiDevInstToInstanceNameW;
mod wmi_enumerate_guids;
pub use wmi_enumerate_guids::WmiEnumerateGuids;
mod wmi_execute_method_a;
pub use wmi_execute_method_a::WmiExecuteMethodA;
mod wmi_execute_method_w;
pub use wmi_execute_method_w::WmiExecuteMethodW;
mod wmi_file_handle_to_instance_name_a;
pub use wmi_file_handle_to_instance_name_a::WmiFileHandleToInstanceNameA;
mod wmi_file_handle_to_instance_name_w;
pub use wmi_file_handle_to_instance_name_w::WmiFileHandleToInstanceNameW;
mod wmi_free_buffer;
pub use wmi_free_buffer::WmiFreeBuffer;
mod wmi_mof_enumerate_resources_a;
pub use wmi_mof_enumerate_resources_a::WmiMofEnumerateResourcesA;
mod wmi_mof_enumerate_resources_w;
pub use wmi_mof_enumerate_resources_w::WmiMofEnumerateResourcesW;
mod wmi_notification_registration_a;
pub use wmi_notification_registration_a::WmiNotificationRegistrationA;
mod wmi_notification_registration_w;
pub use wmi_notification_registration_w::WmiNotificationRegistrationW;
mod wmi_open_block;
pub use wmi_open_block::WmiOpenBlock;
mod wmi_query_all_data_a;
pub use wmi_query_all_data_a::WmiQueryAllDataA;
mod wmi_query_all_data_multiple_a;
pub use wmi_query_all_data_multiple_a::WmiQueryAllDataMultipleA;
mod wmi_query_all_data_multiple_w;
pub use wmi_query_all_data_multiple_w::WmiQueryAllDataMultipleW;
mod wmi_query_all_data_w;
pub use wmi_query_all_data_w::WmiQueryAllDataW;
mod wmi_query_guid_information;
pub use wmi_query_guid_information::WmiQueryGuidInformation;
mod wmi_query_single_instance_a;
pub use wmi_query_single_instance_a::WmiQuerySingleInstanceA;
mod wmi_query_single_instance_multiple_a;
pub use wmi_query_single_instance_multiple_a::WmiQuerySingleInstanceMultipleA;
mod wmi_query_single_instance_multiple_w;
pub use wmi_query_single_instance_multiple_w::WmiQuerySingleInstanceMultipleW;
mod wmi_query_single_instance_w;
pub use wmi_query_single_instance_w::WmiQuerySingleInstanceW;
mod wmi_receive_notifications_a;
pub use wmi_receive_notifications_a::WmiReceiveNotificationsA;
mod wmi_receive_notifications_w;
pub use wmi_receive_notifications_w::WmiReceiveNotificationsW;
mod wmi_set_single_instance_a;
pub use wmi_set_single_instance_a::WmiSetSingleInstanceA;
mod wmi_set_single_instance_w;
pub use wmi_set_single_instance_w::WmiSetSingleInstanceW;
mod wmi_set_single_item_a;
pub use wmi_set_single_item_a::WmiSetSingleItemA;
mod wmi_set_single_item_w;
pub use wmi_set_single_item_w::WmiSetSingleItemW;
mod write_encrypted_file_raw;
pub use write_encrypted_file_raw::WriteEncryptedFileRaw;
mod adjust_token_privileges;
pub use adjust_token_privileges::AdjustTokenPrivileges;
mod crypt_acquire_context_a;
pub use crypt_acquire_context_a::CryptAcquireContextA;
mod crypt_acquire_context_w;
pub use crypt_acquire_context_w::CryptAcquireContextW;
mod crypt_create_hash;
pub use crypt_create_hash::CryptCreateHash;
mod crypt_decrypt;
pub use crypt_decrypt::CryptDecrypt;
mod crypt_derive_key;
use crypt_derive_key::CryptDeriveKey;
mod crypt_encrypt;
pub use crypt_encrypt::CryptEncrypt;
mod crypt_gen_key;
pub use crypt_gen_key::CryptGenKey;
mod crypt_gen_random;
pub use crypt_gen_random::CryptGenRandom;
mod crypt_get_hash_param;
use crypt_get_hash_param::CryptGetHashParam;
mod crypt_get_key_param;
use crypt_get_key_param::CryptGetKeyParam;
mod crypt_hash_data;
pub use crypt_hash_data::CryptHashData;
mod crypt_import_key;
use crypt_import_key::CryptImportKey;
mod crypt_release_context;
pub use crypt_release_context::CryptReleaseContext;
mod crypt_sign_hash_a;
use crypt_sign_hash_a::CryptSignHashA;
mod crypt_sign_hash_w;
use crypt_sign_hash_w::CryptSignHashW;
mod get_user_name_a;
use get_user_name_a::GetUserNameA;
mod get_user_name_w;
use get_user_name_w::GetUserNameW;
mod lookup_privilege_value_w;
use lookup_privilege_value_w::LookupPrivilegeValueW;
mod start_service_ctrl_dispatcher_a;
use start_service_ctrl_dispatcher_a::StartServiceCtrlDispatcherA;
mod start_service_ctrl_dispatcher_w;
use start_service_ctrl_dispatcher_w::StartServiceCtrlDispatcherW;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if winapi64::registry::gateway(api, emu) {
        return String::new();
    }
    match api {
        "StartServiceCtrlDispatcherA" => StartServiceCtrlDispatcherA(emu),
        "StartServiceCtrlDispatcherW" => StartServiceCtrlDispatcherW(emu),
        "GetUserNameA" => GetUserNameA(emu),
        "GetUserNameW" => GetUserNameW(emu),
        "CryptAcquireContextA" => CryptAcquireContextA(emu),
        "CryptAcquireContextW" => CryptAcquireContextW(emu),
        "CryptReleaseContext" => CryptReleaseContext(emu),
        "CryptCreateHash" => CryptCreateHash(emu),
        "CryptHashData" => CryptHashData(emu),
        "CryptGenKey" => CryptGenKey(emu),
        "CryptEncrypt" => CryptEncrypt(emu),
        "CryptDecrypt" => CryptDecrypt(emu),
        "CryptGenRandom" => CryptGenRandom(emu),
        "AdjustTokenPrivileges" => AdjustTokenPrivileges(emu),
        "LookupPrivilegeValueW" => LookupPrivilegeValueW(emu),
        "CryptGetHashParam" => CryptGetHashParam(emu),
        "CryptGetKeyParam" => CryptGetKeyParam(emu),
        "CryptImportKey" => CryptImportKey(emu),
        "CryptSignHashA" => CryptSignHashA(emu),
        "CryptSignHashW" => CryptSignHashW(emu),
        "CryptDeriveKey" => CryptDeriveKey(emu),

        "A_SHAFinal" => A_SHAFinal(emu),

        "A_SHAInit" => A_SHAInit(emu),

        "A_SHAUpdate" => A_SHAUpdate(emu),

        "AbortSystemShutdownA" => AbortSystemShutdownA(emu),

        "AbortSystemShutdownW" => AbortSystemShutdownW(emu),

        "AccessCheck" => AccessCheck(emu),

        "AccessCheckAndAuditAlarmA" => AccessCheckAndAuditAlarmA(emu),

        "AccessCheckAndAuditAlarmW" => AccessCheckAndAuditAlarmW(emu),

        "AccessCheckByType" => AccessCheckByType(emu),

        "AccessCheckByTypeAndAuditAlarmA" => AccessCheckByTypeAndAuditAlarmA(emu),

        "AccessCheckByTypeAndAuditAlarmW" => AccessCheckByTypeAndAuditAlarmW(emu),

        "AccessCheckByTypeResultList" => AccessCheckByTypeResultList(emu),

        "AccessCheckByTypeResultListAndAuditAlarmA" => {
            AccessCheckByTypeResultListAndAuditAlarmA(emu)
        }

        "AccessCheckByTypeResultListAndAuditAlarmByHandleA" => {
            AccessCheckByTypeResultListAndAuditAlarmByHandleA(emu)
        }

        "AccessCheckByTypeResultListAndAuditAlarmByHandleW" => {
            AccessCheckByTypeResultListAndAuditAlarmByHandleW(emu)
        }

        "AccessCheckByTypeResultListAndAuditAlarmW" => {
            AccessCheckByTypeResultListAndAuditAlarmW(emu)
        }

        "AddAccessAllowedAce" => AddAccessAllowedAce(emu),

        "AddAccessAllowedAceEx" => AddAccessAllowedAceEx(emu),

        "AddAccessAllowedObjectAce" => AddAccessAllowedObjectAce(emu),

        "AddAccessDeniedAce" => AddAccessDeniedAce(emu),

        "AddAccessDeniedAceEx" => AddAccessDeniedAceEx(emu),

        "AddAccessDeniedObjectAce" => AddAccessDeniedObjectAce(emu),

        "AddAce" => AddAce(emu),

        "AddAuditAccessAce" => AddAuditAccessAce(emu),

        "AddAuditAccessAceEx" => AddAuditAccessAceEx(emu),

        "AddAuditAccessObjectAce" => AddAuditAccessObjectAce(emu),

        "AddConditionalAce" => AddConditionalAce(emu),

        "AddMandatoryAce" => AddMandatoryAce(emu),

        "AddUsersToEncryptedFile" => AddUsersToEncryptedFile(emu),

        "AddUsersToEncryptedFileEx" => AddUsersToEncryptedFileEx(emu),

        "AdjustTokenGroups" => AdjustTokenGroups(emu),

        "AllocateAndInitializeSid" => AllocateAndInitializeSid(emu),

        "AllocateLocallyUniqueId" => AllocateLocallyUniqueId(emu),

        "AreAllAccessesGranted" => AreAllAccessesGranted(emu),

        "AreAnyAccessesGranted" => AreAnyAccessesGranted(emu),

        "AuditComputeEffectivePolicyBySid" => AuditComputeEffectivePolicyBySid(emu),

        "AuditComputeEffectivePolicyByToken" => AuditComputeEffectivePolicyByToken(emu),

        "AuditEnumerateCategories" => AuditEnumerateCategories(emu),

        "AuditEnumeratePerUserPolicy" => AuditEnumeratePerUserPolicy(emu),

        "AuditEnumerateSubCategories" => AuditEnumerateSubCategories(emu),

        "AuditFree" => AuditFree(emu),

        "AuditLookupCategoryGuidFromCategoryId" => AuditLookupCategoryGuidFromCategoryId(emu),

        "AuditLookupCategoryIdFromCategoryGuid" => AuditLookupCategoryIdFromCategoryGuid(emu),

        "AuditLookupCategoryNameA" => AuditLookupCategoryNameA(emu),

        "AuditLookupCategoryNameW" => AuditLookupCategoryNameW(emu),

        "AuditLookupSubCategoryNameA" => AuditLookupSubCategoryNameA(emu),

        "AuditLookupSubCategoryNameW" => AuditLookupSubCategoryNameW(emu),

        "AuditQueryGlobalSaclA" => AuditQueryGlobalSaclA(emu),

        "AuditQueryGlobalSaclW" => AuditQueryGlobalSaclW(emu),

        "AuditQueryPerUserPolicy" => AuditQueryPerUserPolicy(emu),

        "AuditQuerySecurity" => AuditQuerySecurity(emu),

        "AuditQuerySystemPolicy" => AuditQuerySystemPolicy(emu),

        "AuditSetGlobalSaclA" => AuditSetGlobalSaclA(emu),

        "AuditSetGlobalSaclW" => AuditSetGlobalSaclW(emu),

        "AuditSetPerUserPolicy" => AuditSetPerUserPolicy(emu),

        "AuditSetSecurity" => AuditSetSecurity(emu),

        "AuditSetSystemPolicy" => AuditSetSystemPolicy(emu),

        "BackupEventLogA" => BackupEventLogA(emu),

        "BackupEventLogW" => BackupEventLogW(emu),

        "BaseRegCloseKey" => BaseRegCloseKey(emu),

        "BaseRegCreateKey" => BaseRegCreateKey(emu),

        "BaseRegDeleteKeyEx" => BaseRegDeleteKeyEx(emu),

        "BaseRegDeleteValue" => BaseRegDeleteValue(emu),

        "BaseRegFlushKey" => BaseRegFlushKey(emu),

        "BaseRegGetVersion" => BaseRegGetVersion(emu),

        "BaseRegLoadKey" => BaseRegLoadKey(emu),

        "BaseRegOpenKey" => BaseRegOpenKey(emu),

        "BaseRegRestoreKey" => BaseRegRestoreKey(emu),

        "BaseRegSaveKeyEx" => BaseRegSaveKeyEx(emu),

        "BaseRegSetKeySecurity" => BaseRegSetKeySecurity(emu),

        "BaseRegSetValue" => BaseRegSetValue(emu),

        "BaseRegUnLoadKey" => BaseRegUnLoadKey(emu),

        "BuildExplicitAccessWithNameA" => BuildExplicitAccessWithNameA(emu),

        "BuildExplicitAccessWithNameW" => BuildExplicitAccessWithNameW(emu),

        "BuildImpersonateExplicitAccessWithNameA" => BuildImpersonateExplicitAccessWithNameA(emu),

        "BuildImpersonateExplicitAccessWithNameW" => BuildImpersonateExplicitAccessWithNameW(emu),

        "BuildImpersonateTrusteeA" => BuildImpersonateTrusteeA(emu),

        "BuildImpersonateTrusteeW" => BuildImpersonateTrusteeW(emu),

        "BuildSecurityDescriptorA" => BuildSecurityDescriptorA(emu),

        "BuildSecurityDescriptorW" => BuildSecurityDescriptorW(emu),

        "BuildTrusteeWithNameA" => BuildTrusteeWithNameA(emu),

        "BuildTrusteeWithNameW" => BuildTrusteeWithNameW(emu),

        "BuildTrusteeWithObjectsAndNameA" => BuildTrusteeWithObjectsAndNameA(emu),

        "BuildTrusteeWithObjectsAndNameW" => BuildTrusteeWithObjectsAndNameW(emu),

        "BuildTrusteeWithObjectsAndSidA" => BuildTrusteeWithObjectsAndSidA(emu),

        "BuildTrusteeWithObjectsAndSidW" => BuildTrusteeWithObjectsAndSidW(emu),

        "BuildTrusteeWithSidA" => BuildTrusteeWithSidA(emu),

        "BuildTrusteeWithSidW" => BuildTrusteeWithSidW(emu),

        "CancelOverlappedAccess" => CancelOverlappedAccess(emu),

        "ChangeServiceConfig2A" => ChangeServiceConfig2A(emu),

        "ChangeServiceConfig2W" => ChangeServiceConfig2W(emu),

        "ChangeServiceConfigA" => ChangeServiceConfigA(emu),

        "ChangeServiceConfigW" => ChangeServiceConfigW(emu),

        "CheckForHiberboot" => CheckForHiberboot(emu),

        "CheckTokenMembership" => CheckTokenMembership(emu),

        "ClearEventLogA" => ClearEventLogA(emu),

        "ClearEventLogW" => ClearEventLogW(emu),

        "CloseCodeAuthzLevel" => CloseCodeAuthzLevel(emu),

        "CloseEncryptedFileRaw" => CloseEncryptedFileRaw(emu),

        "CloseEventLog" => CloseEventLog(emu),

        "CloseServiceHandle" => CloseServiceHandle(emu),

        "CloseThreadWaitChainSession" => CloseThreadWaitChainSession(emu),

        "CloseTrace" => CloseTrace(emu),

        "CommandLineFromMsiDescriptor" => CommandLineFromMsiDescriptor(emu),

        "ComputeAccessTokenFromCodeAuthzLevel" => ComputeAccessTokenFromCodeAuthzLevel(emu),

        "ControlService" => ControlService(emu),

        "ControlServiceExA" => ControlServiceExA(emu),

        "ControlServiceExW" => ControlServiceExW(emu),

        "ControlTraceA" => ControlTraceA(emu),

        "ControlTraceW" => ControlTraceW(emu),

        "ConvertAccessToSecurityDescriptorA" => ConvertAccessToSecurityDescriptorA(emu),

        "ConvertAccessToSecurityDescriptorW" => ConvertAccessToSecurityDescriptorW(emu),

        "ConvertSDToStringSDDomainW" => ConvertSDToStringSDDomainW(emu),

        "ConvertSDToStringSDRootDomainA" => ConvertSDToStringSDRootDomainA(emu),

        "ConvertSDToStringSDRootDomainW" => ConvertSDToStringSDRootDomainW(emu),

        "ConvertSecurityDescriptorToAccessA" => ConvertSecurityDescriptorToAccessA(emu),

        "ConvertSecurityDescriptorToAccessNamedA" => ConvertSecurityDescriptorToAccessNamedA(emu),

        "ConvertSecurityDescriptorToAccessNamedW" => ConvertSecurityDescriptorToAccessNamedW(emu),

        "ConvertSecurityDescriptorToAccessW" => ConvertSecurityDescriptorToAccessW(emu),

        "ConvertSecurityDescriptorToStringSecurityDescriptorA" => {
            ConvertSecurityDescriptorToStringSecurityDescriptorA(emu)
        }

        "ConvertSecurityDescriptorToStringSecurityDescriptorW" => {
            ConvertSecurityDescriptorToStringSecurityDescriptorW(emu)
        }

        "ConvertSidToStringSidA" => ConvertSidToStringSidA(emu),

        "ConvertSidToStringSidW" => ConvertSidToStringSidW(emu),

        "ConvertStringSDToSDDomainA" => ConvertStringSDToSDDomainA(emu),

        "ConvertStringSDToSDDomainW" => ConvertStringSDToSDDomainW(emu),

        "ConvertStringSDToSDRootDomainA" => ConvertStringSDToSDRootDomainA(emu),

        "ConvertStringSDToSDRootDomainW" => ConvertStringSDToSDRootDomainW(emu),

        "ConvertStringSecurityDescriptorToSecurityDescriptorA" => {
            ConvertStringSecurityDescriptorToSecurityDescriptorA(emu)
        }

        "ConvertStringSecurityDescriptorToSecurityDescriptorW" => {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(emu)
        }

        "ConvertStringSidToSidA" => ConvertStringSidToSidA(emu),

        "ConvertStringSidToSidW" => ConvertStringSidToSidW(emu),

        "ConvertToAutoInheritPrivateObjectSecurity" => {
            ConvertToAutoInheritPrivateObjectSecurity(emu)
        }

        "CopySid" => CopySid(emu),

        "CreateCodeAuthzLevel" => CreateCodeAuthzLevel(emu),

        "CreatePrivateObjectSecurity" => CreatePrivateObjectSecurity(emu),

        "CreatePrivateObjectSecurityEx" => CreatePrivateObjectSecurityEx(emu),

        "CreatePrivateObjectSecurityWithMultipleInheritance" => {
            CreatePrivateObjectSecurityWithMultipleInheritance(emu)
        }

        "CreateProcessAsUserA" => CreateProcessAsUserA(emu),

        "CreateProcessAsUserW" => CreateProcessAsUserW(emu),

        "CreateProcessWithLogonW" => CreateProcessWithLogonW(emu),

        "CreateProcessWithTokenW" => CreateProcessWithTokenW(emu),

        "CreateRestrictedToken" => CreateRestrictedToken(emu),

        "CreateServiceA" => CreateServiceA(emu),

        "CreateServiceEx" => CreateServiceEx(emu),

        "CreateServiceW" => CreateServiceW(emu),

        "CreateTraceInstanceId" => CreateTraceInstanceId(emu),

        "CreateWellKnownSid" => CreateWellKnownSid(emu),

        "CredBackupCredentials" => CredBackupCredentials(emu),

        "CredDeleteA" => CredDeleteA(emu),

        "CredDeleteW" => CredDeleteW(emu),

        "CredEncryptAndMarshalBinaryBlob" => CredEncryptAndMarshalBinaryBlob(emu),

        "CredEnumerateA" => CredEnumerateA(emu),

        "CredEnumerateW" => CredEnumerateW(emu),

        "CredFindBestCredentialA" => CredFindBestCredentialA(emu),

        "CredFindBestCredentialW" => CredFindBestCredentialW(emu),

        "CredFree" => CredFree(emu),

        "CredGetSessionTypes" => CredGetSessionTypes(emu),

        "CredGetTargetInfoA" => CredGetTargetInfoA(emu),

        "CredGetTargetInfoW" => CredGetTargetInfoW(emu),

        "CredIsMarshaledCredentialA" => CredIsMarshaledCredentialA(emu),

        "CredIsMarshaledCredentialW" => CredIsMarshaledCredentialW(emu),

        "CredIsProtectedA" => CredIsProtectedA(emu),

        "CredIsProtectedW" => CredIsProtectedW(emu),

        "CredMarshalCredentialA" => CredMarshalCredentialA(emu),

        "CredMarshalCredentialW" => CredMarshalCredentialW(emu),

        "CredProfileLoaded" => CredProfileLoaded(emu),

        "CredProfileLoadedEx" => CredProfileLoadedEx(emu),

        "CredProfileUnloaded" => CredProfileUnloaded(emu),

        "CredProtectA" => CredProtectA(emu),

        "CredProtectW" => CredProtectW(emu),

        "CredReadA" => CredReadA(emu),

        "CredReadByTokenHandle" => CredReadByTokenHandle(emu),

        "CredReadDomainCredentialsA" => CredReadDomainCredentialsA(emu),

        "CredReadDomainCredentialsW" => CredReadDomainCredentialsW(emu),

        "CredReadW" => CredReadW(emu),

        "CredRenameA" => CredRenameA(emu),

        "CredRenameW" => CredRenameW(emu),

        "CredRestoreCredentials" => CredRestoreCredentials(emu),

        "CredUnmarshalCredentialA" => CredUnmarshalCredentialA(emu),

        "CredUnmarshalCredentialW" => CredUnmarshalCredentialW(emu),

        "CredUnprotectA" => CredUnprotectA(emu),

        "CredUnprotectW" => CredUnprotectW(emu),

        "CredWriteA" => CredWriteA(emu),

        "CredWriteDomainCredentialsA" => CredWriteDomainCredentialsA(emu),

        "CredWriteDomainCredentialsW" => CredWriteDomainCredentialsW(emu),

        "CredWriteW" => CredWriteW(emu),

        "CredpConvertCredential" => CredpConvertCredential(emu),

        "CredpConvertOneCredentialSize" => CredpConvertOneCredentialSize(emu),

        "CredpConvertTargetInfo" => CredpConvertTargetInfo(emu),

        "CredpDecodeCredential" => CredpDecodeCredential(emu),

        "CredpEncodeCredential" => CredpEncodeCredential(emu),

        "CredpEncodeSecret" => CredpEncodeSecret(emu),

        "CryptContextAddRef" => CryptContextAddRef(emu),

        "CryptDestroyHash" => CryptDestroyHash(emu),

        "CryptDestroyKey" => CryptDestroyKey(emu),

        "CryptDuplicateHash" => CryptDuplicateHash(emu),

        "CryptDuplicateKey" => CryptDuplicateKey(emu),

        "CryptEnumProviderTypesA" => CryptEnumProviderTypesA(emu),

        "CryptEnumProviderTypesW" => CryptEnumProviderTypesW(emu),

        "CryptEnumProvidersA" => CryptEnumProvidersA(emu),

        "CryptEnumProvidersW" => CryptEnumProvidersW(emu),

        "CryptExportKey" => CryptExportKey(emu),

        "CryptGetDefaultProviderA" => CryptGetDefaultProviderA(emu),

        "CryptGetDefaultProviderW" => CryptGetDefaultProviderW(emu),

        "CryptGetProvParam" => CryptGetProvParam(emu),

        "CryptGetUserKey" => CryptGetUserKey(emu),

        "CryptHashSessionKey" => CryptHashSessionKey(emu),

        "CryptSetHashParam" => CryptSetHashParam(emu),

        "CryptSetKeyParam" => CryptSetKeyParam(emu),

        "CryptSetProvParam" => CryptSetProvParam(emu),

        "CryptSetProviderA" => CryptSetProviderA(emu),

        "CryptSetProviderExA" => CryptSetProviderExA(emu),

        "CryptSetProviderExW" => CryptSetProviderExW(emu),

        "CryptSetProviderW" => CryptSetProviderW(emu),

        "CryptVerifySignatureA" => CryptVerifySignatureA(emu),

        "CryptVerifySignatureW" => CryptVerifySignatureW(emu),

        "CveEventWrite" => CveEventWrite(emu),

        "DecryptFileA" => DecryptFileA(emu),

        "DecryptFileW" => DecryptFileW(emu),

        "DeleteAce" => DeleteAce(emu),

        "DeleteService" => DeleteService(emu),

        "DeregisterEventSource" => DeregisterEventSource(emu),

        "DestroyPrivateObjectSecurity" => DestroyPrivateObjectSecurity(emu),

        "DuplicateEncryptionInfoFile" => DuplicateEncryptionInfoFile(emu),

        "DuplicateToken" => DuplicateToken(emu),

        "DuplicateTokenEx" => DuplicateTokenEx(emu),

        "EnableTrace" => EnableTrace(emu),

        "EnableTraceEx" => EnableTraceEx(emu),

        "EnableTraceEx2" => EnableTraceEx2(emu),

        "EncryptFileA" => EncryptFileA(emu),

        "EncryptFileW" => EncryptFileW(emu),

        "EncryptedFileKeyInfo" => EncryptedFileKeyInfo(emu),

        "EncryptionDisable" => EncryptionDisable(emu),

        "EnumDependentServicesA" => EnumDependentServicesA(emu),

        "EnumDependentServicesW" => EnumDependentServicesW(emu),

        "EnumDynamicTimeZoneInformation" => EnumDynamicTimeZoneInformation(emu),

        "EnumServiceGroupW" => EnumServiceGroupW(emu),

        "EnumServicesStatusA" => EnumServicesStatusA(emu),

        "EnumServicesStatusExA" => EnumServicesStatusExA(emu),

        "EnumServicesStatusExW" => EnumServicesStatusExW(emu),

        "EnumServicesStatusW" => EnumServicesStatusW(emu),

        "EnumerateTraceGuids" => EnumerateTraceGuids(emu),

        "EnumerateTraceGuidsEx" => EnumerateTraceGuidsEx(emu),

        "EqualDomainSid" => EqualDomainSid(emu),

        "EqualPrefixSid" => EqualPrefixSid(emu),

        "EqualSid" => EqualSid(emu),

        "EventAccessControl" => EventAccessControl(emu),

        "EventAccessQuery" => EventAccessQuery(emu),

        "EventAccessRemove" => EventAccessRemove(emu),

        "EventActivityIdControl" => EventActivityIdControl(emu),

        "EventEnabled" => EventEnabled(emu),

        "EventProviderEnabled" => EventProviderEnabled(emu),

        "EventRegister" => EventRegister(emu),

        "EventSetInformation" => EventSetInformation(emu),

        "EventUnregister" => EventUnregister(emu),

        "EventWrite" => EventWrite(emu),

        "EventWriteEndScenario" => EventWriteEndScenario(emu),

        "EventWriteEx" => EventWriteEx(emu),

        "EventWriteStartScenario" => EventWriteStartScenario(emu),

        "EventWriteString" => EventWriteString(emu),

        "EventWriteTransfer" => EventWriteTransfer(emu),

        "FileEncryptionStatusA" => FileEncryptionStatusA(emu),

        "FileEncryptionStatusW" => FileEncryptionStatusW(emu),

        "FindFirstFreeAce" => FindFirstFreeAce(emu),

        "FlushEfsCache" => FlushEfsCache(emu),

        "FlushTraceA" => FlushTraceA(emu),

        "FlushTraceW" => FlushTraceW(emu),

        "FreeEncryptedFileKeyInfo" => FreeEncryptedFileKeyInfo(emu),

        "FreeEncryptedFileMetadata" => FreeEncryptedFileMetadata(emu),

        "FreeEncryptionCertificateHashList" => FreeEncryptionCertificateHashList(emu),

        "FreeInheritedFromArray" => FreeInheritedFromArray(emu),

        "FreeSid" => FreeSid(emu),

        "GetAccessPermissionsForObjectA" => GetAccessPermissionsForObjectA(emu),

        "GetAccessPermissionsForObjectW" => GetAccessPermissionsForObjectW(emu),

        "GetAce" => GetAce(emu),

        "GetAclInformation" => GetAclInformation(emu),

        "GetAuditedPermissionsFromAclA" => GetAuditedPermissionsFromAclA(emu),

        "GetAuditedPermissionsFromAclW" => GetAuditedPermissionsFromAclW(emu),

        "GetCurrentHwProfileA" => GetCurrentHwProfileA(emu),

        "GetCurrentHwProfileW" => GetCurrentHwProfileW(emu),

        "GetDynamicTimeZoneInformationEffectiveYears" => {
            GetDynamicTimeZoneInformationEffectiveYears(emu)
        }

        "GetEffectiveRightsFromAclA" => GetEffectiveRightsFromAclA(emu),

        "GetEffectiveRightsFromAclW" => GetEffectiveRightsFromAclW(emu),

        "GetEncryptedFileMetadata" => GetEncryptedFileMetadata(emu),

        "GetEventLogInformation" => GetEventLogInformation(emu),

        "GetExplicitEntriesFromAclA" => GetExplicitEntriesFromAclA(emu),

        "GetExplicitEntriesFromAclW" => GetExplicitEntriesFromAclW(emu),

        "GetFileSecurityA" => GetFileSecurityA(emu),

        "GetFileSecurityW" => GetFileSecurityW(emu),

        "GetInformationCodeAuthzLevelW" => GetInformationCodeAuthzLevelW(emu),

        "GetInformationCodeAuthzPolicyW" => GetInformationCodeAuthzPolicyW(emu),

        "GetInheritanceSourceA" => GetInheritanceSourceA(emu),

        "GetInheritanceSourceW" => GetInheritanceSourceW(emu),

        "GetKernelObjectSecurity" => GetKernelObjectSecurity(emu),

        "GetLengthSid" => GetLengthSid(emu),

        "GetLocalManagedApplicationData" => GetLocalManagedApplicationData(emu),

        "GetLocalManagedApplications" => GetLocalManagedApplications(emu),

        "GetManagedApplicationCategories" => GetManagedApplicationCategories(emu),

        "GetManagedApplications" => GetManagedApplications(emu),

        "GetMultipleTrusteeA" => GetMultipleTrusteeA(emu),

        "GetMultipleTrusteeOperationA" => GetMultipleTrusteeOperationA(emu),

        "GetMultipleTrusteeOperationW" => GetMultipleTrusteeOperationW(emu),

        "GetMultipleTrusteeW" => GetMultipleTrusteeW(emu),

        "GetNamedSecurityInfoA" => GetNamedSecurityInfoA(emu),

        "GetNamedSecurityInfoExA" => GetNamedSecurityInfoExA(emu),

        "GetNamedSecurityInfoExW" => GetNamedSecurityInfoExW(emu),

        "GetNamedSecurityInfoW" => GetNamedSecurityInfoW(emu),

        "GetNumberOfEventLogRecords" => GetNumberOfEventLogRecords(emu),

        "GetOldestEventLogRecord" => GetOldestEventLogRecord(emu),

        "GetOverlappedAccessResults" => GetOverlappedAccessResults(emu),

        "GetPrivateObjectSecurity" => GetPrivateObjectSecurity(emu),

        "GetSecurityDescriptorControl" => GetSecurityDescriptorControl(emu),

        "GetSecurityDescriptorDacl" => GetSecurityDescriptorDacl(emu),

        "GetSecurityDescriptorGroup" => GetSecurityDescriptorGroup(emu),

        "GetSecurityDescriptorLength" => GetSecurityDescriptorLength(emu),

        "GetSecurityDescriptorOwner" => GetSecurityDescriptorOwner(emu),

        "GetSecurityDescriptorRMControl" => GetSecurityDescriptorRMControl(emu),

        "GetSecurityDescriptorSacl" => GetSecurityDescriptorSacl(emu),

        "GetSecurityInfo" => GetSecurityInfo(emu),

        "GetSecurityInfoExA" => GetSecurityInfoExA(emu),

        "GetSecurityInfoExW" => GetSecurityInfoExW(emu),

        "GetServiceDisplayNameA" => GetServiceDisplayNameA(emu),

        "GetServiceDisplayNameW" => GetServiceDisplayNameW(emu),

        "GetServiceKeyNameA" => GetServiceKeyNameA(emu),

        "GetServiceKeyNameW" => GetServiceKeyNameW(emu),

        "GetSidIdentifierAuthority" => GetSidIdentifierAuthority(emu),

        "GetSidLengthRequired" => GetSidLengthRequired(emu),

        "GetSidSubAuthority" => GetSidSubAuthority(emu),

        "GetSidSubAuthorityCount" => GetSidSubAuthorityCount(emu),

        "GetStringConditionFromBinary" => GetStringConditionFromBinary(emu),

        "GetThreadWaitChain" => GetThreadWaitChain(emu),

        "GetTokenInformation" => GetTokenInformation(emu),

        "GetTraceEnableFlags" => GetTraceEnableFlags(emu),

        "GetTraceEnableLevel" => GetTraceEnableLevel(emu),

        "GetTraceLoggerHandle" => GetTraceLoggerHandle(emu),

        "GetTrusteeFormA" => GetTrusteeFormA(emu),

        "GetTrusteeFormW" => GetTrusteeFormW(emu),

        "GetTrusteeNameA" => GetTrusteeNameA(emu),

        "GetTrusteeNameW" => GetTrusteeNameW(emu),

        "GetTrusteeTypeA" => GetTrusteeTypeA(emu),

        "GetTrusteeTypeW" => GetTrusteeTypeW(emu),

        "GetWindowsAccountDomainSid" => GetWindowsAccountDomainSid(emu),

        "I_QueryTagInformation" => I_QueryTagInformation(emu),

        "I_ScGetCurrentGroupStateW" => I_ScGetCurrentGroupStateW(emu),

        "I_ScIsSecurityProcess" => I_ScIsSecurityProcess(emu),

        "I_ScPnPGetServiceName" => I_ScPnPGetServiceName(emu),

        "I_ScQueryServiceConfig" => I_ScQueryServiceConfig(emu),

        "I_ScRegisterPreshutdownRestart" => I_ScRegisterPreshutdownRestart(emu),

        "I_ScReparseServiceDatabase" => I_ScReparseServiceDatabase(emu),

        "I_ScSendPnPMessage" => I_ScSendPnPMessage(emu),

        "I_ScSendTSMessage" => I_ScSendTSMessage(emu),

        "I_ScSetServiceBitsA" => I_ScSetServiceBitsA(emu),

        "I_ScSetServiceBitsW" => I_ScSetServiceBitsW(emu),

        "I_ScValidatePnPService" => I_ScValidatePnPService(emu),

        "IdentifyCodeAuthzLevelW" => IdentifyCodeAuthzLevelW(emu),

        "ImpersonateAnonymousToken" => ImpersonateAnonymousToken(emu),

        "ImpersonateLoggedOnUser" => ImpersonateLoggedOnUser(emu),

        "ImpersonateNamedPipeClient" => ImpersonateNamedPipeClient(emu),

        "ImpersonateSelf" => ImpersonateSelf(emu),

        "InitializeAcl" => InitializeAcl(emu),

        "InitializeSecurityDescriptor" => InitializeSecurityDescriptor(emu),

        "InitializeSid" => InitializeSid(emu),

        "InitiateShutdownA" => InitiateShutdownA(emu),

        "InitiateShutdownW" => InitiateShutdownW(emu),

        "InitiateSystemShutdownA" => InitiateSystemShutdownA(emu),

        "InitiateSystemShutdownExA" => InitiateSystemShutdownExA(emu),

        "InitiateSystemShutdownExW" => InitiateSystemShutdownExW(emu),

        "InitiateSystemShutdownW" => InitiateSystemShutdownW(emu),

        "InstallApplication" => InstallApplication(emu),

        "IsTextUnicode" => IsTextUnicode(emu),

        "IsTokenRestricted" => IsTokenRestricted(emu),

        "IsTokenUntrusted" => IsTokenUntrusted(emu),

        "IsValidAcl" => IsValidAcl(emu),

        "IsValidRelativeSecurityDescriptor" => IsValidRelativeSecurityDescriptor(emu),

        "IsValidSecurityDescriptor" => IsValidSecurityDescriptor(emu),

        "IsValidSid" => IsValidSid(emu),

        "IsWellKnownSid" => IsWellKnownSid(emu),

        "LockServiceDatabase" => LockServiceDatabase(emu),

        "LogonSecondaryUserIntoSessionW" => LogonSecondaryUserIntoSessionW(emu),

        "LogonUserA" => LogonUserA(emu),

        "LogonUserExA" => LogonUserExA(emu),

        "LogonUserExExW" => LogonUserExExW(emu),

        "LogonUserExW" => LogonUserExW(emu),

        "LogonUserW" => LogonUserW(emu),

        "LookupAccountNameA" => LookupAccountNameA(emu),

        "LookupAccountNameW" => LookupAccountNameW(emu),

        "LookupAccountSidA" => LookupAccountSidA(emu),

        "LookupAccountSidW" => LookupAccountSidW(emu),

        "LookupPrivilegeDisplayNameA" => LookupPrivilegeDisplayNameA(emu),

        "LookupPrivilegeDisplayNameW" => LookupPrivilegeDisplayNameW(emu),

        "LookupPrivilegeNameA" => LookupPrivilegeNameA(emu),

        "LookupPrivilegeNameW" => LookupPrivilegeNameW(emu),

        "LookupPrivilegeValueA" => LookupPrivilegeValueA(emu),

        "LookupSecurityDescriptorPartsA" => LookupSecurityDescriptorPartsA(emu),

        "LookupSecurityDescriptorPartsW" => LookupSecurityDescriptorPartsW(emu),

        "LsaAddAccountRights" => LsaAddAccountRights(emu),

        "LsaAddPrivilegesToAccount" => LsaAddPrivilegesToAccount(emu),

        "LsaClearAuditLog" => LsaClearAuditLog(emu),

        "LsaClose" => LsaClose(emu),

        "LsaConfigureAutoLogonCredentials" => LsaConfigureAutoLogonCredentials(emu),

        "LsaCreateAccount" => LsaCreateAccount(emu),

        "LsaCreateSecret" => LsaCreateSecret(emu),

        "LsaCreateTrustedDomain" => LsaCreateTrustedDomain(emu),

        "LsaCreateTrustedDomainEx" => LsaCreateTrustedDomainEx(emu),

        "LsaDelete" => LsaDelete(emu),

        "LsaDeleteTrustedDomain" => LsaDeleteTrustedDomain(emu),

        "LsaDisableUserArso" => LsaDisableUserArso(emu),

        "LsaEnableUserArso" => LsaEnableUserArso(emu),

        "LsaEnumerateAccountRights" => LsaEnumerateAccountRights(emu),

        "LsaEnumerateAccounts" => LsaEnumerateAccounts(emu),

        "LsaEnumerateAccountsWithUserRight" => LsaEnumerateAccountsWithUserRight(emu),

        "LsaEnumeratePrivileges" => LsaEnumeratePrivileges(emu),

        "LsaEnumeratePrivilegesOfAccount" => LsaEnumeratePrivilegesOfAccount(emu),

        "LsaEnumerateTrustedDomains" => LsaEnumerateTrustedDomains(emu),

        "LsaEnumerateTrustedDomainsEx" => LsaEnumerateTrustedDomainsEx(emu),

        "LsaFreeMemory" => LsaFreeMemory(emu),

        "LsaGetAppliedCAPIDs" => LsaGetAppliedCAPIDs(emu),

        "LsaGetDeviceRegistrationInfo" => LsaGetDeviceRegistrationInfo(emu),

        "LsaGetQuotasForAccount" => LsaGetQuotasForAccount(emu),

        "LsaGetRemoteUserName" => LsaGetRemoteUserName(emu),

        "LsaGetSystemAccessAccount" => LsaGetSystemAccessAccount(emu),

        "LsaGetUserName" => LsaGetUserName(emu),

        "LsaICLookupNames" => LsaICLookupNames(emu),

        "LsaICLookupNamesWithCreds" => LsaICLookupNamesWithCreds(emu),

        "LsaICLookupSids" => LsaICLookupSids(emu),

        "LsaICLookupSidsWithCreds" => LsaICLookupSidsWithCreds(emu),

        "LsaInvokeTrustScanner" => LsaInvokeTrustScanner(emu),

        "LsaIsUserArsoAllowed" => LsaIsUserArsoAllowed(emu),

        "LsaIsUserArsoEnabled" => LsaIsUserArsoEnabled(emu),

        "LsaLookupNames" => LsaLookupNames(emu),

        "LsaLookupNames2" => LsaLookupNames2(emu),

        "LsaLookupPrivilegeDisplayName" => LsaLookupPrivilegeDisplayName(emu),

        "LsaLookupPrivilegeName" => LsaLookupPrivilegeName(emu),

        "LsaLookupPrivilegeValue" => LsaLookupPrivilegeValue(emu),

        "LsaLookupSids" => LsaLookupSids(emu),

        "LsaLookupSids2" => LsaLookupSids2(emu),

        "LsaManageSidNameMapping" => LsaManageSidNameMapping(emu),

        "LsaNtStatusToWinError" => LsaNtStatusToWinError(emu),

        "LsaOpenAccount" => LsaOpenAccount(emu),

        "LsaOpenPolicy" => LsaOpenPolicy(emu),

        "LsaOpenPolicySce" => LsaOpenPolicySce(emu),

        "LsaOpenSecret" => LsaOpenSecret(emu),

        "LsaOpenTrustedDomain" => LsaOpenTrustedDomain(emu),

        "LsaOpenTrustedDomainByName" => LsaOpenTrustedDomainByName(emu),

        "LsaProfileDeleted" => LsaProfileDeleted(emu),

        "LsaPurgeLocalSystemAccessTable" => LsaPurgeLocalSystemAccessTable(emu),

        "LsaQueryCAPs" => LsaQueryCAPs(emu),

        "LsaQueryDomainInformationPolicy" => LsaQueryDomainInformationPolicy(emu),

        "LsaQueryForestTrustInformation" => LsaQueryForestTrustInformation(emu),

        "LsaQueryForestTrustInformation2" => LsaQueryForestTrustInformation2(emu),

        "LsaQueryInfoTrustedDomain" => LsaQueryInfoTrustedDomain(emu),

        "LsaQueryInformationPolicy" => LsaQueryInformationPolicy(emu),

        "LsaQueryLocalSystemAccess" => LsaQueryLocalSystemAccess(emu),

        "LsaQueryLocalSystemAccessAll" => LsaQueryLocalSystemAccessAll(emu),

        "LsaQuerySecret" => LsaQuerySecret(emu),

        "LsaQuerySecurityObject" => LsaQuerySecurityObject(emu),

        "LsaQueryTrustedDomainInfo" => LsaQueryTrustedDomainInfo(emu),

        "LsaQueryTrustedDomainInfoByName" => LsaQueryTrustedDomainInfoByName(emu),

        "LsaRemoveAccountRights" => LsaRemoveAccountRights(emu),

        "LsaRemovePrivilegesFromAccount" => LsaRemovePrivilegesFromAccount(emu),

        "LsaRetrievePrivateData" => LsaRetrievePrivateData(emu),

        "LsaSetCAPs" => LsaSetCAPs(emu),

        "LsaSetDomainInformationPolicy" => LsaSetDomainInformationPolicy(emu),

        "LsaSetForestTrustInformation" => LsaSetForestTrustInformation(emu),

        "LsaSetForestTrustInformation2" => LsaSetForestTrustInformation2(emu),

        "LsaSetInformationPolicy" => LsaSetInformationPolicy(emu),

        "LsaSetInformationTrustedDomain" => LsaSetInformationTrustedDomain(emu),

        "LsaSetLocalSystemAccess" => LsaSetLocalSystemAccess(emu),

        "LsaSetQuotasForAccount" => LsaSetQuotasForAccount(emu),

        "LsaSetSecret" => LsaSetSecret(emu),

        "LsaSetSecurityObject" => LsaSetSecurityObject(emu),

        "LsaSetSystemAccessAccount" => LsaSetSystemAccessAccount(emu),

        "LsaSetTrustedDomainInfoByName" => LsaSetTrustedDomainInfoByName(emu),

        "LsaSetTrustedDomainInformation" => LsaSetTrustedDomainInformation(emu),

        "LsaStorePrivateData" => LsaStorePrivateData(emu),

        "LsaValidateProcUniqueLuid" => LsaValidateProcUniqueLuid(emu),

        "MD4Final" => MD4Final(emu),

        "MD4Init" => MD4Init(emu),

        "MD4Update" => MD4Update(emu),

        "MD5Final" => MD5Final(emu),

        "MD5Init" => MD5Init(emu),

        "MD5Update" => MD5Update(emu),

        "MIDL_user_free_Ext" => MIDL_user_free_Ext(emu),

        "MSChapSrvChangePassword" => MSChapSrvChangePassword(emu),

        "MSChapSrvChangePassword2" => MSChapSrvChangePassword2(emu),

        "MakeAbsoluteSD" => MakeAbsoluteSD(emu),

        "MakeAbsoluteSD2" => MakeAbsoluteSD2(emu),

        "MakeSelfRelativeSD" => MakeSelfRelativeSD(emu),

        "MapGenericMask" => MapGenericMask(emu),

        "NotifyBootConfigStatus" => NotifyBootConfigStatus(emu),

        "NotifyChangeEventLog" => NotifyChangeEventLog(emu),

        "NotifyServiceStatusChange" => NotifyServiceStatusChange(emu),

        "NotifyServiceStatusChangeA" => NotifyServiceStatusChangeA(emu),

        "NotifyServiceStatusChangeW" => NotifyServiceStatusChangeW(emu),

        "NpGetUserName" => NpGetUserName(emu),

        "ObjectCloseAuditAlarmA" => ObjectCloseAuditAlarmA(emu),

        "ObjectCloseAuditAlarmW" => ObjectCloseAuditAlarmW(emu),

        "ObjectDeleteAuditAlarmA" => ObjectDeleteAuditAlarmA(emu),

        "ObjectDeleteAuditAlarmW" => ObjectDeleteAuditAlarmW(emu),

        "ObjectOpenAuditAlarmA" => ObjectOpenAuditAlarmA(emu),

        "ObjectOpenAuditAlarmW" => ObjectOpenAuditAlarmW(emu),

        "ObjectPrivilegeAuditAlarmA" => ObjectPrivilegeAuditAlarmA(emu),

        "ObjectPrivilegeAuditAlarmW" => ObjectPrivilegeAuditAlarmW(emu),

        "OpenBackupEventLogA" => OpenBackupEventLogA(emu),

        "OpenBackupEventLogW" => OpenBackupEventLogW(emu),

        "OpenEncryptedFileRawA" => OpenEncryptedFileRawA(emu),

        "OpenEncryptedFileRawW" => OpenEncryptedFileRawW(emu),

        "OpenEventLogA" => OpenEventLogA(emu),

        "OpenEventLogW" => OpenEventLogW(emu),

        "OpenProcessToken" => OpenProcessToken(emu),

        "OpenSCManagerA" => OpenSCManagerA(emu),

        "OpenSCManagerW" => OpenSCManagerW(emu),

        "OpenServiceA" => OpenServiceA(emu),

        "OpenServiceW" => OpenServiceW(emu),

        "OpenThreadToken" => OpenThreadToken(emu),

        "OpenThreadWaitChainSession" => OpenThreadWaitChainSession(emu),

        "OpenTraceA" => OpenTraceA(emu),

        "OpenTraceW" => OpenTraceW(emu),

        "OperationEnd" => OperationEnd(emu),

        "OperationStart" => OperationStart(emu),

        "PerfAddCounters" => PerfAddCounters(emu),

        "PerfCloseQueryHandle" => PerfCloseQueryHandle(emu),

        "PerfCreateInstance" => PerfCreateInstance(emu),

        "PerfDecrementULongCounterValue" => PerfDecrementULongCounterValue(emu),

        "PerfDecrementULongLongCounterValue" => PerfDecrementULongLongCounterValue(emu),

        "PerfDeleteCounters" => PerfDeleteCounters(emu),

        "PerfDeleteInstance" => PerfDeleteInstance(emu),

        "PerfEnumerateCounterSet" => PerfEnumerateCounterSet(emu),

        "PerfEnumerateCounterSetInstances" => PerfEnumerateCounterSetInstances(emu),

        "PerfIncrementULongCounterValue" => PerfIncrementULongCounterValue(emu),

        "PerfIncrementULongLongCounterValue" => PerfIncrementULongLongCounterValue(emu),

        "PerfOpenQueryHandle" => PerfOpenQueryHandle(emu),

        "PerfQueryCounterData" => PerfQueryCounterData(emu),

        "PerfQueryCounterInfo" => PerfQueryCounterInfo(emu),

        "PerfQueryCounterSetRegistrationInfo" => PerfQueryCounterSetRegistrationInfo(emu),

        "PerfQueryInstance" => PerfQueryInstance(emu),

        "PerfRegCloseKey" => PerfRegCloseKey(emu),

        "PerfRegEnumKey" => PerfRegEnumKey(emu),

        "PerfRegEnumValue" => PerfRegEnumValue(emu),

        "PerfRegQueryInfoKey" => PerfRegQueryInfoKey(emu),

        "PerfRegQueryValue" => PerfRegQueryValue(emu),

        "PerfRegSetValue" => PerfRegSetValue(emu),

        "PerfSetCounterRefValue" => PerfSetCounterRefValue(emu),

        "PerfSetCounterSetInfo" => PerfSetCounterSetInfo(emu),

        "PerfSetULongCounterValue" => PerfSetULongCounterValue(emu),

        "PerfSetULongLongCounterValue" => PerfSetULongLongCounterValue(emu),

        "PerfStartProvider" => PerfStartProvider(emu),

        "PerfStartProviderEx" => PerfStartProviderEx(emu),

        "PerfStopProvider" => PerfStopProvider(emu),

        "PrivilegeCheck" => PrivilegeCheck(emu),

        "PrivilegedServiceAuditAlarmA" => PrivilegedServiceAuditAlarmA(emu),

        "PrivilegedServiceAuditAlarmW" => PrivilegedServiceAuditAlarmW(emu),

        "ProcessIdleTasks" => ProcessIdleTasks(emu),

        "ProcessIdleTasksW" => ProcessIdleTasksW(emu),

        "ProcessTrace" => ProcessTrace(emu),

        "QueryAllTracesA" => QueryAllTracesA(emu),

        "QueryAllTracesW" => QueryAllTracesW(emu),

        "QueryLocalUserServiceName" => QueryLocalUserServiceName(emu),

        "QueryRecoveryAgentsOnEncryptedFile" => QueryRecoveryAgentsOnEncryptedFile(emu),

        "QuerySecurityAccessMask" => QuerySecurityAccessMask(emu),

        "QueryServiceConfig2A" => QueryServiceConfig2A(emu),

        "QueryServiceConfig2W" => QueryServiceConfig2W(emu),

        "QueryServiceConfigA" => QueryServiceConfigA(emu),

        "QueryServiceConfigW" => QueryServiceConfigW(emu),

        "QueryServiceDynamicInformation" => QueryServiceDynamicInformation(emu),

        "QueryServiceLockStatusA" => QueryServiceLockStatusA(emu),

        "QueryServiceLockStatusW" => QueryServiceLockStatusW(emu),

        "QueryServiceObjectSecurity" => QueryServiceObjectSecurity(emu),

        "QueryServiceStatus" => QueryServiceStatus(emu),

        "QueryServiceStatusEx" => QueryServiceStatusEx(emu),

        "QueryTraceA" => QueryTraceA(emu),

        "QueryTraceProcessingHandle" => QueryTraceProcessingHandle(emu),

        "QueryTraceW" => QueryTraceW(emu),

        "QueryUserServiceName" => QueryUserServiceName(emu),

        "QueryUserServiceNameForContext" => QueryUserServiceNameForContext(emu),

        "QueryUsersOnEncryptedFile" => QueryUsersOnEncryptedFile(emu),

        "ReadEncryptedFileRaw" => ReadEncryptedFileRaw(emu),

        "ReadEventLogA" => ReadEventLogA(emu),

        "ReadEventLogW" => ReadEventLogW(emu),

        "RegCloseKey" => RegCloseKey_api(emu),

        "RegOpenKeyExA" => RegOpenKeyExA_api(emu),

        "RegOpenKeyExW" => RegOpenKeyExW_api(emu),

        "RegQueryValueExA" => RegQueryValueExA_api(emu),

        "RegQueryValueExW" => RegQueryValueExW_api(emu),

        "RegConnectRegistryA" => RegConnectRegistryA(emu),

        "RegConnectRegistryExA" => RegConnectRegistryExA(emu),

        "RegConnectRegistryExW" => RegConnectRegistryExW(emu),

        "RegConnectRegistryW" => RegConnectRegistryW(emu),

        "RegCopyTreeA" => RegCopyTreeA(emu),

        "RegCopyTreeW" => RegCopyTreeW(emu),

        "RegCreateKeyA" => RegCreateKeyA(emu),

        "RegCreateKeyExA" => RegCreateKeyExA(emu),

        "RegCreateKeyExW" => RegCreateKeyExW(emu),

        "RegCreateKeyTransactedA" => RegCreateKeyTransactedA(emu),

        "RegCreateKeyTransactedW" => RegCreateKeyTransactedW(emu),

        "RegCreateKeyW" => RegCreateKeyW(emu),

        "RegDeleteKeyA" => RegDeleteKeyA(emu),

        "RegDeleteKeyExA" => RegDeleteKeyExA(emu),

        "RegDeleteKeyExW" => RegDeleteKeyExW(emu),

        "RegDeleteKeyTransactedA" => RegDeleteKeyTransactedA(emu),

        "RegDeleteKeyTransactedW" => RegDeleteKeyTransactedW(emu),

        "RegDeleteKeyValueA" => RegDeleteKeyValueA(emu),

        "RegDeleteKeyValueW" => RegDeleteKeyValueW(emu),

        "RegDeleteKeyW" => RegDeleteKeyW(emu),

        "RegDeleteTreeA" => RegDeleteTreeA(emu),

        "RegDeleteTreeW" => RegDeleteTreeW(emu),

        "RegDeleteValueA" => RegDeleteValueA(emu),

        "RegDeleteValueW" => RegDeleteValueW(emu),

        "RegDisablePredefinedCache" => RegDisablePredefinedCache(emu),

        "RegDisablePredefinedCacheEx" => RegDisablePredefinedCacheEx(emu),

        "RegDisableReflectionKey" => RegDisableReflectionKey(emu),

        "RegEnableReflectionKey" => RegEnableReflectionKey(emu),

        "RegEnumKeyA" => RegEnumKeyA(emu),

        "RegEnumKeyExA" => RegEnumKeyExA(emu),

        "RegEnumKeyExW" => RegEnumKeyExW(emu),

        "RegEnumKeyW" => RegEnumKeyW(emu),

        "RegEnumValueA" => RegEnumValueA(emu),

        "RegEnumValueW" => RegEnumValueW(emu),

        "RegFlushKey" => RegFlushKey(emu),

        "RegGetKeySecurity" => RegGetKeySecurity(emu),

        "RegGetValueA" => RegGetValueA(emu),

        "RegGetValueW" => RegGetValueW(emu),

        "RegLoadAppKeyA" => RegLoadAppKeyA(emu),

        "RegLoadAppKeyW" => RegLoadAppKeyW(emu),

        "RegLoadKeyA" => RegLoadKeyA(emu),

        "RegLoadKeyW" => RegLoadKeyW(emu),

        "RegLoadMUIStringA" => RegLoadMUIStringA(emu),

        "RegLoadMUIStringW" => RegLoadMUIStringW(emu),

        "RegNotifyChangeKeyValue" => RegNotifyChangeKeyValue(emu),

        "RegOpenCurrentUser" => RegOpenCurrentUser(emu),

        "RegOpenKeyA" => RegOpenKeyA(emu),

        "RegOpenKeyTransactedA" => RegOpenKeyTransactedA(emu),

        "RegOpenKeyTransactedW" => RegOpenKeyTransactedW(emu),

        "RegOpenKeyW" => RegOpenKeyW(emu),

        "RegOpenUserClassesRoot" => RegOpenUserClassesRoot(emu),

        "RegOverridePredefKey" => RegOverridePredefKey(emu),

        "RegQueryInfoKeyA" => RegQueryInfoKeyA(emu),

        "RegQueryInfoKeyW" => RegQueryInfoKeyW(emu),

        "RegQueryMultipleValuesA" => RegQueryMultipleValuesA(emu),

        "RegQueryMultipleValuesW" => RegQueryMultipleValuesW(emu),

        "RegQueryReflectionKey" => RegQueryReflectionKey(emu),

        "RegQueryValueA" => RegQueryValueA(emu),

        "RegQueryValueW" => RegQueryValueW(emu),

        "RegRenameKey" => RegRenameKey(emu),

        "RegReplaceKeyA" => RegReplaceKeyA(emu),

        "RegReplaceKeyW" => RegReplaceKeyW(emu),

        "RegRestoreKeyA" => RegRestoreKeyA(emu),

        "RegRestoreKeyW" => RegRestoreKeyW(emu),

        "RegSaveKeyA" => RegSaveKeyA(emu),

        "RegSaveKeyExA" => RegSaveKeyExA(emu),

        "RegSaveKeyExW" => RegSaveKeyExW(emu),

        "RegSaveKeyW" => RegSaveKeyW(emu),

        "RegSetKeySecurity" => RegSetKeySecurity(emu),

        "RegSetKeyValueA" => RegSetKeyValueA(emu),

        "RegSetKeyValueW" => RegSetKeyValueW(emu),

        "RegSetValueA" => RegSetValueA(emu),

        "RegSetValueExA" => RegSetValueExA(emu),

        "RegSetValueExW" => RegSetValueExW(emu),

        "RegSetValueW" => RegSetValueW(emu),

        "RegUnLoadKeyA" => RegUnLoadKeyA(emu),

        "RegUnLoadKeyW" => RegUnLoadKeyW(emu),

        "RegisterEventSourceA" => RegisterEventSourceA(emu),

        "RegisterEventSourceW" => RegisterEventSourceW(emu),

        "RegisterIdleTask" => RegisterIdleTask(emu),

        "RegisterServiceCtrlHandlerA" => RegisterServiceCtrlHandlerA(emu),

        "RegisterServiceCtrlHandlerExA" => RegisterServiceCtrlHandlerExA(emu),

        "RegisterServiceCtrlHandlerExW" => RegisterServiceCtrlHandlerExW(emu),

        "RegisterServiceCtrlHandlerW" => RegisterServiceCtrlHandlerW(emu),

        "RegisterTraceGuidsA" => RegisterTraceGuidsA(emu),

        "RegisterTraceGuidsW" => RegisterTraceGuidsW(emu),

        "RegisterWaitChainCOMCallback" => RegisterWaitChainCOMCallback(emu),

        "RemoteRegEnumKeyWrapper" => RemoteRegEnumKeyWrapper(emu),

        "RemoteRegEnumValueWrapper" => RemoteRegEnumValueWrapper(emu),

        "RemoteRegQueryInfoKeyWrapper" => RemoteRegQueryInfoKeyWrapper(emu),

        "RemoteRegQueryMultipleValues2Wrapper" => RemoteRegQueryMultipleValues2Wrapper(emu),

        "RemoteRegQueryMultipleValuesWrapper" => RemoteRegQueryMultipleValuesWrapper(emu),

        "RemoteRegQueryValueWrapper" => RemoteRegQueryValueWrapper(emu),

        "RemoveTraceCallback" => RemoveTraceCallback(emu),

        "RemoveUsersFromEncryptedFile" => RemoveUsersFromEncryptedFile(emu),

        "ReportEventA" => ReportEventA(emu),

        "ReportEventW" => ReportEventW(emu),

        "RevertToSelf" => RevertToSelf(emu),

        "SafeBaseRegGetKeySecurity" => SafeBaseRegGetKeySecurity(emu),

        "SaferCloseLevel" => SaferCloseLevel(emu),

        "SaferComputeTokenFromLevel" => SaferComputeTokenFromLevel(emu),

        "SaferCreateLevel" => SaferCreateLevel(emu),

        "SaferGetLevelInformation" => SaferGetLevelInformation(emu),

        "SaferGetPolicyInformation" => SaferGetPolicyInformation(emu),

        "SaferIdentifyLevel" => SaferIdentifyLevel(emu),

        "SaferRecordEventLogEntry" => SaferRecordEventLogEntry(emu),

        "SaferSetLevelInformation" => SaferSetLevelInformation(emu),

        "SaferSetPolicyInformation" => SaferSetPolicyInformation(emu),

        "SaferiChangeRegistryScope" => SaferiChangeRegistryScope(emu),

        "SaferiCompareTokenLevels" => SaferiCompareTokenLevels(emu),

        "SaferiIsDllAllowed" => SaferiIsDllAllowed(emu),

        "SaferiIsExecutableFileType" => SaferiIsExecutableFileType(emu),

        "SaferiPopulateDefaultsInRegistry" => SaferiPopulateDefaultsInRegistry(emu),

        "SaferiRecordEventLogEntry" => SaferiRecordEventLogEntry(emu),

        "SaferiSearchMatchingHashRules" => SaferiSearchMatchingHashRules(emu),

        "SetAclInformation" => SetAclInformation(emu),

        "SetEncryptedFileMetadata" => SetEncryptedFileMetadata(emu),

        "SetEntriesInAccessListA" => SetEntriesInAccessListA(emu),

        "SetEntriesInAccessListW" => SetEntriesInAccessListW(emu),

        "SetEntriesInAclA" => SetEntriesInAclA(emu),

        "SetEntriesInAclW" => SetEntriesInAclW(emu),

        "SetEntriesInAuditListA" => SetEntriesInAuditListA(emu),

        "SetEntriesInAuditListW" => SetEntriesInAuditListW(emu),

        "SetFileSecurityA" => SetFileSecurityA(emu),

        "SetFileSecurityW" => SetFileSecurityW(emu),

        "SetInformationCodeAuthzLevelW" => SetInformationCodeAuthzLevelW(emu),

        "SetInformationCodeAuthzPolicyW" => SetInformationCodeAuthzPolicyW(emu),

        "SetKernelObjectSecurity" => SetKernelObjectSecurity(emu),

        "SetNamedSecurityInfoA" => SetNamedSecurityInfoA(emu),

        "SetNamedSecurityInfoExA" => SetNamedSecurityInfoExA(emu),

        "SetNamedSecurityInfoExW" => SetNamedSecurityInfoExW(emu),

        "SetNamedSecurityInfoW" => SetNamedSecurityInfoW(emu),

        "SetPrivateObjectSecurity" => SetPrivateObjectSecurity(emu),

        "SetPrivateObjectSecurityEx" => SetPrivateObjectSecurityEx(emu),

        "SetSecurityAccessMask" => SetSecurityAccessMask(emu),

        "SetSecurityDescriptorControl" => SetSecurityDescriptorControl(emu),

        "SetSecurityDescriptorDacl" => SetSecurityDescriptorDacl(emu),

        "SetSecurityDescriptorGroup" => SetSecurityDescriptorGroup(emu),

        "SetSecurityDescriptorOwner" => SetSecurityDescriptorOwner(emu),

        "SetSecurityDescriptorRMControl" => SetSecurityDescriptorRMControl(emu),

        "SetSecurityDescriptorSacl" => SetSecurityDescriptorSacl(emu),

        "SetSecurityInfo" => SetSecurityInfo(emu),

        "SetSecurityInfoExA" => SetSecurityInfoExA(emu),

        "SetSecurityInfoExW" => SetSecurityInfoExW(emu),

        "SetServiceBits" => SetServiceBits(emu),

        "SetServiceObjectSecurity" => SetServiceObjectSecurity(emu),

        "SetServiceStatus" => SetServiceStatus(emu),

        "SetThreadToken" => SetThreadToken(emu),

        "SetTokenInformation" => SetTokenInformation(emu),

        "SetTraceCallback" => SetTraceCallback(emu),

        "SetUserFileEncryptionKey" => SetUserFileEncryptionKey(emu),

        "SetUserFileEncryptionKeyEx" => SetUserFileEncryptionKeyEx(emu),

        "StartServiceA" => StartServiceA(emu),

        "StartServiceW" => StartServiceW(emu),

        "StartTraceA" => StartTraceA(emu),

        "StartTraceW" => StartTraceW(emu),

        "StopTraceA" => StopTraceA(emu),

        "StopTraceW" => StopTraceW(emu),

        "SystemFunction001" => SystemFunction001(emu),

        "SystemFunction002" => SystemFunction002(emu),

        "SystemFunction003" => SystemFunction003(emu),

        "SystemFunction004" => SystemFunction004(emu),

        "SystemFunction005" => SystemFunction005(emu),

        "SystemFunction006" => SystemFunction006(emu),

        "SystemFunction007" => SystemFunction007(emu),

        "SystemFunction008" => SystemFunction008(emu),

        "SystemFunction009" => SystemFunction009(emu),

        "SystemFunction010" => SystemFunction010(emu),

        "SystemFunction011" => SystemFunction011(emu),

        "SystemFunction012" => SystemFunction012(emu),

        "SystemFunction013" => SystemFunction013(emu),

        "SystemFunction014" => SystemFunction014(emu),

        "SystemFunction015" => SystemFunction015(emu),

        "SystemFunction016" => SystemFunction016(emu),

        "SystemFunction017" => SystemFunction017(emu),

        "SystemFunction018" => SystemFunction018(emu),

        "SystemFunction019" => SystemFunction019(emu),

        "SystemFunction020" => SystemFunction020(emu),

        "SystemFunction021" => SystemFunction021(emu),

        "SystemFunction022" => SystemFunction022(emu),

        "SystemFunction023" => SystemFunction023(emu),

        "SystemFunction024" => SystemFunction024(emu),

        "SystemFunction025" => SystemFunction025(emu),

        "SystemFunction026" => SystemFunction026(emu),

        "SystemFunction027" => SystemFunction027(emu),

        "SystemFunction028" => SystemFunction028(emu),

        "SystemFunction029" => SystemFunction029(emu),

        "SystemFunction030" => SystemFunction030(emu),

        "SystemFunction031" => SystemFunction031(emu),

        "SystemFunction032" => SystemFunction032(emu),

        "SystemFunction033" => SystemFunction033(emu),

        "SystemFunction034" => SystemFunction034(emu),

        "SystemFunction035" => SystemFunction035(emu),

        "SystemFunction036" => SystemFunction036(emu),

        "SystemFunction040" => SystemFunction040(emu),

        "SystemFunction041" => SystemFunction041(emu),

        "TraceEvent" => TraceEvent(emu),

        "TraceEventInstance" => TraceEventInstance(emu),

        "TraceMessage" => TraceMessage(emu),

        "TraceMessageVa" => TraceMessageVa(emu),

        "TraceQueryInformation" => TraceQueryInformation(emu),

        "TraceSetInformation" => TraceSetInformation(emu),

        "TreeResetNamedSecurityInfoA" => TreeResetNamedSecurityInfoA(emu),

        "TreeResetNamedSecurityInfoW" => TreeResetNamedSecurityInfoW(emu),

        "TreeSetNamedSecurityInfoA" => TreeSetNamedSecurityInfoA(emu),

        "TreeSetNamedSecurityInfoW" => TreeSetNamedSecurityInfoW(emu),

        "TrusteeAccessToObjectA" => TrusteeAccessToObjectA(emu),

        "TrusteeAccessToObjectW" => TrusteeAccessToObjectW(emu),

        "UninstallApplication" => UninstallApplication(emu),

        "UnlockServiceDatabase" => UnlockServiceDatabase(emu),

        "UnregisterIdleTask" => UnregisterIdleTask(emu),

        "UnregisterTraceGuids" => UnregisterTraceGuids(emu),

        "UpdateTraceA" => UpdateTraceA(emu),

        "UpdateTraceW" => UpdateTraceW(emu),

        "UsePinForEncryptedFilesA" => UsePinForEncryptedFilesA(emu),

        "UsePinForEncryptedFilesW" => UsePinForEncryptedFilesW(emu),

        "WaitServiceState" => WaitServiceState(emu),

        "WmiCloseBlock" => WmiCloseBlock(emu),

        "WmiDevInstToInstanceNameA" => WmiDevInstToInstanceNameA(emu),

        "WmiDevInstToInstanceNameW" => WmiDevInstToInstanceNameW(emu),

        "WmiEnumerateGuids" => WmiEnumerateGuids(emu),

        "WmiExecuteMethodA" => WmiExecuteMethodA(emu),

        "WmiExecuteMethodW" => WmiExecuteMethodW(emu),

        "WmiFileHandleToInstanceNameA" => WmiFileHandleToInstanceNameA(emu),

        "WmiFileHandleToInstanceNameW" => WmiFileHandleToInstanceNameW(emu),

        "WmiFreeBuffer" => WmiFreeBuffer(emu),

        "WmiMofEnumerateResourcesA" => WmiMofEnumerateResourcesA(emu),

        "WmiMofEnumerateResourcesW" => WmiMofEnumerateResourcesW(emu),

        "WmiNotificationRegistrationA" => WmiNotificationRegistrationA(emu),

        "WmiNotificationRegistrationW" => WmiNotificationRegistrationW(emu),

        "WmiOpenBlock" => WmiOpenBlock(emu),

        "WmiQueryAllDataA" => WmiQueryAllDataA(emu),

        "WmiQueryAllDataMultipleA" => WmiQueryAllDataMultipleA(emu),

        "WmiQueryAllDataMultipleW" => WmiQueryAllDataMultipleW(emu),

        "WmiQueryAllDataW" => WmiQueryAllDataW(emu),

        "WmiQueryGuidInformation" => WmiQueryGuidInformation(emu),

        "WmiQuerySingleInstanceA" => WmiQuerySingleInstanceA(emu),

        "WmiQuerySingleInstanceMultipleA" => WmiQuerySingleInstanceMultipleA(emu),

        "WmiQuerySingleInstanceMultipleW" => WmiQuerySingleInstanceMultipleW(emu),

        "WmiQuerySingleInstanceW" => WmiQuerySingleInstanceW(emu),

        "WmiReceiveNotificationsA" => WmiReceiveNotificationsA(emu),

        "WmiReceiveNotificationsW" => WmiReceiveNotificationsW(emu),

        "WmiSetSingleInstanceA" => WmiSetSingleInstanceA(emu),

        "WmiSetSingleInstanceW" => WmiSetSingleInstanceW(emu),

        "WmiSetSingleItemA" => WmiSetSingleItemA(emu),

        "WmiSetSingleItemW" => WmiSetSingleItemW(emu),

        "WriteEncryptedFileRaw" => WriteEncryptedFileRaw(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let _hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let _opts = emu.regs().r8;
    let _sam_desired = emu.regs().r9;
    let phk_result = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let subkey = emu.maps.read_string(subkey_ptr);

    log_red!(emu, "advapi32!RegOpenKeyExA {}", subkey);

    if phk_result != 0 {
        let hndl = helper::handler_create(&format!("registry://{}", subkey));
        emu.maps.write_qword(phk_result, hndl);
    }
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let opts = emu.regs().r8;
    let sam_desired = emu.regs().r9;
    let phk_result = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let subkey = emu.maps.read_wide_string(subkey_ptr);

    log_red!(emu, "advapi32!RegOpenKeyExW {}", subkey);

    if phk_result != 0 {
        let hndl = helper::handler_create(&format!("registry://{}", subkey));
        emu.maps.write_qword(phk_result, hndl);
    }
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

fn RegCloseKey(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;

    log_red!(emu, "advapi32!RegCloseKey");

    helper::handler_close(hkey);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

fn RegQueryValueExA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let typ_out = emu.regs().r9;
    let data_out = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let datasz_out = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let mut value = String::new();
    if value_ptr > 0 {
        value = emu.maps.read_string(value_ptr);
    }

    log_red!(emu, "advapi32!RegQueryValueExA {}", value);

    if data_out > 0 {
        emu.maps.write_string(data_out, "some_random_reg_contents");
    }
    if datasz_out > 0 {
        emu.maps.write_qword(datasz_out, 24);
    }
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

pub fn RegQueryValueExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let typ_out = emu.regs().r9;
    let data_out = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let datasz_out = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let mut value = String::new();
    if value_ptr > 0 {
        value = emu.maps.read_wide_string(value_ptr);
    }

    log_red!(emu, "advapi32!RegQueryValueExW {}", value);

    if typ_out != 0 {
        emu.maps.write_dword(typ_out, 1); // REG_SZ
    }
    if data_out > 0 && datasz_out > 0 {
        emu.maps.write_wide_string(data_out, "");
        let written: u64 = 2; // empty wide string = null terminator (2 bytes)
        emu.maps.write_dword(datasz_out, written as u32);
    }
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

///// CRYPTO API /////
