use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "WSAGetServiceClassNameByClassIdA" | "WSAGetServiceClassNameByClassIdW" => {
            signature!(Generic, 3, 3; Pointer(lp_service_class_id), Pointer(lpsz_service_class_name), Pointer(lpdw_buffer_length))
        }
        "WSCDeinstallProvider" | "WSCDeinstallProvider32" => {
            signature!(Generic, 2, 2; Pointer(lp_provider_id), Pointer(lp_errno))
        }
        "WSCEnableNSProvider" | "WSCEnableNSProvider32" => {
            signature!(Generic, 2, 2; Pointer(lp_provider_id), Dword(f_enable))
        }
        "WSCEnumNameSpaceProviders32" | "WSCEnumNameSpaceProvidersEx32" => {
            signature!(Generic, 2, 2; Pointer(lpdw_buffer_length), Pointer(lpnsp_buffer))
        }
        "WSCEnumProtocols" | "WSCEnumProtocols32" => {
            signature!(Generic, 4, 4; Pointer(lpi_protocols), Pointer(lp_protocol_buffer), Pointer(lpdw_buffer_length), Pointer(lp_errno))
        }
        "WSCGetApplicationCategory" => {
            signature!(Generic, 6, 6; Pointer(path), Dword(path_length), Pointer(extra), Dword(extra_length), Pointer(p_permitted_lsp_categories), Pointer(lp_errno))
        }
        "WSCGetProviderInfo"
        | "WSCGetProviderInfo32"
        | "WSCSetProviderInfo"
        | "WSCSetProviderInfo32" => {
            signature!(Generic, 6, 6; Pointer(lp_provider_id), Dword(info_type), Pointer(info), Pointer(info_size), Dword(flags), Pointer(lp_errno))
        }
        "WSCGetProviderPath" | "WSCGetProviderPath32" => {
            signature!(Generic, 4, 4; Pointer(lp_provider_id), Pointer(lpsz_provider_dll_path), Pointer(lp_provider_dll_path_len), Pointer(lp_errno))
        }
        "WSCInstallNameSpace" | "WSCInstallNameSpace32" => {
            signature!(Generic, 5, 5; WideString(lpsz_identifier), WideString(lpsz_path_name), Dword(dw_name_space), Dword(dw_version), Pointer(lp_provider_id))
        }
        "WSCInstallNameSpaceEx" | "WSCInstallNameSpaceEx32" => {
            signature!(Generic, 6, 6; WideString(lpsz_identifier), WideString(lpsz_path_name), Dword(dw_name_space), Dword(dw_version), Pointer(lp_provider_id), Pointer(lp_provider_specific))
        }
        "WSCInstallProvider"
        | "WSCInstallProvider64_32"
        | "WSCUpdateProvider"
        | "WSCUpdateProvider32" => {
            signature!(Generic, 5, 5; Pointer(lp_provider_id), WideString(lpsz_provider_dll_path), Pointer(lp_protocol_info_list), Dword(dw_number_of_entries), Pointer(lp_errno))
        }
        "WSCInstallProviderAndChains64_32" => {
            signature!(Generic, 9, 9; Pointer(lpproviderid), Pointer(lpszproviderdllpath), Pointer(lpszproviderdllpath32), Pointer(lpszlspname), Dword(dwserviceflags), Pointer(lpprotocolinfolist), Dword(dwnumberofentries), Pointer(lpdwcatalogentryid), Pointer(lperrno))
        }
        "WSCSetApplicationCategory" => {
            signature!(Generic, 7, 7; Pointer(path), Dword(path_length), Pointer(extra), Dword(extra_length), Dword(permitted_lsp_categories), Pointer(p_prev_perm_lsp_cat), Pointer(lp_errno))
        }
        "WSCWriteNameSpaceOrder" | "WSCWriteNameSpaceOrder32" => {
            signature!(Generic, 2, 2; Pointer(lp_provider_id), Dword(dw_number_of_entries))
        }
        "WSCWriteProviderOrder" | "WSCWriteProviderOrder32" => {
            signature!(Generic, 2, 2; Pointer(lpwd_catalog_entry_id), Dword(dw_number_of_entries))
        }
        "FreeAddrInfoEx" | "FreeAddrInfoExW" => signature!(Void, 1, 1; Pointer(p_addr_info_ex)),
        "GetAddrInfoExA" => {
            signature!(Generic, 10, 10; AnsiString(p_name), AnsiString(p_service_name), Dword(dw_name_space), Pointer(lp_nsp_id), Pointer(hints), Pointer(pp_result), Pointer(timeout), Pointer(lp_overlapped), Pointer(lp_completion_routine), Pointer(lp_name_handle))
        }
        "GetAddrInfoExW" => {
            signature!(Generic, 10, 10; WideString(p_name), WideString(p_service_name), Dword(dw_name_space), Pointer(lp_nsp_id), Pointer(hints), Pointer(pp_result), Pointer(timeout), Pointer(lp_overlapped), Pointer(lp_completion_routine), Pointer(lp_handle))
        }
        "ProcessSocketNotifications" => {
            signature!(Generic, 7, 7; Pointer(completion_port), Dword(registration_count), Pointer(registration_infos), Dword(timeout_ms), Dword(completion_count), Pointer(completion_port_entries), Pointer(received_entry_count))
        }
        "SetAddrInfoExA" => {
            signature!(Generic, 12, 12; AnsiString(p_name), AnsiString(p_service_name), Pointer(p_addresses), Dword(dw_address_count), Pointer(lp_blob), Dword(dw_flags), Dword(dw_name_space), Pointer(lp_nsp_id), Pointer(timeout), Pointer(lp_overlapped), Pointer(lp_completion_routine), Pointer(lp_name_handle))
        }
        "SetAddrInfoExW" => {
            signature!(Generic, 12, 12; WideString(p_name), WideString(p_service_name), Pointer(p_addresses), Dword(dw_address_count), Pointer(lp_blob), Dword(dw_flags), Dword(dw_name_space), Pointer(lp_nsp_id), Pointer(timeout), Pointer(lp_overlapped), Pointer(lp_completion_routine), Pointer(lp_name_handle))
        }
        "WPUCompleteOverlappedRequest" => {
            signature!(Generic, 5, 5; Pointer(s), Pointer(lp_overlapped), Dword(dw_error), Dword(cb_transferred), Pointer(lp_errno))
        }
        "WSAAsyncGetHostByName" | "WSAAsyncGetProtoByName" => {
            signature!(Generic, 5, 5; Pointer(h_wnd), Dword(w_msg), AnsiString(name), Pointer(buf), Dword(buflen))
        }
        "WSAAsyncGetServByName" => {
            signature!(Generic, 6, 6; Pointer(h_wnd), Dword(w_msg), AnsiString(name), AnsiString(proto), Pointer(buf), Dword(buflen))
        }
        "WSAConnectByNameA" => {
            signature!(Generic, 9, 9; Pointer(s), AnsiString(nodename), AnsiString(servicename), Pointer(local_address_length), Pointer(local_address), Pointer(remote_address_length), Pointer(remote_address), Pointer(timeout), Pointer(reserved))
        }
        "WSAConnectByNameW" => {
            signature!(Generic, 9, 9; Pointer(s), WideString(nodename), WideString(servicename), Pointer(local_address_length), Pointer(local_address), Pointer(remote_address_length), Pointer(remote_address), Pointer(timeout), Pointer(reserved))
        }
        "WSAEnumNetworkEvents" => {
            signature!(WinsockStatus, 3, 3; Pointer(s), Pointer(h_event_object), Pointer(lp_network_events))
        }
        "WSALookupServiceBeginA" | "WSALookupServiceBeginW" => {
            signature!(WinsockStatus, 3, 3; Pointer(lpqs_restrictions), Dword(dw_control_flags), Pointer(lph_lookup))
        }
        "WSANSPIoctl" => {
            signature!(WinsockStatus, 8, 8; Pointer(h_lookup), Dword(dw_control_code), Pointer(lpv_in_buffer), Dword(cb_in_buffer), Pointer(lpv_out_buffer), Dword(cb_out_buffer), Pointer(lpcb_bytes_returned), Pointer(lp_completion))
        }
        "WSAProviderCompleteAsyncCall" => {
            signature!(Generic, 2, 2; Pointer(h_async_call), Dword(i_ret_code))
        }
        "WSASetBlockingHook" => signature!(Generic, 1, 1; Pointer(lp_block_func)),
        "WSASetServiceA" | "WSASetServiceW" => {
            signature!(WinsockStatus, 3, 3; Pointer(lpqs_reg_info), Dword(essoperation), Dword(dw_control_flags))
        }
        "getprotobyname" => signature!(Generic, 1, 1; AnsiString(name)),
        "getprotobynumber" => signature!(Generic, 1, 1; Dword(proto)),
        "getservbyname" => signature!(Generic, 2, 2; AnsiString(name), AnsiString(proto)),
        "getservbyport" => signature!(Generic, 2, 2; Dword(port), AnsiString(proto)),
        "select" => {
            signature!(WinsockStatus, 5, 5; Dword(nfds), Pointer(readfds), Pointer(writefds), Pointer(exceptfds), Pointer(timeout))
        }
        "recvfrom" => {
            signature!(WinsockStatus, 6, 6; Pointer(socket), Pointer(buffer), Dword(length), Dword(flags), Pointer(from), Pointer(address_length))
        }
        "sendto" => {
            signature!(WinsockStatus, 6, 6; Pointer(socket), Pointer(buffer), Dword(length), Dword(flags), Pointer(to), Dword(address_length))
        }
        "WSAPoll" => signature!(WinsockStatus, 3, 3; Pointer(fds), Dword(count), Dword(timeout)),
        "WSALookupServiceNextA" => {
            signature!(WinsockStatus, 4, 4; Pointer(lookup), Dword(control_flags), Pointer(buffer_length), Pointer(results))
        }
        "WSALookupServiceNextW" => {
            signature!(WinsockStatus, 4, 4; Pointer(lookup), Dword(control_flags), Pointer(buffer_length), Pointer(results))
        }
        "WSAAddressToStringA" => {
            signature!(WinsockStatus, 5, 5; Pointer(address), Dword(address_length), Pointer(protocol_info), Pointer(text), Pointer(text_length))
        }
        "WSAAddressToStringW" => {
            signature!(WinsockStatus, 5, 5; Pointer(address), Dword(address_length), Pointer(protocol_info), Pointer(text), Pointer(text_length))
        }
        "WSAStringToAddressA" => {
            signature!(WinsockStatus, 5, 5; Pointer(text), Dword(family), Pointer(protocol_info), Pointer(address), Pointer(address_length))
        }
        "WSAStringToAddressW" => {
            signature!(WinsockStatus, 5, 5; Pointer(text), Dword(family), Pointer(protocol_info), Pointer(address), Pointer(address_length))
        }
        _ => None,
    }
}
