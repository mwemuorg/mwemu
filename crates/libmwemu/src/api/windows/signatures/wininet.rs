use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "AppCacheCheckManifest" => {
            signature!(Generic, 8, 8; WideString(pwsz_master_url), WideString(pwsz_manifest_url), Pointer(pb_manifest_data), Dword(dw_manifest_data_size), Pointer(pb_manifest_response_headers), Dword(dw_manifest_response_headers_size), Pointer(pe_state), Pointer(ph_new_app_cache))
        }
        "AppCacheCloseHandle" => signature!(Void, 1, 1; Pointer(h_app_cache)),
        "AppCacheCreateAndCommitFile" => {
            signature!(Generic, 5, 5; Pointer(h_app_cache), WideString(pwsz_source_file_path), WideString(pwsz_url), Pointer(pb_response_headers), Dword(dw_response_headers_size))
        }
        "AppCacheDeleteGroup" | "AppCacheDeleteIEGroup" => {
            signature!(Generic, 1, 1; WideString(pwsz_manifest_url))
        }
        "AppCacheDuplicateHandle" => {
            signature!(Generic, 2, 2; Pointer(h_app_cache), Pointer(ph_duplicated_app_cache))
        }
        "AppCacheFinalize" => {
            signature!(Generic, 4, 4; Pointer(h_app_cache), Pointer(pb_manifest_data), Dword(dw_manifest_data_size), Pointer(pe_state))
        }
        "AppCacheFreeDownloadList" => signature!(Void, 1, 1; Pointer(p_download_list)),
        "AppCacheFreeGroupList" => signature!(Void, 1, 1; Pointer(p_app_cache_group_list)),
        "AppCacheFreeIESpace" | "AppCacheFreeSpace" => signature!(Generic, 1, 2),
        "AppCacheGetDownloadList" => {
            signature!(Generic, 2, 2; Pointer(h_app_cache), Pointer(p_download_list))
        }
        "AppCacheGetFallbackUrl" => {
            signature!(Generic, 3, 3; Pointer(h_app_cache), WideString(pwsz_url), Pointer(ppwsz_fallback_url))
        }
        "AppCacheGetGroupList" | "AppCacheGetIEGroupList" => {
            signature!(Generic, 1, 1; Pointer(p_app_cache_group_list))
        }
        "AppCacheGetInfo" => {
            signature!(Generic, 2, 2; Pointer(h_app_cache), Pointer(p_app_cache_info))
        }
        "AppCacheGetManifestUrl" => {
            signature!(Generic, 2, 2; Pointer(h_app_cache), Pointer(ppwsz_manifest_url))
        }
        "AppCacheLookup" => {
            signature!(Generic, 3, 3; WideString(pwsz_url), Dword(dw_flags), Pointer(ph_app_cache))
        }
        "CommitUrlCacheEntryA" | "CommitUrlCacheEntryW" => signature!(Generic, 9, 11),
        "CommitUrlCacheEntryBinaryBlob" => signature!(Generic, 6, 8),
        "CreateMD5SSOHash" => {
            signature!(Generic, 4, 4; WideString(psz_challenge_info), WideString(pwsz_realm), WideString(pwsz_target), Pointer(pb_hex_hash))
        }
        "CreateUrlCacheContainerA" => {
            signature!(Generic, 8, 8; AnsiString(name), AnsiString(lp_cache_prefix), AnsiString(lpsz_cache_path), Dword(kb_cache_limit), Dword(dw_container_type), Dword(dw_options), Pointer(pv_buffer), Pointer(cb_buffer))
        }
        "CreateUrlCacheContainerW" => {
            signature!(Generic, 8, 8; WideString(name), WideString(lp_cache_prefix), WideString(lpsz_cache_path), Dword(kb_cache_limit), Dword(dw_container_type), Dword(dw_options), Pointer(pv_buffer), Pointer(cb_buffer))
        }
        "CreateUrlCacheEntryA" => {
            signature!(Generic, 5, 5; AnsiString(lpsz_url_name), Dword(dw_expected_file_size), AnsiString(lpsz_file_extension), Pointer(lpsz_file_name), Dword(dw_reserved))
        }
        "CreateUrlCacheEntryExW" => {
            signature!(Generic, 6, 6; WideString(lpsz_url_name), Dword(dw_expected_file_size), WideString(lpsz_file_extension), Pointer(lpsz_file_name), Dword(dw_reserved), Dword(f_preserve_incoming_file_name))
        }
        "CreateUrlCacheEntryW" => {
            signature!(Generic, 5, 5; WideString(lpsz_url_name), Dword(dw_expected_file_size), WideString(lpsz_file_extension), Pointer(lpsz_file_name), Dword(dw_reserved))
        }
        "CreateUrlCacheGroup" => signature!(Generic, 2, 2; Dword(dw_flags), Pointer(lp_reserved)),
        "DeleteUrlCacheContainerA" => {
            signature!(Generic, 2, 2; AnsiString(name), Dword(dw_options))
        }
        "DeleteUrlCacheContainerW" => {
            signature!(Generic, 2, 2; WideString(name), Dword(dw_options))
        }
        "DeleteUrlCacheGroup" => {
            signature!(Generic, 3, 4; Qword(group_id), Dword(dw_flags), Pointer(lp_reserved))
        }
        "DetectAutoProxyUrl" => {
            signature!(Generic, 3, 3; Pointer(psz_auto_proxy_url), Dword(cch_auto_proxy_url), Dword(dw_detect_flags))
        }
        "FindFirstUrlCacheContainerA" | "FindFirstUrlCacheContainerW" => {
            signature!(Generic, 4, 4; Pointer(pdw_modified), Pointer(lp_container_info), Pointer(lpcb_container_info), Dword(dw_options))
        }
        "FindFirstUrlCacheEntryA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_url_search_pattern), Pointer(lp_first_cache_entry_info), Pointer(lpcb_cache_entry_info))
        }
        "FindFirstUrlCacheEntryExA" => {
            signature!(Generic, 9, 10; AnsiString(lpsz_url_search_pattern), Dword(dw_flags), Dword(dw_filter), Qword(group_id), Pointer(lp_first_cache_entry_info), Pointer(lpcb_cache_entry_info), Pointer(lp_group_attributes), Pointer(lpcb_group_attributes), Pointer(lp_reserved))
        }
        "FindFirstUrlCacheEntryExW" => {
            signature!(Generic, 9, 10; WideString(lpsz_url_search_pattern), Dword(dw_flags), Dword(dw_filter), Qword(group_id), Pointer(lp_first_cache_entry_info), Pointer(lpcb_cache_entry_info), Pointer(lp_group_attributes), Pointer(lpcb_group_attributes), Pointer(lp_reserved))
        }
        "FindFirstUrlCacheEntryW" => {
            signature!(Generic, 3, 3; WideString(lpsz_url_search_pattern), Pointer(lp_first_cache_entry_info), Pointer(lpcb_cache_entry_info))
        }
        "FindFirstUrlCacheGroup" => {
            signature!(Generic, 6, 6; Dword(dw_flags), Dword(dw_filter), Pointer(lp_search_condition), Dword(dw_search_condition), Pointer(lp_group_id), Pointer(lp_reserved))
        }
        "FindNextUrlCacheContainerA" | "FindNextUrlCacheContainerW" => {
            signature!(Generic, 3, 3; Pointer(h_enum_handle), Pointer(lp_container_info), Pointer(lpcb_container_info))
        }
        "FindNextUrlCacheEntryA" | "FindNextUrlCacheEntryW" => {
            signature!(Generic, 3, 3; Pointer(h_enum_handle), Pointer(lp_next_cache_entry_info), Pointer(lpcb_cache_entry_info))
        }
        "FindNextUrlCacheEntryExA" | "FindNextUrlCacheEntryExW" => {
            signature!(Generic, 6, 6; Pointer(h_enum_handle), Pointer(lp_next_cache_entry_info), Pointer(lpcb_cache_entry_info), Pointer(lp_group_attributes), Pointer(lpcb_group_attributes), Pointer(lp_reserved))
        }
        "FindNextUrlCacheGroup" => {
            signature!(Generic, 3, 3; Pointer(h_find), Pointer(lp_group_id), Pointer(lp_reserved))
        }
        "FreeUrlCacheSpaceA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_cache_path), Dword(dw_size), Dword(dw_filter))
        }
        "FreeUrlCacheSpaceW" => {
            signature!(Generic, 3, 3; WideString(lpsz_cache_path), Dword(dw_size), Dword(dw_filter))
        }
        "FtpCommandA" => {
            signature!(Generic, 6, 6; Pointer(h_connect), Dword(f_expect_response), Dword(dw_flags), AnsiString(lpsz_command), Pointer(dw_context), Pointer(ph_ftp_command))
        }
        "FtpCommandW" => {
            signature!(Generic, 6, 6; Pointer(h_connect), Dword(f_expect_response), Dword(dw_flags), WideString(lpsz_command), Pointer(dw_context), Pointer(ph_ftp_command))
        }
        "FtpCreateDirectoryA" | "FtpRemoveDirectoryA" | "FtpSetCurrentDirectoryA" => {
            signature!(Generic, 2, 2; Pointer(h_connect), AnsiString(lpsz_directory))
        }
        "FtpCreateDirectoryW" | "FtpRemoveDirectoryW" | "FtpSetCurrentDirectoryW" => {
            signature!(Generic, 2, 2; Pointer(h_connect), WideString(lpsz_directory))
        }
        "FtpDeleteFileA" => {
            signature!(Generic, 2, 2; Pointer(h_connect), AnsiString(lpsz_file_name))
        }
        "FtpDeleteFileW" => {
            signature!(Generic, 2, 2; Pointer(h_connect), WideString(lpsz_file_name))
        }
        "FtpFindFirstFileA" => {
            signature!(Generic, 5, 5; Pointer(h_connect), AnsiString(lpsz_search_file), Pointer(lp_find_file_data), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpFindFirstFileW" => {
            signature!(Generic, 5, 5; Pointer(h_connect), WideString(lpsz_search_file), Pointer(lp_find_file_data), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpGetCurrentDirectoryA" | "FtpGetCurrentDirectoryW" => {
            signature!(Generic, 3, 3; Pointer(h_connect), Pointer(lpsz_current_directory), Pointer(lpdw_current_directory))
        }
        "FtpGetFileA" => {
            signature!(Generic, 7, 7; Pointer(h_connect), AnsiString(lpsz_remote_file), AnsiString(lpsz_new_file), Dword(f_fail_if_exists), Dword(dw_flags_and_attributes), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpGetFileEx" => {
            signature!(Generic, 7, 7; Pointer(h_ftp_session), AnsiString(lpsz_remote_file), WideString(lpsz_new_file), Dword(f_fail_if_exists), Dword(dw_flags_and_attributes), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpGetFileSize" => {
            signature!(Generic, 2, 2; Pointer(h_file), Pointer(lpdw_file_size_high))
        }
        "FtpGetFileW" => {
            signature!(Generic, 7, 7; Pointer(h_connect), WideString(lpsz_remote_file), WideString(lpsz_new_file), Dword(f_fail_if_exists), Dword(dw_flags_and_attributes), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpOpenFileA" => {
            signature!(Handle, 5, 5; Pointer(h_connect), AnsiString(lpsz_file_name), Dword(dw_access), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpOpenFileW" => {
            signature!(Handle, 5, 5; Pointer(h_connect), WideString(lpsz_file_name), Dword(dw_access), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpPutFileA" => {
            signature!(Generic, 5, 5; Pointer(h_connect), AnsiString(lpsz_local_file), AnsiString(lpsz_new_remote_file), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpPutFileEx" => {
            signature!(Generic, 5, 5; Pointer(h_ftp_session), WideString(lpsz_local_file), AnsiString(lpsz_new_remote_file), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpPutFileW" => {
            signature!(Generic, 5, 5; Pointer(h_connect), WideString(lpsz_local_file), WideString(lpsz_new_remote_file), Dword(dw_flags), Pointer(dw_context))
        }
        "FtpRenameFileA" => {
            signature!(Generic, 3, 3; Pointer(h_connect), AnsiString(lpsz_existing), AnsiString(lpsz_new))
        }
        "FtpRenameFileW" => {
            signature!(Generic, 3, 3; Pointer(h_connect), WideString(lpsz_existing), WideString(lpsz_new))
        }
        "GetUrlCacheConfigInfoA" | "GetUrlCacheConfigInfoW" => {
            signature!(Generic, 3, 3; Pointer(lp_cache_config_info), Pointer(lpcb_cache_config_info), Dword(dw_field_control))
        }
        "GetUrlCacheEntryBinaryBlob" => {
            signature!(Generic, 7, 7; WideString(pwsz_url_name), Pointer(dw_type), Pointer(pft_expire_time), Pointer(pft_access_time), Pointer(pft_modified_time), Pointer(ppb_blob), Pointer(pcb_blob))
        }
        "GetUrlCacheEntryInfoA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_url_name), Pointer(lp_cache_entry_info), Pointer(lpcb_cache_entry_info))
        }
        "GetUrlCacheEntryInfoExA" => {
            signature!(Generic, 7, 7; AnsiString(lpsz_url), Pointer(lp_cache_entry_info), Pointer(lpcb_cache_entry_info), Pointer(lpsz_redirect_url), Pointer(lpcb_redirect_url), Pointer(lp_reserved), Dword(dw_flags))
        }
        "GetUrlCacheEntryInfoExW" => {
            signature!(Generic, 7, 7; WideString(lpsz_url), Pointer(lp_cache_entry_info), Pointer(lpcb_cache_entry_info), Pointer(lpsz_redirect_url), Pointer(lpcb_redirect_url), Pointer(lp_reserved), Dword(dw_flags))
        }
        "GetUrlCacheEntryInfoW" => {
            signature!(Generic, 3, 3; WideString(lpsz_url_name), Pointer(lp_cache_entry_info), Pointer(lpcb_cache_entry_info))
        }
        "GetUrlCacheGroupAttributeA" | "GetUrlCacheGroupAttributeW" => {
            signature!(Generic, 6, 7; Qword(gid), Dword(dw_flags), Dword(dw_attributes), Pointer(lp_group_info), Pointer(lpcb_group_info), Pointer(lp_reserved))
        }
        "GopherCreateLocatorA" => {
            signature!(Generic, 7, 7; AnsiString(lpsz_host), Word(n_server_port), AnsiString(lpsz_display_string), AnsiString(lpsz_selector_string), Dword(dw_gopher_type), Pointer(lpsz_locator), Pointer(lpdw_buffer_length))
        }
        "GopherCreateLocatorW" => {
            signature!(Generic, 7, 7; WideString(lpsz_host), Word(n_server_port), WideString(lpsz_display_string), WideString(lpsz_selector_string), Dword(dw_gopher_type), Pointer(lpsz_locator), Pointer(lpdw_buffer_length))
        }
        "GopherFindFirstFileA" => {
            signature!(Generic, 6, 6; Pointer(h_connect), AnsiString(lpsz_locator), AnsiString(lpsz_search_string), Pointer(lp_find_data), Dword(dw_flags), Pointer(dw_context))
        }
        "GopherFindFirstFileW" => {
            signature!(Generic, 6, 6; Pointer(h_connect), WideString(lpsz_locator), WideString(lpsz_search_string), Pointer(lp_find_data), Dword(dw_flags), Pointer(dw_context))
        }
        "GopherGetAttributeA" => {
            signature!(Generic, 8, 8; Pointer(h_connect), AnsiString(lpsz_locator), AnsiString(lpsz_attribute_name), Pointer(lp_buffer), Dword(dw_buffer_length), Pointer(lpdw_characters_returned), Pointer(lpfn_enumerator), Pointer(dw_context))
        }
        "GopherGetAttributeW" => {
            signature!(Generic, 8, 8; Pointer(h_connect), WideString(lpsz_locator), WideString(lpsz_attribute_name), Pointer(lp_buffer), Dword(dw_buffer_length), Pointer(lpdw_characters_returned), Pointer(lpfn_enumerator), Pointer(dw_context))
        }
        "GopherGetLocatorTypeA" => {
            signature!(Generic, 2, 2; AnsiString(lpsz_locator), Pointer(lpdw_gopher_type))
        }
        "GopherGetLocatorTypeW" => {
            signature!(Generic, 2, 2; WideString(lpsz_locator), Pointer(lpdw_gopher_type))
        }
        "GopherOpenFileA" => {
            signature!(Generic, 5, 5; Pointer(h_connect), AnsiString(lpsz_locator), AnsiString(lpsz_view), Dword(dw_flags), Pointer(dw_context))
        }
        "GopherOpenFileW" => {
            signature!(Generic, 5, 5; Pointer(h_connect), WideString(lpsz_locator), WideString(lpsz_view), Dword(dw_flags), Pointer(dw_context))
        }
        "HttpAddRequestHeadersA" | "HttpAddRequestHeadersW" => {
            signature!(Generic, 4, 4; Pointer(h_request), Pointer(lpsz_headers), Dword(dw_headers_length), Dword(dw_modifiers))
        }
        "HttpDuplicateDependencyHandle" => {
            signature!(Generic, 2, 2; Pointer(h_dependency_handle), Pointer(ph_duplicated_dependency_handle))
        }
        "HttpEndRequestA" | "HttpEndRequestW" => {
            signature!(Generic, 4, 4; Pointer(h_request), Pointer(lp_buffers_out), Dword(dw_flags), Pointer(dw_context))
        }
        "HttpGetServerCredentials" => {
            signature!(Generic, 3, 3; WideString(pwsz_url), Pointer(ppwsz_user_name), Pointer(ppwsz_password))
        }
        "HttpIndicatePageLoadComplete" => signature!(Generic, 1, 1; Pointer(h_dependency_handle)),
        "HttpIsHostHstsEnabled" => {
            signature!(Generic, 2, 2; WideString(pcwsz_url), Pointer(pf_is_hsts))
        }
        "HttpOpenDependencyHandle" => {
            signature!(Generic, 3, 3; Pointer(h_request_handle), Dword(f_background), Pointer(ph_dependency_handle))
        }
        "HttpPushEnable" => {
            signature!(Generic, 3, 3; Pointer(h_request), Pointer(p_transport_setting), Pointer(ph_wait))
        }
        "HttpPushWait" => {
            signature!(Generic, 3, 3; Pointer(h_wait), Dword(e_type), Pointer(p_notification_status))
        }
        "HttpSendRequestExA" | "HttpSendRequestExW" => {
            signature!(Generic, 5, 5; Pointer(h_request), Pointer(lp_buffers_in), Pointer(lp_buffers_out), Dword(dw_flags), Pointer(dw_context))
        }
        "HttpWebSocketCompleteUpgrade" => {
            signature!(Generic, 2, 2; Pointer(h_request), Pointer(dw_context))
        }
        "HttpWebSocketSend" => {
            signature!(Generic, 4, 4; Pointer(h_web_socket), Dword(buffer_type), Pointer(pv_buffer), Dword(dw_buffer_length))
        }
        "HttpWebSocketShutdown" => {
            signature!(Generic, 4, 4; Pointer(h_web_socket), Word(us_status), Pointer(pv_reason), Dword(dw_reason_length))
        }
        "InternetAlgIdToStringA" | "InternetAlgIdToStringW" => {
            signature!(Generic, 4, 4; Dword(ai), Pointer(lpstr), Pointer(lpdwstr_length), Dword(dw_reserved))
        }
        "InternetAttemptConnect" | "InternetAutodialHangup" | "InternetInitializeAutoProxyDll" => {
            signature!(Generic, 1, 1; Dword(dw_reserved))
        }
        "InternetAutodial" => signature!(Generic, 2, 2; Dword(dw_flags), Pointer(hwnd_parent)),
        "InternetCanonicalizeUrlA" => {
            signature!(Win32Bool, 4, 4; AnsiString(lpsz_url), Pointer(lpsz_buffer), Pointer(lpdw_buffer_length), Dword(dw_flags))
        }
        "InternetCanonicalizeUrlW" => {
            signature!(Win32Bool, 4, 4; WideString(lpsz_url), Pointer(lpsz_buffer), Pointer(lpdw_buffer_length), Dword(dw_flags))
        }
        "InternetCheckConnectionA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_url), Dword(dw_flags), Dword(dw_reserved))
        }
        "InternetCheckConnectionW" => {
            signature!(Generic, 3, 3; WideString(lpsz_url), Dword(dw_flags), Dword(dw_reserved))
        }
        "InternetClearAllPerSiteCookieDecisions"
        | "LoadUrlCacheContent"
        | "UrlCacheReloadSettings"
        | "UrlCacheServer" => signature!(Generic, 0, 0; ),
        "InternetCombineUrlA" => {
            signature!(Win32Bool, 5, 5; AnsiString(lpsz_base_url), AnsiString(lpsz_relative_url), Pointer(lpsz_buffer), Pointer(lpdw_buffer_length), Dword(dw_flags))
        }
        "InternetCombineUrlW" => {
            signature!(Win32Bool, 5, 5; WideString(lpsz_base_url), WideString(lpsz_relative_url), Pointer(lpsz_buffer), Pointer(lpdw_buffer_length), Dword(dw_flags))
        }
        "InternetConfirmZoneCrossing" | "InternetConfirmZoneCrossingA" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), AnsiString(sz_url_prev), AnsiString(sz_url_new), Dword(b_post))
        }
        "InternetConfirmZoneCrossingW" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), WideString(sz_url_prev), WideString(sz_url_new), Dword(b_post))
        }
        "InternetConvertUrlFromWireToWideChar" => {
            signature!(Generic, 8, 8; Pointer(pcsz_url), Dword(cch_url), WideString(pcwsz_base_url), Dword(dw_code_page_host), Dword(dw_code_page_path), Dword(f_encode_path_extra), Dword(dw_code_page_extra), Pointer(ppwsz_converted_url))
        }
        "InternetCreateUrlA" | "InternetCreateUrlW" => {
            signature!(Generic, 4, 4; Pointer(lp_url_components), Dword(dw_flags), Pointer(lpsz_url), Pointer(lpdw_url_length))
        }
        "InternetDial" | "InternetDialA" => {
            signature!(Generic, 5, 5; Pointer(hwnd_parent), AnsiString(lpsz_connectoid), Dword(dw_flags), Pointer(lpdw_connection), Dword(dw_reserved))
        }
        "InternetDialW" => {
            signature!(Generic, 5, 5; Pointer(hwnd_parent), WideString(lpsz_connectoid), Dword(dw_flags), Pointer(lpdw_connection), Dword(dw_reserved))
        }
        "InternetEnumPerSiteCookieDecisionA" | "InternetEnumPerSiteCookieDecisionW" => {
            signature!(Generic, 4, 4; Pointer(psz_site_name), Pointer(pc_site_name_size), Pointer(pdw_decision), Dword(dw_index))
        }
        "InternetFindNextFileA" | "InternetFindNextFileW" => {
            signature!(Generic, 2, 2; Pointer(h_find), Pointer(lpv_find_data))
        }
        "InternetFortezzaCommand" => {
            signature!(Generic, 3, 3; Dword(dw_command), Pointer(hwnd), Pointer(dw_reserved))
        }
        "InternetGetConnectedState" => {
            signature!(Generic, 2, 2; Pointer(lpdw_flags), Dword(dw_reserved))
        }
        "InternetGetConnectedStateEx" => {
            signature!(Generic, 4, 4; Pointer(lpdw_flags), Pointer(lpsz_connection_name), Dword(dw_name_len), Dword(dw_reserved))
        }
        "InternetGetConnectedStateExA" | "InternetGetConnectedStateExW" => {
            signature!(Generic, 4, 4; Pointer(lpdw_flags), Pointer(lpsz_connection_name), Dword(cch_name_len), Dword(dw_reserved))
        }
        "InternetGetCookieA" => {
            signature!(Generic, 4, 4; AnsiString(lpsz_url), AnsiString(lpsz_cookie_name), Pointer(lpsz_cookie_data), Pointer(lpdw_size))
        }
        "InternetGetCookieEx2" => {
            signature!(Generic, 5, 5; WideString(pcwsz_url), WideString(pcwsz_cookie_name), Dword(dw_flags), Pointer(pp_cookies), Pointer(pdw_cookie_count))
        }
        "InternetGetCookieExA" => {
            signature!(Generic, 6, 6; AnsiString(lpsz_url), AnsiString(lpsz_cookie_name), Pointer(lpsz_cookie_data), Pointer(lpdw_size), Dword(dw_flags), Pointer(lp_reserved))
        }
        "InternetGetCookieExW" => {
            signature!(Generic, 6, 6; WideString(lpsz_url), WideString(lpsz_cookie_name), Pointer(lpsz_cookie_data), Pointer(lpdw_size), Dword(dw_flags), Pointer(lp_reserved))
        }
        "InternetGetCookieW" => {
            signature!(Generic, 4, 4; WideString(lpsz_url), WideString(lpsz_cookie_name), Pointer(lpsz_cookie_data), Pointer(lpdw_size))
        }
        "InternetGetLastResponseInfoA" | "InternetGetLastResponseInfoW" => {
            signature!(Generic, 3, 3; Pointer(lpdw_error), Pointer(lpsz_buffer), Pointer(lpdw_buffer_length))
        }
        "InternetGetPerSiteCookieDecisionA" => {
            signature!(Generic, 2, 2; AnsiString(pch_host_name), Pointer(p_result))
        }
        "InternetGetPerSiteCookieDecisionW" => {
            signature!(Generic, 2, 2; WideString(pch_host_name), Pointer(p_result))
        }
        "InternetGetProxyForUrl" => {
            signature!(Generic, 3, 3; Pointer(h_internet), WideString(pcwsz_url), Pointer(p_proxy_info_list))
        }
        "InternetGetSecurityInfoByURL" | "InternetGetSecurityInfoByURLA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_url), Pointer(pp_cert_chain), Pointer(pdw_secure_flags))
        }
        "InternetGetSecurityInfoByURLW" => {
            signature!(Generic, 3, 3; WideString(lpsz_url), Pointer(pp_cert_chain), Pointer(pdw_secure_flags))
        }
        "InternetGoOnline" | "InternetGoOnlineA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_url), Pointer(hwnd_parent), Dword(dw_flags))
        }
        "InternetGoOnlineW" => {
            signature!(Generic, 3, 3; WideString(lpsz_url), Pointer(hwnd_parent), Dword(dw_flags))
        }
        "InternetHangUp" => signature!(Generic, 2, 2; Pointer(dw_connection), Dword(dw_reserved)),
        "InternetLockRequestFile" => {
            signature!(Generic, 2, 2; Pointer(h_internet), Pointer(lph_lock_request_info))
        }
        "InternetOpenUrlA" => {
            signature!(Handle, 6, 6; Pointer(h_internet), AnsiString(lpsz_url), Pointer(lpsz_headers), Dword(dw_headers_length), Dword(dw_flags), Pointer(dw_context))
        }
        "InternetOpenUrlW" => {
            signature!(Handle, 6, 6; Pointer(h_internet), WideString(lpsz_url), Pointer(lpsz_headers), Dword(dw_headers_length), Dword(dw_flags), Pointer(dw_context))
        }
        "InternetQueryDataAvailable" => {
            signature!(Generic, 4, 4; Pointer(h_file), Pointer(lpdw_number_of_bytes_available), Dword(dw_flags), Pointer(dw_context))
        }
        "InternetQueryOptionA" | "InternetQueryOptionW" => {
            signature!(Win32Bool, 4, 4; Pointer(h_internet), Dword(dw_option), Pointer(lp_buffer), Pointer(lpdw_buffer_length))
        }
        "InternetSecurityProtocolToStringA" | "InternetSecurityProtocolToStringW" => {
            signature!(Generic, 4, 4; Dword(dw_protocol), Pointer(lpstr), Pointer(lpdwstr_length), Dword(dw_reserved))
        }
        "InternetSetCookieA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_url), AnsiString(lpsz_cookie_name), AnsiString(lpsz_cookie_data))
        }
        "InternetSetCookieEx2" => {
            signature!(Generic, 5, 5; WideString(pcwsz_url), Pointer(p_cookie), WideString(pcwsz_p3_p_policy), Dword(dw_flags), Pointer(pdw_cookie_state))
        }
        "InternetSetCookieExA" => {
            signature!(Generic, 5, 5; AnsiString(lpsz_url), AnsiString(lpsz_cookie_name), AnsiString(lpsz_cookie_data), Dword(dw_flags), Pointer(dw_reserved))
        }
        "InternetSetCookieExW" => {
            signature!(Generic, 5, 5; WideString(lpsz_url), WideString(lpsz_cookie_name), WideString(lpsz_cookie_data), Dword(dw_flags), Pointer(dw_reserved))
        }
        "InternetSetCookieW" => {
            signature!(Generic, 3, 3; WideString(lpsz_url), WideString(lpsz_cookie_name), WideString(lpsz_cookie_data))
        }
        "InternetSetDialState" | "InternetSetDialStateA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_connectoid), Dword(dw_state), Dword(dw_reserved))
        }
        "InternetSetDialStateW" => {
            signature!(Generic, 3, 3; WideString(lpsz_connectoid), Dword(dw_state), Dword(dw_reserved))
        }
        "InternetSetFilePointer" => {
            signature!(Generic, 5, 5; Pointer(h_file), Dword(l_distance_to_move), Pointer(lp_distance_to_move_high), Dword(dw_move_method), Pointer(dw_context))
        }
        "InternetSetOptionExA" | "InternetSetOptionExW" => {
            signature!(Generic, 5, 5; Pointer(h_internet), Dword(dw_option), Pointer(lp_buffer), Dword(dw_buffer_length), Dword(dw_flags))
        }
        "InternetSetPerSiteCookieDecisionA" => {
            signature!(Generic, 2, 2; AnsiString(pch_host_name), Dword(dw_decision))
        }
        "InternetSetPerSiteCookieDecisionW" => {
            signature!(Generic, 2, 2; WideString(pch_host_name), Dword(dw_decision))
        }
        "InternetShowSecurityInfoByURL" | "InternetShowSecurityInfoByURLA" => {
            signature!(Generic, 2, 2; AnsiString(lpsz_url), Pointer(hwnd_parent))
        }
        "InternetShowSecurityInfoByURLW" => {
            signature!(Generic, 2, 2; WideString(lpsz_url), Pointer(hwnd_parent))
        }
        "InternetTimeFromSystemTime"
        | "InternetTimeFromSystemTimeA"
        | "InternetTimeFromSystemTimeW" => {
            signature!(Generic, 4, 4; Pointer(pst), Dword(dw_rfc), Pointer(lpsz_time), Dword(cb_time))
        }
        "InternetTimeToSystemTime" | "InternetTimeToSystemTimeA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_time), Pointer(pst), Dword(dw_reserved))
        }
        "InternetTimeToSystemTimeW" => {
            signature!(Generic, 3, 3; WideString(lpsz_time), Pointer(pst), Dword(dw_reserved))
        }
        "InternetWriteFile" => {
            signature!(Win32Bool, 4, 4; Pointer(h_file), Pointer(lp_buffer), Dword(dw_number_of_bytes_to_write), Pointer(lpdw_number_of_bytes_written))
        }
        "InternetWriteFileExA" | "InternetWriteFileExW" => {
            signature!(Win32Bool, 4, 4; Pointer(h_file), Pointer(lp_buffers_in), Dword(dw_flags), Pointer(dw_context))
        }
        "IsHostInProxyBypassList" => {
            signature!(Generic, 3, 3; Dword(t_scheme), Pointer(lpsz_host), Dword(cch_host))
        }
        "IsUrlCacheEntryExpiredA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_url_name), Dword(dw_flags), Pointer(pft_last_modified))
        }
        "IsUrlCacheEntryExpiredW" => {
            signature!(Generic, 3, 3; WideString(lpsz_url_name), Dword(dw_flags), Pointer(pft_last_modified))
        }
        "ParseX509EncodedCertificateForListBoxEntry" => {
            signature!(Generic, 4, 4; Pointer(lp_cert), Dword(cb_cert), Pointer(lpsz_list_box_entry), Pointer(lpdw_list_box_entry))
        }
        "PrivacyGetZonePreferenceW" => {
            signature!(Generic, 5, 5; Dword(dw_zone), Dword(dw_type), Pointer(pdw_template), Pointer(psz_buffer), Pointer(pdw_buffer_length))
        }
        "PrivacySetZonePreferenceW" => {
            signature!(Generic, 4, 4; Dword(dw_zone), Dword(dw_type), Dword(dw_template), WideString(psz_preference))
        }
        "RegisterUrlCacheNotification" => {
            signature!(Generic, 5, 6; Pointer(h_wnd), Dword(u_msg), Qword(gid), Dword(dw_ops_filter), Dword(dw_reserved))
        }
        "ResumeSuspendedDownload" => {
            signature!(Generic, 2, 2; Pointer(h_request), Dword(dw_result_code))
        }
        "RetrieveUrlCacheEntryFileA" => {
            signature!(Generic, 4, 4; AnsiString(lpsz_url_name), Pointer(lp_cache_entry_info), Pointer(lpcb_cache_entry_info), Dword(dw_reserved))
        }
        "RetrieveUrlCacheEntryFileW" => {
            signature!(Generic, 4, 4; WideString(lpsz_url_name), Pointer(lp_cache_entry_info), Pointer(lpcb_cache_entry_info), Dword(dw_reserved))
        }
        "RetrieveUrlCacheEntryStreamA" => {
            signature!(Generic, 5, 5; AnsiString(lpsz_url_name), Pointer(lp_cache_entry_info), Pointer(lpcb_cache_entry_info), Dword(f_random_read), Dword(dw_reserved))
        }
        "RetrieveUrlCacheEntryStreamW" => {
            signature!(Generic, 5, 5; WideString(lpsz_url_name), Pointer(lp_cache_entry_info), Pointer(lpcb_cache_entry_info), Dword(f_random_read), Dword(dw_reserved))
        }
        "RunOnceUrlCache" => {
            signature!(Generic, 4, 4; Pointer(hwnd), Pointer(hinst), AnsiString(lpsz_cmd), Dword(n_cmd_show))
        }
        "SetUrlCacheConfigInfoA" | "SetUrlCacheConfigInfoW" => {
            signature!(Generic, 2, 2; Pointer(lp_cache_config_info), Dword(dw_field_control))
        }
        "SetUrlCacheEntryGroup" | "SetUrlCacheEntryGroupA" => {
            signature!(Generic, 6, 7; AnsiString(lpsz_url_name), Dword(dw_flags), Qword(group_id), Pointer(pb_group_attributes), Dword(cb_group_attributes), Pointer(lp_reserved))
        }
        "SetUrlCacheEntryGroupW" => {
            signature!(Generic, 6, 7; WideString(lpsz_url_name), Dword(dw_flags), Qword(group_id), Pointer(pb_group_attributes), Dword(cb_group_attributes), Pointer(lp_reserved))
        }
        "SetUrlCacheEntryInfoA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_url_name), Pointer(lp_cache_entry_info), Dword(dw_field_control))
        }
        "SetUrlCacheEntryInfoW" => {
            signature!(Generic, 3, 3; WideString(lpsz_url_name), Pointer(lp_cache_entry_info), Dword(dw_field_control))
        }
        "SetUrlCacheGroupAttributeA" | "SetUrlCacheGroupAttributeW" => {
            signature!(Generic, 5, 6; Qword(gid), Dword(dw_flags), Dword(dw_attributes), Pointer(lp_group_info), Pointer(lp_reserved))
        }
        "SetUrlCacheHeaderData" => signature!(Generic, 2, 2; Dword(n_idx), Dword(dw_data)),
        "ShowClientAuthCerts" => signature!(Generic, 1, 1; Pointer(h_wnd_parent)),
        "ShowSecurityInfo" => {
            signature!(Generic, 2, 2; Pointer(h_wnd_parent), Pointer(p_security_info))
        }
        "ShowX509EncodedCertificate" => {
            signature!(Generic, 3, 3; Pointer(h_wnd_parent), Pointer(lp_cert), Dword(cb_cert))
        }
        "UnlockUrlCacheEntryFile" | "UnlockUrlCacheEntryFileA" => {
            signature!(Generic, 2, 2; AnsiString(lpsz_url_name), Dword(dw_reserved))
        }
        "UnlockUrlCacheEntryFileW" => {
            signature!(Generic, 2, 2; WideString(lpsz_url_name), Dword(dw_reserved))
        }
        "UnlockUrlCacheEntryStream" => {
            signature!(Generic, 2, 2; Pointer(h_url_cache_stream), Dword(reserved))
        }
        "UpdateUrlCacheContentPath" => signature!(Generic, 1, 1; AnsiString(sz_new_path)),
        "UrlCacheCheckEntriesExist" => {
            signature!(Generic, 3, 3; Pointer(rgpwsz_urls), Dword(c_entries), Pointer(rgf_exist))
        }
        "UrlCacheContainerSetEntryMaximumAge" => {
            signature!(Generic, 2, 2; WideString(pwsz_prefix), Dword(dw_entry_max_age))
        }
        "UrlCacheCreateContainer" => {
            signature!(Generic, 5, 6; WideString(pwsz_name), WideString(pwsz_prefix), WideString(pwsz_directory), Qword(ull_limit), Dword(dw_options))
        }
        "UrlCacheFindFirstEntry" => {
            signature!(Generic, 6, 7; WideString(pwsz_prefix), Dword(dw_flags), Dword(dw_filter), Qword(group_id), Pointer(p_cache_entry_info), Pointer(ph_find))
        }
        "UrlCacheFindNextEntry" => {
            signature!(Generic, 2, 2; Pointer(h_find), Pointer(p_cache_entry_info))
        }
        "UrlCacheFreeEntryInfo" => signature!(Void, 1, 1; Pointer(p_cache_entry_info)),
        "UrlCacheFreeGlobalSpace" => {
            signature!(Generic, 2, 3; Qword(ull_target_size), Dword(dw_filter))
        }
        "UrlCacheGetContentPaths" => {
            signature!(Generic, 2, 2; Pointer(pppwsz_directories), Pointer(pc_directories))
        }
        "UrlCacheGetEntryInfo" => {
            signature!(Generic, 3, 3; Pointer(h_app_cache), WideString(pcwsz_url), Pointer(p_cache_entry_info))
        }
        "UrlCacheGetGlobalCacheSize" => {
            signature!(Generic, 3, 3; Dword(dw_filter), Pointer(pull_size), Pointer(pull_limit))
        }
        "UrlCacheGetGlobalLimit" => {
            signature!(Generic, 2, 2; Dword(limit_type), Pointer(pull_limit))
        }
        "UrlCacheReadEntryStream" => {
            signature!(Generic, 5, 6; Pointer(h_url_cache_stream), Qword(ull_location), Pointer(p_buffer), Dword(dw_buffer_len), Pointer(pdw_buffer_len))
        }
        "UrlCacheRetrieveEntryFile" => {
            signature!(Generic, 4, 4; Pointer(h_app_cache), WideString(pcwsz_url), Pointer(p_cache_entry_info), Pointer(ph_entry_file))
        }
        "UrlCacheRetrieveEntryStream" => {
            signature!(Generic, 5, 5; Pointer(h_app_cache), WideString(pcwsz_url), Dword(f_random_read), Pointer(p_cache_entry_info), Pointer(ph_entry_stream))
        }
        "UrlCacheSetGlobalLimit" => signature!(Generic, 2, 3; Dword(limit_type), Qword(ull_limit)),
        "UrlCacheUpdateEntryExtraData" => {
            signature!(Generic, 4, 4; Pointer(h_app_cache), WideString(pcwsz_url), Pointer(pb_extra_data), Dword(cb_extra_data))
        }
        "InternetConnectA" => {
            signature!(Handle, 8, 8; Pointer(session), AnsiString(server), Word(port), Pointer(user_name), Pointer(password), Dword(service), Dword(flags), Pointer(context))
        }
        "InternetConnectW" => {
            signature!(Handle, 8, 8; Pointer(session), WideString(server), Word(port), Pointer(user_name), Pointer(password), Dword(service), Dword(flags), Pointer(context))
        }
        "InternetSetOptionA" => {
            signature!(Win32Bool, 4, 4; Pointer(handle), Dword(option), Pointer(buffer), Dword(buffer_length))
        }
        "InternetSetOptionW" => {
            signature!(Win32Bool, 4, 4; Pointer(handle), Dword(option), Pointer(buffer), Dword(buffer_length))
        }
        "HttpQueryInfoA" => {
            signature!(Win32Bool, 5, 5; Pointer(request), Dword(info_level), Pointer(buffer), Pointer(buffer_length), Pointer(index))
        }
        "HttpQueryInfoW" => {
            signature!(Win32Bool, 5, 5; Pointer(request), Dword(info_level), Pointer(buffer), Pointer(buffer_length), Pointer(index))
        }
        "HttpSendRequestA" => {
            signature!(Win32Bool, 5, 5; Pointer(request), Pointer(headers), Dword(headers_length), Pointer(optional), Dword(optional_length))
        }
        "HttpSendRequestW" => {
            signature!(Win32Bool, 5, 5; Pointer(request), Pointer(headers), Dword(headers_length), Pointer(optional), Dword(optional_length))
        }
        "InternetCrackUrlA" => {
            signature!(Win32Bool, 4, 4; Pointer(url), Dword(url_length), Dword(flags), Pointer(components))
        }
        "InternetCrackUrlW" => {
            signature!(Win32Bool, 4, 4; Pointer(url), Dword(url_length), Dword(flags), Pointer(components))
        }
        "InternetErrorDlg" => {
            signature!(Win32Status, 5, 5; Pointer(window), Pointer(request), Dword(error), Dword(flags), Pointer(data))
        }
        "DeleteIE3Cache" => {
            signature!(Generic, 4, 4; Pointer(hwnd), Pointer(hinst), AnsiString(lpsz_cmd), Dword(n_cmd_show))
        }
        "DeleteUrlCacheEntry" | "DeleteUrlCacheEntryA" => {
            signature!(Generic, 1, 1; AnsiString(lpsz_url_name))
        }
        "DeleteUrlCacheEntryW" => signature!(Generic, 1, 1; WideString(lpsz_url_name)),
        "DeleteWpadCacheForNetworks" => signature!(Generic, 1, 1; Dword(wpad_cache_delete)),
        "FindCloseUrlCache" => signature!(Generic, 1, 1; Pointer(h_enum_handle)),
        "HttpCloseDependencyHandle" => signature!(Void, 1, 1; Pointer(h_dependency_handle)),
        "HttpPushClose" => signature!(Void, 1, 1; Pointer(h_wait)),
        "HttpWebSocketClose" => {
            signature!(Generic, 4, 4; Pointer(h_web_socket), Word(us_status), Pointer(pv_reason), Dword(dw_reason_length))
        }
        "HttpWebSocketQueryCloseStatus" => {
            signature!(Generic, 5, 5; Pointer(h_web_socket), Pointer(pus_status), Pointer(pv_reason), Dword(dw_reason_length), Pointer(pdw_reason_length_consumed))
        }
        "InternetFreeCookies" => signature!(Void, 2, 2; Pointer(p_cookies), Dword(dw_cookie_count)),
        "InternetFreeProxyInfoList" => signature!(Void, 1, 1; Pointer(p_proxy_info_list)),
        "InternetSetStatusCallback"
        | "InternetSetStatusCallbackA"
        | "InternetSetStatusCallbackW" => {
            signature!(Generic, 2, 2; Pointer(h_internet), Pointer(lpfn_internet_callback))
        }
        "InternetUnlockRequestFile" => signature!(Generic, 1, 1; Pointer(h_lock_request_info)),
        "UrlCacheCloseEntryHandle" => signature!(Void, 1, 1; Pointer(h_entry_file)),
        _ => None,
    }
}
