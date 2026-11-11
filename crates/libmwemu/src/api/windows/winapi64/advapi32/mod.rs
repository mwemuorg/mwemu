use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

mod wmi_set_single_item;
pub use wmi_set_single_item::*;

mod wmi_set_single_instance;
pub use wmi_set_single_instance::*;

mod wmi_receive_notifications;
pub use wmi_receive_notifications::*;

mod wmi_query_single_instance_multiple;
pub use wmi_query_single_instance_multiple::*;

mod wmi_query_single_instance;
pub use wmi_query_single_instance::*;

mod wmi_query_all_data_multiple;
pub use wmi_query_all_data_multiple::*;

mod wmi_query_all_data;
pub use wmi_query_all_data::*;

mod wmi_notification_registration;
pub use wmi_notification_registration::*;

mod wmi_mof_enumerate_resources;
pub use wmi_mof_enumerate_resources::*;

mod wmi_file_handle_to_instance_name;
pub use wmi_file_handle_to_instance_name::*;

mod wmi_execute_method;
pub use wmi_execute_method::*;

mod wmi_dev_inst_to_instance_name;
pub use wmi_dev_inst_to_instance_name::*;

mod use_pin_for_encrypted_files;
pub use use_pin_for_encrypted_files::*;

mod update_trace;
pub use update_trace::*;

mod trustee_access_to_object;
pub use trustee_access_to_object::*;

mod tree_set_named_security_info;
pub use tree_set_named_security_info::*;

mod tree_reset_named_security_info;
pub use tree_reset_named_security_info::*;

mod stop_trace;
pub use stop_trace::*;

mod start_trace;
pub use start_trace::*;

mod start_service;
pub use start_service::*;

mod set_security_info_ex;
pub use set_security_info_ex::*;

mod set_named_security_info_ex;
pub use set_named_security_info_ex::*;

mod set_named_security_info;
pub use set_named_security_info::*;

mod set_file_security;
pub use set_file_security::*;

mod set_entries_in_audit_list;
pub use set_entries_in_audit_list::*;

mod set_entries_in_acl;
pub use set_entries_in_acl::*;

mod set_entries_in_access_list;
pub use set_entries_in_access_list::*;

mod report_event;
pub use report_event::*;

mod register_trace_guids;
pub use register_trace_guids::*;

mod register_service_ctrl_handler_ex;
pub use register_service_ctrl_handler_ex::*;

mod register_service_ctrl_handler;
pub use register_service_ctrl_handler::*;

mod register_event_source;
pub use register_event_source::*;

mod reg_un_load_key;
pub use reg_un_load_key::*;

mod reg_set_value_ex;
pub use reg_set_value_ex::*;

mod reg_set_value;
pub use reg_set_value::*;

mod reg_set_key_value;
pub use reg_set_key_value::*;

mod reg_save_key_ex;
pub use reg_save_key_ex::*;

mod reg_save_key;
pub use reg_save_key::*;

mod reg_restore_key;
pub use reg_restore_key::*;

mod reg_replace_key;
pub use reg_replace_key::*;

mod reg_query_value_ex;
pub use reg_query_value_ex::*;

mod reg_query_value;
pub use reg_query_value::*;

mod reg_query_multiple_values;
pub use reg_query_multiple_values::*;

mod reg_query_info_key;
pub use reg_query_info_key::*;

mod reg_open_key_transacted;
pub use reg_open_key_transacted::*;

mod reg_open_key_ex;
pub use reg_open_key_ex::*;

mod reg_open_key;
pub use reg_open_key::*;

mod reg_load_mui_string;
pub use reg_load_mui_string::*;

mod reg_load_key;
pub use reg_load_key::*;

mod reg_load_app_key;
pub use reg_load_app_key::*;

mod reg_get_value;
pub use reg_get_value::*;

mod reg_enum_value;
pub use reg_enum_value::*;

mod reg_enum_key_ex;
pub use reg_enum_key_ex::*;

