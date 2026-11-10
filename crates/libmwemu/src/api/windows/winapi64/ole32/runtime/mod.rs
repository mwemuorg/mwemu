use crate::emu::Emu;
use crate::winapi::winapi64::ole32;

#[path = "../co_initialize.rs"]
mod co_initialize;
pub use co_initialize::CoInitialize;

#[path = "../co_resume_class_objects.rs"]
mod co_resume_class_objects;
pub use co_resume_class_objects::CoResumeClassObjects;

#[path = "../co_add_ref_server_process.rs"]
mod co_add_ref_server_process;
pub use co_add_ref_server_process::CoAddRefServerProcess;

#[path = "../co_release_server_process.rs"]
mod co_release_server_process;
pub use co_release_server_process::CoReleaseServerProcess;

#[path = "../clsid_from_ole1_class.rs"]
mod clsid_from_ole1_class;
pub use clsid_from_ole1_class::CLSIDFromOle1Class;

#[path = "../clsid_from_prog_id.rs"]
mod clsid_from_prog_id;
pub use clsid_from_prog_id::CLSIDFromProgID;

#[path = "../clsid_from_prog_id_ex.rs"]
mod clsid_from_prog_id_ex;
pub use clsid_from_prog_id_ex::CLSIDFromProgIDEx;

#[path = "../co_aic_get_token_for_com.rs"]
mod co_aic_get_token_for_com;
pub use co_aic_get_token_for_com::CoAicGetTokenForCOM;

#[path = "../co_allow_set_foreground_window.rs"]
mod co_allow_set_foreground_window;
pub use co_allow_set_foreground_window::CoAllowSetForegroundWindow;

#[path = "../co_allow_unmarshaler_clsid.rs"]
mod co_allow_unmarshaler_clsid;
pub use co_allow_unmarshaler_clsid::CoAllowUnmarshalerCLSID;

#[path = "../co_build_version.rs"]
mod co_build_version;
pub use co_build_version::CoBuildVersion;

#[path = "../co_cancel_call.rs"]
mod co_cancel_call;
pub use co_cancel_call::CoCancelCall;

#[path = "../co_check_elevation_enabled.rs"]
mod co_check_elevation_enabled;
pub use co_check_elevation_enabled::CoCheckElevationEnabled;

#[path = "../co_create_instance_ex.rs"]
mod co_create_instance_ex;
pub use co_create_instance_ex::CoCreateInstanceEx;

#[path = "../co_create_instance_from_app.rs"]
mod co_create_instance_from_app;
pub use co_create_instance_from_app::CoCreateInstanceFromApp;

#[path = "../co_create_object_in_context.rs"]
mod co_create_object_in_context;
pub use co_create_object_in_context::CoCreateObjectInContext;

#[path = "../co_deactivate_object.rs"]
mod co_deactivate_object;
pub use co_deactivate_object::CoDeactivateObject;

#[path = "../co_decrement_mta_usage.rs"]
mod co_decrement_mta_usage;
pub use co_decrement_mta_usage::CoDecrementMTAUsage;

#[path = "../co_disable_call_cancellation.rs"]
mod co_disable_call_cancellation;
pub use co_disable_call_cancellation::CoDisableCallCancellation;

#[path = "../co_disconnect_context.rs"]
mod co_disconnect_context;
pub use co_disconnect_context::CoDisconnectContext;

#[path = "../co_disconnect_object.rs"]
mod co_disconnect_object;
pub use co_disconnect_object::CoDisconnectObject;

#[path = "../co_dos_date_time_to_file_time.rs"]
mod co_dos_date_time_to_file_time;
pub use co_dos_date_time_to_file_time::CoDosDateTimeToFileTime;

#[path = "../co_enable_call_cancellation.rs"]
mod co_enable_call_cancellation;
pub use co_enable_call_cancellation::CoEnableCallCancellation;

#[path = "../co_file_time_now.rs"]
mod co_file_time_now;
pub use co_file_time_now::CoFileTimeNow;

#[path = "../co_file_time_to_dos_date_time.rs"]
mod co_file_time_to_dos_date_time;
pub use co_file_time_to_dos_date_time::CoFileTimeToDosDateTime;

#[path = "../co_free_all_libraries.rs"]
mod co_free_all_libraries;
pub use co_free_all_libraries::CoFreeAllLibraries;

