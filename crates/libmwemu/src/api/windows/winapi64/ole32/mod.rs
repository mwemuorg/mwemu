use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64::kernel32;

mod co_initialize;
pub use co_initialize::CoInitialize;
mod co_resume_class_objects;
pub use co_resume_class_objects::CoResumeClassObjects;
mod co_add_ref_server_process;
pub use co_add_ref_server_process::CoAddRefServerProcess;
mod co_release_server_process;
pub use co_release_server_process::CoReleaseServerProcess;
mod create_class_moniker;
pub use create_class_moniker::CreateClassMoniker;
mod get_running_object_table;
pub use get_running_object_table::GetRunningObjectTable;
mod bind_moniker;
pub use bind_moniker::BindMoniker;
mod clipformat_user_free;
pub use clipformat_user_free::CLIPFORMAT_UserFree;
mod clipformat_user_free64;
pub use clipformat_user_free64::CLIPFORMAT_UserFree64;
mod clipformat_user_free_ext;
pub use clipformat_user_free_ext::CLIPFORMAT_UserFreeExt;
mod clipformat_user_marshal;
pub use clipformat_user_marshal::CLIPFORMAT_UserMarshal;
mod clipformat_user_marshal64;
pub use clipformat_user_marshal64::CLIPFORMAT_UserMarshal64;
mod clipformat_user_marshal_ext;
pub use clipformat_user_marshal_ext::CLIPFORMAT_UserMarshalExt;
mod clipformat_user_size;
pub use clipformat_user_size::CLIPFORMAT_UserSize;
mod clipformat_user_size64;
pub use clipformat_user_size64::CLIPFORMAT_UserSize64;
mod clipformat_user_size_ext;
pub use clipformat_user_size_ext::CLIPFORMAT_UserSizeExt;
mod clipformat_user_unmarshal;
pub use clipformat_user_unmarshal::CLIPFORMAT_UserUnmarshal;
mod clipformat_user_unmarshal64;
pub use clipformat_user_unmarshal64::CLIPFORMAT_UserUnmarshal64;
mod clipformat_user_unmarshal_ext;
pub use clipformat_user_unmarshal_ext::CLIPFORMAT_UserUnmarshalExt;
mod clsid_from_ole1_class;
pub use clsid_from_ole1_class::CLSIDFromOle1Class;
mod clsid_from_prog_id;
pub use clsid_from_prog_id::CLSIDFromProgID;
mod clsid_from_prog_id_ex;
pub use clsid_from_prog_id_ex::CLSIDFromProgIDEx;
mod c_std_async_stub_buffer2_connect;
pub use c_std_async_stub_buffer2_connect::CStdAsyncStubBuffer2_Connect;
mod c_std_async_stub_buffer2_disconnect;
pub use c_std_async_stub_buffer2_disconnect::CStdAsyncStubBuffer2_Disconnect;
mod c_std_async_stub_buffer2_release;
pub use c_std_async_stub_buffer2_release::CStdAsyncStubBuffer2_Release;
mod c_std_async_stub_buffer_add_ref;
pub use c_std_async_stub_buffer_add_ref::CStdAsyncStubBuffer_AddRef;
mod c_std_async_stub_buffer_connect;
pub use c_std_async_stub_buffer_connect::CStdAsyncStubBuffer_Connect;
mod c_std_async_stub_buffer_disconnect;
pub use c_std_async_stub_buffer_disconnect::CStdAsyncStubBuffer_Disconnect;
mod c_std_async_stub_buffer_invoke;
pub use c_std_async_stub_buffer_invoke::CStdAsyncStubBuffer_Invoke;
mod c_std_async_stub_buffer_query_interface;
pub use c_std_async_stub_buffer_query_interface::CStdAsyncStubBuffer_QueryInterface;
mod c_std_async_stub_buffer_release;
pub use c_std_async_stub_buffer_release::CStdAsyncStubBuffer_Release;
mod c_std_stub_buffer2_connect;
pub use c_std_stub_buffer2_connect::CStdStubBuffer2_Connect;
mod c_std_stub_buffer2_count_refs;
pub use c_std_stub_buffer2_count_refs::CStdStubBuffer2_CountRefs;
mod c_std_stub_buffer2_disconnect;
pub use c_std_stub_buffer2_disconnect::CStdStubBuffer2_Disconnect;
mod c_std_stub_buffer2_query_interface;
pub use c_std_stub_buffer2_query_interface::CStdStubBuffer2_QueryInterface;
mod check_init_dde;
pub use check_init_dde::CheckInitDde;
mod clean_rot_for_apartment;
pub use clean_rot_for_apartment::CleanROTForApartment;
mod clipboard_process_uninitialize;
pub use clipboard_process_uninitialize::ClipboardProcessUninitialize;
mod co_aic_get_token_for_com;
pub use co_aic_get_token_for_com::CoAicGetTokenForCOM;
mod co_allow_set_foreground_window;
pub use co_allow_set_foreground_window::CoAllowSetForegroundWindow;
mod co_allow_unmarshaler_clsid;
pub use co_allow_unmarshaler_clsid::CoAllowUnmarshalerCLSID;
mod co_build_version;
pub use co_build_version::CoBuildVersion;
mod co_cancel_call;
pub use co_cancel_call::CoCancelCall;
mod co_check_elevation_enabled;
pub use co_check_elevation_enabled::CoCheckElevationEnabled;
mod co_copy_proxy;
pub use co_copy_proxy::CoCopyProxy;
mod co_create_free_threaded_marshaler;
pub use co_create_free_threaded_marshaler::CoCreateFreeThreadedMarshaler;
mod co_create_instance_ex;
pub use co_create_instance_ex::CoCreateInstanceEx;
mod co_create_instance_from_app;
pub use co_create_instance_from_app::CoCreateInstanceFromApp;
mod co_create_object_in_context;
pub use co_create_object_in_context::CoCreateObjectInContext;
mod co_deactivate_object;
pub use co_deactivate_object::CoDeactivateObject;
mod co_decode_proxy;
pub use co_decode_proxy::CoDecodeProxy;
mod co_decrement_mta_usage;
pub use co_decrement_mta_usage::CoDecrementMTAUsage;
mod co_disable_call_cancellation;
pub use co_disable_call_cancellation::CoDisableCallCancellation;
mod co_disconnect_context;
pub use co_disconnect_context::CoDisconnectContext;
mod co_disconnect_object;
pub use co_disconnect_object::CoDisconnectObject;
mod co_dos_date_time_to_file_time;
pub use co_dos_date_time_to_file_time::CoDosDateTimeToFileTime;
mod co_enable_call_cancellation;
pub use co_enable_call_cancellation::CoEnableCallCancellation;
mod co_file_time_now;
pub use co_file_time_now::CoFileTimeNow;
mod co_file_time_to_dos_date_time;
pub use co_file_time_to_dos_date_time::CoFileTimeToDosDateTime;
mod co_free_all_libraries;
pub use co_free_all_libraries::CoFreeAllLibraries;
mod co_free_library;
pub use co_free_library::CoFreeLibrary;
mod co_free_unused_libraries;
pub use co_free_unused_libraries::CoFreeUnusedLibraries;
mod co_free_unused_libraries_ex;
pub use co_free_unused_libraries_ex::CoFreeUnusedLibrariesEx;
mod co_get_activation_state;
pub use co_get_activation_state::CoGetActivationState;
mod co_get_apartment_id;
pub use co_get_apartment_id::CoGetApartmentID;
mod co_get_apartment_type;
pub use co_get_apartment_type::CoGetApartmentType;
mod co_get_call_context;
pub use co_get_call_context::CoGetCallContext;
mod co_get_call_state;
pub use co_get_call_state::CoGetCallState;
mod co_get_caller_tid;
pub use co_get_caller_tid::CoGetCallerTID;
mod co_get_cancel_object;
pub use co_get_cancel_object::CoGetCancelObject;
mod co_get_class_object;
pub use co_get_class_object::CoGetClassObject;
mod co_get_class_version;
pub use co_get_class_version::CoGetClassVersion;
mod co_get_com_catalog;
pub use co_get_com_catalog::CoGetComCatalog;
mod co_get_context_token;
pub use co_get_context_token::CoGetContextToken;
mod co_get_current_logical_thread_id;
pub use co_get_current_logical_thread_id::CoGetCurrentLogicalThreadId;
mod co_get_current_process;
pub use co_get_current_process::CoGetCurrentProcess;
mod co_get_default_context;
pub use co_get_default_context::CoGetDefaultContext;
mod co_get_instance_from_file;
pub use co_get_instance_from_file::CoGetInstanceFromFile;
mod co_get_instance_from_i_storage;
pub use co_get_instance_from_i_storage::CoGetInstanceFromIStorage;
mod co_get_interceptor;
pub use co_get_interceptor::CoGetInterceptor;
mod co_get_interceptor_for_ole32;
pub use co_get_interceptor_for_ole32::CoGetInterceptorForOle32;
mod co_get_interceptor_from_type_info;
pub use co_get_interceptor_from_type_info::CoGetInterceptorFromTypeInfo;
mod co_get_interface_and_release_stream;
pub use co_get_interface_and_release_stream::CoGetInterfaceAndReleaseStream;
mod co_get_malloc;
pub use co_get_malloc::CoGetMalloc;
mod co_get_marshal_size_max;
pub use co_get_marshal_size_max::CoGetMarshalSizeMax;
mod co_get_module_type;
pub use co_get_module_type::CoGetModuleType;
mod co_get_object;
pub use co_get_object::CoGetObject;
mod co_get_object_context;
pub use co_get_object_context::CoGetObjectContext;
mod co_get_ps_clsid;
pub use co_get_ps_clsid::CoGetPSClsid;
mod co_get_process_identifier;
pub use co_get_process_identifier::CoGetProcessIdentifier;
mod co_get_standard_marshal;
pub use co_get_standard_marshal::CoGetStandardMarshal;
mod co_get_std_marshal_ex;
pub use co_get_std_marshal_ex::CoGetStdMarshalEx;
mod co_get_system_security_permissions;
pub use co_get_system_security_permissions::CoGetSystemSecurityPermissions;
mod co_get_system_wow64_directory_w;
pub use co_get_system_wow64_directory_w::CoGetSystemWow64DirectoryW;
mod co_get_treat_as_class;
pub use co_get_treat_as_class::CoGetTreatAsClass;
mod co_handle_priority_events_from_message_pump;
pub use co_handle_priority_events_from_message_pump::CoHandlePriorityEventsFromMessagePump;
mod co_impersonate_client;
pub use co_impersonate_client::CoImpersonateClient;
mod co_increment_mta_usage;
pub use co_increment_mta_usage::CoIncrementMTAUsage;
mod co_initialize_security;
pub use co_initialize_security::CoInitializeSecurity;
mod co_initialize_wow;
pub use co_initialize_wow::CoInitializeWOW;
mod co_install;
pub use co_install::CoInstall;
mod co_invalidate_remote_machine_bindings;
pub use co_invalidate_remote_machine_bindings::CoInvalidateRemoteMachineBindings;
mod co_is_handler_connected;
pub use co_is_handler_connected::CoIsHandlerConnected;
mod co_is_ole1_class;
pub use co_is_ole1_class::CoIsOle1Class;
mod co_load_library;
pub use co_load_library::CoLoadLibrary;
mod co_lock_object_external;
pub use co_lock_object_external::CoLockObjectExternal;
mod co_marshal_hresult;
pub use co_marshal_hresult::CoMarshalHresult;
mod co_marshal_inter_thread_interface_in_stream;
pub use co_marshal_inter_thread_interface_in_stream::CoMarshalInterThreadInterfaceInStream;
mod co_marshal_interface;
pub use co_marshal_interface::CoMarshalInterface;
mod co_pop_service_domain;
pub use co_pop_service_domain::CoPopServiceDomain;
mod co_push_service_domain;
pub use co_push_service_domain::CoPushServiceDomain;
mod co_query_authentication_services;
pub use co_query_authentication_services::CoQueryAuthenticationServices;
mod co_query_client_blanket;
pub use co_query_client_blanket::CoQueryClientBlanket;
mod co_query_proxy_blanket;
pub use co_query_proxy_blanket::CoQueryProxyBlanket;
mod co_query_release_object;
pub use co_query_release_object::CoQueryReleaseObject;
mod co_reactivate_object;
pub use co_reactivate_object::CoReactivateObject;
mod co_register_activation_filter;
pub use co_register_activation_filter::CoRegisterActivationFilter;
mod co_register_channel_hook;
pub use co_register_channel_hook::CoRegisterChannelHook;
mod co_register_class_object;
pub use co_register_class_object::CoRegisterClassObject;
mod co_register_initialize_spy;
pub use co_register_initialize_spy::CoRegisterInitializeSpy;
mod co_register_malloc_spy;
pub use co_register_malloc_spy::CoRegisterMallocSpy;
mod co_register_message_filter;
pub use co_register_message_filter::CoRegisterMessageFilter;
mod co_register_ps_clsid;
pub use co_register_ps_clsid::CoRegisterPSClsid;
mod co_register_surrogate;
pub use co_register_surrogate::CoRegisterSurrogate;
mod co_register_surrogate_ex;
pub use co_register_surrogate_ex::CoRegisterSurrogateEx;
mod co_release_marshal_data;
pub use co_release_marshal_data::CoReleaseMarshalData;
mod co_retire_server;
pub use co_retire_server::CoRetireServer;
mod co_revert_to_self;
pub use co_revert_to_self::CoRevertToSelf;
mod co_revoke_class_object;
pub use co_revoke_class_object::CoRevokeClassObject;
mod co_revoke_initialize_spy;
pub use co_revoke_initialize_spy::CoRevokeInitializeSpy;
mod co_revoke_malloc_spy;
pub use co_revoke_malloc_spy::CoRevokeMallocSpy;
mod co_set_cancel_object;
pub use co_set_cancel_object::CoSetCancelObject;
mod co_set_message_dispatcher;
pub use co_set_message_dispatcher::CoSetMessageDispatcher;
mod co_set_proxy_blanket;
pub use co_set_proxy_blanket::CoSetProxyBlanket;
mod co_set_state;
pub use co_set_state::CoSetState;
mod co_suspend_class_objects;
pub use co_suspend_class_objects::CoSuspendClassObjects;
mod co_switch_call_context;
pub use co_switch_call_context::CoSwitchCallContext;
mod co_task_mem_realloc;
pub use co_task_mem_realloc::CoTaskMemRealloc;
mod co_test_cancel;
pub use co_test_cancel::CoTestCancel;
mod co_treat_as_class;
pub use co_treat_as_class::CoTreatAsClass;
mod co_try_get_interceptor_for_app_model_virtualization;
pub use co_try_get_interceptor_for_app_model_virtualization::CoTryGetInterceptorForAppModelVirtualization;
mod co_unloading_wow;
pub use co_unloading_wow::CoUnloadingWOW;
mod co_unmarshal_hresult;
pub use co_unmarshal_hresult::CoUnmarshalHresult;
mod co_unmarshal_interface;
pub use co_unmarshal_interface::CoUnmarshalInterface;
mod co_vrf_check_thread_state;
pub use co_vrf_check_thread_state::CoVrfCheckThreadState;
mod co_vrf_get_thread_state;
pub use co_vrf_get_thread_state::CoVrfGetThreadState;
mod co_vrf_release_thread_state;
pub use co_vrf_release_thread_state::CoVrfReleaseThreadState;
mod co_wait_for_multiple_handles;
pub use co_wait_for_multiple_handles::CoWaitForMultipleHandles;
mod co_wait_for_multiple_objects;
pub use co_wait_for_multiple_objects::CoWaitForMultipleObjects;
mod com_ps_ndr_dll_can_unload_now;
pub use com_ps_ndr_dll_can_unload_now::ComPs_NdrDllCanUnloadNow;
mod com_ps_ndr_dll_get_class_object;
pub use com_ps_ndr_dll_get_class_object::ComPs_NdrDllGetClassObject;
mod com_ps_ndr_dll_register_proxy;
pub use com_ps_ndr_dll_register_proxy::ComPs_NdrDllRegisterProxy;
mod com_ps_ndr_dll_unregister_proxy;
pub use com_ps_ndr_dll_unregister_proxy::ComPs_NdrDllUnregisterProxy;
mod create_anti_moniker;
pub use create_anti_moniker::CreateAntiMoniker;
mod create_bind_ctx;
pub use create_bind_ctx::CreateBindCtx;
mod create_data_advise_holder;
pub use create_data_advise_holder::CreateDataAdviseHolder;
mod create_data_cache;
pub use create_data_cache::CreateDataCache;
mod create_error_info;
pub use create_error_info::CreateErrorInfo;
mod create_file_moniker;
pub use create_file_moniker::CreateFileMoniker;
mod create_generic_composite;
pub use create_generic_composite::CreateGenericComposite;
mod create_i_lock_bytes_on_h_global;
pub use create_i_lock_bytes_on_h_global::CreateILockBytesOnHGlobal;
mod create_item_moniker;
pub use create_item_moniker::CreateItemMoniker;
mod create_objref_moniker;
pub use create_objref_moniker::CreateObjrefMoniker;
mod create_ole_advise_holder;
pub use create_ole_advise_holder::CreateOleAdviseHolder;
mod create_pointer_moniker;
pub use create_pointer_moniker::CreatePointerMoniker;
mod create_std_progress_indicator;
pub use create_std_progress_indicator::CreateStdProgressIndicator;
mod create_stream_on_h_global;
pub use create_stream_on_h_global::CreateStreamOnHGlobal;
mod dcom_channel_set_h_result;
pub use dcom_channel_set_h_result::DcomChannelSetHResult;
mod dde_bind_to_object;
pub use dde_bind_to_object::DdeBindToObject;
mod delete_pattern_and_extension_tables;
pub use delete_pattern_and_extension_tables::DeletePatternAndExtensionTables;
mod destroy_running_object_table;
pub use destroy_running_object_table::DestroyRunningObjectTable;
mod dll_debug_object_rpc_hook;
pub use dll_debug_object_rpc_hook::DllDebugObjectRPCHook;
mod dll_get_class_object;
pub use dll_get_class_object::DllGetClassObject;
mod dll_get_class_object_wow;
pub use dll_get_class_object_wow::DllGetClassObjectWOW;
mod dll_register_server;
pub use dll_register_server::DllRegisterServer;
mod do_drag_drop;
pub use do_drag_drop::DoDragDrop;
mod drag_drop_set_fdt;
pub use drag_drop_set_fdt::DragDropSetFDT;
mod enable_hook_object;
pub use enable_hook_object::EnableHookObject;
mod find_ext;
pub use find_ext::FindExt;
mod fmt_id_to_prop_stg_name;
pub use fmt_id_to_prop_stg_name::FmtIdToPropStgName;
mod free_prop_variant_array;
pub use free_prop_variant_array::FreePropVariantArray;
mod get_active_object_ext;
pub use get_active_object_ext::GetActiveObjectExt;
mod get_class_file;
pub use get_class_file::GetClassFile;
mod get_convert_stg;
pub use get_convert_stg::GetConvertStg;
mod get_document_bit_stg;
pub use get_document_bit_stg::GetDocumentBitStg;
mod get_error_info;
pub use get_error_info::GetErrorInfo;
mod get_h_global_from_i_lock_bytes;
pub use get_h_global_from_i_lock_bytes::GetHGlobalFromILockBytes;
mod get_h_global_from_stream;
pub use get_h_global_from_stream::GetHGlobalFromStream;
mod get_hook_interface;
pub use get_hook_interface::GetHookInterface;
mod get_object_from_rot_by_path;
pub use get_object_from_rot_by_path::GetObjectFromRotByPath;
mod haccel_user_free;
pub use haccel_user_free::HACCEL_UserFree;
mod haccel_user_free64;
pub use haccel_user_free64::HACCEL_UserFree64;
mod haccel_user_marshal;
pub use haccel_user_marshal::HACCEL_UserMarshal;
mod haccel_user_marshal64;
pub use haccel_user_marshal64::HACCEL_UserMarshal64;
mod haccel_user_size;
pub use haccel_user_size::HACCEL_UserSize;
mod haccel_user_size64;
pub use haccel_user_size64::HACCEL_UserSize64;
mod haccel_user_unmarshal;
pub use haccel_user_unmarshal::HACCEL_UserUnmarshal;
mod haccel_user_unmarshal64;
pub use haccel_user_unmarshal64::HACCEL_UserUnmarshal64;
mod hbitmap_user_free;
pub use hbitmap_user_free::HBITMAP_UserFree;
mod hbitmap_user_free64;
pub use hbitmap_user_free64::HBITMAP_UserFree64;
mod hbitmap_user_marshal;
pub use hbitmap_user_marshal::HBITMAP_UserMarshal;
mod hbitmap_user_marshal64;
pub use hbitmap_user_marshal64::HBITMAP_UserMarshal64;
mod hbitmap_user_size;
pub use hbitmap_user_size::HBITMAP_UserSize;
mod hbitmap_user_size64;
pub use hbitmap_user_size64::HBITMAP_UserSize64;
mod hbitmap_user_unmarshal;
pub use hbitmap_user_unmarshal::HBITMAP_UserUnmarshal;
mod hbitmap_user_unmarshal64;
pub use hbitmap_user_unmarshal64::HBITMAP_UserUnmarshal64;
mod hbrush_user_free;
pub use hbrush_user_free::HBRUSH_UserFree;
mod hbrush_user_free64;
pub use hbrush_user_free64::HBRUSH_UserFree64;
mod hbrush_user_marshal;
pub use hbrush_user_marshal::HBRUSH_UserMarshal;
mod hbrush_user_marshal64;
pub use hbrush_user_marshal64::HBRUSH_UserMarshal64;
mod hbrush_user_size;
pub use hbrush_user_size::HBRUSH_UserSize;
mod hbrush_user_size64;
pub use hbrush_user_size64::HBRUSH_UserSize64;
mod hbrush_user_unmarshal;
pub use hbrush_user_unmarshal::HBRUSH_UserUnmarshal;
mod hbrush_user_unmarshal64;
pub use hbrush_user_unmarshal64::HBRUSH_UserUnmarshal64;
mod hdc_user_free;
pub use hdc_user_free::HDC_UserFree;
mod hdc_user_free64;
pub use hdc_user_free64::HDC_UserFree64;
mod hdc_user_marshal;
pub use hdc_user_marshal::HDC_UserMarshal;
mod hdc_user_marshal64;
pub use hdc_user_marshal64::HDC_UserMarshal64;
mod hdc_user_size;
pub use hdc_user_size::HDC_UserSize;
mod hdc_user_size64;
pub use hdc_user_size64::HDC_UserSize64;
mod hdc_user_unmarshal;
pub use hdc_user_unmarshal::HDC_UserUnmarshal;
mod hdc_user_unmarshal64;
pub use hdc_user_unmarshal64::HDC_UserUnmarshal64;
mod henhmetafile_user_free;
pub use henhmetafile_user_free::HENHMETAFILE_UserFree;
mod henhmetafile_user_free64;
pub use henhmetafile_user_free64::HENHMETAFILE_UserFree64;
mod henhmetafile_user_marshal;
pub use henhmetafile_user_marshal::HENHMETAFILE_UserMarshal;
mod henhmetafile_user_marshal64;
pub use henhmetafile_user_marshal64::HENHMETAFILE_UserMarshal64;
mod henhmetafile_user_size;
pub use henhmetafile_user_size::HENHMETAFILE_UserSize;
mod henhmetafile_user_size64;
pub use henhmetafile_user_size64::HENHMETAFILE_UserSize64;
mod henhmetafile_user_unmarshal;
pub use henhmetafile_user_unmarshal::HENHMETAFILE_UserUnmarshal;
mod henhmetafile_user_unmarshal64;
pub use henhmetafile_user_unmarshal64::HENHMETAFILE_UserUnmarshal64;
mod hglobal_user_free;
pub use hglobal_user_free::HGLOBAL_UserFree;
mod hglobal_user_free64;
pub use hglobal_user_free64::HGLOBAL_UserFree64;
mod hglobal_user_marshal;
pub use hglobal_user_marshal::HGLOBAL_UserMarshal;
mod hglobal_user_marshal64;
pub use hglobal_user_marshal64::HGLOBAL_UserMarshal64;
mod hglobal_user_size;
pub use hglobal_user_size::HGLOBAL_UserSize;
mod hglobal_user_size64;
pub use hglobal_user_size64::HGLOBAL_UserSize64;
mod hglobal_user_unmarshal;
pub use hglobal_user_unmarshal::HGLOBAL_UserUnmarshal;
mod hglobal_user_unmarshal64;
pub use hglobal_user_unmarshal64::HGLOBAL_UserUnmarshal64;
mod hicon_user_free;
pub use hicon_user_free::HICON_UserFree;
mod hicon_user_free64;
pub use hicon_user_free64::HICON_UserFree64;
mod hicon_user_marshal;
pub use hicon_user_marshal::HICON_UserMarshal;
mod hicon_user_marshal64;
pub use hicon_user_marshal64::HICON_UserMarshal64;
mod hicon_user_size;
pub use hicon_user_size::HICON_UserSize;
mod hicon_user_size64;
pub use hicon_user_size64::HICON_UserSize64;
mod hicon_user_unmarshal;
pub use hicon_user_unmarshal::HICON_UserUnmarshal;
mod hicon_user_unmarshal64;
pub use hicon_user_unmarshal64::HICON_UserUnmarshal64;
mod hmenu_user_free;
pub use hmenu_user_free::HMENU_UserFree;
mod hmenu_user_free64;
pub use hmenu_user_free64::HMENU_UserFree64;
mod hmenu_user_marshal;
pub use hmenu_user_marshal::HMENU_UserMarshal;
mod hmenu_user_marshal64;
pub use hmenu_user_marshal64::HMENU_UserMarshal64;
mod hmenu_user_size;
pub use hmenu_user_size::HMENU_UserSize;
mod hmenu_user_size64;
pub use hmenu_user_size64::HMENU_UserSize64;
mod hmenu_user_unmarshal;
pub use hmenu_user_unmarshal::HMENU_UserUnmarshal;
mod hmenu_user_unmarshal64;
pub use hmenu_user_unmarshal64::HMENU_UserUnmarshal64;
mod hmetafilepict_user_free;
pub use hmetafilepict_user_free::HMETAFILEPICT_UserFree;
mod hmetafilepict_user_free64;
pub use hmetafilepict_user_free64::HMETAFILEPICT_UserFree64;
mod hmetafilepict_user_marshal;
pub use hmetafilepict_user_marshal::HMETAFILEPICT_UserMarshal;
mod hmetafilepict_user_marshal64;
pub use hmetafilepict_user_marshal64::HMETAFILEPICT_UserMarshal64;
mod hmetafilepict_user_size;
pub use hmetafilepict_user_size::HMETAFILEPICT_UserSize;
mod hmetafilepict_user_size64;
pub use hmetafilepict_user_size64::HMETAFILEPICT_UserSize64;
mod hmetafilepict_user_unmarshal;
pub use hmetafilepict_user_unmarshal::HMETAFILEPICT_UserUnmarshal;
mod hmetafilepict_user_unmarshal64;
pub use hmetafilepict_user_unmarshal64::HMETAFILEPICT_UserUnmarshal64;
mod hmetafile_user_free;
pub use hmetafile_user_free::HMETAFILE_UserFree;
mod hmetafile_user_free64;
pub use hmetafile_user_free64::HMETAFILE_UserFree64;
mod hmetafile_user_marshal;
pub use hmetafile_user_marshal::HMETAFILE_UserMarshal;
mod hmetafile_user_marshal64;
pub use hmetafile_user_marshal64::HMETAFILE_UserMarshal64;
mod hmetafile_user_size;
pub use hmetafile_user_size::HMETAFILE_UserSize;
mod hmetafile_user_size64;
pub use hmetafile_user_size64::HMETAFILE_UserSize64;
mod hmetafile_user_unmarshal;
pub use hmetafile_user_unmarshal::HMETAFILE_UserUnmarshal;
mod hmetafile_user_unmarshal64;
pub use hmetafile_user_unmarshal64::HMETAFILE_UserUnmarshal64;
mod hmonitor_user_free;
pub use hmonitor_user_free::HMONITOR_UserFree;
mod hmonitor_user_free64;
pub use hmonitor_user_free64::HMONITOR_UserFree64;
mod hmonitor_user_marshal;
pub use hmonitor_user_marshal::HMONITOR_UserMarshal;
mod hmonitor_user_marshal64;
pub use hmonitor_user_marshal64::HMONITOR_UserMarshal64;
mod hmonitor_user_size;
pub use hmonitor_user_size::HMONITOR_UserSize;
mod hmonitor_user_size64;
pub use hmonitor_user_size64::HMONITOR_UserSize64;
mod hmonitor_user_unmarshal;
pub use hmonitor_user_unmarshal::HMONITOR_UserUnmarshal;
mod hmonitor_user_unmarshal64;
pub use hmonitor_user_unmarshal64::HMONITOR_UserUnmarshal64;
mod hpalette_user_free;
pub use hpalette_user_free::HPALETTE_UserFree;
mod hpalette_user_free64;
pub use hpalette_user_free64::HPALETTE_UserFree64;
mod hpalette_user_free_ext;
pub use hpalette_user_free_ext::HPALETTE_UserFreeExt;
mod hpalette_user_marshal;
pub use hpalette_user_marshal::HPALETTE_UserMarshal;
mod hpalette_user_marshal64;
pub use hpalette_user_marshal64::HPALETTE_UserMarshal64;
mod hpalette_user_marshal_ext;
pub use hpalette_user_marshal_ext::HPALETTE_UserMarshalExt;
mod hpalette_user_size;
pub use hpalette_user_size::HPALETTE_UserSize;
mod hpalette_user_size64;
pub use hpalette_user_size64::HPALETTE_UserSize64;
mod hpalette_user_size_ext;
pub use hpalette_user_size_ext::HPALETTE_UserSizeExt;
mod hpalette_user_unmarshal;
pub use hpalette_user_unmarshal::HPALETTE_UserUnmarshal;
mod hpalette_user_unmarshal64;
pub use hpalette_user_unmarshal64::HPALETTE_UserUnmarshal64;
mod hpalette_user_unmarshal_ext;
pub use hpalette_user_unmarshal_ext::HPALETTE_UserUnmarshalExt;
mod hrgn_user_free;
pub use hrgn_user_free::HRGN_UserFree;
mod hrgn_user_marshal;
pub use hrgn_user_marshal::HRGN_UserMarshal;
mod hrgn_user_size;
pub use hrgn_user_size::HRGN_UserSize;
mod hrgn_user_unmarshal;
pub use hrgn_user_unmarshal::HRGN_UserUnmarshal;
mod hwnd_user_free;
pub use hwnd_user_free::HWND_UserFree;
mod hwnd_user_free64;
pub use hwnd_user_free64::HWND_UserFree64;
mod hwnd_user_free64_ext;
pub use hwnd_user_free64_ext::HWND_UserFree64Ext;
mod hwnd_user_free_ext;
pub use hwnd_user_free_ext::HWND_UserFreeExt;
mod hwnd_user_marshal;
pub use hwnd_user_marshal::HWND_UserMarshal;
mod hwnd_user_marshal64;
pub use hwnd_user_marshal64::HWND_UserMarshal64;
mod hwnd_user_marshal64_ext;
pub use hwnd_user_marshal64_ext::HWND_UserMarshal64Ext;
mod hwnd_user_marshal_ext;
pub use hwnd_user_marshal_ext::HWND_UserMarshalExt;
mod hwnd_user_size;
pub use hwnd_user_size::HWND_UserSize;
mod hwnd_user_size64;
pub use hwnd_user_size64::HWND_UserSize64;
mod hwnd_user_size64_ext;
pub use hwnd_user_size64_ext::HWND_UserSize64Ext;
mod hwnd_user_size_ext;
pub use hwnd_user_size_ext::HWND_UserSizeExt;
mod hwnd_user_unmarshal;
pub use hwnd_user_unmarshal::HWND_UserUnmarshal;
mod hwnd_user_unmarshal64;
pub use hwnd_user_unmarshal64::HWND_UserUnmarshal64;
mod hwnd_user_unmarshal64_ext;
pub use hwnd_user_unmarshal64_ext::HWND_UserUnmarshal64Ext;
mod hwnd_user_unmarshal_ext;
pub use hwnd_user_unmarshal_ext::HWND_UserUnmarshalExt;
mod hk_ole_register_object;
pub use hk_ole_register_object::HkOleRegisterObject;
mod iid_from_string;
pub use iid_from_string::IIDFromString;
mod is_accelerator;
pub use is_accelerator::IsAccelerator;
mod is_equal_guid;
pub use is_equal_guid::IsEqualGUID;
mod is_ro_initialize_asta_allowed_in_desktop;
pub use is_ro_initialize_asta_allowed_in_desktop::IsRoInitializeASTAAllowedInDesktop;
mod is_valid_iid;
pub use is_valid_iid::IsValidIid;
mod is_valid_interface;
pub use is_valid_interface::IsValidInterface;
mod is_valid_ptr_in;
pub use is_valid_ptr_in::IsValidPtrIn;
mod is_valid_ptr_out;
pub use is_valid_ptr_out::IsValidPtrOut;
mod mk_parse_display_name;
pub use mk_parse_display_name::MkParseDisplayName;
mod moniker_common_prefix_with;
pub use moniker_common_prefix_with::MonikerCommonPrefixWith;
mod moniker_load_type_lib;
pub use moniker_load_type_lib::MonikerLoadTypeLib;
mod moniker_relative_path_to;
pub use moniker_relative_path_to::MonikerRelativePathTo;
mod ndr_ole_initialize_extension;
pub use ndr_ole_initialize_extension::NdrOleInitializeExtension;
mod ndr_proxy_forwarding_function10;
pub use ndr_proxy_forwarding_function10::NdrProxyForwardingFunction10;
mod ndr_proxy_forwarding_function11;
pub use ndr_proxy_forwarding_function11::NdrProxyForwardingFunction11;
mod ndr_proxy_forwarding_function12;
pub use ndr_proxy_forwarding_function12::NdrProxyForwardingFunction12;
mod ndr_proxy_forwarding_function13;
pub use ndr_proxy_forwarding_function13::NdrProxyForwardingFunction13;
mod ndr_proxy_forwarding_function14;
pub use ndr_proxy_forwarding_function14::NdrProxyForwardingFunction14;
mod ndr_proxy_forwarding_function15;
pub use ndr_proxy_forwarding_function15::NdrProxyForwardingFunction15;
mod ndr_proxy_forwarding_function16;
pub use ndr_proxy_forwarding_function16::NdrProxyForwardingFunction16;
mod ndr_proxy_forwarding_function17;
pub use ndr_proxy_forwarding_function17::NdrProxyForwardingFunction17;
mod ndr_proxy_forwarding_function18;
pub use ndr_proxy_forwarding_function18::NdrProxyForwardingFunction18;
mod ndr_proxy_forwarding_function19;
pub use ndr_proxy_forwarding_function19::NdrProxyForwardingFunction19;
mod ndr_proxy_forwarding_function20;
pub use ndr_proxy_forwarding_function20::NdrProxyForwardingFunction20;
mod ndr_proxy_forwarding_function21;
pub use ndr_proxy_forwarding_function21::NdrProxyForwardingFunction21;
mod ndr_proxy_forwarding_function22;
pub use ndr_proxy_forwarding_function22::NdrProxyForwardingFunction22;
mod ndr_proxy_forwarding_function23;
pub use ndr_proxy_forwarding_function23::NdrProxyForwardingFunction23;
mod ndr_proxy_forwarding_function24;
pub use ndr_proxy_forwarding_function24::NdrProxyForwardingFunction24;
mod ndr_proxy_forwarding_function25;
pub use ndr_proxy_forwarding_function25::NdrProxyForwardingFunction25;
mod ndr_proxy_forwarding_function26;
pub use ndr_proxy_forwarding_function26::NdrProxyForwardingFunction26;
mod ndr_proxy_forwarding_function27;
pub use ndr_proxy_forwarding_function27::NdrProxyForwardingFunction27;
mod ndr_proxy_forwarding_function28;
pub use ndr_proxy_forwarding_function28::NdrProxyForwardingFunction28;
mod ndr_proxy_forwarding_function29;
pub use ndr_proxy_forwarding_function29::NdrProxyForwardingFunction29;
mod ndr_proxy_forwarding_function3;
pub use ndr_proxy_forwarding_function3::NdrProxyForwardingFunction3;
mod ndr_proxy_forwarding_function30;
pub use ndr_proxy_forwarding_function30::NdrProxyForwardingFunction30;
mod ndr_proxy_forwarding_function31;
pub use ndr_proxy_forwarding_function31::NdrProxyForwardingFunction31;
mod ndr_proxy_forwarding_function32;
pub use ndr_proxy_forwarding_function32::NdrProxyForwardingFunction32;
mod ndr_proxy_forwarding_function4;
pub use ndr_proxy_forwarding_function4::NdrProxyForwardingFunction4;
mod ndr_proxy_forwarding_function5;
pub use ndr_proxy_forwarding_function5::NdrProxyForwardingFunction5;
mod ndr_proxy_forwarding_function6;
pub use ndr_proxy_forwarding_function6::NdrProxyForwardingFunction6;
mod ndr_proxy_forwarding_function7;
pub use ndr_proxy_forwarding_function7::NdrProxyForwardingFunction7;
mod ndr_proxy_forwarding_function8;
pub use ndr_proxy_forwarding_function8::NdrProxyForwardingFunction8;
mod ndr_proxy_forwarding_function9;
pub use ndr_proxy_forwarding_function9::NdrProxyForwardingFunction9;
mod object_stubless_client10;
pub use object_stubless_client10::ObjectStublessClient10;
mod object_stubless_client11;
pub use object_stubless_client11::ObjectStublessClient11;
mod object_stubless_client12;
pub use object_stubless_client12::ObjectStublessClient12;
mod object_stubless_client13;
pub use object_stubless_client13::ObjectStublessClient13;
mod object_stubless_client14;
pub use object_stubless_client14::ObjectStublessClient14;
mod object_stubless_client15;
pub use object_stubless_client15::ObjectStublessClient15;
mod object_stubless_client16;
pub use object_stubless_client16::ObjectStublessClient16;
mod object_stubless_client17;
pub use object_stubless_client17::ObjectStublessClient17;
mod object_stubless_client18;
pub use object_stubless_client18::ObjectStublessClient18;
mod object_stubless_client19;
pub use object_stubless_client19::ObjectStublessClient19;
mod object_stubless_client20;
pub use object_stubless_client20::ObjectStublessClient20;
mod object_stubless_client21;
pub use object_stubless_client21::ObjectStublessClient21;
mod object_stubless_client22;
pub use object_stubless_client22::ObjectStublessClient22;
mod object_stubless_client23;
pub use object_stubless_client23::ObjectStublessClient23;
mod object_stubless_client24;
pub use object_stubless_client24::ObjectStublessClient24;
mod object_stubless_client25;
pub use object_stubless_client25::ObjectStublessClient25;
mod object_stubless_client26;
pub use object_stubless_client26::ObjectStublessClient26;
mod object_stubless_client27;
pub use object_stubless_client27::ObjectStublessClient27;
mod object_stubless_client28;
pub use object_stubless_client28::ObjectStublessClient28;
mod object_stubless_client29;
pub use object_stubless_client29::ObjectStublessClient29;
mod object_stubless_client3;
pub use object_stubless_client3::ObjectStublessClient3;
mod object_stubless_client30;
pub use object_stubless_client30::ObjectStublessClient30;
mod object_stubless_client31;
pub use object_stubless_client31::ObjectStublessClient31;
mod object_stubless_client32;
pub use object_stubless_client32::ObjectStublessClient32;
mod object_stubless_client4;
pub use object_stubless_client4::ObjectStublessClient4;
mod object_stubless_client5;
pub use object_stubless_client5::ObjectStublessClient5;
mod object_stubless_client6;
pub use object_stubless_client6::ObjectStublessClient6;
mod object_stubless_client7;
pub use object_stubless_client7::ObjectStublessClient7;
mod object_stubless_client8;
pub use object_stubless_client8::ObjectStublessClient8;
mod object_stubless_client9;
pub use object_stubless_client9::ObjectStublessClient9;
mod ole32_dll_get_class_object;
pub use ole32_dll_get_class_object::Ole32DllGetClassObject;
mod ole_build_version;
pub use ole_build_version::OleBuildVersion;
mod ole_convert_i_storage_to_olestream;
pub use ole_convert_i_storage_to_olestream::OleConvertIStorageToOLESTREAM;
mod ole_convert_i_storage_to_olestream_ex;
pub use ole_convert_i_storage_to_olestream_ex::OleConvertIStorageToOLESTREAMEx;
mod ole_convert_olestream_to_i_storage;
pub use ole_convert_olestream_to_i_storage::OleConvertOLESTREAMToIStorage;
mod ole_convert_olestream_to_i_storage2;
pub use ole_convert_olestream_to_i_storage2::OleConvertOLESTREAMToIStorage2;
mod ole_convert_olestream_to_i_storage_ex;
pub use ole_convert_olestream_to_i_storage_ex::OleConvertOLESTREAMToIStorageEx;
mod ole_convert_olestream_to_i_storage_ex2;
pub use ole_convert_olestream_to_i_storage_ex2::OleConvertOLESTREAMToIStorageEx2;
mod ole_create;
pub use ole_create::OleCreate;
mod ole_create_default_handler;
pub use ole_create_default_handler::OleCreateDefaultHandler;
mod ole_create_embedding_helper;
pub use ole_create_embedding_helper::OleCreateEmbeddingHelper;
mod ole_create_ex;
pub use ole_create_ex::OleCreateEx;
mod ole_create_font_indirect_ext;
pub use ole_create_font_indirect_ext::OleCreateFontIndirectExt;
mod ole_create_from_data;
pub use ole_create_from_data::OleCreateFromData;
mod ole_create_from_data_ex;
pub use ole_create_from_data_ex::OleCreateFromDataEx;
mod ole_create_from_file;
pub use ole_create_from_file::OleCreateFromFile;
mod ole_create_from_file_ex;
pub use ole_create_from_file_ex::OleCreateFromFileEx;
mod ole_create_link;
pub use ole_create_link::OleCreateLink;
mod ole_create_link_ex;
pub use ole_create_link_ex::OleCreateLinkEx;
mod ole_create_link_from_data;
pub use ole_create_link_from_data::OleCreateLinkFromData;
mod ole_create_link_from_data_ex;
pub use ole_create_link_from_data_ex::OleCreateLinkFromDataEx;
mod ole_create_link_to_file;
pub use ole_create_link_to_file::OleCreateLinkToFile;
mod ole_create_link_to_file_ex;
pub use ole_create_link_to_file_ex::OleCreateLinkToFileEx;
mod ole_create_menu_descriptor;
pub use ole_create_menu_descriptor::OleCreateMenuDescriptor;
mod ole_create_picture_indirect_ext;
pub use ole_create_picture_indirect_ext::OleCreatePictureIndirectExt;
mod ole_create_property_frame_indirect_ext;
pub use ole_create_property_frame_indirect_ext::OleCreatePropertyFrameIndirectExt;
mod ole_create_static_from_data;
pub use ole_create_static_from_data::OleCreateStaticFromData;
mod ole_destroy_menu_descriptor;
pub use ole_destroy_menu_descriptor::OleDestroyMenuDescriptor;
mod ole_do_auto_convert;
pub use ole_do_auto_convert::OleDoAutoConvert;
mod ole_draw;
pub use ole_draw::OleDraw;
mod ole_duplicate_data;
pub use ole_duplicate_data::OleDuplicateData;
mod ole_flush_clipboard;
pub use ole_flush_clipboard::OleFlushClipboard;
mod ole_get_auto_convert;
pub use ole_get_auto_convert::OleGetAutoConvert;
mod ole_get_clipboard;
pub use ole_get_clipboard::OleGetClipboard;
mod ole_get_clipboard_with_enterprise_info;
pub use ole_get_clipboard_with_enterprise_info::OleGetClipboardWithEnterpriseInfo;
mod ole_get_icon_of_class;
pub use ole_get_icon_of_class::OleGetIconOfClass;
mod ole_get_icon_of_file;
pub use ole_get_icon_of_file::OleGetIconOfFile;
mod ole_get_package_clipboard_owner;
pub use ole_get_package_clipboard_owner::OleGetPackageClipboardOwner;
mod ole_icon_to_cursor_ext;
pub use ole_icon_to_cursor_ext::OleIconToCursorExt;
mod ole_initialize_wow;
pub use ole_initialize_wow::OleInitializeWOW;
mod ole_is_current_clipboard;
pub use ole_is_current_clipboard::OleIsCurrentClipboard;
mod ole_is_running;
pub use ole_is_running::OleIsRunning;
mod ole_load;
pub use ole_load::OleLoad;
mod ole_load_from_stream;
pub use ole_load_from_stream::OleLoadFromStream;
mod ole_load_picture_ext;
pub use ole_load_picture_ext::OleLoadPictureExt;
mod ole_load_picture_file_ext;
pub use ole_load_picture_file_ext::OleLoadPictureFileExt;
mod ole_load_picture_path_ext;
pub use ole_load_picture_path_ext::OleLoadPicturePathExt;
mod ole_lock_running;
pub use ole_lock_running::OleLockRunning;
mod ole_metafile_pict_from_icon_and_label;
pub use ole_metafile_pict_from_icon_and_label::OleMetafilePictFromIconAndLabel;
mod ole_note_object_visible;
pub use ole_note_object_visible::OleNoteObjectVisible;
mod ole_query_create_from_data;
pub use ole_query_create_from_data::OleQueryCreateFromData;
mod ole_query_link_from_data;
pub use ole_query_link_from_data::OleQueryLinkFromData;
mod ole_reg_enum_format_etc;
pub use ole_reg_enum_format_etc::OleRegEnumFormatEtc;
mod ole_reg_enum_verbs;
pub use ole_reg_enum_verbs::OleRegEnumVerbs;
mod ole_reg_get_misc_status;
pub use ole_reg_get_misc_status::OleRegGetMiscStatus;
mod ole_reg_get_user_type;
pub use ole_reg_get_user_type::OleRegGetUserType;
mod ole_release_enum_verb_cache;
pub use ole_release_enum_verb_cache::OleReleaseEnumVerbCache;
mod ole_run;
pub use ole_run::OleRun;
mod ole_save;
pub use ole_save::OleSave;
mod ole_save_picture_file_ext;
pub use ole_save_picture_file_ext::OleSavePictureFileExt;
mod ole_save_to_stream;
pub use ole_save_to_stream::OleSaveToStream;
mod ole_set_auto_convert;
pub use ole_set_auto_convert::OleSetAutoConvert;
mod ole_set_clipboard;
pub use ole_set_clipboard::OleSetClipboard;
mod ole_set_contained_object;
pub use ole_set_contained_object::OleSetContainedObject;
mod ole_set_menu_descriptor;
pub use ole_set_menu_descriptor::OleSetMenuDescriptor;
mod ole_translate_accelerator;
pub use ole_translate_accelerator::OleTranslateAccelerator;
mod ole_translate_color_ext;
pub use ole_translate_color_ext::OleTranslateColorExt;
mod ole_uninitialize;
pub use ole_uninitialize::OleUninitialize;
mod open_or_create_stream;
pub use open_or_create_stream::OpenOrCreateStream;
mod prog_id_from_clsid;
pub use prog_id_from_clsid::ProgIDFromCLSID;
mod prop_stg_name_to_fmt_id;
pub use prop_stg_name_to_fmt_id::PropStgNameToFmtId;
mod prop_sys_alloc_string;
pub use prop_sys_alloc_string::PropSysAllocString;
mod prop_sys_free_string;
pub use prop_sys_free_string::PropSysFreeString;
mod prop_variant_change_type;
pub use prop_variant_change_type::PropVariantChangeType;
mod prop_variant_clear;
pub use prop_variant_clear::PropVariantClear;
mod prop_variant_copy;
pub use prop_variant_copy::PropVariantCopy;
mod read_class_stg;
pub use read_class_stg::ReadClassStg;
mod read_class_stm;
pub use read_class_stm::ReadClassStm;
mod read_fmt_user_type_stg;
pub use read_fmt_user_type_stg::ReadFmtUserTypeStg;
mod read_ole_stg;
pub use read_ole_stg::ReadOleStg;
mod read_storage_properties;
pub use read_storage_properties::ReadStorageProperties;
mod read_string_stream;
pub use read_string_stream::ReadStringStream;
mod register_active_object_ext;
pub use register_active_object_ext::RegisterActiveObjectExt;
mod register_drag_drop;
pub use register_drag_drop::RegisterDragDrop;
mod release_stg_medium;
pub use release_stg_medium::ReleaseStgMedium;
mod revoke_active_object_ext;
pub use revoke_active_object_ext::RevokeActiveObjectExt;
mod revoke_drag_drop;
pub use revoke_drag_drop::RevokeDragDrop;
mod ro_get_agile_reference;
pub use ro_get_agile_reference::RoGetAgileReference;
mod snb_user_free;
pub use snb_user_free::SNB_UserFree;
mod snb_user_free64;
pub use snb_user_free64::SNB_UserFree64;
mod snb_user_marshal;
pub use snb_user_marshal::SNB_UserMarshal;
mod snb_user_marshal64;
pub use snb_user_marshal64::SNB_UserMarshal64;
mod snb_user_size;
pub use snb_user_size::SNB_UserSize;
mod snb_user_size64;
pub use snb_user_size64::SNB_UserSize64;
mod snb_user_unmarshal;
pub use snb_user_unmarshal::SNB_UserUnmarshal;
mod snb_user_unmarshal64;
pub use snb_user_unmarshal64::SNB_UserUnmarshal64;
mod stgmedium_user_free;
pub use stgmedium_user_free::STGMEDIUM_UserFree;
mod stgmedium_user_free64;
pub use stgmedium_user_free64::STGMEDIUM_UserFree64;
mod stgmedium_user_free_ext;
pub use stgmedium_user_free_ext::STGMEDIUM_UserFreeExt;
mod stgmedium_user_marshal;
pub use stgmedium_user_marshal::STGMEDIUM_UserMarshal;
mod stgmedium_user_marshal64;
pub use stgmedium_user_marshal64::STGMEDIUM_UserMarshal64;
mod stgmedium_user_marshal_ext;
pub use stgmedium_user_marshal_ext::STGMEDIUM_UserMarshalExt;
mod stgmedium_user_size;
pub use stgmedium_user_size::STGMEDIUM_UserSize;
mod stgmedium_user_size64;
pub use stgmedium_user_size64::STGMEDIUM_UserSize64;
mod stgmedium_user_size_ext;
pub use stgmedium_user_size_ext::STGMEDIUM_UserSizeExt;
mod stgmedium_user_unmarshal;
pub use stgmedium_user_unmarshal::STGMEDIUM_UserUnmarshal;
mod stgmedium_user_unmarshal64;
pub use stgmedium_user_unmarshal64::STGMEDIUM_UserUnmarshal64;
mod stgmedium_user_unmarshal_ext;
pub use stgmedium_user_unmarshal_ext::STGMEDIUM_UserUnmarshalExt;
mod set_convert_stg;
pub use set_convert_stg::SetConvertStg;
mod set_document_bit_stg;
pub use set_document_bit_stg::SetDocumentBitStg;
mod set_error_info;
pub use set_error_info::SetErrorInfo;
mod set_oleaut_module;
pub use set_oleaut_module::SetOleautModule;
mod set_wow_thunk_global_ptr;
pub use set_wow_thunk_global_ptr::SetWOWThunkGlobalPtr;
mod std_types_get_class_object;
pub use std_types_get_class_object::StdTypesGetClassObject;
mod std_types_register_server;
pub use std_types_register_server::StdTypesRegisterServer;
mod stg_convert_property_to_variant;
pub use stg_convert_property_to_variant::StgConvertPropertyToVariant;
mod stg_convert_variant_to_property;
pub use stg_convert_variant_to_property::StgConvertVariantToProperty;
mod stg_create_docfile;
pub use stg_create_docfile::StgCreateDocfile;
mod stg_create_docfile_on_i_lock_bytes;
pub use stg_create_docfile_on_i_lock_bytes::StgCreateDocfileOnILockBytes;
mod stg_create_prop_set_stg;
pub use stg_create_prop_set_stg::StgCreatePropSetStg;
mod stg_create_prop_stg;
pub use stg_create_prop_stg::StgCreatePropStg;
mod stg_create_storage_ex;
pub use stg_create_storage_ex::StgCreateStorageEx;
mod stg_get_i_fill_lock_bytes_on_file;
pub use stg_get_i_fill_lock_bytes_on_file::StgGetIFillLockBytesOnFile;
mod stg_get_i_fill_lock_bytes_on_i_lock_bytes;
pub use stg_get_i_fill_lock_bytes_on_i_lock_bytes::StgGetIFillLockBytesOnILockBytes;
mod stg_is_storage_file;
pub use stg_is_storage_file::StgIsStorageFile;
mod stg_is_storage_i_lock_bytes;
pub use stg_is_storage_i_lock_bytes::StgIsStorageILockBytes;
mod stg_open_async_docfile_on_i_fill_lock_bytes;
pub use stg_open_async_docfile_on_i_fill_lock_bytes::StgOpenAsyncDocfileOnIFillLockBytes;
mod stg_open_prop_stg;
pub use stg_open_prop_stg::StgOpenPropStg;
mod stg_open_storage;
pub use stg_open_storage::StgOpenStorage;
mod stg_open_storage_ex;
pub use stg_open_storage_ex::StgOpenStorageEx;
mod stg_open_storage_on_handle;
pub use stg_open_storage_on_handle::StgOpenStorageOnHandle;
mod stg_open_storage_on_i_lock_bytes;
pub use stg_open_storage_on_i_lock_bytes::StgOpenStorageOnILockBytes;
mod stg_property_length_as_variant;
pub use stg_property_length_as_variant::StgPropertyLengthAsVariant;
mod stg_set_times;
pub use stg_set_times::StgSetTimes;
mod string_from_iid;
pub use string_from_iid::StringFromIID;
mod update_dcom_settings;
pub use update_dcom_settings::UpdateDCOMSettings;
mod update_process_tracing;
pub use update_process_tracing::UpdateProcessTracing;
mod ut_convert_dvtd16to_dvtd32;
pub use ut_convert_dvtd16to_dvtd32::UtConvertDvtd16toDvtd32;
mod ut_convert_dvtd32to_dvtd16;
pub use ut_convert_dvtd32to_dvtd16::UtConvertDvtd32toDvtd16;
mod ut_get_dvtd16_info;
pub use ut_get_dvtd16_info::UtGetDvtd16Info;
mod ut_get_dvtd32_info;
pub use ut_get_dvtd32_info::UtGetDvtd32Info;
mod wdtp_interface_pointer_user_free;
pub use wdtp_interface_pointer_user_free::WdtpInterfacePointer_UserFree;
mod wdtp_interface_pointer_user_free64;
pub use wdtp_interface_pointer_user_free64::WdtpInterfacePointer_UserFree64;
mod wdtp_interface_pointer_user_marshal;
pub use wdtp_interface_pointer_user_marshal::WdtpInterfacePointer_UserMarshal;
mod wdtp_interface_pointer_user_marshal64;
pub use wdtp_interface_pointer_user_marshal64::WdtpInterfacePointer_UserMarshal64;
mod wdtp_interface_pointer_user_size;
pub use wdtp_interface_pointer_user_size::WdtpInterfacePointer_UserSize;
mod wdtp_interface_pointer_user_size64;
pub use wdtp_interface_pointer_user_size64::WdtpInterfacePointer_UserSize64;
mod wdtp_interface_pointer_user_unmarshal;
pub use wdtp_interface_pointer_user_unmarshal::WdtpInterfacePointer_UserUnmarshal;
mod wdtp_interface_pointer_user_unmarshal64;
pub use wdtp_interface_pointer_user_unmarshal64::WdtpInterfacePointer_UserUnmarshal64;
mod write_class_stg;
pub use write_class_stg::WriteClassStg;
mod write_class_stm;
pub use write_class_stm::WriteClassStm;
mod write_fmt_user_type_stg;
pub use write_fmt_user_type_stg::WriteFmtUserTypeStg;
mod write_ole_stg;
pub use write_ole_stg::WriteOleStg;
mod write_storage_properties;
pub use write_storage_properties::WriteStorageProperties;
mod write_string_stream;
pub use write_string_stream::WriteStringStream;
mod clsid_from_string;
pub use clsid_from_string::CLSIDFromString;
mod co_create_guid;
pub use co_create_guid::CoCreateGuid;
mod co_create_instance;
pub use co_create_instance::CoCreateInstance;
mod co_initialize_ex;
pub use co_initialize_ex::CoInitializeEx;
mod co_task_mem_alloc;
pub use co_task_mem_alloc::CoTaskMemAlloc;
mod co_task_mem_free;
pub use co_task_mem_free::CoTaskMemFree;
mod co_uninitialize;
pub use co_uninitialize::CoUninitialize;
mod ole_initialize;
pub use ole_initialize::OleInitialize;
mod string_from_clsid;
pub use string_from_clsid::StringFromCLSID;
mod string_from_guid2;
pub use string_from_guid2::StringFromGUID2;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "OleInitialize" => OleInitialize(emu),
        "CoInitialize" => CoInitialize(emu),
        "CoInitializeEx" => CoInitializeEx(emu),
        "CoCreateInstance" => CoCreateInstance(emu),
        "CoTaskMemAlloc" => CoTaskMemAlloc(emu),
        "CoTaskMemFree" => CoTaskMemFree(emu),
        "CoUninitialize" => CoUninitialize(emu),
        "CoResumeClassObjects" => CoResumeClassObjects(emu),
        "CoAddRefServerProcess" => CoAddRefServerProcess(emu),
        "CoReleaseServerProcess" => CoReleaseServerProcess(emu),
        "CreateClassMoniker" => CreateClassMoniker(emu),
        "GetRunningObjectTable" => GetRunningObjectTable(emu),
        "BindMoniker" => BindMoniker(emu),
        "CLIPFORMAT_UserFree" => CLIPFORMAT_UserFree(emu),
        "CLIPFORMAT_UserFree64" => CLIPFORMAT_UserFree64(emu),
        "CLIPFORMAT_UserFreeExt" => CLIPFORMAT_UserFreeExt(emu),
        "CLIPFORMAT_UserMarshal" => CLIPFORMAT_UserMarshal(emu),
        "CLIPFORMAT_UserMarshal64" => CLIPFORMAT_UserMarshal64(emu),
        "CLIPFORMAT_UserMarshalExt" => CLIPFORMAT_UserMarshalExt(emu),
        "CLIPFORMAT_UserSize" => CLIPFORMAT_UserSize(emu),
        "CLIPFORMAT_UserSize64" => CLIPFORMAT_UserSize64(emu),
        "CLIPFORMAT_UserSizeExt" => CLIPFORMAT_UserSizeExt(emu),
        "CLIPFORMAT_UserUnmarshal" => CLIPFORMAT_UserUnmarshal(emu),
        "CLIPFORMAT_UserUnmarshal64" => CLIPFORMAT_UserUnmarshal64(emu),
        "CLIPFORMAT_UserUnmarshalExt" => CLIPFORMAT_UserUnmarshalExt(emu),
        "CLSIDFromOle1Class" => CLSIDFromOle1Class(emu),
        "CLSIDFromProgID" => CLSIDFromProgID(emu),
        "CLSIDFromProgIDEx" => CLSIDFromProgIDEx(emu),
        "CLSIDFromString" => CLSIDFromString(emu),
        "CStdAsyncStubBuffer2_Connect" => CStdAsyncStubBuffer2_Connect(emu),
        "CStdAsyncStubBuffer2_Disconnect" => CStdAsyncStubBuffer2_Disconnect(emu),
        "CStdAsyncStubBuffer2_Release" => CStdAsyncStubBuffer2_Release(emu),
        "CStdAsyncStubBuffer_AddRef" => CStdAsyncStubBuffer_AddRef(emu),
        "CStdAsyncStubBuffer_Connect" => CStdAsyncStubBuffer_Connect(emu),
        "CStdAsyncStubBuffer_Disconnect" => CStdAsyncStubBuffer_Disconnect(emu),
        "CStdAsyncStubBuffer_Invoke" => CStdAsyncStubBuffer_Invoke(emu),
        "CStdAsyncStubBuffer_QueryInterface" => CStdAsyncStubBuffer_QueryInterface(emu),
        "CStdAsyncStubBuffer_Release" => CStdAsyncStubBuffer_Release(emu),
        "CStdStubBuffer2_Connect" => CStdStubBuffer2_Connect(emu),
        "CStdStubBuffer2_CountRefs" => CStdStubBuffer2_CountRefs(emu),
        "CStdStubBuffer2_Disconnect" => CStdStubBuffer2_Disconnect(emu),
        "CStdStubBuffer2_QueryInterface" => CStdStubBuffer2_QueryInterface(emu),
        "CheckInitDde" => CheckInitDde(emu),
        "CleanROTForApartment" => CleanROTForApartment(emu),
        "ClipboardProcessUninitialize" => ClipboardProcessUninitialize(emu),
        "CoAicGetTokenForCOM" => CoAicGetTokenForCOM(emu),
        "CoAllowSetForegroundWindow" => CoAllowSetForegroundWindow(emu),
        "CoAllowUnmarshalerCLSID" => CoAllowUnmarshalerCLSID(emu),
        "CoBuildVersion" => CoBuildVersion(emu),
        "CoCancelCall" => CoCancelCall(emu),
        "CoCheckElevationEnabled" => CoCheckElevationEnabled(emu),
        "CoCopyProxy" => CoCopyProxy(emu),
        "CoCreateFreeThreadedMarshaler" => CoCreateFreeThreadedMarshaler(emu),
        "CoCreateGuid" => CoCreateGuid(emu),
        "CoCreateInstanceEx" => CoCreateInstanceEx(emu),
        "CoCreateInstanceFromApp" => CoCreateInstanceFromApp(emu),
        "CoCreateObjectInContext" => CoCreateObjectInContext(emu),
        "CoDeactivateObject" => CoDeactivateObject(emu),
        "CoDecodeProxy" => CoDecodeProxy(emu),
        "CoDecrementMTAUsage" => CoDecrementMTAUsage(emu),
        "CoDisableCallCancellation" => CoDisableCallCancellation(emu),
        "CoDisconnectContext" => CoDisconnectContext(emu),
        "CoDisconnectObject" => CoDisconnectObject(emu),
        "CoDosDateTimeToFileTime" => CoDosDateTimeToFileTime(emu),
        "CoEnableCallCancellation" => CoEnableCallCancellation(emu),
        "CoFileTimeNow" => CoFileTimeNow(emu),
        "CoFileTimeToDosDateTime" => CoFileTimeToDosDateTime(emu),
        "CoFreeAllLibraries" => CoFreeAllLibraries(emu),
        "CoFreeLibrary" => CoFreeLibrary(emu),
        "CoFreeUnusedLibraries" => CoFreeUnusedLibraries(emu),
        "CoFreeUnusedLibrariesEx" => CoFreeUnusedLibrariesEx(emu),
        "CoGetActivationState" => CoGetActivationState(emu),
        "CoGetApartmentID" => CoGetApartmentID(emu),
        "CoGetApartmentType" => CoGetApartmentType(emu),
        "CoGetCallContext" => CoGetCallContext(emu),
        "CoGetCallState" => CoGetCallState(emu),
        "CoGetCallerTID" => CoGetCallerTID(emu),
        "CoGetCancelObject" => CoGetCancelObject(emu),
        "CoGetClassObject" => CoGetClassObject(emu),
        "CoGetClassVersion" => CoGetClassVersion(emu),
        "CoGetComCatalog" => CoGetComCatalog(emu),
        "CoGetContextToken" => CoGetContextToken(emu),
        "CoGetCurrentLogicalThreadId" => CoGetCurrentLogicalThreadId(emu),
        "CoGetCurrentProcess" => CoGetCurrentProcess(emu),
        "CoGetDefaultContext" => CoGetDefaultContext(emu),
        "CoGetInstanceFromFile" => CoGetInstanceFromFile(emu),
        "CoGetInstanceFromIStorage" => CoGetInstanceFromIStorage(emu),
        "CoGetInterceptor" => CoGetInterceptor(emu),
        "CoGetInterceptorForOle32" => CoGetInterceptorForOle32(emu),
        "CoGetInterceptorFromTypeInfo" => CoGetInterceptorFromTypeInfo(emu),
        "CoGetInterfaceAndReleaseStream" => CoGetInterfaceAndReleaseStream(emu),
        "CoGetMalloc" => CoGetMalloc(emu),
        "CoGetMarshalSizeMax" => CoGetMarshalSizeMax(emu),
        "CoGetModuleType" => CoGetModuleType(emu),
        "CoGetObject" => CoGetObject(emu),
        "CoGetObjectContext" => CoGetObjectContext(emu),
        "CoGetPSClsid" => CoGetPSClsid(emu),
        "CoGetProcessIdentifier" => CoGetProcessIdentifier(emu),
        "CoGetStandardMarshal" => CoGetStandardMarshal(emu),
        "CoGetStdMarshalEx" => CoGetStdMarshalEx(emu),
        "CoGetSystemSecurityPermissions" => CoGetSystemSecurityPermissions(emu),
        "CoGetSystemWow64DirectoryW" => CoGetSystemWow64DirectoryW(emu),
        "CoGetTreatAsClass" => CoGetTreatAsClass(emu),
        "CoHandlePriorityEventsFromMessagePump" => CoHandlePriorityEventsFromMessagePump(emu),
        "CoImpersonateClient" => CoImpersonateClient(emu),
        "CoIncrementMTAUsage" => CoIncrementMTAUsage(emu),
        "CoInitializeSecurity" => CoInitializeSecurity(emu),
        "CoInitializeWOW" => CoInitializeWOW(emu),
        "CoInstall" => CoInstall(emu),
        "CoInvalidateRemoteMachineBindings" => CoInvalidateRemoteMachineBindings(emu),
        "CoIsHandlerConnected" => CoIsHandlerConnected(emu),
        "CoIsOle1Class" => CoIsOle1Class(emu),
        "CoLoadLibrary" => CoLoadLibrary(emu),
        "CoLockObjectExternal" => CoLockObjectExternal(emu),
        "CoMarshalHresult" => CoMarshalHresult(emu),
        "CoMarshalInterThreadInterfaceInStream" => CoMarshalInterThreadInterfaceInStream(emu),
        "CoMarshalInterface" => CoMarshalInterface(emu),
        "CoPopServiceDomain" => CoPopServiceDomain(emu),
        "CoPushServiceDomain" => CoPushServiceDomain(emu),
        "CoQueryAuthenticationServices" => CoQueryAuthenticationServices(emu),
        "CoQueryClientBlanket" => CoQueryClientBlanket(emu),
        "CoQueryProxyBlanket" => CoQueryProxyBlanket(emu),
        "CoQueryReleaseObject" => CoQueryReleaseObject(emu),
        "CoReactivateObject" => CoReactivateObject(emu),
        "CoRegisterActivationFilter" => CoRegisterActivationFilter(emu),
        "CoRegisterChannelHook" => CoRegisterChannelHook(emu),
        "CoRegisterClassObject" => CoRegisterClassObject(emu),
        "CoRegisterInitializeSpy" => CoRegisterInitializeSpy(emu),
        "CoRegisterMallocSpy" => CoRegisterMallocSpy(emu),
        "CoRegisterMessageFilter" => CoRegisterMessageFilter(emu),
        "CoRegisterPSClsid" => CoRegisterPSClsid(emu),
        "CoRegisterSurrogate" => CoRegisterSurrogate(emu),
        "CoRegisterSurrogateEx" => CoRegisterSurrogateEx(emu),
        "CoReleaseMarshalData" => CoReleaseMarshalData(emu),
        "CoRetireServer" => CoRetireServer(emu),
        "CoRevertToSelf" => CoRevertToSelf(emu),
        "CoRevokeClassObject" => CoRevokeClassObject(emu),
        "CoRevokeInitializeSpy" => CoRevokeInitializeSpy(emu),
        "CoRevokeMallocSpy" => CoRevokeMallocSpy(emu),
        "CoSetCancelObject" => CoSetCancelObject(emu),
        "CoSetMessageDispatcher" => CoSetMessageDispatcher(emu),
        "CoSetProxyBlanket" => CoSetProxyBlanket(emu),
        "CoSetState" => CoSetState(emu),
        "CoSuspendClassObjects" => CoSuspendClassObjects(emu),
        "CoSwitchCallContext" => CoSwitchCallContext(emu),
        "CoTaskMemRealloc" => CoTaskMemRealloc(emu),
        "CoTestCancel" => CoTestCancel(emu),
        "CoTreatAsClass" => CoTreatAsClass(emu),
        "CoTryGetInterceptorForAppModelVirtualization" => {
            CoTryGetInterceptorForAppModelVirtualization(emu)
        }
        "CoUnloadingWOW" => CoUnloadingWOW(emu),
        "CoUnmarshalHresult" => CoUnmarshalHresult(emu),
        "CoUnmarshalInterface" => CoUnmarshalInterface(emu),
        "CoVrfCheckThreadState" => CoVrfCheckThreadState(emu),
        "CoVrfGetThreadState" => CoVrfGetThreadState(emu),
        "CoVrfReleaseThreadState" => CoVrfReleaseThreadState(emu),
        "CoWaitForMultipleHandles" => CoWaitForMultipleHandles(emu),
        "CoWaitForMultipleObjects" => CoWaitForMultipleObjects(emu),
        "ComPs_NdrDllCanUnloadNow" => ComPs_NdrDllCanUnloadNow(emu),
        "ComPs_NdrDllGetClassObject" => ComPs_NdrDllGetClassObject(emu),
        "ComPs_NdrDllRegisterProxy" => ComPs_NdrDllRegisterProxy(emu),
        "ComPs_NdrDllUnregisterProxy" => ComPs_NdrDllUnregisterProxy(emu),
        "CreateAntiMoniker" => CreateAntiMoniker(emu),
        "CreateBindCtx" => CreateBindCtx(emu),
        "CreateDataAdviseHolder" => CreateDataAdviseHolder(emu),
        "CreateDataCache" => CreateDataCache(emu),
        "CreateErrorInfo" => CreateErrorInfo(emu),
        "CreateFileMoniker" => CreateFileMoniker(emu),
        "CreateGenericComposite" => CreateGenericComposite(emu),
        "CreateILockBytesOnHGlobal" => CreateILockBytesOnHGlobal(emu),
        "CreateItemMoniker" => CreateItemMoniker(emu),
        "CreateObjrefMoniker" => CreateObjrefMoniker(emu),
        "CreateOleAdviseHolder" => CreateOleAdviseHolder(emu),
        "CreatePointerMoniker" => CreatePointerMoniker(emu),
        "CreateStdProgressIndicator" => CreateStdProgressIndicator(emu),
        "CreateStreamOnHGlobal" => CreateStreamOnHGlobal(emu),
        "DcomChannelSetHResult" => DcomChannelSetHResult(emu),
        "DdeBindToObject" => DdeBindToObject(emu),
        "DeletePatternAndExtensionTables" => DeletePatternAndExtensionTables(emu),
        "DestroyRunningObjectTable" => DestroyRunningObjectTable(emu),
        "DllDebugObjectRPCHook" => DllDebugObjectRPCHook(emu),
        "DllGetClassObject" => DllGetClassObject(emu),
        "DllGetClassObjectWOW" => DllGetClassObjectWOW(emu),
        "DllRegisterServer" => DllRegisterServer(emu),
        "DoDragDrop" => DoDragDrop(emu),
        "DragDropSetFDT" => DragDropSetFDT(emu),
        "EnableHookObject" => EnableHookObject(emu),
        "FindExt" => FindExt(emu),
        "FmtIdToPropStgName" => FmtIdToPropStgName(emu),
        "FreePropVariantArray" => FreePropVariantArray(emu),
        "GetActiveObjectExt" => GetActiveObjectExt(emu),
        "GetClassFile" => GetClassFile(emu),
        "GetConvertStg" => GetConvertStg(emu),
        "GetDocumentBitStg" => GetDocumentBitStg(emu),
        "GetErrorInfo" => GetErrorInfo(emu),
        "GetHGlobalFromILockBytes" => GetHGlobalFromILockBytes(emu),
        "GetHGlobalFromStream" => GetHGlobalFromStream(emu),
        "GetHookInterface" => GetHookInterface(emu),
        "GetObjectFromRotByPath" => GetObjectFromRotByPath(emu),
        "HACCEL_UserFree" => HACCEL_UserFree(emu),
        "HACCEL_UserFree64" => HACCEL_UserFree64(emu),
        "HACCEL_UserMarshal" => HACCEL_UserMarshal(emu),
        "HACCEL_UserMarshal64" => HACCEL_UserMarshal64(emu),
        "HACCEL_UserSize" => HACCEL_UserSize(emu),
        "HACCEL_UserSize64" => HACCEL_UserSize64(emu),
        "HACCEL_UserUnmarshal" => HACCEL_UserUnmarshal(emu),
        "HACCEL_UserUnmarshal64" => HACCEL_UserUnmarshal64(emu),
        "HBITMAP_UserFree" => HBITMAP_UserFree(emu),
        "HBITMAP_UserFree64" => HBITMAP_UserFree64(emu),
        "HBITMAP_UserMarshal" => HBITMAP_UserMarshal(emu),
        "HBITMAP_UserMarshal64" => HBITMAP_UserMarshal64(emu),
        "HBITMAP_UserSize" => HBITMAP_UserSize(emu),
        "HBITMAP_UserSize64" => HBITMAP_UserSize64(emu),
        "HBITMAP_UserUnmarshal" => HBITMAP_UserUnmarshal(emu),
        "HBITMAP_UserUnmarshal64" => HBITMAP_UserUnmarshal64(emu),
        "HBRUSH_UserFree" => HBRUSH_UserFree(emu),
        "HBRUSH_UserFree64" => HBRUSH_UserFree64(emu),
        "HBRUSH_UserMarshal" => HBRUSH_UserMarshal(emu),
        "HBRUSH_UserMarshal64" => HBRUSH_UserMarshal64(emu),
        "HBRUSH_UserSize" => HBRUSH_UserSize(emu),
        "HBRUSH_UserSize64" => HBRUSH_UserSize64(emu),
        "HBRUSH_UserUnmarshal" => HBRUSH_UserUnmarshal(emu),
        "HBRUSH_UserUnmarshal64" => HBRUSH_UserUnmarshal64(emu),
        "HDC_UserFree" => HDC_UserFree(emu),
        "HDC_UserFree64" => HDC_UserFree64(emu),
        "HDC_UserMarshal" => HDC_UserMarshal(emu),
        "HDC_UserMarshal64" => HDC_UserMarshal64(emu),
        "HDC_UserSize" => HDC_UserSize(emu),
        "HDC_UserSize64" => HDC_UserSize64(emu),
        "HDC_UserUnmarshal" => HDC_UserUnmarshal(emu),
        "HDC_UserUnmarshal64" => HDC_UserUnmarshal64(emu),
        "HENHMETAFILE_UserFree" => HENHMETAFILE_UserFree(emu),
        "HENHMETAFILE_UserFree64" => HENHMETAFILE_UserFree64(emu),
        "HENHMETAFILE_UserMarshal" => HENHMETAFILE_UserMarshal(emu),
        "HENHMETAFILE_UserMarshal64" => HENHMETAFILE_UserMarshal64(emu),
        "HENHMETAFILE_UserSize" => HENHMETAFILE_UserSize(emu),
        "HENHMETAFILE_UserSize64" => HENHMETAFILE_UserSize64(emu),
        "HENHMETAFILE_UserUnmarshal" => HENHMETAFILE_UserUnmarshal(emu),
        "HENHMETAFILE_UserUnmarshal64" => HENHMETAFILE_UserUnmarshal64(emu),
        "HGLOBAL_UserFree" => HGLOBAL_UserFree(emu),
        "HGLOBAL_UserFree64" => HGLOBAL_UserFree64(emu),
        "HGLOBAL_UserMarshal" => HGLOBAL_UserMarshal(emu),
        "HGLOBAL_UserMarshal64" => HGLOBAL_UserMarshal64(emu),
        "HGLOBAL_UserSize" => HGLOBAL_UserSize(emu),
        "HGLOBAL_UserSize64" => HGLOBAL_UserSize64(emu),
        "HGLOBAL_UserUnmarshal" => HGLOBAL_UserUnmarshal(emu),
        "HGLOBAL_UserUnmarshal64" => HGLOBAL_UserUnmarshal64(emu),
        "HICON_UserFree" => HICON_UserFree(emu),
        "HICON_UserFree64" => HICON_UserFree64(emu),
        "HICON_UserMarshal" => HICON_UserMarshal(emu),
        "HICON_UserMarshal64" => HICON_UserMarshal64(emu),
        "HICON_UserSize" => HICON_UserSize(emu),
        "HICON_UserSize64" => HICON_UserSize64(emu),
        "HICON_UserUnmarshal" => HICON_UserUnmarshal(emu),
        "HICON_UserUnmarshal64" => HICON_UserUnmarshal64(emu),
        "HMENU_UserFree" => HMENU_UserFree(emu),
        "HMENU_UserFree64" => HMENU_UserFree64(emu),
        "HMENU_UserMarshal" => HMENU_UserMarshal(emu),
        "HMENU_UserMarshal64" => HMENU_UserMarshal64(emu),
        "HMENU_UserSize" => HMENU_UserSize(emu),
        "HMENU_UserSize64" => HMENU_UserSize64(emu),
        "HMENU_UserUnmarshal" => HMENU_UserUnmarshal(emu),
        "HMENU_UserUnmarshal64" => HMENU_UserUnmarshal64(emu),
        "HMETAFILEPICT_UserFree" => HMETAFILEPICT_UserFree(emu),
        "HMETAFILEPICT_UserFree64" => HMETAFILEPICT_UserFree64(emu),
        "HMETAFILEPICT_UserMarshal" => HMETAFILEPICT_UserMarshal(emu),
        "HMETAFILEPICT_UserMarshal64" => HMETAFILEPICT_UserMarshal64(emu),
        "HMETAFILEPICT_UserSize" => HMETAFILEPICT_UserSize(emu),
        "HMETAFILEPICT_UserSize64" => HMETAFILEPICT_UserSize64(emu),
        "HMETAFILEPICT_UserUnmarshal" => HMETAFILEPICT_UserUnmarshal(emu),
        "HMETAFILEPICT_UserUnmarshal64" => HMETAFILEPICT_UserUnmarshal64(emu),
        "HMETAFILE_UserFree" => HMETAFILE_UserFree(emu),
        "HMETAFILE_UserFree64" => HMETAFILE_UserFree64(emu),
        "HMETAFILE_UserMarshal" => HMETAFILE_UserMarshal(emu),
        "HMETAFILE_UserMarshal64" => HMETAFILE_UserMarshal64(emu),
        "HMETAFILE_UserSize" => HMETAFILE_UserSize(emu),
        "HMETAFILE_UserSize64" => HMETAFILE_UserSize64(emu),
        "HMETAFILE_UserUnmarshal" => HMETAFILE_UserUnmarshal(emu),
        "HMETAFILE_UserUnmarshal64" => HMETAFILE_UserUnmarshal64(emu),
        "HMONITOR_UserFree" => HMONITOR_UserFree(emu),
        "HMONITOR_UserFree64" => HMONITOR_UserFree64(emu),
        "HMONITOR_UserMarshal" => HMONITOR_UserMarshal(emu),
        "HMONITOR_UserMarshal64" => HMONITOR_UserMarshal64(emu),
        "HMONITOR_UserSize" => HMONITOR_UserSize(emu),
        "HMONITOR_UserSize64" => HMONITOR_UserSize64(emu),
        "HMONITOR_UserUnmarshal" => HMONITOR_UserUnmarshal(emu),
        "HMONITOR_UserUnmarshal64" => HMONITOR_UserUnmarshal64(emu),
        "HPALETTE_UserFree" => HPALETTE_UserFree(emu),
        "HPALETTE_UserFree64" => HPALETTE_UserFree64(emu),
        "HPALETTE_UserFreeExt" => HPALETTE_UserFreeExt(emu),
        "HPALETTE_UserMarshal" => HPALETTE_UserMarshal(emu),
        "HPALETTE_UserMarshal64" => HPALETTE_UserMarshal64(emu),
        "HPALETTE_UserMarshalExt" => HPALETTE_UserMarshalExt(emu),
        "HPALETTE_UserSize" => HPALETTE_UserSize(emu),
        "HPALETTE_UserSize64" => HPALETTE_UserSize64(emu),
        "HPALETTE_UserSizeExt" => HPALETTE_UserSizeExt(emu),
        "HPALETTE_UserUnmarshal" => HPALETTE_UserUnmarshal(emu),
        "HPALETTE_UserUnmarshal64" => HPALETTE_UserUnmarshal64(emu),
        "HPALETTE_UserUnmarshalExt" => HPALETTE_UserUnmarshalExt(emu),
        "HRGN_UserFree" => HRGN_UserFree(emu),
        "HRGN_UserMarshal" => HRGN_UserMarshal(emu),
        "HRGN_UserSize" => HRGN_UserSize(emu),
        "HRGN_UserUnmarshal" => HRGN_UserUnmarshal(emu),
        "HWND_UserFree" => HWND_UserFree(emu),
        "HWND_UserFree64" => HWND_UserFree64(emu),
        "HWND_UserFree64Ext" => HWND_UserFree64Ext(emu),
        "HWND_UserFreeExt" => HWND_UserFreeExt(emu),
        "HWND_UserMarshal" => HWND_UserMarshal(emu),
        "HWND_UserMarshal64" => HWND_UserMarshal64(emu),
        "HWND_UserMarshal64Ext" => HWND_UserMarshal64Ext(emu),
        "HWND_UserMarshalExt" => HWND_UserMarshalExt(emu),
        "HWND_UserSize" => HWND_UserSize(emu),
        "HWND_UserSize64" => HWND_UserSize64(emu),
        "HWND_UserSize64Ext" => HWND_UserSize64Ext(emu),
        "HWND_UserSizeExt" => HWND_UserSizeExt(emu),
        "HWND_UserUnmarshal" => HWND_UserUnmarshal(emu),
        "HWND_UserUnmarshal64" => HWND_UserUnmarshal64(emu),
        "HWND_UserUnmarshal64Ext" => HWND_UserUnmarshal64Ext(emu),
        "HWND_UserUnmarshalExt" => HWND_UserUnmarshalExt(emu),
        "HkOleRegisterObject" => HkOleRegisterObject(emu),
        "IIDFromString" => IIDFromString(emu),
        "IsAccelerator" => IsAccelerator(emu),
        "IsEqualGUID" => IsEqualGUID(emu),
        "IsRoInitializeASTAAllowedInDesktop" => IsRoInitializeASTAAllowedInDesktop(emu),
        "IsValidIid" => IsValidIid(emu),
        "IsValidInterface" => IsValidInterface(emu),
        "IsValidPtrIn" => IsValidPtrIn(emu),
        "IsValidPtrOut" => IsValidPtrOut(emu),
        "MkParseDisplayName" => MkParseDisplayName(emu),
        "MonikerCommonPrefixWith" => MonikerCommonPrefixWith(emu),
        "MonikerLoadTypeLib" => MonikerLoadTypeLib(emu),
        "MonikerRelativePathTo" => MonikerRelativePathTo(emu),
        "NdrOleInitializeExtension" => NdrOleInitializeExtension(emu),
        "NdrProxyForwardingFunction10" => NdrProxyForwardingFunction10(emu),
        "NdrProxyForwardingFunction11" => NdrProxyForwardingFunction11(emu),
        "NdrProxyForwardingFunction12" => NdrProxyForwardingFunction12(emu),
        "NdrProxyForwardingFunction13" => NdrProxyForwardingFunction13(emu),
        "NdrProxyForwardingFunction14" => NdrProxyForwardingFunction14(emu),
        "NdrProxyForwardingFunction15" => NdrProxyForwardingFunction15(emu),
        "NdrProxyForwardingFunction16" => NdrProxyForwardingFunction16(emu),
        "NdrProxyForwardingFunction17" => NdrProxyForwardingFunction17(emu),
        "NdrProxyForwardingFunction18" => NdrProxyForwardingFunction18(emu),
        "NdrProxyForwardingFunction19" => NdrProxyForwardingFunction19(emu),
        "NdrProxyForwardingFunction20" => NdrProxyForwardingFunction20(emu),
        "NdrProxyForwardingFunction21" => NdrProxyForwardingFunction21(emu),
        "NdrProxyForwardingFunction22" => NdrProxyForwardingFunction22(emu),
        "NdrProxyForwardingFunction23" => NdrProxyForwardingFunction23(emu),
        "NdrProxyForwardingFunction24" => NdrProxyForwardingFunction24(emu),
        "NdrProxyForwardingFunction25" => NdrProxyForwardingFunction25(emu),
        "NdrProxyForwardingFunction26" => NdrProxyForwardingFunction26(emu),
        "NdrProxyForwardingFunction27" => NdrProxyForwardingFunction27(emu),
        "NdrProxyForwardingFunction28" => NdrProxyForwardingFunction28(emu),
        "NdrProxyForwardingFunction29" => NdrProxyForwardingFunction29(emu),
        "NdrProxyForwardingFunction3" => NdrProxyForwardingFunction3(emu),
        "NdrProxyForwardingFunction30" => NdrProxyForwardingFunction30(emu),
        "NdrProxyForwardingFunction31" => NdrProxyForwardingFunction31(emu),
        "NdrProxyForwardingFunction32" => NdrProxyForwardingFunction32(emu),
        "NdrProxyForwardingFunction4" => NdrProxyForwardingFunction4(emu),
        "NdrProxyForwardingFunction5" => NdrProxyForwardingFunction5(emu),
        "NdrProxyForwardingFunction6" => NdrProxyForwardingFunction6(emu),
        "NdrProxyForwardingFunction7" => NdrProxyForwardingFunction7(emu),
        "NdrProxyForwardingFunction8" => NdrProxyForwardingFunction8(emu),
        "NdrProxyForwardingFunction9" => NdrProxyForwardingFunction9(emu),
        "ObjectStublessClient10" => ObjectStublessClient10(emu),
        "ObjectStublessClient11" => ObjectStublessClient11(emu),
        "ObjectStublessClient12" => ObjectStublessClient12(emu),
        "ObjectStublessClient13" => ObjectStublessClient13(emu),
        "ObjectStublessClient14" => ObjectStublessClient14(emu),
        "ObjectStublessClient15" => ObjectStublessClient15(emu),
        "ObjectStublessClient16" => ObjectStublessClient16(emu),
        "ObjectStublessClient17" => ObjectStublessClient17(emu),
        "ObjectStublessClient18" => ObjectStublessClient18(emu),
        "ObjectStublessClient19" => ObjectStublessClient19(emu),
        "ObjectStublessClient20" => ObjectStublessClient20(emu),
        "ObjectStublessClient21" => ObjectStublessClient21(emu),
        "ObjectStublessClient22" => ObjectStublessClient22(emu),
        "ObjectStublessClient23" => ObjectStublessClient23(emu),
        "ObjectStublessClient24" => ObjectStublessClient24(emu),
        "ObjectStublessClient25" => ObjectStublessClient25(emu),
        "ObjectStublessClient26" => ObjectStublessClient26(emu),
        "ObjectStublessClient27" => ObjectStublessClient27(emu),
        "ObjectStublessClient28" => ObjectStublessClient28(emu),
        "ObjectStublessClient29" => ObjectStublessClient29(emu),
        "ObjectStublessClient3" => ObjectStublessClient3(emu),
        "ObjectStublessClient30" => ObjectStublessClient30(emu),
        "ObjectStublessClient31" => ObjectStublessClient31(emu),
        "ObjectStublessClient32" => ObjectStublessClient32(emu),
        "ObjectStublessClient4" => ObjectStublessClient4(emu),
        "ObjectStublessClient5" => ObjectStublessClient5(emu),
        "ObjectStublessClient6" => ObjectStublessClient6(emu),
        "ObjectStublessClient7" => ObjectStublessClient7(emu),
        "ObjectStublessClient8" => ObjectStublessClient8(emu),
        "ObjectStublessClient9" => ObjectStublessClient9(emu),
        "Ole32DllGetClassObject" => Ole32DllGetClassObject(emu),
        "OleBuildVersion" => OleBuildVersion(emu),
        "OleConvertIStorageToOLESTREAM" => OleConvertIStorageToOLESTREAM(emu),
        "OleConvertIStorageToOLESTREAMEx" => OleConvertIStorageToOLESTREAMEx(emu),
        "OleConvertOLESTREAMToIStorage" => OleConvertOLESTREAMToIStorage(emu),
        "OleConvertOLESTREAMToIStorage2" => OleConvertOLESTREAMToIStorage2(emu),
        "OleConvertOLESTREAMToIStorageEx" => OleConvertOLESTREAMToIStorageEx(emu),
        "OleConvertOLESTREAMToIStorageEx2" => OleConvertOLESTREAMToIStorageEx2(emu),
        "OleCreate" => OleCreate(emu),
        "OleCreateDefaultHandler" => OleCreateDefaultHandler(emu),
        "OleCreateEmbeddingHelper" => OleCreateEmbeddingHelper(emu),
        "OleCreateEx" => OleCreateEx(emu),
        "OleCreateFontIndirectExt" => OleCreateFontIndirectExt(emu),
        "OleCreateFromData" => OleCreateFromData(emu),
        "OleCreateFromDataEx" => OleCreateFromDataEx(emu),
        "OleCreateFromFile" => OleCreateFromFile(emu),
        "OleCreateFromFileEx" => OleCreateFromFileEx(emu),
        "OleCreateLink" => OleCreateLink(emu),
        "OleCreateLinkEx" => OleCreateLinkEx(emu),
        "OleCreateLinkFromData" => OleCreateLinkFromData(emu),
        "OleCreateLinkFromDataEx" => OleCreateLinkFromDataEx(emu),
        "OleCreateLinkToFile" => OleCreateLinkToFile(emu),
        "OleCreateLinkToFileEx" => OleCreateLinkToFileEx(emu),
        "OleCreateMenuDescriptor" => OleCreateMenuDescriptor(emu),
        "OleCreatePictureIndirectExt" => OleCreatePictureIndirectExt(emu),
        "OleCreatePropertyFrameIndirectExt" => OleCreatePropertyFrameIndirectExt(emu),
        "OleCreateStaticFromData" => OleCreateStaticFromData(emu),
        "OleDestroyMenuDescriptor" => OleDestroyMenuDescriptor(emu),
        "OleDoAutoConvert" => OleDoAutoConvert(emu),
        "OleDraw" => OleDraw(emu),
        "OleDuplicateData" => OleDuplicateData(emu),
        "OleFlushClipboard" => OleFlushClipboard(emu),
        "OleGetAutoConvert" => OleGetAutoConvert(emu),
        "OleGetClipboard" => OleGetClipboard(emu),
        "OleGetClipboardWithEnterpriseInfo" => OleGetClipboardWithEnterpriseInfo(emu),
        "OleGetIconOfClass" => OleGetIconOfClass(emu),
        "OleGetIconOfFile" => OleGetIconOfFile(emu),
        "OleGetPackageClipboardOwner" => OleGetPackageClipboardOwner(emu),
        "OleIconToCursorExt" => OleIconToCursorExt(emu),
        "OleInitializeWOW" => OleInitializeWOW(emu),
        "OleIsCurrentClipboard" => OleIsCurrentClipboard(emu),
        "OleIsRunning" => OleIsRunning(emu),
        "OleLoad" => OleLoad(emu),
        "OleLoadFromStream" => OleLoadFromStream(emu),
        "OleLoadPictureExt" => OleLoadPictureExt(emu),
        "OleLoadPictureFileExt" => OleLoadPictureFileExt(emu),
        "OleLoadPicturePathExt" => OleLoadPicturePathExt(emu),
        "OleLockRunning" => OleLockRunning(emu),
        "OleMetafilePictFromIconAndLabel" => OleMetafilePictFromIconAndLabel(emu),
        "OleNoteObjectVisible" => OleNoteObjectVisible(emu),
        "OleQueryCreateFromData" => OleQueryCreateFromData(emu),
        "OleQueryLinkFromData" => OleQueryLinkFromData(emu),
        "OleRegEnumFormatEtc" => OleRegEnumFormatEtc(emu),
        "OleRegEnumVerbs" => OleRegEnumVerbs(emu),
        "OleRegGetMiscStatus" => OleRegGetMiscStatus(emu),
        "OleRegGetUserType" => OleRegGetUserType(emu),
        "OleReleaseEnumVerbCache" => OleReleaseEnumVerbCache(emu),
        "OleRun" => OleRun(emu),
        "OleSave" => OleSave(emu),
        "OleSavePictureFileExt" => OleSavePictureFileExt(emu),
        "OleSaveToStream" => OleSaveToStream(emu),
        "OleSetAutoConvert" => OleSetAutoConvert(emu),
        "OleSetClipboard" => OleSetClipboard(emu),
        "OleSetContainedObject" => OleSetContainedObject(emu),
        "OleSetMenuDescriptor" => OleSetMenuDescriptor(emu),
        "OleTranslateAccelerator" => OleTranslateAccelerator(emu),
        "OleTranslateColorExt" => OleTranslateColorExt(emu),
        "OleUninitialize" => OleUninitialize(emu),
        "OpenOrCreateStream" => OpenOrCreateStream(emu),
        "ProgIDFromCLSID" => ProgIDFromCLSID(emu),
        "PropStgNameToFmtId" => PropStgNameToFmtId(emu),
        "PropSysAllocString" => PropSysAllocString(emu),
        "PropSysFreeString" => PropSysFreeString(emu),
        "PropVariantChangeType" => PropVariantChangeType(emu),
        "PropVariantClear" => PropVariantClear(emu),
        "PropVariantCopy" => PropVariantCopy(emu),
        "ReadClassStg" => ReadClassStg(emu),
        "ReadClassStm" => ReadClassStm(emu),
        "ReadFmtUserTypeStg" => ReadFmtUserTypeStg(emu),
        "ReadOleStg" => ReadOleStg(emu),
        "ReadStorageProperties" => ReadStorageProperties(emu),
        "ReadStringStream" => ReadStringStream(emu),
        "RegisterActiveObjectExt" => RegisterActiveObjectExt(emu),
        "RegisterDragDrop" => RegisterDragDrop(emu),
        "ReleaseStgMedium" => ReleaseStgMedium(emu),
        "RevokeActiveObjectExt" => RevokeActiveObjectExt(emu),
        "RevokeDragDrop" => RevokeDragDrop(emu),
        "RoGetAgileReference" => RoGetAgileReference(emu),
        "SNB_UserFree" => SNB_UserFree(emu),
        "SNB_UserFree64" => SNB_UserFree64(emu),
        "SNB_UserMarshal" => SNB_UserMarshal(emu),
        "SNB_UserMarshal64" => SNB_UserMarshal64(emu),
        "SNB_UserSize" => SNB_UserSize(emu),
        "SNB_UserSize64" => SNB_UserSize64(emu),
        "SNB_UserUnmarshal" => SNB_UserUnmarshal(emu),
        "SNB_UserUnmarshal64" => SNB_UserUnmarshal64(emu),
        "STGMEDIUM_UserFree" => STGMEDIUM_UserFree(emu),
        "STGMEDIUM_UserFree64" => STGMEDIUM_UserFree64(emu),
        "STGMEDIUM_UserFreeExt" => STGMEDIUM_UserFreeExt(emu),
        "STGMEDIUM_UserMarshal" => STGMEDIUM_UserMarshal(emu),
        "STGMEDIUM_UserMarshal64" => STGMEDIUM_UserMarshal64(emu),
        "STGMEDIUM_UserMarshalExt" => STGMEDIUM_UserMarshalExt(emu),
        "STGMEDIUM_UserSize" => STGMEDIUM_UserSize(emu),
        "STGMEDIUM_UserSize64" => STGMEDIUM_UserSize64(emu),
        "STGMEDIUM_UserSizeExt" => STGMEDIUM_UserSizeExt(emu),
        "STGMEDIUM_UserUnmarshal" => STGMEDIUM_UserUnmarshal(emu),
        "STGMEDIUM_UserUnmarshal64" => STGMEDIUM_UserUnmarshal64(emu),
        "STGMEDIUM_UserUnmarshalExt" => STGMEDIUM_UserUnmarshalExt(emu),
        "SetConvertStg" => SetConvertStg(emu),
        "SetDocumentBitStg" => SetDocumentBitStg(emu),
        "SetErrorInfo" => SetErrorInfo(emu),
        "SetOleautModule" => SetOleautModule(emu),
        "SetWOWThunkGlobalPtr" => SetWOWThunkGlobalPtr(emu),
        "StdTypesGetClassObject" => StdTypesGetClassObject(emu),
        "StdTypesRegisterServer" => StdTypesRegisterServer(emu),
        "StgConvertPropertyToVariant" => StgConvertPropertyToVariant(emu),
        "StgConvertVariantToProperty" => StgConvertVariantToProperty(emu),
        "StgCreateDocfile" => StgCreateDocfile(emu),
        "StgCreateDocfileOnILockBytes" => StgCreateDocfileOnILockBytes(emu),
        "StgCreatePropSetStg" => StgCreatePropSetStg(emu),
        "StgCreatePropStg" => StgCreatePropStg(emu),
        "StgCreateStorageEx" => StgCreateStorageEx(emu),
        "StgGetIFillLockBytesOnFile" => StgGetIFillLockBytesOnFile(emu),
        "StgGetIFillLockBytesOnILockBytes" => StgGetIFillLockBytesOnILockBytes(emu),
        "StgIsStorageFile" => StgIsStorageFile(emu),
        "StgIsStorageILockBytes" => StgIsStorageILockBytes(emu),
        "StgOpenAsyncDocfileOnIFillLockBytes" => StgOpenAsyncDocfileOnIFillLockBytes(emu),
        "StgOpenPropStg" => StgOpenPropStg(emu),
        "StgOpenStorage" => StgOpenStorage(emu),
        "StgOpenStorageEx" => StgOpenStorageEx(emu),
        "StgOpenStorageOnHandle" => StgOpenStorageOnHandle(emu),
        "StgOpenStorageOnILockBytes" => StgOpenStorageOnILockBytes(emu),
        "StgPropertyLengthAsVariant" => StgPropertyLengthAsVariant(emu),
        "StgSetTimes" => StgSetTimes(emu),
        "StringFromCLSID" => StringFromCLSID(emu),
        "StringFromGUID2" => StringFromGUID2(emu),
        "StringFromIID" => StringFromIID(emu),
        "UpdateDCOMSettings" => UpdateDCOMSettings(emu),
        "UpdateProcessTracing" => UpdateProcessTracing(emu),
        "UtConvertDvtd16toDvtd32" => UtConvertDvtd16toDvtd32(emu),
        "UtConvertDvtd32toDvtd16" => UtConvertDvtd32toDvtd16(emu),
        "UtGetDvtd16Info" => UtGetDvtd16Info(emu),
        "UtGetDvtd32Info" => UtGetDvtd32Info(emu),
        "WdtpInterfacePointer_UserFree" => WdtpInterfacePointer_UserFree(emu),
        "WdtpInterfacePointer_UserFree64" => WdtpInterfacePointer_UserFree64(emu),
        "WdtpInterfacePointer_UserMarshal" => WdtpInterfacePointer_UserMarshal(emu),
        "WdtpInterfacePointer_UserMarshal64" => WdtpInterfacePointer_UserMarshal64(emu),
        "WdtpInterfacePointer_UserSize" => WdtpInterfacePointer_UserSize(emu),
        "WdtpInterfacePointer_UserSize64" => WdtpInterfacePointer_UserSize64(emu),
        "WdtpInterfacePointer_UserUnmarshal" => WdtpInterfacePointer_UserUnmarshal(emu),
        "WdtpInterfacePointer_UserUnmarshal64" => WdtpInterfacePointer_UserUnmarshal64(emu),
        "WriteClassStg" => WriteClassStg(emu),
        "WriteClassStm" => WriteClassStm(emu),
        "WriteFmtUserTypeStg" => WriteFmtUserTypeStg(emu),
        "WriteOleStg" => WriteOleStg(emu),
        "WriteStorageProperties" => WriteStorageProperties(emu),
        "WriteStringStream" => WriteStringStream(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn read_guid_string(emu: &emu::Emu, addr: u64) -> String {
    let d1 = emu.maps.read_dword(addr).unwrap_or(0);
    let d2 = emu.maps.read_word(addr + 4).unwrap_or(0);
    let d3 = emu.maps.read_word(addr + 6).unwrap_or(0);
    let mut d4 = [0u8; 8];
    for (i, byte) in d4.iter_mut().enumerate() {
        *byte = emu.maps.read_byte(addr + 8 + i as u64).unwrap_or(0);
    }
    format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        d1, d2, d3, d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]
    )
}