mod reg_enum_key;
pub use reg_enum_key::*;

mod reg_delete_value;
pub use reg_delete_value::*;

mod reg_delete_tree;
pub use reg_delete_tree::*;

mod reg_delete_key_value;
pub use reg_delete_key_value::*;

mod reg_delete_key_transacted;
pub use reg_delete_key_transacted::*;

mod reg_delete_key_ex;
pub use reg_delete_key_ex::*;

mod reg_delete_key;
pub use reg_delete_key::*;

mod reg_create_key_transacted;
pub use reg_create_key_transacted::*;

mod reg_create_key_ex;
pub use reg_create_key_ex::*;

mod reg_create_key;
pub use reg_create_key::*;

mod reg_copy_tree;
pub use reg_copy_tree::*;

mod reg_connect_registry_ex;
pub use reg_connect_registry_ex::*;

mod reg_connect_registry;
pub use reg_connect_registry::*;

mod read_event_log;
pub use read_event_log::*;

mod query_trace;
pub use query_trace::*;

mod query_service_lock_status;
pub use query_service_lock_status::*;

mod query_service_config;
pub use query_service_config::*;

mod query_service_config2;
pub use query_service_config2::*;

mod query_all_traces;
pub use query_all_traces::*;

mod privileged_service_audit_alarm;
pub use privileged_service_audit_alarm::*;

mod open_trace;
pub use open_trace::*;

mod open_service;
pub use open_service::*;

mod open_sc_manager;
pub use open_sc_manager::*;

mod open_event_log;
pub use open_event_log::*;

mod open_encrypted_file_raw;
pub use open_encrypted_file_raw::*;

mod open_backup_event_log;
pub use open_backup_event_log::*;

mod object_privilege_audit_alarm;
pub use object_privilege_audit_alarm::*;

mod object_open_audit_alarm;
pub use object_open_audit_alarm::*;

mod object_delete_audit_alarm;
pub use object_delete_audit_alarm::*;

mod object_close_audit_alarm;
pub use object_close_audit_alarm::*;

mod lookup_security_descriptor_parts;
pub use lookup_security_descriptor_parts::*;

mod lookup_privilege_name;
pub use lookup_privilege_name::*;

mod lookup_privilege_display_name;
pub use lookup_privilege_display_name::*;

mod lookup_account_sid;
pub use lookup_account_sid::*;

mod lookup_account_name;
pub use lookup_account_name::*;

mod logon_user_ex;
pub use logon_user_ex::*;

mod logon_user;
pub use logon_user::*;

mod initiate_system_shutdown_ex;
pub use initiate_system_shutdown_ex::*;

mod initiate_system_shutdown;
pub use initiate_system_shutdown::*;

mod initiate_shutdown;
pub use initiate_shutdown::*;

mod i_sc_set_service_bits;
pub use i_sc_set_service_bits::*;

mod get_trustee_type;
pub use get_trustee_type::*;

mod get_trustee_name;
pub use get_trustee_name::*;

mod get_trustee_form;
pub use get_trustee_form::*;

mod get_service_key_name;
pub use get_service_key_name::*;

mod get_service_display_name;
pub use get_service_display_name::*;

mod get_security_info_ex;
pub use get_security_info_ex::*;

mod get_named_security_info_ex;
pub use get_named_security_info_ex::*;

mod get_named_security_info;
pub use get_named_security_info::*;

mod get_multiple_trustee_operation;
pub use get_multiple_trustee_operation::*;

mod get_multiple_trustee;
pub use get_multiple_trustee::*;

mod get_inheritance_source;
pub use get_inheritance_source::*;

mod get_file_security;
pub use get_file_security::*;

mod get_explicit_entries_from_acl;
pub use get_explicit_entries_from_acl::*;

mod get_effective_rights_from_acl;
pub use get_effective_rights_from_acl::*;

mod get_current_hw_profile;
pub use get_current_hw_profile::*;

mod get_audited_permissions_from_acl;
pub use get_audited_permissions_from_acl::*;