#[path = "../co_free_library.rs"]
mod co_free_library;
pub use co_free_library::CoFreeLibrary;

#[path = "../co_free_unused_libraries.rs"]
mod co_free_unused_libraries;
pub use co_free_unused_libraries::CoFreeUnusedLibraries;

#[path = "../co_free_unused_libraries_ex.rs"]
mod co_free_unused_libraries_ex;
pub use co_free_unused_libraries_ex::CoFreeUnusedLibrariesEx;

#[path = "../co_get_activation_state.rs"]
mod co_get_activation_state;
pub use co_get_activation_state::CoGetActivationState;

#[path = "../co_get_apartment_id.rs"]
mod co_get_apartment_id;
pub use co_get_apartment_id::CoGetApartmentID;

#[path = "../co_get_apartment_type.rs"]
mod co_get_apartment_type;
pub use co_get_apartment_type::CoGetApartmentType;

#[path = "../co_get_call_context.rs"]
mod co_get_call_context;
pub use co_get_call_context::CoGetCallContext;

#[path = "../co_get_call_state.rs"]
mod co_get_call_state;
pub use co_get_call_state::CoGetCallState;

#[path = "../co_get_caller_tid.rs"]
mod co_get_caller_tid;
pub use co_get_caller_tid::CoGetCallerTID;

#[path = "../co_get_cancel_object.rs"]
mod co_get_cancel_object;
pub use co_get_cancel_object::CoGetCancelObject;

#[path = "../co_get_class_object.rs"]
mod co_get_class_object;
pub use co_get_class_object::CoGetClassObject;

#[path = "../co_get_class_version.rs"]
mod co_get_class_version;
pub use co_get_class_version::CoGetClassVersion;

#[path = "../co_get_com_catalog.rs"]
mod co_get_com_catalog;
pub use co_get_com_catalog::CoGetComCatalog;

#[path = "../co_get_context_token.rs"]
mod co_get_context_token;
pub use co_get_context_token::CoGetContextToken;

#[path = "../co_get_current_logical_thread_id.rs"]
mod co_get_current_logical_thread_id;
pub use co_get_current_logical_thread_id::CoGetCurrentLogicalThreadId;

#[path = "../co_get_current_process.rs"]
mod co_get_current_process;
pub use co_get_current_process::CoGetCurrentProcess;

#[path = "../co_get_default_context.rs"]
mod co_get_default_context;
pub use co_get_default_context::CoGetDefaultContext;

#[path = "../co_get_instance_from_file.rs"]
mod co_get_instance_from_file;
pub use co_get_instance_from_file::CoGetInstanceFromFile;

#[path = "../co_get_interceptor.rs"]
mod co_get_interceptor;
pub use co_get_interceptor::CoGetInterceptor;

#[path = "../co_get_interceptor_for_ole32.rs"]
mod co_get_interceptor_for_ole32;
pub use co_get_interceptor_for_ole32::CoGetInterceptorForOle32;

#[path = "../co_get_interceptor_from_type_info.rs"]
mod co_get_interceptor_from_type_info;
pub use co_get_interceptor_from_type_info::CoGetInterceptorFromTypeInfo;

#[path = "../co_get_malloc.rs"]
mod co_get_malloc;
pub use co_get_malloc::CoGetMalloc;

#[path = "../co_get_module_type.rs"]
mod co_get_module_type;
pub use co_get_module_type::CoGetModuleType;

#[path = "../co_get_object.rs"]
mod co_get_object;
pub use co_get_object::CoGetObject;

#[path = "../co_get_object_context.rs"]
mod co_get_object_context;
pub use co_get_object_context::CoGetObjectContext;

#[path = "../co_get_ps_clsid.rs"]
mod co_get_ps_clsid;
pub use co_get_ps_clsid::CoGetPSClsid;

#[path = "../co_get_process_identifier.rs"]
mod co_get_process_identifier;
pub use co_get_process_identifier::CoGetProcessIdentifier;

#[path = "../co_get_system_security_permissions.rs"]
mod co_get_system_security_permissions;
pub use co_get_system_security_permissions::CoGetSystemSecurityPermissions;

#[path = "../co_get_system_wow64_directory_w.rs"]
mod co_get_system_wow64_directory_w;
pub use co_get_system_wow64_directory_w::CoGetSystemWow64DirectoryW;

