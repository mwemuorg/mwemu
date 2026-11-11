use crate::api::windows::common::fallback;
use crate::emu;
//use crate::endpoint;
use crate::winapi::winapi32::kernel32;

mod free_addr_info_ex;
pub use free_addr_info_ex::*;
mod free_addr_info_w;
pub use free_addr_info_w::FreeAddrInfoW;
mod get_addr_info_ex;
pub use get_addr_info_ex::GetAddrInfoExA;
mod get_addr_info_ex_cancel;
pub use get_addr_info_ex_cancel::GetAddrInfoExCancel;
mod get_addr_info_ex_overlapped_result;
pub use get_addr_info_ex::GetAddrInfoExW;
pub use get_addr_info_ex_overlapped_result::GetAddrInfoExOverlappedResult;
mod get_addr_info_w;
pub use get_addr_info_w::GetAddrInfoW;
mod get_host_name_w;
pub use get_host_name_w::GetHostNameW;
mod get_name_info_w;
pub use get_name_info_w::GetNameInfoW;
mod process_socket_notifications;
pub use process_socket_notifications::ProcessSocketNotifications;
mod set_addr_info_ex;
pub use set_addr_info_ex::SetAddrInfoExA;
pub use set_addr_info_ex::SetAddrInfoExW;
mod wep;
pub use wep::WEP;
mod wpu_complete_overlapped_request;
pub use wpu_complete_overlapped_request::WPUCompleteOverlappedRequest;
mod wpu_get_provider_path_ex;
pub use wpu_get_provider_path_ex::WPUGetProviderPathEx;
mod wsa_accept;
pub use wsa_accept::WSAAccept;
mod wsa_address_to_string;
pub use wsa_address_to_string::WSAAddressToStringA;
pub use wsa_address_to_string::WSAAddressToStringW;
mod wsa_advertise_provider;
pub use wsa_advertise_provider::WSAAdvertiseProvider;
mod wsa_async_get_host_by_addr;
pub use wsa_async_get_host_by_addr::WSAAsyncGetHostByAddr;
mod wsa_async_get_host_by_name;
pub use wsa_async_get_host_by_name::WSAAsyncGetHostByName;
mod wsa_async_get_proto_by_name;
pub use wsa_async_get_proto_by_name::WSAAsyncGetProtoByName;
mod wsa_async_get_proto_by_number;
pub use wsa_async_get_proto_by_number::WSAAsyncGetProtoByNumber;
mod wsa_async_get_serv_by_name;
pub use wsa_async_get_serv_by_name::WSAAsyncGetServByName;
mod wsa_async_get_serv_by_port;
pub use wsa_async_get_serv_by_port::WSAAsyncGetServByPort;
mod wsa_async_select;
pub use wsa_async_select::WSAAsyncSelect;
mod wsa_cancel_async_request;
pub use wsa_cancel_async_request::WSACancelAsyncRequest;
mod wsa_cancel_blocking_call;
pub use wsa_cancel_blocking_call::WSACancelBlockingCall;
mod wsa_cleanup;
pub use wsa_cleanup::WSACleanup;
mod wsa_close_event;
pub use wsa_close_event::WSACloseEvent;
mod wsa_connect;
pub use wsa_connect::WSAConnect;
mod wsa_connect_by_list;
pub use wsa_connect_by_list::WSAConnectByList;
mod wsa_connect_by_name;
pub use wsa_connect_by_name::WSAConnectByNameA;
pub use wsa_connect_by_name::WSAConnectByNameW;
mod wsa_create_event;
pub use wsa_create_event::WSACreateEvent;
mod wsa_duplicate_socket;
pub use wsa_duplicate_socket::WSADuplicateSocketA;
pub use wsa_duplicate_socket::WSADuplicateSocketW;
mod wsa_enum_name_space_providers;
pub use wsa_enum_name_space_providers::WSAEnumNameSpaceProvidersA;
mod wsa_enum_name_space_providers_ex;
pub use wsa_enum_name_space_providers::WSAEnumNameSpaceProvidersW;
pub use wsa_enum_name_space_providers_ex::WSAEnumNameSpaceProvidersExA;
pub use wsa_enum_name_space_providers_ex::WSAEnumNameSpaceProvidersExW;
mod wsa_enum_network_events;
pub use wsa_enum_network_events::WSAEnumNetworkEvents;
mod wsa_enum_protocols;
pub use wsa_enum_protocols::WSAEnumProtocolsA;
pub use wsa_enum_protocols::WSAEnumProtocolsW;
mod wsa_event_select;
pub use wsa_event_select::WSAEventSelect;
mod wsa_get_overlapped_result;
pub use wsa_get_overlapped_result::WSAGetOverlappedResult;
mod wsa_get_qos_by_name;
pub use wsa_get_qos_by_name::WSAGetQOSByName;
mod wsa_get_service_class_info;
pub use wsa_get_service_class_info::WSAGetServiceClassInfoA;
pub use wsa_get_service_class_info::WSAGetServiceClassInfoW;
mod wsa_get_service_class_name_by_class_id;
pub use wsa_get_service_class_name_by_class_id::WSAGetServiceClassNameByClassIdA;
pub use wsa_get_service_class_name_by_class_id::WSAGetServiceClassNameByClassIdW;
mod wsa_htonl;
pub use wsa_htonl::WSAHtonl;
mod wsa_htons;
pub use wsa_htons::WSAHtons;
mod wsa_install_service_class;
pub use wsa_install_service_class::WSAInstallServiceClassA;
pub use wsa_install_service_class::WSAInstallServiceClassW;
mod wsa_ioctl;
pub use wsa_ioctl::WSAIoctl;
mod wsa_is_blocking;
pub use wsa_is_blocking::WSAIsBlocking;
mod wsa_join_leaf;
pub use wsa_join_leaf::WSAJoinLeaf;
mod wsa_lookup_service_begin;
pub use wsa_lookup_service_begin::WSALookupServiceBeginA;
pub use wsa_lookup_service_begin::WSALookupServiceBeginW;
mod wsa_lookup_service_end;
pub use wsa_lookup_service_end::WSALookupServiceEnd;
mod wsa_lookup_service_next;
pub use wsa_lookup_service_next::WSALookupServiceNextA;
pub use wsa_lookup_service_next::WSALookupServiceNextW;
mod wsansp_ioctl;
pub use wsansp_ioctl::WSANSPIoctl;
mod wsa_ntohl;
pub use wsa_ntohl::WSANtohl;
mod wsa_ntohs;
pub use wsa_ntohs::WSANtohs;
mod wsa_poll;
pub use wsa_poll::WSAPoll;
mod wsa_provider_complete_async_call;
pub use wsa_provider_complete_async_call::WSAProviderCompleteAsyncCall;
mod wsa_provider_config_change;
pub use wsa_provider_config_change::WSAProviderConfigChange;
mod wsa_recv;
pub use wsa_recv::WSARecv;
mod wsa_recv_disconnect;
pub use wsa_recv_disconnect::WSARecvDisconnect;
mod wsa_recv_from;
pub use wsa_recv_from::WSARecvFrom;
mod wsa_remove_service_class;
pub use wsa_remove_service_class::WSARemoveServiceClass;
mod wsa_reset_event;
pub use wsa_reset_event::WSAResetEvent;
mod wsa_send_disconnect;
pub use wsa_send_disconnect::WSASendDisconnect;
mod wsa_send_msg;
pub use wsa_send_msg::WSASendMsg;
mod wsa_send_to;
pub use wsa_send_to::WSASendTo;
mod wsa_set_blocking_hook;
pub use wsa_set_blocking_hook::WSASetBlockingHook;
mod wsa_set_event;
pub use wsa_set_event::WSASetEvent;
mod wsa_set_service;
pub use wsa_set_service::WSASetServiceA;
pub use wsa_set_service::WSASetServiceW;
mod wsa_socket;
pub use wsa_socket::WSASocketA;
pub use wsa_socket::WSASocketW;
mod wsa_startup;
pub use wsa_startup::WSAStartup;
mod wsa_string_to_address;
pub use wsa_string_to_address::WSAStringToAddressA;
pub use wsa_string_to_address::WSAStringToAddressW;
mod wsa_unadvertise_provider;
pub use wsa_unadvertise_provider::WSAUnadvertiseProvider;
mod wsa_unhook_blocking_hook;
pub use wsa_unhook_blocking_hook::WSAUnhookBlockingHook;
mod wsa_wait_for_multiple_events;
pub use wsa_wait_for_multiple_events::WSAWaitForMultipleEvents;
mod ws_ap_set_post_routine;
pub use ws_ap_set_post_routine::WSApSetPostRoutine;
mod wsc_deinstall_provider;
pub use wsc_deinstall_provider::WSCDeinstallProvider;
mod wsc_deinstall_provider32;
pub use wsc_deinstall_provider32::WSCDeinstallProvider32;
mod wsc_deinstall_provider_ex;
pub use wsc_deinstall_provider_ex::WSCDeinstallProviderEx;
mod wsc_enable_ns_provider;
pub use wsc_enable_ns_provider::WSCEnableNSProvider;
mod wsc_enable_ns_provider32;
pub use wsc_enable_ns_provider32::WSCEnableNSProvider32;
mod wsc_enum_name_space_providers32;
pub use wsc_enum_name_space_providers32::WSCEnumNameSpaceProviders32;
mod wsc_enum_name_space_providers_ex32;
pub use wsc_enum_name_space_providers_ex32::WSCEnumNameSpaceProvidersEx32;
mod wsc_enum_protocols;
pub use wsc_enum_protocols::WSCEnumProtocols;
mod wsc_enum_protocols32;
pub use wsc_enum_protocols32::WSCEnumProtocols32;
mod wsc_enum_protocols_ex;
pub use wsc_enum_protocols_ex::WSCEnumProtocolsEx;
mod wsc_get_application_category;
pub use wsc_get_application_category::WSCGetApplicationCategory;
mod wsc_get_application_category_ex;
pub use wsc_get_application_category_ex::WSCGetApplicationCategoryEx;
mod wsc_get_provider_info;
pub use wsc_get_provider_info::WSCGetProviderInfo;
mod wsc_get_provider_info32;
pub use wsc_get_provider_info32::WSCGetProviderInfo32;
mod wsc_get_provider_path;
pub use wsc_get_provider_path::WSCGetProviderPath;
mod wsc_get_provider_path32;
pub use wsc_get_provider_path32::WSCGetProviderPath32;
mod wsc_install_name_space;
pub use wsc_install_name_space::WSCInstallNameSpace;
mod wsc_install_name_space32;
pub use wsc_install_name_space32::WSCInstallNameSpace32;
mod wsc_install_name_space_ex;
pub use wsc_install_name_space_ex::WSCInstallNameSpaceEx;
mod wsc_install_name_space_ex2;
pub use wsc_install_name_space_ex2::WSCInstallNameSpaceEx2;
mod wsc_install_name_space_ex32;
pub use wsc_install_name_space_ex32::WSCInstallNameSpaceEx32;
mod wsc_install_provider;
pub use wsc_install_provider::WSCInstallProvider;
mod wsc_install_provider64_32;
pub use wsc_install_provider64_32::WSCInstallProvider64_32;
mod wsc_install_provider_and_chains;
pub use wsc_install_provider_and_chains::WSCInstallProviderAndChains;
mod wsc_install_provider_and_chains64_32;
pub use wsc_install_provider_and_chains64_32::WSCInstallProviderAndChains64_32;
mod wsc_install_provider_ex;
pub use wsc_install_provider_ex::WSCInstallProviderEx;
mod wsc_set_application_category;
pub use wsc_set_application_category::WSCSetApplicationCategory;
mod wsc_set_application_category_ex;
pub use wsc_set_application_category_ex::WSCSetApplicationCategoryEx;
mod wsc_set_provider_info;
pub use wsc_set_provider_info::WSCSetProviderInfo;
mod wsc_set_provider_info32;
pub use wsc_set_provider_info32::WSCSetProviderInfo32;
mod wsc_un_install_name_space;
pub use wsc_un_install_name_space::WSCUnInstallNameSpace;
mod wsc_un_install_name_space32;
pub use wsc_un_install_name_space32::WSCUnInstallNameSpace32;
mod wsc_un_install_name_space_ex2;
pub use wsc_un_install_name_space_ex2::WSCUnInstallNameSpaceEx2;
mod wsc_update_provider;
pub use wsc_update_provider::WSCUpdateProvider;
mod wsc_update_provider32;
pub use wsc_update_provider32::WSCUpdateProvider32;
mod wsc_update_provider_ex;
pub use wsc_update_provider_ex::WSCUpdateProviderEx;
mod wsc_write_name_space_order;
pub use wsc_write_name_space_order::WSCWriteNameSpaceOrder;
mod wsc_write_name_space_order32;
pub use wsc_write_name_space_order32::WSCWriteNameSpaceOrder32;
mod wsc_write_provider_order;
pub use wsc_write_provider_order::WSCWriteProviderOrder;
mod wsc_write_provider_order32;
pub use wsc_write_provider_order32::WSCWriteProviderOrder32;
mod wsc_write_provider_order_ex;
pub use wsc_write_provider_order_ex::WSCWriteProviderOrderEx;
mod wah_close_apc_helper;
pub use wah_close_apc_helper::WahCloseApcHelper;
mod wah_close_handle_helper;
pub use wah_close_handle_helper::WahCloseHandleHelper;
mod wah_close_notification_handle_helper;
pub use wah_close_notification_handle_helper::WahCloseNotificationHandleHelper;
mod wah_close_socket_handle;
pub use wah_close_socket_handle::WahCloseSocketHandle;
mod wah_close_thread;
pub use wah_close_thread::WahCloseThread;
mod wah_complete_request;
pub use wah_complete_request::WahCompleteRequest;
mod wah_create_handle_context_table;
pub use wah_create_handle_context_table::WahCreateHandleContextTable;
mod wah_create_notification_handle;
pub use wah_create_notification_handle::WahCreateNotificationHandle;
mod wah_create_socket_handle;
pub use wah_create_socket_handle::WahCreateSocketHandle;
mod wah_destroy_handle_context_table;
pub use wah_destroy_handle_context_table::WahDestroyHandleContextTable;
mod wah_disable_non_ifs_handle_support;
pub use wah_disable_non_ifs_handle_support::WahDisableNonIFSHandleSupport;
mod wah_enable_non_ifs_handle_support;
pub use wah_enable_non_ifs_handle_support::WahEnableNonIFSHandleSupport;
mod wah_enumerate_handle_contexts;
pub use wah_enumerate_handle_contexts::WahEnumerateHandleContexts;
mod wah_insert_handle_context;
pub use wah_insert_handle_context::WahInsertHandleContext;
mod wah_notify_all_processes;
pub use wah_notify_all_processes::WahNotifyAllProcesses;
mod wah_open_apc_helper;
pub use wah_open_apc_helper::WahOpenApcHelper;
mod wah_open_current_thread;
pub use wah_open_current_thread::WahOpenCurrentThread;
mod wah_open_handle_helper;
pub use wah_open_handle_helper::WahOpenHandleHelper;
mod wah_open_notification_handle_helper;
pub use wah_open_notification_handle_helper::WahOpenNotificationHandleHelper;
mod wah_queue_user_apc;
pub use wah_queue_user_apc::WahQueueUserApc;
mod wah_reference_context_by_handle;
pub use wah_reference_context_by_handle::WahReferenceContextByHandle;
mod wah_remove_handle_context;
pub use wah_remove_handle_context::WahRemoveHandleContext;
mod wah_wait_for_notification;
pub use wah_wait_for_notification::WahWaitForNotification;
mod wah_write_lsp_event;
pub use wah_write_lsp_event::WahWriteLSPEvent;
mod __wsafd_is_set;
pub use __wsafd_is_set::__WSAFDIsSet;
mod freeaddrinfo;
pub use freeaddrinfo::freeaddrinfo;
mod gethostbyaddr;
pub use gethostbyaddr::gethostbyaddr;
mod gethostname;
pub use gethostname::gethostname;
mod getnameinfo;
pub use getnameinfo::getnameinfo;
mod getprotobyname;
pub use getprotobyname::getprotobyname;
mod getprotobynumber;
pub use getprotobynumber::getprotobynumber;
mod getservbyname;
pub use getservbyname::getservbyname;
mod getservbyport;
pub use getservbyport::getservbyport;
mod getsockname;
pub use getsockname::getsockname;
mod inet_ntoa;
pub use inet_ntoa::inet_ntoa;
mod inet_ntop;
pub use inet_ntop::*;
mod inet_pton;
pub use inet_pton::*;
mod get_sock_name;
use get_sock_name::GetSockName;
mod wsa_accept_2;
use wsa_accept_2::WsaAccept;
mod wsa_connect_2;
use wsa_connect_2::WsaConnect;
mod wsa_get_last_error;
use wsa_get_last_error::WsaGetLastError;
mod wsa_htons_2;
use wsa_htons_2::WsaHtons;
mod wsa_recv_2;
use wsa_recv_2::WsaRecv;
mod wsa_recv_from_2;
use wsa_recv_from_2::WsaRecvFrom;
mod wsa_send;
use wsa_send::WsaSend;
mod wsa_set_last_error;
use wsa_set_last_error::WsaSetLastError;
mod wsa_socket_a_2;
use wsa_socket_a_2::WsaSocketA;
mod wsa_startup_2;
pub use wsa_startup_2::WsaStartup;
mod accept;
use accept::accept;
mod bind;
use bind::bind;
mod closesocket;
pub use closesocket::closesocket;
mod connect;
pub use connect::connect;
mod getaddrinfo;
use getaddrinfo::getaddrinfo;
mod gethostbyname;
pub use gethostbyname::gethostbyname;
mod getpeername;
use getpeername::getpeername;
mod getsockopt;
use getsockopt::getsockopt;
mod htonl;
use htonl::htonl;
mod htons;
pub use htons::htons;
mod inet_addr;
pub use inet_addr::inet_addr;
mod ioctlsocket;
use ioctlsocket::ioctlsocket;
mod listen;
use listen::listen;
mod ntohl;
use ntohl::ntohl;
mod ntohs;
use ntohs::ntohs;
mod recv;
pub use recv::recv;
mod recvfrom;
use recvfrom::recvfrom;
mod select;
use select::select;
mod send;
pub use send::send;
mod sendto;
use sendto::sendto;
mod setsockopt;
use setsockopt::setsockopt;
mod shutdown;
use shutdown::shutdown;
mod socket;
pub use socket::socket;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "WsaStartup" => WsaStartup(emu),
        "WsaSocketA" => WsaSocketA(emu),
        "socket" => socket(emu),
        "WsaHtons" => WsaHtons(emu),
        "htons" => htons(emu),
        "inet_addr" => inet_addr(emu),
        "connect" => connect(emu),
        "recv" => recv(emu),
        "send" => send(emu),
        "bind" => bind(emu),
        "listen" => listen(emu),
        "accept" => accept(emu),
        "closesocket" => closesocket(emu),
        "setsockopt" => setsockopt(emu),
        "getsockopt" => getsockopt(emu),
        "WsaAccept" => WsaAccept(emu),
        "getaddrinfo" => getaddrinfo(emu),

        /*
        0x774834b5 => sendto(emu),
        0x7748b6dc => recvfrom(emu),
        0x77487089 => WsaRecv(emu),
        0x7748cba6 => WsaRecvFrom(emu),
        0x7748cc3f => WsaConnect(emu),
        */
        "gethostbyname" => gethostbyname(emu),
        "sendto" => sendto(emu),
        "recvfrom" => recvfrom(emu),
        "WsaConnect" => WsaConnect(emu),
        "WsaRecv" => WsaRecv(emu),
        "WsaRecvFrom" => WsaRecvFrom(emu),
        "WsaSend" | "WSASend" => WsaSend(emu),
        "GetSockName" => GetSockName(emu),
        "htonl" => htonl(emu),
        "ntohl" => ntohl(emu),
        "ntohs" => ntohs(emu),
        "select" => select(emu),
        "ioctlsocket" => ioctlsocket(emu),
        "shutdown" => shutdown(emu),
        "getpeername" => getpeername(emu),
        "WSAGetLastError" => WsaGetLastError(emu),
        "WSASetLastError" => WsaSetLastError(emu),
        "FreeAddrInfoEx" => FreeAddrInfoEx(emu),
        "FreeAddrInfoExW" => FreeAddrInfoExW(emu),
        "FreeAddrInfoW" => FreeAddrInfoW(emu),
        "GetAddrInfoExA" => GetAddrInfoExA(emu),
        "GetAddrInfoExCancel" => GetAddrInfoExCancel(emu),
        "GetAddrInfoExOverlappedResult" => GetAddrInfoExOverlappedResult(emu),
        "GetAddrInfoExW" => GetAddrInfoExW(emu),
        "GetAddrInfoW" => GetAddrInfoW(emu),
        "GetHostNameW" => GetHostNameW(emu),
        "GetNameInfoW" => GetNameInfoW(emu),
        "InetNtopW" => InetNtopW(emu),
        "InetPtonW" => InetPtonW(emu, addr),
        "ProcessSocketNotifications" => ProcessSocketNotifications(emu),
        "SetAddrInfoExA" => SetAddrInfoExA(emu),
        "SetAddrInfoExW" => SetAddrInfoExW(emu),
        "WEP" => WEP(emu),
        "WPUCompleteOverlappedRequest" => WPUCompleteOverlappedRequest(emu),
        "WPUGetProviderPathEx" => WPUGetProviderPathEx(emu),
        "WSAAccept" => WSAAccept(emu),
        "WSAAddressToStringA" => WSAAddressToStringA(emu),
        "WSAAddressToStringW" => WSAAddressToStringW(emu),
        "WSAAdvertiseProvider" => WSAAdvertiseProvider(emu),
        "WSAAsyncGetHostByAddr" => WSAAsyncGetHostByAddr(emu),
        "WSAAsyncGetHostByName" => WSAAsyncGetHostByName(emu),
        "WSAAsyncGetProtoByName" => WSAAsyncGetProtoByName(emu),
        "WSAAsyncGetProtoByNumber" => WSAAsyncGetProtoByNumber(emu),
        "WSAAsyncGetServByName" => WSAAsyncGetServByName(emu),
        "WSAAsyncGetServByPort" => WSAAsyncGetServByPort(emu),
        "WSAAsyncSelect" => WSAAsyncSelect(emu),
        "WSACancelAsyncRequest" => WSACancelAsyncRequest(emu),
        "WSACancelBlockingCall" => WSACancelBlockingCall(emu),
        "WSACleanup" => WSACleanup(emu),
        "WSACloseEvent" => WSACloseEvent(emu),
        "WSAConnect" => WSAConnect(emu),
        "WSAConnectByList" => WSAConnectByList(emu),
        "WSAConnectByNameA" => WSAConnectByNameA(emu),
        "WSAConnectByNameW" => WSAConnectByNameW(emu),
        "WSACreateEvent" => WSACreateEvent(emu),
        "WSADuplicateSocketA" => WSADuplicateSocketA(emu),
        "WSADuplicateSocketW" => WSADuplicateSocketW(emu),
        "WSAEnumNameSpaceProvidersA" => WSAEnumNameSpaceProvidersA(emu),
        "WSAEnumNameSpaceProvidersExA" => WSAEnumNameSpaceProvidersExA(emu),
        "WSAEnumNameSpaceProvidersExW" => WSAEnumNameSpaceProvidersExW(emu),
        "WSAEnumNameSpaceProvidersW" => WSAEnumNameSpaceProvidersW(emu),
        "WSAEnumNetworkEvents" => WSAEnumNetworkEvents(emu),
        "WSAEnumProtocolsA" => WSAEnumProtocolsA(emu),
        "WSAEnumProtocolsW" => WSAEnumProtocolsW(emu),
        "WSAEventSelect" => WSAEventSelect(emu),
        "WSAGetOverlappedResult" => WSAGetOverlappedResult(emu),
        "WSAGetQOSByName" => WSAGetQOSByName(emu),
        "WSAGetServiceClassInfoA" => WSAGetServiceClassInfoA(emu),
        "WSAGetServiceClassInfoW" => WSAGetServiceClassInfoW(emu),
        "WSAGetServiceClassNameByClassIdA" => WSAGetServiceClassNameByClassIdA(emu),
        "WSAGetServiceClassNameByClassIdW" => WSAGetServiceClassNameByClassIdW(emu),
        "WSAHtonl" => WSAHtonl(emu),
        "WSAHtons" => WSAHtons(emu),
        "WSAInstallServiceClassA" => WSAInstallServiceClassA(emu),
        "WSAInstallServiceClassW" => WSAInstallServiceClassW(emu),
        "WSAIoctl" => WSAIoctl(emu),
        "WSAIsBlocking" => WSAIsBlocking(emu),
        "WSAJoinLeaf" => WSAJoinLeaf(emu),
        "WSALookupServiceBeginA" => WSALookupServiceBeginA(emu),
        "WSALookupServiceBeginW" => WSALookupServiceBeginW(emu),
        "WSALookupServiceEnd" => WSALookupServiceEnd(emu),
        "WSALookupServiceNextA" => WSALookupServiceNextA(emu),
        "WSALookupServiceNextW" => WSALookupServiceNextW(emu),
        "WSANSPIoctl" => WSANSPIoctl(emu),
        "WSANtohl" => WSANtohl(emu),
        "WSANtohs" => WSANtohs(emu),
        "WSAPoll" => WSAPoll(emu),
        "WSAProviderCompleteAsyncCall" => WSAProviderCompleteAsyncCall(emu),
        "WSAProviderConfigChange" => WSAProviderConfigChange(emu),
        "WSARecv" => WSARecv(emu),
        "WSARecvDisconnect" => WSARecvDisconnect(emu),
        "WSARecvFrom" => WSARecvFrom(emu),
        "WSARemoveServiceClass" => WSARemoveServiceClass(emu),
        "WSAResetEvent" => WSAResetEvent(emu),
        "WSASendDisconnect" => WSASendDisconnect(emu),
        "WSASendMsg" => WSASendMsg(emu),
        "WSASendTo" => WSASendTo(emu),
        "WSASetBlockingHook" => WSASetBlockingHook(emu),
        "WSASetEvent" => WSASetEvent(emu),
        "WSASetServiceA" => WSASetServiceA(emu),
        "WSASetServiceW" => WSASetServiceW(emu),
        "WSASocketA" => WSASocketA(emu),
        "WSASocketW" => WSASocketW(emu),
        "WSAStartup" => WSAStartup(emu),
        "WSAStringToAddressA" => WSAStringToAddressA(emu),
        "WSAStringToAddressW" => WSAStringToAddressW(emu),
        "WSAUnadvertiseProvider" => WSAUnadvertiseProvider(emu),
        "WSAUnhookBlockingHook" => WSAUnhookBlockingHook(emu),
        "WSAWaitForMultipleEvents" => WSAWaitForMultipleEvents(emu),
        "WSApSetPostRoutine" => WSApSetPostRoutine(emu),
        "WSCDeinstallProvider" => WSCDeinstallProvider(emu),
        "WSCDeinstallProvider32" => WSCDeinstallProvider32(emu),
        "WSCDeinstallProviderEx" => WSCDeinstallProviderEx(emu),
        "WSCEnableNSProvider" => WSCEnableNSProvider(emu),
        "WSCEnableNSProvider32" => WSCEnableNSProvider32(emu),
        "WSCEnumNameSpaceProviders32" => WSCEnumNameSpaceProviders32(emu),
        "WSCEnumNameSpaceProvidersEx32" => WSCEnumNameSpaceProvidersEx32(emu),
        "WSCEnumProtocols" => WSCEnumProtocols(emu),
        "WSCEnumProtocols32" => WSCEnumProtocols32(emu),
        "WSCEnumProtocolsEx" => WSCEnumProtocolsEx(emu),
        "WSCGetApplicationCategory" => WSCGetApplicationCategory(emu),
        "WSCGetApplicationCategoryEx" => WSCGetApplicationCategoryEx(emu),
        "WSCGetProviderInfo" => WSCGetProviderInfo(emu),
        "WSCGetProviderInfo32" => WSCGetProviderInfo32(emu),
        "WSCGetProviderPath" => WSCGetProviderPath(emu),
        "WSCGetProviderPath32" => WSCGetProviderPath32(emu),
        "WSCInstallNameSpace" => WSCInstallNameSpace(emu),
        "WSCInstallNameSpace32" => WSCInstallNameSpace32(emu),
        "WSCInstallNameSpaceEx" => WSCInstallNameSpaceEx(emu),
        "WSCInstallNameSpaceEx2" => WSCInstallNameSpaceEx2(emu),
        "WSCInstallNameSpaceEx32" => WSCInstallNameSpaceEx32(emu),
        "WSCInstallProvider" => WSCInstallProvider(emu),
        "WSCInstallProvider64_32" => WSCInstallProvider64_32(emu),
        "WSCInstallProviderAndChains" => WSCInstallProviderAndChains(emu),
        "WSCInstallProviderAndChains64_32" => WSCInstallProviderAndChains64_32(emu),
        "WSCInstallProviderEx" => WSCInstallProviderEx(emu),
        "WSCSetApplicationCategory" => WSCSetApplicationCategory(emu),
        "WSCSetApplicationCategoryEx" => WSCSetApplicationCategoryEx(emu),
        "WSCSetProviderInfo" => WSCSetProviderInfo(emu),
        "WSCSetProviderInfo32" => WSCSetProviderInfo32(emu),
        "WSCUnInstallNameSpace" => WSCUnInstallNameSpace(emu),
        "WSCUnInstallNameSpace32" => WSCUnInstallNameSpace32(emu),
        "WSCUnInstallNameSpaceEx2" => WSCUnInstallNameSpaceEx2(emu),
        "WSCUpdateProvider" => WSCUpdateProvider(emu),
        "WSCUpdateProvider32" => WSCUpdateProvider32(emu),
        "WSCUpdateProviderEx" => WSCUpdateProviderEx(emu),
        "WSCWriteNameSpaceOrder" => WSCWriteNameSpaceOrder(emu),
        "WSCWriteNameSpaceOrder32" => WSCWriteNameSpaceOrder32(emu),
        "WSCWriteProviderOrder" => WSCWriteProviderOrder(emu),
        "WSCWriteProviderOrder32" => WSCWriteProviderOrder32(emu),
        "WSCWriteProviderOrderEx" => WSCWriteProviderOrderEx(emu),
        "WahCloseApcHelper" => WahCloseApcHelper(emu),
        "WahCloseHandleHelper" => WahCloseHandleHelper(emu),
        "WahCloseNotificationHandleHelper" => WahCloseNotificationHandleHelper(emu),
        "WahCloseSocketHandle" => WahCloseSocketHandle(emu),
        "WahCloseThread" => WahCloseThread(emu),
        "WahCompleteRequest" => WahCompleteRequest(emu),
        "WahCreateHandleContextTable" => WahCreateHandleContextTable(emu),
        "WahCreateNotificationHandle" => WahCreateNotificationHandle(emu),
        "WahCreateSocketHandle" => WahCreateSocketHandle(emu),
        "WahDestroyHandleContextTable" => WahDestroyHandleContextTable(emu),
        "WahDisableNonIFSHandleSupport" => WahDisableNonIFSHandleSupport(emu),
        "WahEnableNonIFSHandleSupport" => WahEnableNonIFSHandleSupport(emu),
        "WahEnumerateHandleContexts" => WahEnumerateHandleContexts(emu),
        "WahInsertHandleContext" => WahInsertHandleContext(emu),
        "WahNotifyAllProcesses" => WahNotifyAllProcesses(emu),
        "WahOpenApcHelper" => WahOpenApcHelper(emu),
        "WahOpenCurrentThread" => WahOpenCurrentThread(emu),
        "WahOpenHandleHelper" => WahOpenHandleHelper(emu),
        "WahOpenNotificationHandleHelper" => WahOpenNotificationHandleHelper(emu),
        "WahQueueUserApc" => WahQueueUserApc(emu),
        "WahReferenceContextByHandle" => WahReferenceContextByHandle(emu),
        "WahRemoveHandleContext" => WahRemoveHandleContext(emu),
        "WahWaitForNotification" => WahWaitForNotification(emu),
        "WahWriteLSPEvent" => WahWriteLSPEvent(emu),
        "__WSAFDIsSet" => __WSAFDIsSet(emu),
        "freeaddrinfo" => freeaddrinfo(emu),
        "gethostbyaddr" => gethostbyaddr(emu),
        "gethostname" => gethostname(emu),
        "getnameinfo" => getnameinfo(emu),
        "getprotobyname" => getprotobyname(emu),
        "getprotobynumber" => getprotobynumber(emu),
        "getservbyname" => getservbyname(emu),
        "getservbyport" => getservbyport(emu),
        "getsockname" => getsockname(emu),
        "inet_ntoa" => inet_ntoa(emu),
        "inet_ntop" => inet_ntop(emu),
        "inet_pton" => inet_pton(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}