mod get_access_permissions_for_object;
pub use get_access_permissions_for_object::*;

mod flush_trace;
pub use flush_trace::*;

mod file_encryption_status;
pub use file_encryption_status::*;

mod enum_services_status_ex;
pub use enum_services_status_ex::*;

mod enum_services_status;
pub use enum_services_status::*;

mod enum_dependent_services;
pub use enum_dependent_services::*;

mod encrypt_file;
pub use encrypt_file::*;

mod decrypt_file;
pub use decrypt_file::*;

mod crypt_verify_signature;
pub use crypt_verify_signature::*;

mod crypt_set_provider_ex;
pub use crypt_set_provider_ex::*;

mod crypt_set_provider;
pub use crypt_set_provider::*;

mod crypt_get_default_provider;
pub use crypt_get_default_provider::*;

mod crypt_enum_providers;
pub use crypt_enum_providers::*;

mod crypt_enum_provider_types;
pub use crypt_enum_provider_types::*;

mod crypt_acquire_context;
pub use crypt_acquire_context::*;

mod cred_write_domain_credentials;
pub use cred_write_domain_credentials::*;

mod cred_write;
pub use cred_write::*;

mod cred_unprotect;
pub use cred_unprotect::*;

mod cred_unmarshal_credential;
pub use cred_unmarshal_credential::*;

mod cred_rename;
pub use cred_rename::*;

mod cred_read_domain_credentials;
pub use cred_read_domain_credentials::*;

mod cred_read;
pub use cred_read::*;

mod cred_protect;
pub use cred_protect::*;

mod cred_marshal_credential;
pub use cred_marshal_credential::*;

mod cred_is_protected;
pub use cred_is_protected::*;

mod cred_is_marshaled_credential;
pub use cred_is_marshaled_credential::*;

mod cred_get_target_info;
pub use cred_get_target_info::*;

mod cred_find_best_credential;
pub use cred_find_best_credential::*;

mod cred_enumerate;
pub use cred_enumerate::*;

mod cred_delete;
pub use cred_delete::*;

mod create_service;
pub use create_service::*;

mod create_process_as_user;
pub use create_process_as_user::*;

mod convert_string_sid_to_sid;
pub use convert_string_sid_to_sid::*;

mod convert_string_security_descriptor_to_security_descriptor;
pub use convert_string_security_descriptor_to_security_descriptor::*;

mod convert_string_sd_to_sd_root_domain;
pub use convert_string_sd_to_sd_root_domain::*;

mod convert_string_sd_to_sd_domain;
pub use convert_string_sd_to_sd_domain::*;

mod convert_sid_to_string_sid;
pub use convert_sid_to_string_sid::*;

mod convert_security_descriptor_to_string_security_descriptor;
pub use convert_security_descriptor_to_string_security_descriptor::*;

mod convert_security_descriptor_to_access_named;
pub use convert_security_descriptor_to_access_named::*;

mod convert_security_descriptor_to_access;
pub use convert_security_descriptor_to_access::*;

mod convert_sd_to_string_sd_root_domain;
pub use convert_sd_to_string_sd_root_domain::*;

mod convert_access_to_security_descriptor;
pub use convert_access_to_security_descriptor::*;

mod control_trace;
pub use control_trace::*;

mod control_service_ex;
pub use control_service_ex::*;

mod clear_event_log;
pub use clear_event_log::*;

mod change_service_config;
pub use change_service_config::*;

mod change_service_config2;
pub use change_service_config2::*;

mod build_trustee_with_sid;
pub use build_trustee_with_sid::*;

mod build_trustee_with_objects_and_sid;
pub use build_trustee_with_objects_and_sid::*;

mod build_trustee_with_objects_and_name;
pub use build_trustee_with_objects_and_name::*;

mod build_trustee_with_name;
pub use build_trustee_with_name::*;

mod build_security_descriptor;
pub use build_security_descriptor::*;

mod build_impersonate_trustee;
pub use build_impersonate_trustee::*;