#[path = "../co_get_treat_as_class.rs"]
mod co_get_treat_as_class;
pub use co_get_treat_as_class::CoGetTreatAsClass;

#[path = "../co_handle_priority_events_from_message_pump.rs"]
mod co_handle_priority_events_from_message_pump;
pub use co_handle_priority_events_from_message_pump::CoHandlePriorityEventsFromMessagePump;

#[path = "../co_impersonate_client.rs"]
mod co_impersonate_client;
pub use co_impersonate_client::CoImpersonateClient;

#[path = "../co_increment_mta_usage.rs"]
mod co_increment_mta_usage;
pub use co_increment_mta_usage::CoIncrementMTAUsage;

#[path = "../co_initialize_security.rs"]
mod co_initialize_security;
pub use co_initialize_security::CoInitializeSecurity;

#[path = "../co_initialize_wow.rs"]
mod co_initialize_wow;
pub use co_initialize_wow::CoInitializeWOW;

#[path = "../co_install.rs"]
mod co_install;
pub use co_install::CoInstall;

#[path = "../co_invalidate_remote_machine_bindings.rs"]
mod co_invalidate_remote_machine_bindings;
pub use co_invalidate_remote_machine_bindings::CoInvalidateRemoteMachineBindings;

#[path = "../co_is_handler_connected.rs"]
mod co_is_handler_connected;
pub use co_is_handler_connected::CoIsHandlerConnected;

#[path = "../co_is_ole1_class.rs"]
mod co_is_ole1_class;
pub use co_is_ole1_class::CoIsOle1Class;

#[path = "../co_load_library.rs"]
mod co_load_library;
pub use co_load_library::CoLoadLibrary;

#[path = "../co_lock_object_external.rs"]
mod co_lock_object_external;
pub use co_lock_object_external::CoLockObjectExternal;

#[path = "../co_pop_service_domain.rs"]
mod co_pop_service_domain;
pub use co_pop_service_domain::CoPopServiceDomain;

#[path = "../co_push_service_domain.rs"]
mod co_push_service_domain;
pub use co_push_service_domain::CoPushServiceDomain;

#[path = "../co_query_authentication_services.rs"]
mod co_query_authentication_services;
pub use co_query_authentication_services::CoQueryAuthenticationServices;

#[path = "../co_query_client_blanket.rs"]
mod co_query_client_blanket;
pub use co_query_client_blanket::CoQueryClientBlanket;

#[path = "../co_query_release_object.rs"]
mod co_query_release_object;
pub use co_query_release_object::CoQueryReleaseObject;

#[path = "../co_reactivate_object.rs"]
mod co_reactivate_object;
pub use co_reactivate_object::CoReactivateObject;

#[path = "../co_register_activation_filter.rs"]
mod co_register_activation_filter;
pub use co_register_activation_filter::CoRegisterActivationFilter;

#[path = "../co_register_class_object.rs"]
mod co_register_class_object;
pub use co_register_class_object::CoRegisterClassObject;

#[path = "../co_register_initialize_spy.rs"]
mod co_register_initialize_spy;
pub use co_register_initialize_spy::CoRegisterInitializeSpy;

#[path = "../co_register_malloc_spy.rs"]
mod co_register_malloc_spy;
pub use co_register_malloc_spy::CoRegisterMallocSpy;

#[path = "../co_register_message_filter.rs"]
mod co_register_message_filter;
pub use co_register_message_filter::CoRegisterMessageFilter;

#[path = "../co_register_ps_clsid.rs"]
mod co_register_ps_clsid;
pub use co_register_ps_clsid::CoRegisterPSClsid;

#[path = "../co_register_surrogate.rs"]
mod co_register_surrogate;
pub use co_register_surrogate::CoRegisterSurrogate;

#[path = "../co_register_surrogate_ex.rs"]
mod co_register_surrogate_ex;
pub use co_register_surrogate_ex::CoRegisterSurrogateEx;

#[path = "../co_retire_server.rs"]
mod co_retire_server;
pub use co_retire_server::CoRetireServer;

#[path = "../co_revert_to_self.rs"]
mod co_revert_to_self;
pub use co_revert_to_self::CoRevertToSelf;

#[path = "../co_revoke_class_object.rs"]
mod co_revoke_class_object;
pub use co_revoke_class_object::CoRevokeClassObject;

