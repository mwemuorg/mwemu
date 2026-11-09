use crate::api::windows::common::fallback;
use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::emu;
//use crate::endpoint;
use crate::winapi::winapi64;

mod app_cache_check_manifest;
pub use app_cache_check_manifest::AppCacheCheckManifest;
mod app_cache_close_handle;
pub use app_cache_close_handle::AppCacheCloseHandle;
mod app_cache_create_and_commit_file;
pub use app_cache_create_and_commit_file::AppCacheCreateAndCommitFile;
mod app_cache_delete_group;
pub use app_cache_delete_group::AppCacheDeleteGroup;
mod app_cache_delete_ie_group;
pub use app_cache_delete_ie_group::AppCacheDeleteIEGroup;
mod app_cache_duplicate_handle;
pub use app_cache_duplicate_handle::AppCacheDuplicateHandle;
mod app_cache_finalize;
pub use app_cache_finalize::AppCacheFinalize;
mod app_cache_free_download_list;
pub use app_cache_free_download_list::AppCacheFreeDownloadList;
mod app_cache_free_group_list;
pub use app_cache_free_group_list::AppCacheFreeGroupList;
mod app_cache_free_ie_space;
pub use app_cache_free_ie_space::AppCacheFreeIESpace;
mod app_cache_free_space;
pub use app_cache_free_space::AppCacheFreeSpace;
mod app_cache_get_download_list;
pub use app_cache_get_download_list::AppCacheGetDownloadList;
mod app_cache_get_fallback_url;
pub use app_cache_get_fallback_url::AppCacheGetFallbackUrl;
mod app_cache_get_group_list;
pub use app_cache_get_group_list::AppCacheGetGroupList;
mod app_cache_get_ie_group_list;
pub use app_cache_get_ie_group_list::AppCacheGetIEGroupList;
mod app_cache_get_info;
pub use app_cache_get_info::AppCacheGetInfo;
mod app_cache_get_manifest_url;
pub use app_cache_get_manifest_url::AppCacheGetManifestUrl;
mod app_cache_lookup;
pub use app_cache_lookup::AppCacheLookup;
mod commit_url_cache_entry_a;
pub use commit_url_cache_entry_a::CommitUrlCacheEntryA;
mod commit_url_cache_entry_binary_blob;
pub use commit_url_cache_entry_binary_blob::CommitUrlCacheEntryBinaryBlob;
mod commit_url_cache_entry_w;
pub use commit_url_cache_entry_w::CommitUrlCacheEntryW;
mod create_md5_sso_hash;
pub use create_md5_sso_hash::CreateMD5SSOHash;
mod create_url_cache_container_a;
pub use create_url_cache_container_a::CreateUrlCacheContainerA;
mod create_url_cache_container_w;
pub use create_url_cache_container_w::CreateUrlCacheContainerW;
mod create_url_cache_entry_a;
pub use create_url_cache_entry_a::CreateUrlCacheEntryA;
mod create_url_cache_entry_ex_w;
pub use create_url_cache_entry_ex_w::CreateUrlCacheEntryExW;
mod create_url_cache_entry_w;
pub use create_url_cache_entry_w::CreateUrlCacheEntryW;
mod create_url_cache_group;
pub use create_url_cache_group::CreateUrlCacheGroup;
mod delete_ie3_cache;
pub use delete_ie3_cache::DeleteIE3Cache;
mod delete_url_cache_container_a;
pub use delete_url_cache_container_a::DeleteUrlCacheContainerA;
mod delete_url_cache_container_w;
pub use delete_url_cache_container_w::DeleteUrlCacheContainerW;
mod delete_url_cache_entry;
pub use delete_url_cache_entry::DeleteUrlCacheEntry;
mod delete_url_cache_entry_a;
pub use delete_url_cache_entry_a::DeleteUrlCacheEntryA;
mod delete_url_cache_entry_w;
pub use delete_url_cache_entry_w::DeleteUrlCacheEntryW;
mod delete_url_cache_group;
pub use delete_url_cache_group::DeleteUrlCacheGroup;
mod delete_wpad_cache_for_networks;
pub use delete_wpad_cache_for_networks::DeleteWpadCacheForNetworks;
mod detect_auto_proxy_url;
pub use detect_auto_proxy_url::DetectAutoProxyUrl;
mod dispatch_api_call;
pub use dispatch_api_call::DispatchAPICall;
mod dll_can_unload_now;
pub use dll_can_unload_now::DllCanUnloadNow;
mod dll_get_class_object;
pub use dll_get_class_object::DllGetClassObject;
mod dll_install;
pub use dll_install::DllInstall;
mod dll_register_server;
pub use dll_register_server::DllRegisterServer;
mod dll_unregister_server;
pub use dll_unregister_server::DllUnregisterServer;
mod find_close_url_cache;
pub use find_close_url_cache::FindCloseUrlCache;
mod find_first_url_cache_container_a;
pub use find_first_url_cache_container_a::FindFirstUrlCacheContainerA;
mod find_first_url_cache_container_w;
pub use find_first_url_cache_container_w::FindFirstUrlCacheContainerW;
mod find_first_url_cache_entry_a;
pub use find_first_url_cache_entry_a::FindFirstUrlCacheEntryA;
mod find_first_url_cache_entry_ex_a;
pub use find_first_url_cache_entry_ex_a::FindFirstUrlCacheEntryExA;
mod find_first_url_cache_entry_ex_w;
pub use find_first_url_cache_entry_ex_w::FindFirstUrlCacheEntryExW;
mod find_first_url_cache_entry_w;
pub use find_first_url_cache_entry_w::FindFirstUrlCacheEntryW;
mod find_first_url_cache_group;
pub use find_first_url_cache_group::FindFirstUrlCacheGroup;
mod find_next_url_cache_container_a;
pub use find_next_url_cache_container_a::FindNextUrlCacheContainerA;
mod find_next_url_cache_container_w;
pub use find_next_url_cache_container_w::FindNextUrlCacheContainerW;
mod find_next_url_cache_entry_a;
pub use find_next_url_cache_entry_a::FindNextUrlCacheEntryA;
mod find_next_url_cache_entry_ex_a;
pub use find_next_url_cache_entry_ex_a::FindNextUrlCacheEntryExA;
mod find_next_url_cache_entry_ex_w;
pub use find_next_url_cache_entry_ex_w::FindNextUrlCacheEntryExW;
mod find_next_url_cache_entry_w;
pub use find_next_url_cache_entry_w::FindNextUrlCacheEntryW;
mod find_next_url_cache_group;
pub use find_next_url_cache_group::FindNextUrlCacheGroup;
mod force_nexus_lookup;
pub use force_nexus_lookup::ForceNexusLookup;
mod force_nexus_lookup_ex_w;
pub use force_nexus_lookup_ex_w::ForceNexusLookupExW;
mod free_url_cache_space_a;
pub use free_url_cache_space_a::FreeUrlCacheSpaceA;
mod free_url_cache_space_w;
pub use free_url_cache_space_w::FreeUrlCacheSpaceW;
mod ftp_command_a;
pub use ftp_command_a::FtpCommandA;
mod ftp_command_w;
pub use ftp_command_w::FtpCommandW;
mod ftp_create_directory_a;
pub use ftp_create_directory_a::FtpCreateDirectoryA;
mod ftp_create_directory_w;
pub use ftp_create_directory_w::FtpCreateDirectoryW;
mod ftp_delete_file_a;
pub use ftp_delete_file_a::FtpDeleteFileA;
mod ftp_delete_file_w;
pub use ftp_delete_file_w::FtpDeleteFileW;
mod ftp_find_first_file_a;
pub use ftp_find_first_file_a::FtpFindFirstFileA;
mod ftp_find_first_file_w;
pub use ftp_find_first_file_w::FtpFindFirstFileW;
mod ftp_get_current_directory_a;
pub use ftp_get_current_directory_a::FtpGetCurrentDirectoryA;
mod ftp_get_current_directory_w;
pub use ftp_get_current_directory_w::FtpGetCurrentDirectoryW;
mod ftp_get_file_a;
pub use ftp_get_file_a::FtpGetFileA;
mod ftp_get_file_ex;
pub use ftp_get_file_ex::FtpGetFileEx;
mod ftp_get_file_size;
pub use ftp_get_file_size::FtpGetFileSize;
mod ftp_get_file_w;
pub use ftp_get_file_w::FtpGetFileW;
mod ftp_open_file_a;
pub use ftp_open_file_a::FtpOpenFileA;
mod ftp_open_file_w;
pub use ftp_open_file_w::FtpOpenFileW;
mod ftp_put_file_a;
pub use ftp_put_file_a::FtpPutFileA;
mod ftp_put_file_ex;
pub use ftp_put_file_ex::FtpPutFileEx;
mod ftp_put_file_w;
pub use ftp_put_file_w::FtpPutFileW;
mod ftp_remove_directory_a;
pub use ftp_remove_directory_a::FtpRemoveDirectoryA;
mod ftp_remove_directory_w;
pub use ftp_remove_directory_w::FtpRemoveDirectoryW;
mod ftp_rename_file_a;
pub use ftp_rename_file_a::FtpRenameFileA;
mod ftp_rename_file_w;
pub use ftp_rename_file_w::FtpRenameFileW;
mod ftp_set_current_directory_a;
pub use ftp_set_current_directory_a::FtpSetCurrentDirectoryA;
mod ftp_set_current_directory_w;
pub use ftp_set_current_directory_w::FtpSetCurrentDirectoryW;
mod get_proxy_dll_info;
pub use get_proxy_dll_info::GetProxyDllInfo;
mod get_url_cache_config_info_a;
pub use get_url_cache_config_info_a::GetUrlCacheConfigInfoA;
mod get_url_cache_config_info_w;
pub use get_url_cache_config_info_w::GetUrlCacheConfigInfoW;
mod get_url_cache_entry_binary_blob;
pub use get_url_cache_entry_binary_blob::GetUrlCacheEntryBinaryBlob;
mod get_url_cache_entry_info_a;
pub use get_url_cache_entry_info_a::GetUrlCacheEntryInfoA;
mod get_url_cache_entry_info_ex_a;
pub use get_url_cache_entry_info_ex_a::GetUrlCacheEntryInfoExA;
mod get_url_cache_entry_info_ex_w;
pub use get_url_cache_entry_info_ex_w::GetUrlCacheEntryInfoExW;
mod get_url_cache_entry_info_w;
pub use get_url_cache_entry_info_w::GetUrlCacheEntryInfoW;
mod get_url_cache_group_attribute_a;
pub use get_url_cache_group_attribute_a::GetUrlCacheGroupAttributeA;
mod get_url_cache_group_attribute_w;
pub use get_url_cache_group_attribute_w::GetUrlCacheGroupAttributeW;
mod get_url_cache_header_data;
pub use get_url_cache_header_data::GetUrlCacheHeaderData;
mod gopher_create_locator_a;
pub use gopher_create_locator_a::GopherCreateLocatorA;
mod gopher_create_locator_w;
pub use gopher_create_locator_w::GopherCreateLocatorW;
mod gopher_find_first_file_a;
pub use gopher_find_first_file_a::GopherFindFirstFileA;
mod gopher_find_first_file_w;
pub use gopher_find_first_file_w::GopherFindFirstFileW;
mod gopher_get_attribute_a;
pub use gopher_get_attribute_a::GopherGetAttributeA;
mod gopher_get_attribute_w;
pub use gopher_get_attribute_w::GopherGetAttributeW;
mod gopher_get_locator_type_a;
pub use gopher_get_locator_type_a::GopherGetLocatorTypeA;
mod gopher_get_locator_type_w;
pub use gopher_get_locator_type_w::GopherGetLocatorTypeW;
mod gopher_open_file_a;
pub use gopher_open_file_a::GopherOpenFileA;
mod gopher_open_file_w;
pub use gopher_open_file_w::GopherOpenFileW;
mod http_add_request_headers_a;
pub use http_add_request_headers_a::HttpAddRequestHeadersA;
mod http_add_request_headers_w;
pub use http_add_request_headers_w::HttpAddRequestHeadersW;
mod http_check_dav_compliance;
pub use http_check_dav_compliance::HttpCheckDavCompliance;
mod http_close_dependency_handle;
pub use http_close_dependency_handle::HttpCloseDependencyHandle;
mod http_duplicate_dependency_handle;
pub use http_duplicate_dependency_handle::HttpDuplicateDependencyHandle;
mod http_end_request_a;
pub use http_end_request_a::HttpEndRequestA;
mod http_end_request_w;
pub use http_end_request_w::HttpEndRequestW;
mod http_get_server_credentials;
pub use http_get_server_credentials::HttpGetServerCredentials;
mod http_get_tunnel_socket;
pub use http_get_tunnel_socket::HttpGetTunnelSocket;
mod http_indicate_page_load_complete;
pub use http_indicate_page_load_complete::HttpIndicatePageLoadComplete;
mod http_is_host_hsts_enabled;
pub use http_is_host_hsts_enabled::HttpIsHostHstsEnabled;
mod http_open_dependency_handle;
pub use http_open_dependency_handle::HttpOpenDependencyHandle;
mod http_push_close;
pub use http_push_close::HttpPushClose;
mod http_push_enable;
pub use http_push_enable::HttpPushEnable;
mod http_push_wait;
pub use http_push_wait::HttpPushWait;
mod http_send_request_ex_a;
pub use http_send_request_ex_a::HttpSendRequestExA;
mod http_send_request_ex_w;
pub use http_send_request_ex_w::HttpSendRequestExW;
mod http_web_socket_close;
pub use http_web_socket_close::HttpWebSocketClose;
mod http_web_socket_complete_upgrade;
pub use http_web_socket_complete_upgrade::HttpWebSocketCompleteUpgrade;
mod http_web_socket_query_close_status;
pub use http_web_socket_query_close_status::HttpWebSocketQueryCloseStatus;
mod http_web_socket_receive;
pub use http_web_socket_receive::HttpWebSocketReceive;
mod http_web_socket_send;
pub use http_web_socket_send::HttpWebSocketSend;
mod http_web_socket_shutdown;
pub use http_web_socket_shutdown::HttpWebSocketShutdown;
mod increment_url_cache_header_data;
pub use increment_url_cache_header_data::IncrementUrlCacheHeaderData;
mod internet_alg_id_to_string_a;
pub use internet_alg_id_to_string_a::InternetAlgIdToStringA;
mod internet_alg_id_to_string_w;
pub use internet_alg_id_to_string_w::InternetAlgIdToStringW;
mod internet_attempt_connect;
pub use internet_attempt_connect::InternetAttemptConnect;
mod internet_autodial;
pub use internet_autodial::InternetAutodial;
mod internet_autodial_callback;
pub use internet_autodial_callback::InternetAutodialCallback;
mod internet_autodial_hangup;
pub use internet_autodial_hangup::InternetAutodialHangup;
mod internet_canonicalize_url_a;
pub use internet_canonicalize_url_a::InternetCanonicalizeUrlA;
mod internet_canonicalize_url_w;
pub use internet_canonicalize_url_w::InternetCanonicalizeUrlW;
mod internet_check_connection_a;
pub use internet_check_connection_a::InternetCheckConnectionA;
mod internet_check_connection_w;
pub use internet_check_connection_w::InternetCheckConnectionW;
mod internet_clear_all_per_site_cookie_decisions;
pub use internet_clear_all_per_site_cookie_decisions::InternetClearAllPerSiteCookieDecisions;
mod internet_close_handle;
pub use internet_close_handle::InternetCloseHandle;
mod internet_combine_url_a;
pub use internet_combine_url_a::InternetCombineUrlA;
mod internet_combine_url_w;
pub use internet_combine_url_w::InternetCombineUrlW;
mod internet_confirm_zone_crossing;
pub use internet_confirm_zone_crossing::InternetConfirmZoneCrossing;
mod internet_confirm_zone_crossing_a;
pub use internet_confirm_zone_crossing_a::InternetConfirmZoneCrossingA;
mod internet_confirm_zone_crossing_w;
pub use internet_confirm_zone_crossing_w::InternetConfirmZoneCrossingW;
mod internet_convert_url_from_wire_to_wide_char;
pub use internet_convert_url_from_wire_to_wide_char::InternetConvertUrlFromWireToWideChar;
mod internet_create_url_a;
pub use internet_create_url_a::InternetCreateUrlA;
mod internet_create_url_w;
pub use internet_create_url_w::InternetCreateUrlW;
mod internet_dial;
pub use internet_dial::InternetDial;
mod internet_dial_a;
pub use internet_dial_a::InternetDialA;
mod internet_dial_w;
pub use internet_dial_w::InternetDialW;
mod internet_enum_per_site_cookie_decision_a;
pub use internet_enum_per_site_cookie_decision_a::InternetEnumPerSiteCookieDecisionA;
mod internet_enum_per_site_cookie_decision_w;
pub use internet_enum_per_site_cookie_decision_w::InternetEnumPerSiteCookieDecisionW;
mod internet_find_next_file_a;
pub use internet_find_next_file_a::InternetFindNextFileA;
mod internet_find_next_file_w;
pub use internet_find_next_file_w::InternetFindNextFileW;
mod internet_fortezza_command;
pub use internet_fortezza_command::InternetFortezzaCommand;
mod internet_free_cookies;
pub use internet_free_cookies::InternetFreeCookies;
mod internet_free_proxy_info_list;
pub use internet_free_proxy_info_list::InternetFreeProxyInfoList;
mod internet_get_cert_by_url;
pub use internet_get_cert_by_url::InternetGetCertByURL;
mod internet_get_cert_by_urla;
pub use internet_get_cert_by_urla::InternetGetCertByURLA;
mod internet_get_connected_state;
pub use internet_get_connected_state::InternetGetConnectedState;
mod internet_get_connected_state_ex;
pub use internet_get_connected_state_ex::InternetGetConnectedStateEx;
mod internet_get_connected_state_ex_a;
pub use internet_get_connected_state_ex_a::InternetGetConnectedStateExA;
mod internet_get_connected_state_ex_w;
pub use internet_get_connected_state_ex_w::InternetGetConnectedStateExW;
mod internet_get_cookie_a;
pub use internet_get_cookie_a::InternetGetCookieA;
mod internet_get_cookie_ex2;
pub use internet_get_cookie_ex2::InternetGetCookieEx2;
mod internet_get_cookie_ex_a;
pub use internet_get_cookie_ex_a::InternetGetCookieExA;
mod internet_get_cookie_ex_w;
pub use internet_get_cookie_ex_w::InternetGetCookieExW;
mod internet_get_cookie_w;
pub use internet_get_cookie_w::InternetGetCookieW;
mod internet_get_last_response_info_a;
pub use internet_get_last_response_info_a::InternetGetLastResponseInfoA;
mod internet_get_last_response_info_w;
pub use internet_get_last_response_info_w::InternetGetLastResponseInfoW;
mod internet_get_per_site_cookie_decision_a;
pub use internet_get_per_site_cookie_decision_a::InternetGetPerSiteCookieDecisionA;
mod internet_get_per_site_cookie_decision_w;
pub use internet_get_per_site_cookie_decision_w::InternetGetPerSiteCookieDecisionW;
mod internet_get_proxy_for_url;
pub use internet_get_proxy_for_url::InternetGetProxyForUrl;
mod internet_get_security_info_by_url;
pub use internet_get_security_info_by_url::InternetGetSecurityInfoByURL;
mod internet_get_security_info_by_urla;
pub use internet_get_security_info_by_urla::InternetGetSecurityInfoByURLA;
mod internet_get_security_info_by_urlw;
pub use internet_get_security_info_by_urlw::InternetGetSecurityInfoByURLW;
mod internet_go_online;
pub use internet_go_online::InternetGoOnline;
mod internet_go_online_a;
pub use internet_go_online_a::InternetGoOnlineA;
mod internet_go_online_w;
pub use internet_go_online_w::InternetGoOnlineW;
mod internet_hang_up;
pub use internet_hang_up::InternetHangUp;
mod internet_initialize_auto_proxy_dll;
pub use internet_initialize_auto_proxy_dll::InternetInitializeAutoProxyDll;
mod internet_lock_request_file;
pub use internet_lock_request_file::InternetLockRequestFile;
mod internet_open_url_a;
pub use internet_open_url_a::InternetOpenUrlA;
mod internet_open_url_w;
pub use internet_open_url_w::InternetOpenUrlW;
mod internet_query_data_available;
pub use internet_query_data_available::InternetQueryDataAvailable;
mod internet_query_fortezza_status;
pub use internet_query_fortezza_status::InternetQueryFortezzaStatus;
mod internet_security_protocol_to_string_a;
pub use internet_security_protocol_to_string_a::InternetSecurityProtocolToStringA;
mod internet_security_protocol_to_string_w;
pub use internet_security_protocol_to_string_w::InternetSecurityProtocolToStringW;
mod internet_set_cookie_a;
pub use internet_set_cookie_a::InternetSetCookieA;
mod internet_set_cookie_ex2;
pub use internet_set_cookie_ex2::InternetSetCookieEx2;
mod internet_set_cookie_ex_a;
pub use internet_set_cookie_ex_a::InternetSetCookieExA;
mod internet_set_cookie_ex_w;
pub use internet_set_cookie_ex_w::InternetSetCookieExW;
mod internet_set_cookie_w;
pub use internet_set_cookie_w::InternetSetCookieW;
mod internet_set_dial_state;
pub use internet_set_dial_state::InternetSetDialState;
mod internet_set_dial_state_a;
pub use internet_set_dial_state_a::InternetSetDialStateA;
mod internet_set_dial_state_w;
pub use internet_set_dial_state_w::InternetSetDialStateW;
mod internet_set_file_pointer;
pub use internet_set_file_pointer::InternetSetFilePointer;
mod internet_set_option_ex_a;
pub use internet_set_option_ex_a::InternetSetOptionExA;
mod internet_set_option_ex_w;
pub use internet_set_option_ex_w::InternetSetOptionExW;
mod internet_set_per_site_cookie_decision_a;
pub use internet_set_per_site_cookie_decision_a::InternetSetPerSiteCookieDecisionA;
mod internet_set_per_site_cookie_decision_w;
pub use internet_set_per_site_cookie_decision_w::InternetSetPerSiteCookieDecisionW;
mod internet_set_secure_legacy_servers_app_compat;
pub use internet_set_secure_legacy_servers_app_compat::InternetSetSecureLegacyServersAppCompat;
mod internet_set_status_callback;
pub use internet_set_status_callback::InternetSetStatusCallback;
mod internet_show_security_info_by_url;
pub use internet_show_security_info_by_url::InternetShowSecurityInfoByURL;
mod internet_show_security_info_by_urla;
pub use internet_show_security_info_by_urla::InternetShowSecurityInfoByURLA;
mod internet_show_security_info_by_urlw;
pub use internet_show_security_info_by_urlw::InternetShowSecurityInfoByURLW;
mod internet_time_from_system_time;
pub use internet_time_from_system_time::InternetTimeFromSystemTime;
mod internet_time_from_system_time_a;
pub use internet_time_from_system_time_a::InternetTimeFromSystemTimeA;
mod internet_time_from_system_time_w;
pub use internet_time_from_system_time_w::InternetTimeFromSystemTimeW;
mod internet_time_to_system_time;
pub use internet_time_to_system_time::InternetTimeToSystemTime;
mod internet_time_to_system_time_a;
pub use internet_time_to_system_time_a::InternetTimeToSystemTimeA;
mod internet_time_to_system_time_w;
pub use internet_time_to_system_time_w::InternetTimeToSystemTimeW;
mod internet_unlock_request_file;
pub use internet_unlock_request_file::InternetUnlockRequestFile;
mod internet_write_file;
pub use internet_write_file::InternetWriteFile;
mod internet_write_file_ex_a;
pub use internet_write_file_ex_a::InternetWriteFileExA;
mod internet_write_file_ex_w;
pub use internet_write_file_ex_w::InternetWriteFileExW;
mod is_host_in_proxy_bypass_list;
pub use is_host_in_proxy_bypass_list::IsHostInProxyBypassList;
mod is_url_cache_entry_expired_a;
pub use is_url_cache_entry_expired_a::IsUrlCacheEntryExpiredA;
mod is_url_cache_entry_expired_w;
pub use is_url_cache_entry_expired_w::IsUrlCacheEntryExpiredW;
mod load_url_cache_content;
pub use load_url_cache_content::LoadUrlCacheContent;
mod parse_x509_encoded_certificate_for_list_box_entry;
pub use parse_x509_encoded_certificate_for_list_box_entry::ParseX509EncodedCertificateForListBoxEntry;
mod privacy_get_zone_preference_w;
pub use privacy_get_zone_preference_w::PrivacyGetZonePreferenceW;
mod privacy_set_zone_preference_w;
pub use privacy_set_zone_preference_w::PrivacySetZonePreferenceW;
mod read_url_cache_entry_stream;
pub use read_url_cache_entry_stream::ReadUrlCacheEntryStream;
mod read_url_cache_entry_stream_ex;
pub use read_url_cache_entry_stream_ex::ReadUrlCacheEntryStreamEx;
mod register_url_cache_notification;
pub use register_url_cache_notification::RegisterUrlCacheNotification;
mod resume_suspended_download;
pub use resume_suspended_download::ResumeSuspendedDownload;
mod retrieve_url_cache_entry_file_a;
pub use retrieve_url_cache_entry_file_a::RetrieveUrlCacheEntryFileA;
mod retrieve_url_cache_entry_file_w;
pub use retrieve_url_cache_entry_file_w::RetrieveUrlCacheEntryFileW;
mod retrieve_url_cache_entry_stream_a;
pub use retrieve_url_cache_entry_stream_a::RetrieveUrlCacheEntryStreamA;
mod retrieve_url_cache_entry_stream_w;
pub use retrieve_url_cache_entry_stream_w::RetrieveUrlCacheEntryStreamW;
mod run_once_url_cache;
pub use run_once_url_cache::RunOnceUrlCache;
mod set_url_cache_config_info_a;
pub use set_url_cache_config_info_a::SetUrlCacheConfigInfoA;
mod set_url_cache_config_info_w;
pub use set_url_cache_config_info_w::SetUrlCacheConfigInfoW;
mod set_url_cache_entry_group;
pub use set_url_cache_entry_group::SetUrlCacheEntryGroup;
mod set_url_cache_entry_group_a;
pub use set_url_cache_entry_group_a::SetUrlCacheEntryGroupA;
mod set_url_cache_entry_group_w;
pub use set_url_cache_entry_group_w::SetUrlCacheEntryGroupW;
mod set_url_cache_entry_info_a;
pub use set_url_cache_entry_info_a::SetUrlCacheEntryInfoA;
mod set_url_cache_entry_info_w;
pub use set_url_cache_entry_info_w::SetUrlCacheEntryInfoW;
mod set_url_cache_group_attribute_a;
pub use set_url_cache_group_attribute_a::SetUrlCacheGroupAttributeA;
mod set_url_cache_group_attribute_w;
pub use set_url_cache_group_attribute_w::SetUrlCacheGroupAttributeW;
mod set_url_cache_header_data;
pub use set_url_cache_header_data::SetUrlCacheHeaderData;
mod show_certificate;
pub use show_certificate::ShowCertificate;
mod show_client_auth_certs;
pub use show_client_auth_certs::ShowClientAuthCerts;
mod show_security_info;
pub use show_security_info::ShowSecurityInfo;
mod show_x509_encoded_certificate;
pub use show_x509_encoded_certificate::ShowX509EncodedCertificate;
mod unlock_url_cache_entry_file;
pub use unlock_url_cache_entry_file::UnlockUrlCacheEntryFile;
mod unlock_url_cache_entry_file_a;
pub use unlock_url_cache_entry_file_a::UnlockUrlCacheEntryFileA;
mod unlock_url_cache_entry_file_w;
pub use unlock_url_cache_entry_file_w::UnlockUrlCacheEntryFileW;
mod unlock_url_cache_entry_stream;
pub use unlock_url_cache_entry_stream::UnlockUrlCacheEntryStream;
mod update_url_cache_content_path;
pub use update_url_cache_content_path::UpdateUrlCacheContentPath;
mod url_cache_check_entries_exist;
pub use url_cache_check_entries_exist::UrlCacheCheckEntriesExist;
mod url_cache_close_entry_handle;
pub use url_cache_close_entry_handle::UrlCacheCloseEntryHandle;
mod url_cache_container_set_entry_maximum_age;
pub use url_cache_container_set_entry_maximum_age::UrlCacheContainerSetEntryMaximumAge;
mod url_cache_create_container;
pub use url_cache_create_container::UrlCacheCreateContainer;
mod url_cache_find_first_entry;
pub use url_cache_find_first_entry::UrlCacheFindFirstEntry;
mod url_cache_find_next_entry;
pub use url_cache_find_next_entry::UrlCacheFindNextEntry;
mod url_cache_free_entry_info;
pub use url_cache_free_entry_info::UrlCacheFreeEntryInfo;
mod url_cache_free_global_space;
pub use url_cache_free_global_space::UrlCacheFreeGlobalSpace;
mod url_cache_get_content_paths;
pub use url_cache_get_content_paths::UrlCacheGetContentPaths;
mod url_cache_get_entry_info;
pub use url_cache_get_entry_info::UrlCacheGetEntryInfo;
mod url_cache_get_global_cache_size;
pub use url_cache_get_global_cache_size::UrlCacheGetGlobalCacheSize;
mod url_cache_get_global_limit;
pub use url_cache_get_global_limit::UrlCacheGetGlobalLimit;
mod url_cache_read_entry_stream;
pub use url_cache_read_entry_stream::UrlCacheReadEntryStream;
mod url_cache_reload_settings;
pub use url_cache_reload_settings::UrlCacheReloadSettings;
mod url_cache_retrieve_entry_file;
pub use url_cache_retrieve_entry_file::UrlCacheRetrieveEntryFile;
mod url_cache_retrieve_entry_stream;
pub use url_cache_retrieve_entry_stream::UrlCacheRetrieveEntryStream;
mod url_cache_server;
pub use url_cache_server::UrlCacheServer;
mod url_cache_set_global_limit;
pub use url_cache_set_global_limit::UrlCacheSetGlobalLimit;
mod url_cache_update_entry_extra_data;
pub use url_cache_update_entry_extra_data::UrlCacheUpdateEntryExtraData;
mod url_zones_detach;
pub use url_zones_detach::UrlZonesDetach;
mod _get_file_extension_from_url;
pub use _get_file_extension_from_url::_GetFileExtensionFromUrl;
mod http_open_request_a;
use http_open_request_a::HttpOpenRequestA;
mod http_open_request_w;
use http_open_request_w::HttpOpenRequestW;
mod http_query_info_a;
use http_query_info_a::HttpQueryInfoA;
mod http_query_info_w;
use http_query_info_w::HttpQueryInfoW;
mod http_send_request_a;
use http_send_request_a::HttpSendRequestA;
mod http_send_request_w;
use http_send_request_w::HttpSendRequestW;
mod internet_connect_a;
pub use internet_connect_a::InternetConnectA;
mod internet_connect_w;
pub use internet_connect_w::InternetConnectW;
mod internet_crack_url_a;
use internet_crack_url_a::InternetCrackUrlA;
mod internet_crack_url_w;
use internet_crack_url_w::InternetCrackUrlW;
mod internet_error_dlg;
use internet_error_dlg::InternetErrorDlg;
mod internet_open_a;
pub use internet_open_a::InternetOpenA;
mod internet_open_w;
pub use internet_open_w::InternetOpenW;
mod internet_query_option_a;
use internet_query_option_a::InternetQueryOptionA;
mod internet_query_option_w;
use internet_query_option_w::InternetQueryOptionW;
mod internet_read_file;
use internet_read_file::InternetReadFile;
mod internet_read_file_ex_a;
use internet_read_file_ex_a::InternetReadFileExA;
mod internet_read_file_ex_w;
use internet_read_file_ex_w::InternetReadFileExW;
mod internet_set_option_a;
use internet_set_option_a::InternetSetOptionA;
mod internet_set_option_w;
use internet_set_option_w::InternetSetOptionW;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "InternetOpenA" => InternetOpenA(emu),
        "InternetOpenW" => InternetOpenW(emu),
        "InternetConnectA" => InternetConnectA(emu),
        "InternetConnectW" => InternetConnectW(emu),
        "HttpOpenRequestA" => HttpOpenRequestA(emu),
        "HttpOpenRequestW" => HttpOpenRequestW(emu),
        "InternetSetOptionA" => InternetSetOptionA(emu),
        "InternetSetOptionW" => InternetSetOptionW(emu),
        "HttpSendRequestA" => HttpSendRequestA(emu),
        "HttpSendRequestW" => HttpSendRequestW(emu),
        "InternetReadFile" => InternetReadFile(emu),
        "InternetReadFileExA" => InternetReadFileExA(emu),
        "InternetReadFileExW" => InternetReadFileExW(emu),
        "InternetErrorDlg" => InternetErrorDlg(emu),
        "AppCacheCheckManifest" => AppCacheCheckManifest(emu),
        "AppCacheCloseHandle" => AppCacheCloseHandle(emu),
        "AppCacheCreateAndCommitFile" => AppCacheCreateAndCommitFile(emu),
        "AppCacheDeleteGroup" => AppCacheDeleteGroup(emu),
        "AppCacheDeleteIEGroup" => AppCacheDeleteIEGroup(emu),
        "AppCacheDuplicateHandle" => AppCacheDuplicateHandle(emu),
        "AppCacheFinalize" => AppCacheFinalize(emu),
        "AppCacheFreeDownloadList" => AppCacheFreeDownloadList(emu),
        "AppCacheFreeGroupList" => AppCacheFreeGroupList(emu),
        "AppCacheFreeIESpace" => AppCacheFreeIESpace(emu),
        "AppCacheFreeSpace" => AppCacheFreeSpace(emu),
        "AppCacheGetDownloadList" => AppCacheGetDownloadList(emu),
        "AppCacheGetFallbackUrl" => AppCacheGetFallbackUrl(emu),
        "AppCacheGetGroupList" => AppCacheGetGroupList(emu),
        "AppCacheGetIEGroupList" => AppCacheGetIEGroupList(emu),
        "AppCacheGetInfo" => AppCacheGetInfo(emu),
        "AppCacheGetManifestUrl" => AppCacheGetManifestUrl(emu),
        "AppCacheLookup" => AppCacheLookup(emu),
        "CommitUrlCacheEntryA" => CommitUrlCacheEntryA(emu),
        "CommitUrlCacheEntryBinaryBlob" => CommitUrlCacheEntryBinaryBlob(emu),
        "CommitUrlCacheEntryW" => CommitUrlCacheEntryW(emu),
        "CreateMD5SSOHash" => CreateMD5SSOHash(emu),
        "CreateUrlCacheContainerA" => CreateUrlCacheContainerA(emu),
        "CreateUrlCacheContainerW" => CreateUrlCacheContainerW(emu),
        "CreateUrlCacheEntryA" => CreateUrlCacheEntryA(emu),
        "CreateUrlCacheEntryExW" => CreateUrlCacheEntryExW(emu),
        "CreateUrlCacheEntryW" => CreateUrlCacheEntryW(emu),
        "CreateUrlCacheGroup" => CreateUrlCacheGroup(emu),
        "DeleteIE3Cache" => DeleteIE3Cache(emu),
        "DeleteUrlCacheContainerA" => DeleteUrlCacheContainerA(emu),
        "DeleteUrlCacheContainerW" => DeleteUrlCacheContainerW(emu),
        "DeleteUrlCacheEntry" => DeleteUrlCacheEntry(emu),
        "DeleteUrlCacheEntryA" => DeleteUrlCacheEntryA(emu),
        "DeleteUrlCacheEntryW" => DeleteUrlCacheEntryW(emu),
        "DeleteUrlCacheGroup" => DeleteUrlCacheGroup(emu),
        "DeleteWpadCacheForNetworks" => DeleteWpadCacheForNetworks(emu),
        "DetectAutoProxyUrl" => DetectAutoProxyUrl(emu),
        "DispatchAPICall" => DispatchAPICall(emu),
        "DllCanUnloadNow" => DllCanUnloadNow(emu),
        "DllGetClassObject" => DllGetClassObject(emu),
        "DllInstall" => DllInstall(emu),
        "DllRegisterServer" => DllRegisterServer(emu),
        "DllUnregisterServer" => DllUnregisterServer(emu),
        "FindCloseUrlCache" => FindCloseUrlCache(emu),
        "FindFirstUrlCacheContainerA" => FindFirstUrlCacheContainerA(emu),
        "FindFirstUrlCacheContainerW" => FindFirstUrlCacheContainerW(emu),
        "FindFirstUrlCacheEntryA" => FindFirstUrlCacheEntryA(emu),
        "FindFirstUrlCacheEntryExA" => FindFirstUrlCacheEntryExA(emu),
        "FindFirstUrlCacheEntryExW" => FindFirstUrlCacheEntryExW(emu),
        "FindFirstUrlCacheEntryW" => FindFirstUrlCacheEntryW(emu),
        "FindFirstUrlCacheGroup" => FindFirstUrlCacheGroup(emu),
        "FindNextUrlCacheContainerA" => FindNextUrlCacheContainerA(emu),
        "FindNextUrlCacheContainerW" => FindNextUrlCacheContainerW(emu),
        "FindNextUrlCacheEntryA" => FindNextUrlCacheEntryA(emu),
        "FindNextUrlCacheEntryExA" => FindNextUrlCacheEntryExA(emu),
        "FindNextUrlCacheEntryExW" => FindNextUrlCacheEntryExW(emu),
        "FindNextUrlCacheEntryW" => FindNextUrlCacheEntryW(emu),
        "FindNextUrlCacheGroup" => FindNextUrlCacheGroup(emu),
        "ForceNexusLookup" => ForceNexusLookup(emu),
        "ForceNexusLookupExW" => ForceNexusLookupExW(emu),
        "FreeUrlCacheSpaceA" => FreeUrlCacheSpaceA(emu),
        "FreeUrlCacheSpaceW" => FreeUrlCacheSpaceW(emu),
        "FtpCommandA" => FtpCommandA(emu),
        "FtpCommandW" => FtpCommandW(emu),
        "FtpCreateDirectoryA" => FtpCreateDirectoryA(emu),
        "FtpCreateDirectoryW" => FtpCreateDirectoryW(emu),
        "FtpDeleteFileA" => FtpDeleteFileA(emu),
        "FtpDeleteFileW" => FtpDeleteFileW(emu),
        "FtpFindFirstFileA" => FtpFindFirstFileA(emu),
        "FtpFindFirstFileW" => FtpFindFirstFileW(emu),
        "FtpGetCurrentDirectoryA" => FtpGetCurrentDirectoryA(emu),
        "FtpGetCurrentDirectoryW" => FtpGetCurrentDirectoryW(emu),
        "FtpGetFileA" => FtpGetFileA(emu),
        "FtpGetFileEx" => FtpGetFileEx(emu),
        "FtpGetFileSize" => FtpGetFileSize(emu),
        "FtpGetFileW" => FtpGetFileW(emu),
        "FtpOpenFileA" => FtpOpenFileA(emu),
        "FtpOpenFileW" => FtpOpenFileW(emu),
        "FtpPutFileA" => FtpPutFileA(emu),
        "FtpPutFileEx" => FtpPutFileEx(emu),
        "FtpPutFileW" => FtpPutFileW(emu),
        "FtpRemoveDirectoryA" => FtpRemoveDirectoryA(emu),
        "FtpRemoveDirectoryW" => FtpRemoveDirectoryW(emu),
        "FtpRenameFileA" => FtpRenameFileA(emu),
        "FtpRenameFileW" => FtpRenameFileW(emu),
        "FtpSetCurrentDirectoryA" => FtpSetCurrentDirectoryA(emu),
        "FtpSetCurrentDirectoryW" => FtpSetCurrentDirectoryW(emu),
        "GetProxyDllInfo" => GetProxyDllInfo(emu),
        "GetUrlCacheConfigInfoA" => GetUrlCacheConfigInfoA(emu),
        "GetUrlCacheConfigInfoW" => GetUrlCacheConfigInfoW(emu),
        "GetUrlCacheEntryBinaryBlob" => GetUrlCacheEntryBinaryBlob(emu),
        "GetUrlCacheEntryInfoA" => GetUrlCacheEntryInfoA(emu),
        "GetUrlCacheEntryInfoExA" => GetUrlCacheEntryInfoExA(emu),
        "GetUrlCacheEntryInfoExW" => GetUrlCacheEntryInfoExW(emu),
        "GetUrlCacheEntryInfoW" => GetUrlCacheEntryInfoW(emu),
        "GetUrlCacheGroupAttributeA" => GetUrlCacheGroupAttributeA(emu),
        "GetUrlCacheGroupAttributeW" => GetUrlCacheGroupAttributeW(emu),
        "GetUrlCacheHeaderData" => GetUrlCacheHeaderData(emu),
        "GopherCreateLocatorA" => GopherCreateLocatorA(emu),
        "GopherCreateLocatorW" => GopherCreateLocatorW(emu),
        "GopherFindFirstFileA" => GopherFindFirstFileA(emu),
        "GopherFindFirstFileW" => GopherFindFirstFileW(emu),
        "GopherGetAttributeA" => GopherGetAttributeA(emu),
        "GopherGetAttributeW" => GopherGetAttributeW(emu),
        "GopherGetLocatorTypeA" => GopherGetLocatorTypeA(emu),
        "GopherGetLocatorTypeW" => GopherGetLocatorTypeW(emu),
        "GopherOpenFileA" => GopherOpenFileA(emu),
        "GopherOpenFileW" => GopherOpenFileW(emu),
        "HttpAddRequestHeadersA" => HttpAddRequestHeadersA(emu),
        "HttpAddRequestHeadersW" => HttpAddRequestHeadersW(emu),
        "HttpCheckDavCompliance" => HttpCheckDavCompliance(emu),
        "HttpCloseDependencyHandle" => HttpCloseDependencyHandle(emu),
        "HttpDuplicateDependencyHandle" => HttpDuplicateDependencyHandle(emu),
        "HttpEndRequestA" => HttpEndRequestA(emu),
        "HttpEndRequestW" => HttpEndRequestW(emu),
        "HttpGetServerCredentials" => HttpGetServerCredentials(emu),
        "HttpGetTunnelSocket" => HttpGetTunnelSocket(emu),
        "HttpIndicatePageLoadComplete" => HttpIndicatePageLoadComplete(emu),
        "HttpIsHostHstsEnabled" => HttpIsHostHstsEnabled(emu),
        "HttpOpenDependencyHandle" => HttpOpenDependencyHandle(emu),
        "HttpPushClose" => HttpPushClose(emu),
        "HttpPushEnable" => HttpPushEnable(emu),
        "HttpPushWait" => HttpPushWait(emu),
        "HttpQueryInfoA" => HttpQueryInfoA(emu),
        "HttpQueryInfoW" => HttpQueryInfoW(emu),
        "HttpSendRequestExA" => HttpSendRequestExA(emu),
        "HttpSendRequestExW" => HttpSendRequestExW(emu),
        "HttpWebSocketClose" => HttpWebSocketClose(emu),
        "HttpWebSocketCompleteUpgrade" => HttpWebSocketCompleteUpgrade(emu),
        "HttpWebSocketQueryCloseStatus" => HttpWebSocketQueryCloseStatus(emu),
        "HttpWebSocketReceive" => HttpWebSocketReceive(emu),
        "HttpWebSocketSend" => HttpWebSocketSend(emu),
        "HttpWebSocketShutdown" => HttpWebSocketShutdown(emu),
        "IncrementUrlCacheHeaderData" => IncrementUrlCacheHeaderData(emu),
        "InternetAlgIdToStringA" => InternetAlgIdToStringA(emu),
        "InternetAlgIdToStringW" => InternetAlgIdToStringW(emu),
        "InternetAttemptConnect" => InternetAttemptConnect(emu),
        "InternetAutodial" => InternetAutodial(emu),
        "InternetAutodialCallback" => InternetAutodialCallback(emu),
        "InternetAutodialHangup" => InternetAutodialHangup(emu),
        "InternetCanonicalizeUrlA" => InternetCanonicalizeUrlA(emu),
        "InternetCanonicalizeUrlW" => InternetCanonicalizeUrlW(emu),
        "InternetCheckConnectionA" => InternetCheckConnectionA(emu),
        "InternetCheckConnectionW" => InternetCheckConnectionW(emu),
        "InternetClearAllPerSiteCookieDecisions" => InternetClearAllPerSiteCookieDecisions(emu),
        "InternetCloseHandle" => InternetCloseHandle(emu),
        "InternetCombineUrlA" => InternetCombineUrlA(emu),
        "InternetCombineUrlW" => InternetCombineUrlW(emu),
        "InternetConfirmZoneCrossing" => InternetConfirmZoneCrossing(emu),
        "InternetConfirmZoneCrossingA" => InternetConfirmZoneCrossingA(emu),
        "InternetConfirmZoneCrossingW" => InternetConfirmZoneCrossingW(emu),
        "InternetConvertUrlFromWireToWideChar" => InternetConvertUrlFromWireToWideChar(emu),
        "InternetCrackUrlA" => InternetCrackUrlA(emu),
        "InternetCrackUrlW" => InternetCrackUrlW(emu),
        "InternetCreateUrlA" => InternetCreateUrlA(emu),
        "InternetCreateUrlW" => InternetCreateUrlW(emu),
        "InternetDial" => InternetDial(emu),
        "InternetDialA" => InternetDialA(emu),
        "InternetDialW" => InternetDialW(emu),
        "InternetEnumPerSiteCookieDecisionA" => InternetEnumPerSiteCookieDecisionA(emu),
        "InternetEnumPerSiteCookieDecisionW" => InternetEnumPerSiteCookieDecisionW(emu),
        "InternetFindNextFileA" => InternetFindNextFileA(emu),
        "InternetFindNextFileW" => InternetFindNextFileW(emu),
        "InternetFortezzaCommand" => InternetFortezzaCommand(emu),
        "InternetFreeCookies" => InternetFreeCookies(emu),
        "InternetFreeProxyInfoList" => InternetFreeProxyInfoList(emu),
        "InternetGetCertByURL" => InternetGetCertByURL(emu),
        "InternetGetCertByURLA" => InternetGetCertByURLA(emu),
        "InternetGetConnectedState" => InternetGetConnectedState(emu),
        "InternetGetConnectedStateEx" => InternetGetConnectedStateEx(emu),
        "InternetGetConnectedStateExA" => InternetGetConnectedStateExA(emu),
        "InternetGetConnectedStateExW" => InternetGetConnectedStateExW(emu),
        "InternetGetCookieA" => InternetGetCookieA(emu),
        "InternetGetCookieEx2" => InternetGetCookieEx2(emu),
        "InternetGetCookieExA" => InternetGetCookieExA(emu),
        "InternetGetCookieExW" => InternetGetCookieExW(emu),
        "InternetGetCookieW" => InternetGetCookieW(emu),
        "InternetGetLastResponseInfoA" => InternetGetLastResponseInfoA(emu),
        "InternetGetLastResponseInfoW" => InternetGetLastResponseInfoW(emu),
        "InternetGetPerSiteCookieDecisionA" => InternetGetPerSiteCookieDecisionA(emu),
        "InternetGetPerSiteCookieDecisionW" => InternetGetPerSiteCookieDecisionW(emu),
        "InternetGetProxyForUrl" => InternetGetProxyForUrl(emu),
        "InternetGetSecurityInfoByURL" => InternetGetSecurityInfoByURL(emu),
        "InternetGetSecurityInfoByURLA" => InternetGetSecurityInfoByURLA(emu),
        "InternetGetSecurityInfoByURLW" => InternetGetSecurityInfoByURLW(emu),
        "InternetGoOnline" => InternetGoOnline(emu),
        "InternetGoOnlineA" => InternetGoOnlineA(emu),
        "InternetGoOnlineW" => InternetGoOnlineW(emu),
        "InternetHangUp" => InternetHangUp(emu),
        "InternetInitializeAutoProxyDll" => InternetInitializeAutoProxyDll(emu),
        "InternetLockRequestFile" => InternetLockRequestFile(emu),
        "InternetOpenUrlA" => InternetOpenUrlA(emu),
        "InternetOpenUrlW" => InternetOpenUrlW(emu),
        "InternetQueryDataAvailable" => InternetQueryDataAvailable(emu),
        "InternetQueryFortezzaStatus" => InternetQueryFortezzaStatus(emu),
        "InternetQueryOptionA" => InternetQueryOptionA(emu),
        "InternetQueryOptionW" => InternetQueryOptionW(emu),
        "InternetSecurityProtocolToStringA" => InternetSecurityProtocolToStringA(emu),
        "InternetSecurityProtocolToStringW" => InternetSecurityProtocolToStringW(emu),
        "InternetSetCookieA" => InternetSetCookieA(emu),
        "InternetSetCookieEx2" => InternetSetCookieEx2(emu),
        "InternetSetCookieExA" => InternetSetCookieExA(emu),
        "InternetSetCookieExW" => InternetSetCookieExW(emu),
        "InternetSetCookieW" => InternetSetCookieW(emu),
        "InternetSetDialState" => InternetSetDialState(emu),
        "InternetSetDialStateA" => InternetSetDialStateA(emu),
        "InternetSetDialStateW" => InternetSetDialStateW(emu),
        "InternetSetFilePointer" => InternetSetFilePointer(emu),
        "InternetSetOptionExA" => InternetSetOptionExA(emu),
        "InternetSetOptionExW" => InternetSetOptionExW(emu),
        "InternetSetPerSiteCookieDecisionA" => InternetSetPerSiteCookieDecisionA(emu),
        "InternetSetPerSiteCookieDecisionW" => InternetSetPerSiteCookieDecisionW(emu),
        "InternetSetSecureLegacyServersAppCompat" => InternetSetSecureLegacyServersAppCompat(emu),
        "InternetSetStatusCallback"
        | "InternetSetStatusCallbackA"
        | "InternetSetStatusCallbackW" => InternetSetStatusCallback(emu),
        "InternetShowSecurityInfoByURL" => InternetShowSecurityInfoByURL(emu),
        "InternetShowSecurityInfoByURLA" => InternetShowSecurityInfoByURLA(emu),
        "InternetShowSecurityInfoByURLW" => InternetShowSecurityInfoByURLW(emu),
        "InternetTimeFromSystemTime" => InternetTimeFromSystemTime(emu),
        "InternetTimeFromSystemTimeA" => InternetTimeFromSystemTimeA(emu),
        "InternetTimeFromSystemTimeW" => InternetTimeFromSystemTimeW(emu),
        "InternetTimeToSystemTime" => InternetTimeToSystemTime(emu),
        "InternetTimeToSystemTimeA" => InternetTimeToSystemTimeA(emu),
        "InternetTimeToSystemTimeW" => InternetTimeToSystemTimeW(emu),
        "InternetUnlockRequestFile" => InternetUnlockRequestFile(emu),
        "InternetWriteFile" => InternetWriteFile(emu),
        "InternetWriteFileExA" => InternetWriteFileExA(emu),
        "InternetWriteFileExW" => InternetWriteFileExW(emu),
        "IsHostInProxyBypassList" => IsHostInProxyBypassList(emu),
        "IsUrlCacheEntryExpiredA" => IsUrlCacheEntryExpiredA(emu),
        "IsUrlCacheEntryExpiredW" => IsUrlCacheEntryExpiredW(emu),
        "LoadUrlCacheContent" => LoadUrlCacheContent(emu),
        "ParseX509EncodedCertificateForListBoxEntry" => {
            ParseX509EncodedCertificateForListBoxEntry(emu)
        }
        "PrivacyGetZonePreferenceW" => PrivacyGetZonePreferenceW(emu),
        "PrivacySetZonePreferenceW" => PrivacySetZonePreferenceW(emu),
        "ReadUrlCacheEntryStream" => ReadUrlCacheEntryStream(emu),
        "ReadUrlCacheEntryStreamEx" => ReadUrlCacheEntryStreamEx(emu),
        "RegisterUrlCacheNotification" => RegisterUrlCacheNotification(emu),
        "ResumeSuspendedDownload" => ResumeSuspendedDownload(emu),
        "RetrieveUrlCacheEntryFileA" => RetrieveUrlCacheEntryFileA(emu),
        "RetrieveUrlCacheEntryFileW" => RetrieveUrlCacheEntryFileW(emu),
        "RetrieveUrlCacheEntryStreamA" => RetrieveUrlCacheEntryStreamA(emu),
        "RetrieveUrlCacheEntryStreamW" => RetrieveUrlCacheEntryStreamW(emu),
        "RunOnceUrlCache" => RunOnceUrlCache(emu),
        "SetUrlCacheConfigInfoA" => SetUrlCacheConfigInfoA(emu),
        "SetUrlCacheConfigInfoW" => SetUrlCacheConfigInfoW(emu),
        "SetUrlCacheEntryGroup" => SetUrlCacheEntryGroup(emu),
        "SetUrlCacheEntryGroupA" => SetUrlCacheEntryGroupA(emu),
        "SetUrlCacheEntryGroupW" => SetUrlCacheEntryGroupW(emu),
        "SetUrlCacheEntryInfoA" => SetUrlCacheEntryInfoA(emu),
        "SetUrlCacheEntryInfoW" => SetUrlCacheEntryInfoW(emu),
        "SetUrlCacheGroupAttributeA" => SetUrlCacheGroupAttributeA(emu),
        "SetUrlCacheGroupAttributeW" => SetUrlCacheGroupAttributeW(emu),
        "SetUrlCacheHeaderData" => SetUrlCacheHeaderData(emu),
        "ShowCertificate" => ShowCertificate(emu),
        "ShowClientAuthCerts" => ShowClientAuthCerts(emu),
        "ShowSecurityInfo" => ShowSecurityInfo(emu),
        "ShowX509EncodedCertificate" => ShowX509EncodedCertificate(emu),
        "UnlockUrlCacheEntryFile" => UnlockUrlCacheEntryFile(emu),
        "UnlockUrlCacheEntryFileA" => UnlockUrlCacheEntryFileA(emu),
        "UnlockUrlCacheEntryFileW" => UnlockUrlCacheEntryFileW(emu),
        "UnlockUrlCacheEntryStream" => UnlockUrlCacheEntryStream(emu),
        "UpdateUrlCacheContentPath" => UpdateUrlCacheContentPath(emu),
        "UrlCacheCheckEntriesExist" => UrlCacheCheckEntriesExist(emu),
        "UrlCacheCloseEntryHandle" => UrlCacheCloseEntryHandle(emu),
        "UrlCacheContainerSetEntryMaximumAge" => UrlCacheContainerSetEntryMaximumAge(emu),
        "UrlCacheCreateContainer" => UrlCacheCreateContainer(emu),
        "UrlCacheFindFirstEntry" => UrlCacheFindFirstEntry(emu),
        "UrlCacheFindNextEntry" => UrlCacheFindNextEntry(emu),
        "UrlCacheFreeEntryInfo" => UrlCacheFreeEntryInfo(emu),
        "UrlCacheFreeGlobalSpace" => UrlCacheFreeGlobalSpace(emu),
        "UrlCacheGetContentPaths" => UrlCacheGetContentPaths(emu),
        "UrlCacheGetEntryInfo" => UrlCacheGetEntryInfo(emu),
        "UrlCacheGetGlobalCacheSize" => UrlCacheGetGlobalCacheSize(emu),
        "UrlCacheGetGlobalLimit" => UrlCacheGetGlobalLimit(emu),
        "UrlCacheReadEntryStream" => UrlCacheReadEntryStream(emu),
        "UrlCacheReloadSettings" => UrlCacheReloadSettings(emu),
        "UrlCacheRetrieveEntryFile" => UrlCacheRetrieveEntryFile(emu),
        "UrlCacheRetrieveEntryStream" => UrlCacheRetrieveEntryStream(emu),
        "UrlCacheServer" => UrlCacheServer(emu),
        "UrlCacheSetGlobalLimit" => UrlCacheSetGlobalLimit(emu),
        "UrlCacheUpdateEntryExtraData" => UrlCacheUpdateEntryExtraData(emu),
        "UrlZonesDetach" => UrlZonesDetach(emu),
        "_GetFileExtensionFromUrl" => _GetFileExtensionFromUrl(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

lazy_static! {
    static ref COUNT_RECEIVE: Mutex<u32> = Mutex::new(0);
}