mod build_impersonate_explicit_access_with_name;
pub use build_impersonate_explicit_access_with_name::*;

mod build_explicit_access_with_name;
pub use build_explicit_access_with_name::*;

mod backup_event_log;
pub use backup_event_log::*;

mod audit_set_global_sacl;
pub use audit_set_global_sacl::*;

mod audit_query_global_sacl;
pub use audit_query_global_sacl::*;

mod audit_lookup_sub_category_name;
pub use audit_lookup_sub_category_name::*;

mod audit_lookup_category_name;
pub use audit_lookup_category_name::*;

mod access_check_by_type_result_list_and_audit_alarm_by_handle;
pub use access_check_by_type_result_list_and_audit_alarm_by_handle::*;

mod access_check_by_type_result_list_and_audit_alarm;
pub use access_check_by_type_result_list_and_audit_alarm::*;

mod access_check_by_type_and_audit_alarm;
pub use access_check_by_type_and_audit_alarm::*;

mod access_check_and_audit_alarm;
pub use access_check_and_audit_alarm::*;

mod abort_system_shutdown;
pub use abort_system_shutdown::*;

mod a_sha_final;
mod accounts;
mod crypto;
mod registry_api;
mod security;
mod services;
mod system;
mod tokens;
mod tracing;
pub use a_sha_final::A_SHAFinal;
mod a_sha_init;
pub use a_sha_init::A_SHAInit;
mod a_sha_update;
pub use a_sha_update::A_SHAUpdate;
mod access_check;
pub use access_check::AccessCheck;
mod access_check_by_type;
pub use access_check_by_type::AccessCheckByType;
mod access_check_by_type_result_list;
pub use access_check_by_type_result_list::AccessCheckByTypeResultList;
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
mod audit_query_per_user_policy;
pub use audit_query_per_user_policy::AuditQueryPerUserPolicy;
mod audit_query_security;
pub use audit_query_security::AuditQuerySecurity;
mod audit_query_system_policy;
pub use audit_query_system_policy::AuditQuerySystemPolicy;
mod audit_set_per_user_policy;
pub use audit_set_per_user_policy::AuditSetPerUserPolicy;
mod audit_set_security;
pub use audit_set_security::AuditSetSecurity;
mod audit_set_system_policy;
pub use audit_set_system_policy::AuditSetSystemPolicy;
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
mod cancel_overlapped_access;
pub use cancel_overlapped_access::CancelOverlappedAccess;
mod check_for_hiberboot;
pub use check_for_hiberboot::CheckForHiberboot;
mod check_token_membership;
pub use check_token_membership::CheckTokenMembership;
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
mod convert_sd_to_string_sd_domain_w;
pub use convert_sd_to_string_sd_domain_w::ConvertSDToStringSDDomainW;
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
mod create_process_with_logon_w;
pub use create_process_with_logon_w::CreateProcessWithLogonW;
mod create_process_with_token_w;
pub use create_process_with_token_w::CreateProcessWithTokenW;
mod create_restricted_token;
pub use create_restricted_token::CreateRestrictedToken;
mod create_service_ex;
pub use create_service_ex::CreateServiceEx;
mod create_trace_instance_id;
pub use create_trace_instance_id::CreateTraceInstanceId;
mod create_well_known_sid;
pub use create_well_known_sid::CreateWellKnownSid;
mod cred_backup_credentials;
pub use cred_backup_credentials::CredBackupCredentials;
mod cred_encrypt_and_marshal_binary_blob;
pub use cred_encrypt_and_marshal_binary_blob::CredEncryptAndMarshalBinaryBlob;
mod cred_free;
pub use cred_free::CredFree;
mod cred_get_session_types;
pub use cred_get_session_types::CredGetSessionTypes;
mod cred_profile_loaded;
pub use cred_profile_loaded::CredProfileLoaded;
mod cred_profile_loaded_ex;
pub use cred_profile_loaded_ex::CredProfileLoadedEx;
mod cred_profile_unloaded;
pub use cred_profile_unloaded::CredProfileUnloaded;
mod cred_read_by_token_handle;
pub use cred_read_by_token_handle::CredReadByTokenHandle;
mod cred_restore_credentials;
pub use cred_restore_credentials::CredRestoreCredentials;
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
mod crypt_export_key;
pub use crypt_export_key::CryptExportKey;
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
mod cve_event_write;
pub use cve_event_write::CveEventWrite;
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
mod encrypted_file_key_info;
pub use encrypted_file_key_info::EncryptedFileKeyInfo;
mod encryption_disable;
pub use encryption_disable::EncryptionDisable;
mod enum_dynamic_time_zone_information;
pub use enum_dynamic_time_zone_information::EnumDynamicTimeZoneInformation;
mod enum_service_group_w;
pub use enum_service_group_w::EnumServiceGroupW;
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
mod find_first_free_ace;
pub use find_first_free_ace::FindFirstFreeAce;
mod flush_efs_cache;
pub use flush_efs_cache::FlushEfsCache;
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
mod get_ace;
pub use get_ace::GetAce;
mod get_acl_information;
pub use get_acl_information::GetAclInformation;
mod get_dynamic_time_zone_information_effective_years;
pub use get_dynamic_time_zone_information_effective_years::GetDynamicTimeZoneInformationEffectiveYears;
mod get_encrypted_file_metadata;
pub use get_encrypted_file_metadata::GetEncryptedFileMetadata;
mod get_event_log_information;
pub use get_event_log_information::GetEventLogInformation;
mod get_information_code_authz_level_w;
pub use get_information_code_authz_level_w::GetInformationCodeAuthzLevelW;
mod get_information_code_authz_policy_w;
pub use get_information_code_authz_policy_w::GetInformationCodeAuthzPolicyW;
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
mod logon_user_ex_ex_w;
pub use logon_user_ex_ex_w::LogonUserExExW;
mod lookup_privilege_value;
pub use lookup_privilege_value::LookupPrivilegeValueA;
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
pub use notify_service_status_change::NotifyServiceStatusChangeA;
pub use notify_service_status_change::NotifyServiceStatusChangeW;
mod np_get_user_name;
pub use np_get_user_name::NpGetUserName;
mod open_process_token;
pub use open_process_token::OpenProcessToken;
mod open_thread_token;
pub use open_thread_token::OpenThreadToken;
mod open_thread_wait_chain_session;
pub use open_thread_wait_chain_session::OpenThreadWaitChainSession;
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
mod process_idle_tasks;
pub use process_idle_tasks::*;
mod process_trace;
pub use process_trace::ProcessTrace;
mod query_local_user_service_name;
pub use query_local_user_service_name::QueryLocalUserServiceName;
mod query_recovery_agents_on_encrypted_file;
pub use query_recovery_agents_on_encrypted_file::QueryRecoveryAgentsOnEncryptedFile;
mod query_security_access_mask;
pub use query_security_access_mask::QuerySecurityAccessMask;
mod query_service_dynamic_information;
pub use query_service_dynamic_information::QueryServiceDynamicInformation;
mod query_service_object_security;
pub use query_service_object_security::QueryServiceObjectSecurity;
mod query_service_status;
pub use query_service_status::QueryServiceStatus;
mod query_service_status_ex;
pub use query_service_status_ex::QueryServiceStatusEx;
mod query_trace_processing_handle;
pub use query_trace_processing_handle::QueryTraceProcessingHandle;
mod query_user_service_name;
pub use query_user_service_name::QueryUserServiceName;
mod query_user_service_name_for_context;
pub use query_user_service_name_for_context::QueryUserServiceNameForContext;
mod query_users_on_encrypted_file;
pub use query_users_on_encrypted_file::QueryUsersOnEncryptedFile;
mod read_encrypted_file_raw;
pub use read_encrypted_file_raw::ReadEncryptedFileRaw;
mod reg_close_key;
pub use reg_close_key::RegCloseKey;
mod reg_disable_predefined_cache;
pub use reg_disable_predefined_cache::RegDisablePredefinedCache;
mod reg_disable_predefined_cache_ex;
pub use reg_disable_predefined_cache_ex::RegDisablePredefinedCacheEx;
mod reg_disable_reflection_key;
pub use reg_disable_reflection_key::RegDisableReflectionKey;
mod reg_enable_reflection_key;
pub use reg_enable_reflection_key::RegEnableReflectionKey;
mod reg_flush_key;
pub use reg_flush_key::RegFlushKey;
mod reg_get_key_security;
pub use reg_get_key_security::RegGetKeySecurity;
mod reg_notify_change_key_value;
pub use reg_notify_change_key_value::RegNotifyChangeKeyValue;
mod reg_open_current_user;
pub use reg_open_current_user::RegOpenCurrentUser;
mod reg_open_user_classes_root;
pub use reg_open_user_classes_root::RegOpenUserClassesRoot;
mod reg_override_predef_key;
pub use reg_override_predef_key::RegOverridePredefKey;
mod reg_query_reflection_key;
pub use reg_query_reflection_key::RegQueryReflectionKey;
mod reg_rename_key;
pub use reg_rename_key::RegRenameKey;
mod reg_set_key_security;
pub use reg_set_key_security::RegSetKeySecurity;
mod register_idle_task;
pub use register_idle_task::RegisterIdleTask;
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
mod set_information_code_authz_level_w;
pub use set_information_code_authz_level_w::SetInformationCodeAuthzLevelW;
mod set_information_code_authz_policy_w;
pub use set_information_code_authz_policy_w::SetInformationCodeAuthzPolicyW;
mod set_kernel_object_security;
pub use set_kernel_object_security::SetKernelObjectSecurity;
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
mod uninstall_application;
pub use uninstall_application::UninstallApplication;
mod unlock_service_database;
pub use unlock_service_database::UnlockServiceDatabase;
mod unregister_idle_task;
pub use unregister_idle_task::UnregisterIdleTask;
mod unregister_trace_guids;
pub use unregister_trace_guids::UnregisterTraceGuids;
mod wait_service_state;
pub use wait_service_state::WaitServiceState;
mod wmi_close_block;
pub use wmi_close_block::WmiCloseBlock;
mod wmi_enumerate_guids;
pub use wmi_enumerate_guids::WmiEnumerateGuids;
mod wmi_free_buffer;
pub use wmi_free_buffer::WmiFreeBuffer;
mod wmi_open_block;
pub use wmi_open_block::WmiOpenBlock;
mod wmi_query_guid_information;
pub use wmi_query_guid_information::WmiQueryGuidInformation;
mod write_encrypted_file_raw;
pub use write_encrypted_file_raw::WriteEncryptedFileRaw;
mod adjust_token_privileges;
pub use adjust_token_privileges::AdjustTokenPrivileges;
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
mod crypt_sign_hash;
use crypt_sign_hash::CryptSignHashA;
use crypt_sign_hash::CryptSignHashW;
mod get_user_name;
use get_user_name::GetUserNameA;
use get_user_name::GetUserNameW;
use lookup_privilege_value::LookupPrivilegeValueW;
mod start_service_ctrl_dispatcher;
use start_service_ctrl_dispatcher::StartServiceCtrlDispatcherA;
use start_service_ctrl_dispatcher::StartServiceCtrlDispatcherW;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if winapi64::registry::gateway(api, emu) {
        return String::new();
    }
    if accounts::dispatch(api, addr, emu)
        || crypto::dispatch(api, addr, emu)
        || registry_api::dispatch(api, addr, emu)
        || security::dispatch(api, addr, emu)
        || services::dispatch(api, addr, emu)
        || system::dispatch(api, addr, emu)
        || tokens::dispatch(api, addr, emu)
        || tracing::dispatch(api, addr, emu)
    {
        return String::new();
    }

    fallback::unimplemented(addr, api, emu)
}