#[path = "../co_revoke_initialize_spy.rs"]
mod co_revoke_initialize_spy;
pub use co_revoke_initialize_spy::CoRevokeInitializeSpy;

#[path = "../co_revoke_malloc_spy.rs"]
mod co_revoke_malloc_spy;
pub use co_revoke_malloc_spy::CoRevokeMallocSpy;

#[path = "../co_set_cancel_object.rs"]
mod co_set_cancel_object;
pub use co_set_cancel_object::CoSetCancelObject;

#[path = "../co_set_message_dispatcher.rs"]
mod co_set_message_dispatcher;
pub use co_set_message_dispatcher::CoSetMessageDispatcher;

#[path = "../co_set_state.rs"]
mod co_set_state;
pub use co_set_state::CoSetState;

#[path = "../co_suspend_class_objects.rs"]
mod co_suspend_class_objects;
pub use co_suspend_class_objects::CoSuspendClassObjects;

#[path = "../co_switch_call_context.rs"]
mod co_switch_call_context;
pub use co_switch_call_context::CoSwitchCallContext;

#[path = "../co_task_mem_realloc.rs"]
mod co_task_mem_realloc;
pub use co_task_mem_realloc::CoTaskMemRealloc;

#[path = "../co_test_cancel.rs"]
mod co_test_cancel;
pub use co_test_cancel::CoTestCancel;

#[path = "../co_treat_as_class.rs"]
mod co_treat_as_class;
pub use co_treat_as_class::CoTreatAsClass;

#[path = "../co_try_get_interceptor_for_app_model_virtualization.rs"]
mod co_try_get_interceptor_for_app_model_virtualization;
pub use co_try_get_interceptor_for_app_model_virtualization::CoTryGetInterceptorForAppModelVirtualization;

#[path = "../co_unloading_wow.rs"]
mod co_unloading_wow;
pub use co_unloading_wow::CoUnloadingWOW;

#[path = "../co_unmarshal_hresult.rs"]
mod co_unmarshal_hresult;
pub use co_unmarshal_hresult::CoUnmarshalHresult;

#[path = "../co_unmarshal_interface.rs"]
mod co_unmarshal_interface;
pub use co_unmarshal_interface::CoUnmarshalInterface;

#[path = "../co_vrf_check_thread_state.rs"]
mod co_vrf_check_thread_state;
pub use co_vrf_check_thread_state::CoVrfCheckThreadState;

#[path = "../co_vrf_get_thread_state.rs"]
mod co_vrf_get_thread_state;
pub use co_vrf_get_thread_state::CoVrfGetThreadState;

#[path = "../co_vrf_release_thread_state.rs"]
mod co_vrf_release_thread_state;
pub use co_vrf_release_thread_state::CoVrfReleaseThreadState;

#[path = "../co_wait_for_multiple_handles.rs"]
mod co_wait_for_multiple_handles;
pub use co_wait_for_multiple_handles::CoWaitForMultipleHandles;

#[path = "../co_wait_for_multiple_objects.rs"]
mod co_wait_for_multiple_objects;
pub use co_wait_for_multiple_objects::CoWaitForMultipleObjects;

#[path = "../create_error_info.rs"]
mod create_error_info;
pub use create_error_info::CreateErrorInfo;

#[path = "../create_std_progress_indicator.rs"]
mod create_std_progress_indicator;
pub use create_std_progress_indicator::CreateStdProgressIndicator;

#[path = "../delete_pattern_and_extension_tables.rs"]
mod delete_pattern_and_extension_tables;
pub use delete_pattern_and_extension_tables::DeletePatternAndExtensionTables;

#[path = "../dll_get_class_object.rs"]
mod dll_get_class_object;
pub use dll_get_class_object::DllGetClassObject;

#[path = "../dll_get_class_object_wow.rs"]
mod dll_get_class_object_wow;
pub use dll_get_class_object_wow::DllGetClassObjectWOW;

#[path = "../dll_register_server.rs"]
mod dll_register_server;
pub use dll_register_server::DllRegisterServer;

#[path = "../enable_hook_object.rs"]
mod enable_hook_object;
pub use enable_hook_object::EnableHookObject;

#[path = "../find_ext.rs"]
mod find_ext;
pub use find_ext::FindExt;

#[path = "../get_active_object_ext.rs"]
mod get_active_object_ext;
pub use get_active_object_ext::GetActiveObjectExt;

#[path = "../get_class_file.rs"]
mod get_class_file;
pub use get_class_file::GetClassFile;

#[path = "../get_error_info.rs"]
mod get_error_info;
pub use get_error_info::GetErrorInfo;

#[path = "../get_hook_interface.rs"]
mod get_hook_interface;
pub use get_hook_interface::GetHookInterface;

#[path = "../iid_from_string.rs"]
mod iid_from_string;
pub use iid_from_string::IIDFromString;

#[path = "../is_equal_guid.rs"]
mod is_equal_guid;
pub use is_equal_guid::IsEqualGUID;

#[path = "../is_ro_initialize_asta_allowed_in_desktop.rs"]
mod is_ro_initialize_asta_allowed_in_desktop;
pub use is_ro_initialize_asta_allowed_in_desktop::IsRoInitializeASTAAllowedInDesktop;

#[path = "../is_valid_iid.rs"]
mod is_valid_iid;
pub use is_valid_iid::IsValidIid;

#[path = "../is_valid_interface.rs"]
mod is_valid_interface;
pub use is_valid_interface::IsValidInterface;

#[path = "../is_valid_ptr_in.rs"]
mod is_valid_ptr_in;
pub use is_valid_ptr_in::IsValidPtrIn;

#[path = "../is_valid_ptr_out.rs"]
mod is_valid_ptr_out;
pub use is_valid_ptr_out::IsValidPtrOut;

#[path = "../mk_parse_display_name.rs"]
mod mk_parse_display_name;
pub use mk_parse_display_name::MkParseDisplayName;

#[path = "../prog_id_from_clsid.rs"]
mod prog_id_from_clsid;
pub use prog_id_from_clsid::ProgIDFromCLSID;

#[path = "../register_active_object_ext.rs"]
mod register_active_object_ext;
pub use register_active_object_ext::RegisterActiveObjectExt;

#[path = "../revoke_active_object_ext.rs"]
mod revoke_active_object_ext;
pub use revoke_active_object_ext::RevokeActiveObjectExt;

#[path = "../ro_get_agile_reference.rs"]
mod ro_get_agile_reference;
pub use ro_get_agile_reference::RoGetAgileReference;

#[path = "../set_error_info.rs"]
mod set_error_info;
pub use set_error_info::SetErrorInfo;

#[path = "../set_oleaut_module.rs"]
mod set_oleaut_module;
pub use set_oleaut_module::SetOleautModule;

#[path = "../set_wow_thunk_global_ptr.rs"]
mod set_wow_thunk_global_ptr;
pub use set_wow_thunk_global_ptr::SetWOWThunkGlobalPtr;

#[path = "../std_types_get_class_object.rs"]
mod std_types_get_class_object;
pub use std_types_get_class_object::StdTypesGetClassObject;

#[path = "../std_types_register_server.rs"]
mod std_types_register_server;
pub use std_types_register_server::StdTypesRegisterServer;

#[path = "../string_from_iid.rs"]
mod string_from_iid;
pub use string_from_iid::StringFromIID;

#[path = "../update_dcom_settings.rs"]
mod update_dcom_settings;
pub use update_dcom_settings::UpdateDCOMSettings;

#[path = "../update_process_tracing.rs"]
mod update_process_tracing;
pub use update_process_tracing::UpdateProcessTracing;

#[path = "../clsid_from_string.rs"]
mod clsid_from_string;
pub use clsid_from_string::CLSIDFromString;

#[path = "../co_create_guid.rs"]
mod co_create_guid;
pub use co_create_guid::CoCreateGuid;

#[path = "../co_create_instance.rs"]
mod co_create_instance;
pub use co_create_instance::CoCreateInstance;

#[path = "../co_initialize_ex.rs"]
mod co_initialize_ex;
pub use co_initialize_ex::CoInitializeEx;

#[path = "../co_task_mem_alloc.rs"]
mod co_task_mem_alloc;
pub use co_task_mem_alloc::CoTaskMemAlloc;

#[path = "../co_task_mem_free.rs"]
mod co_task_mem_free;
pub use co_task_mem_free::CoTaskMemFree;

#[path = "../co_uninitialize.rs"]
mod co_uninitialize;
pub use co_uninitialize::CoUninitialize;

#[path = "../string_from_clsid.rs"]
mod string_from_clsid;
pub use string_from_clsid::StringFromCLSID;

#[path = "../string_from_guid2.rs"]
mod string_from_guid2;
pub use string_from_guid2::StringFromGUID2;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "CoInitialize" => ole32::CoInitialize(emu),
        "CoInitializeEx" => ole32::CoInitializeEx(emu),
        "CoCreateInstance" => ole32::CoCreateInstance(emu),
        "CoTaskMemAlloc" => ole32::CoTaskMemAlloc(emu),
        "CoTaskMemFree" => ole32::CoTaskMemFree(emu),
        "CoUninitialize" => ole32::CoUninitialize(emu),
        "CoResumeClassObjects" => ole32::CoResumeClassObjects(emu),
        "CoAddRefServerProcess" => ole32::CoAddRefServerProcess(emu),
        "CoReleaseServerProcess" => ole32::CoReleaseServerProcess(emu),
        "CLSIDFromOle1Class" => ole32::CLSIDFromOle1Class(emu),
        "CLSIDFromProgID" => ole32::CLSIDFromProgID(emu),
        "CLSIDFromProgIDEx" => ole32::CLSIDFromProgIDEx(emu),
        "CLSIDFromString" => ole32::CLSIDFromString(emu),
        "CoAicGetTokenForCOM" => ole32::CoAicGetTokenForCOM(emu),
        "CoAllowSetForegroundWindow" => ole32::CoAllowSetForegroundWindow(emu),
        "CoAllowUnmarshalerCLSID" => ole32::CoAllowUnmarshalerCLSID(emu),
        "CoBuildVersion" => ole32::CoBuildVersion(emu),
        "CoCancelCall" => ole32::CoCancelCall(emu),
        "CoCheckElevationEnabled" => ole32::CoCheckElevationEnabled(emu),
        "CoCreateGuid" => ole32::CoCreateGuid(emu),
        "CoCreateInstanceEx" => ole32::CoCreateInstanceEx(emu),
        "CoCreateInstanceFromApp" => ole32::CoCreateInstanceFromApp(emu),
        "CoCreateObjectInContext" => ole32::CoCreateObjectInContext(emu),
        "CoDeactivateObject" => ole32::CoDeactivateObject(emu),
        "CoDecrementMTAUsage" => ole32::CoDecrementMTAUsage(emu),
        "CoDisableCallCancellation" => ole32::CoDisableCallCancellation(emu),
        "CoDisconnectContext" => ole32::CoDisconnectContext(emu),
        "CoDisconnectObject" => ole32::CoDisconnectObject(emu),
        "CoDosDateTimeToFileTime" => ole32::CoDosDateTimeToFileTime(emu),
        "CoEnableCallCancellation" => ole32::CoEnableCallCancellation(emu),
        "CoFileTimeNow" => ole32::CoFileTimeNow(emu),
        "CoFileTimeToDosDateTime" => ole32::CoFileTimeToDosDateTime(emu),
        "CoFreeAllLibraries" => ole32::CoFreeAllLibraries(emu),
        "CoFreeLibrary" => ole32::CoFreeLibrary(emu),
        "CoFreeUnusedLibraries" => ole32::CoFreeUnusedLibraries(emu),
        "CoFreeUnusedLibrariesEx" => ole32::CoFreeUnusedLibrariesEx(emu),
        "CoGetActivationState" => ole32::CoGetActivationState(emu),
        "CoGetApartmentID" => ole32::CoGetApartmentID(emu),
        "CoGetApartmentType" => ole32::CoGetApartmentType(emu),
        "CoGetCallContext" => ole32::CoGetCallContext(emu),
        "CoGetCallState" => ole32::CoGetCallState(emu),
        "CoGetCallerTID" => ole32::CoGetCallerTID(emu),
        "CoGetCancelObject" => ole32::CoGetCancelObject(emu),
        "CoGetClassObject" => ole32::CoGetClassObject(emu),
        "CoGetClassVersion" => ole32::CoGetClassVersion(emu),
        "CoGetComCatalog" => ole32::CoGetComCatalog(emu),
        "CoGetContextToken" => ole32::CoGetContextToken(emu),
        "CoGetCurrentLogicalThreadId" => ole32::CoGetCurrentLogicalThreadId(emu),
        "CoGetCurrentProcess" => ole32::CoGetCurrentProcess(emu),
        "CoGetDefaultContext" => ole32::CoGetDefaultContext(emu),
        "CoGetInstanceFromFile" => ole32::CoGetInstanceFromFile(emu),
        "CoGetInterceptor" => ole32::CoGetInterceptor(emu),
        "CoGetInterceptorForOle32" => ole32::CoGetInterceptorForOle32(emu),
        "CoGetInterceptorFromTypeInfo" => ole32::CoGetInterceptorFromTypeInfo(emu),
        "CoGetMalloc" => ole32::CoGetMalloc(emu),
        "CoGetModuleType" => ole32::CoGetModuleType(emu),
        "CoGetObject" => ole32::CoGetObject(emu),
        "CoGetObjectContext" => ole32::CoGetObjectContext(emu),
        "CoGetPSClsid" => ole32::CoGetPSClsid(emu),
        "CoGetProcessIdentifier" => ole32::CoGetProcessIdentifier(emu),
        "CoGetSystemSecurityPermissions" => ole32::CoGetSystemSecurityPermissions(emu),
        "CoGetSystemWow64DirectoryW" => ole32::CoGetSystemWow64DirectoryW(emu),
        "CoGetTreatAsClass" => ole32::CoGetTreatAsClass(emu),
        "CoHandlePriorityEventsFromMessagePump" => {
            ole32::CoHandlePriorityEventsFromMessagePump(emu)
        }
        "CoImpersonateClient" => ole32::CoImpersonateClient(emu),
        "CoIncrementMTAUsage" => ole32::CoIncrementMTAUsage(emu),
        "CoInitializeSecurity" => ole32::CoInitializeSecurity(emu),
        "CoInitializeWOW" => ole32::CoInitializeWOW(emu),
        "CoInstall" => ole32::CoInstall(emu),
        "CoInvalidateRemoteMachineBindings" => ole32::CoInvalidateRemoteMachineBindings(emu),
        "CoIsHandlerConnected" => ole32::CoIsHandlerConnected(emu),
        "CoIsOle1Class" => ole32::CoIsOle1Class(emu),
        "CoLoadLibrary" => ole32::CoLoadLibrary(emu),
        "CoLockObjectExternal" => ole32::CoLockObjectExternal(emu),
        "CoPopServiceDomain" => ole32::CoPopServiceDomain(emu),
        "CoPushServiceDomain" => ole32::CoPushServiceDomain(emu),
        "CoQueryAuthenticationServices" => ole32::CoQueryAuthenticationServices(emu),
        "CoQueryClientBlanket" => ole32::CoQueryClientBlanket(emu),
        "CoQueryReleaseObject" => ole32::CoQueryReleaseObject(emu),
        "CoReactivateObject" => ole32::CoReactivateObject(emu),
        "CoRegisterActivationFilter" => ole32::CoRegisterActivationFilter(emu),
        "CoRegisterClassObject" => ole32::CoRegisterClassObject(emu),
        "CoRegisterInitializeSpy" => ole32::CoRegisterInitializeSpy(emu),
        "CoRegisterMallocSpy" => ole32::CoRegisterMallocSpy(emu),
        "CoRegisterMessageFilter" => ole32::CoRegisterMessageFilter(emu),
        "CoRegisterPSClsid" => ole32::CoRegisterPSClsid(emu),
        "CoRegisterSurrogate" => ole32::CoRegisterSurrogate(emu),
        "CoRegisterSurrogateEx" => ole32::CoRegisterSurrogateEx(emu),
        "CoRetireServer" => ole32::CoRetireServer(emu),
        "CoRevertToSelf" => ole32::CoRevertToSelf(emu),
        "CoRevokeClassObject" => ole32::CoRevokeClassObject(emu),
        "CoRevokeInitializeSpy" => ole32::CoRevokeInitializeSpy(emu),
        "CoRevokeMallocSpy" => ole32::CoRevokeMallocSpy(emu),
        "CoSetCancelObject" => ole32::CoSetCancelObject(emu),
        "CoSetMessageDispatcher" => ole32::CoSetMessageDispatcher(emu),
        "CoSetState" => ole32::CoSetState(emu),
        "CoSuspendClassObjects" => ole32::CoSuspendClassObjects(emu),
        "CoSwitchCallContext" => ole32::CoSwitchCallContext(emu),
        "CoTaskMemRealloc" => ole32::CoTaskMemRealloc(emu),
        "CoTestCancel" => ole32::CoTestCancel(emu),
        "CoTreatAsClass" => ole32::CoTreatAsClass(emu),
        "CoTryGetInterceptorForAppModelVirtualization" => {
            ole32::CoTryGetInterceptorForAppModelVirtualization(emu)
        }
        "CoUnloadingWOW" => ole32::CoUnloadingWOW(emu),
        "CoUnmarshalHresult" => ole32::CoUnmarshalHresult(emu),
        "CoUnmarshalInterface" => ole32::CoUnmarshalInterface(emu),
        "CoVrfCheckThreadState" => ole32::CoVrfCheckThreadState(emu),
        "CoVrfGetThreadState" => ole32::CoVrfGetThreadState(emu),
        "CoVrfReleaseThreadState" => ole32::CoVrfReleaseThreadState(emu),
        "CoWaitForMultipleHandles" => ole32::CoWaitForMultipleHandles(emu),
        "CoWaitForMultipleObjects" => ole32::CoWaitForMultipleObjects(emu),
        "CreateErrorInfo" => ole32::CreateErrorInfo(emu),
        "CreateStdProgressIndicator" => ole32::CreateStdProgressIndicator(emu),
        "DeletePatternAndExtensionTables" => ole32::DeletePatternAndExtensionTables(emu),
        "DllGetClassObject" => ole32::DllGetClassObject(emu),
        "DllGetClassObjectWOW" => ole32::DllGetClassObjectWOW(emu),
        "DllRegisterServer" => ole32::DllRegisterServer(emu),
        "EnableHookObject" => ole32::EnableHookObject(emu),
        "FindExt" => ole32::FindExt(emu),
        "GetActiveObjectExt" => ole32::GetActiveObjectExt(emu),
        "GetClassFile" => ole32::GetClassFile(emu),
        "GetErrorInfo" => ole32::GetErrorInfo(emu),
        "GetHookInterface" => ole32::GetHookInterface(emu),
        "IIDFromString" => ole32::IIDFromString(emu),
        "IsEqualGUID" => ole32::IsEqualGUID(emu),
        "IsRoInitializeASTAAllowedInDesktop" => ole32::IsRoInitializeASTAAllowedInDesktop(emu),
        "IsValidIid" => ole32::IsValidIid(emu),
        "IsValidInterface" => ole32::IsValidInterface(emu),
        "IsValidPtrIn" => ole32::IsValidPtrIn(emu),
        "IsValidPtrOut" => ole32::IsValidPtrOut(emu),
        "MkParseDisplayName" => ole32::MkParseDisplayName(emu),
        "ProgIDFromCLSID" => ole32::ProgIDFromCLSID(emu),
        "RegisterActiveObjectExt" => ole32::RegisterActiveObjectExt(emu),
        "RevokeActiveObjectExt" => ole32::RevokeActiveObjectExt(emu),
        "RoGetAgileReference" => ole32::RoGetAgileReference(emu),
        "SetErrorInfo" => ole32::SetErrorInfo(emu),
        "SetOleautModule" => ole32::SetOleautModule(emu),
        "SetWOWThunkGlobalPtr" => ole32::SetWOWThunkGlobalPtr(emu),
        "StdTypesGetClassObject" => ole32::StdTypesGetClassObject(emu),
        "StdTypesRegisterServer" => ole32::StdTypesRegisterServer(emu),
        "StringFromCLSID" => ole32::StringFromCLSID(emu),
        "StringFromGUID2" => ole32::StringFromGUID2(emu),
        "StringFromIID" => ole32::StringFromIID(emu),
        "UpdateDCOMSettings" => ole32::UpdateDCOMSettings(emu),
        "UpdateProcessTracing" => ole32::UpdateProcessTracing(emu),
        "OleInitialize" => ole32::OleInitialize(emu),
        "OleBuildVersion" => ole32::OleBuildVersion(emu),
        "OleInitializeWOW" => ole32::OleInitializeWOW(emu),
        "OleUninitialize" => ole32::OleUninitialize(emu),
        _ => return false,
    }
    true
}
