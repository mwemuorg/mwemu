use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "KillTimer" => signature!(Win32Bool, 2, 2; Pointer(h_wnd), Pointer(u_id_event)),
        "ActivateKeyboardLayout" => signature!(Generic, 2, 2; Pointer(hkl), Dword(flags)),
        "AddClipboardFormatListener"
        | "DeregisterShellHookWindow"
        | "EnableNonClientDpiScaling"
        | "GetDpiForWindow"
        | "GetListBoxInfo"
        | "GetWindowContextHelpId"
        | "GetWindowDpiAwarenessContext"
        | "GetWindowDpiHostingBehavior"
        | "IsHungAppWindow"
        | "IsWindowArranged"
        | "RegisterShellHookWindow"
        | "RemoveClipboardFormatListener"
        | "UnregisterTouchWindow"
        | "WINNLSGetEnableStatus"
        | "WINNLSGetIMEHotkey" => signature!(Generic, 1, 1; Pointer(hwnd)),
        "AdjustWindowRectExForDpi" => {
            signature!(Win32Bool, 5, 5; Pointer(lp_rect), Dword(dw_style), Dword(b_menu), Dword(dw_ex_style), Dword(dpi))
        }
        "AllowSetForegroundWindow" => signature!(Generic, 1, 1; Dword(dw_process_id)),
        "AnimateWindow" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Dword(dw_time), Dword(dw_flags))
        }
        "AnyPopup"
        | "CancelShutdown"
        | "CountClipboardFormats"
        | "CreateMenu"
        | "CreatePopupMenu"
        | "EmptyClipboard"
        | "EndMenu"
        | "GetActiveWindow"
        | "GetCapture"
        | "GetCaretBlinkTime"
        | "GetClipboardOwner"
        | "GetClipboardSequenceNumber"
        | "GetClipboardViewer"
        | "GetCursor"
        | "GetDialogBaseUnits"
        | "GetDoubleClickTime"
        | "GetDpiForSystem"
        | "GetInputState"
        | "GetKBCodePage"
        | "GetMenuCheckMarkDimensions"
        | "GetMessageExtraInfo"
        | "GetMessagePos"
        | "GetMessageTime"
        | "GetOpenClipboardWindow"
        | "GetShellWindow"
        | "GetThreadDpiAwarenessContext"
        | "GetThreadDpiHostingBehavior"
        | "GetUnpredictedMessagePos"
        | "InSendMessage"
        | "IsMouseInPointerEnabled"
        | "IsProcessDPIAware"
        | "IsWow64Message"
        | "LockWorkStation"
        | "SetProcessDPIAware"
        | "SoundSentry"
        | "WaitMessage" => signature!(Generic, 0, 0; ),
        "AppendMenuA" => {
            signature!(Generic, 4, 4; Pointer(h_menu), Dword(u_flags), Pointer(u_id_new_item), AnsiString(lp_new_item))
        }
        "AppendMenuW" => {
            signature!(Generic, 4, 4; Pointer(h_menu), Dword(u_flags), Pointer(u_id_new_item), WideString(lp_new_item))
        }
        "AreDpiAwarenessContextsEqual" => {
            signature!(Generic, 2, 2; Pointer(dpi_context_a), Pointer(dpi_context_b))
        }
        "ArrangeIconicWindows"
        | "BringWindowToTop"
        | "DrawMenuBar"
        | "GetDialogControlDpiChangeBehavior"
        | "GetDlgCtrlID"
        | "GetLastActivePopup"
        | "GetMenu"
        | "GetParent"
        | "GetTopWindow"
        | "GetWindowTextLengthA"
        | "GetWindowTextLengthW"
        | "HideCaret"
        | "IsIconic"
        | "IsWindowEnabled"
        | "IsWindowUnicode"
        | "IsZoomed"
        | "OpenIcon"
        | "SetActiveWindow"
        | "SetCapture"
        | "ShowCaret"
        | "ShutdownBlockReasonDestroy" => signature!(Generic, 1, 1; Pointer(h_wnd)),
        "AttachThreadInput" => {
            signature!(Generic, 3, 3; Dword(id_attach), Dword(id_attach_to), Dword(f_attach))
        }
        "BeginDeferWindowPos" => signature!(Generic, 1, 1; Dword(n_num_windows)),
        "BlockInput" => signature!(Generic, 1, 1; Dword(f_block_it)),
        "BroadcastSystemMessageA" | "BroadcastSystemMessageW" => {
            signature!(Generic, 5, 5; Dword(flags), Pointer(lp_info), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "BroadcastSystemMessageExA" | "BroadcastSystemMessageExW" => {
            signature!(Generic, 6, 6; Dword(flags), Pointer(lp_info), Dword(msg), Pointer(w_param), Pointer(l_param), Pointer(pbsm_info))
        }
        "CalculatePopupWindowPosition" => {
            signature!(Generic, 5, 5; Pointer(anchor_point), Pointer(window_size), Dword(flags), Pointer(exclude_rect), Pointer(popup_window_position))
        }
        "CallMsgFilterA" | "CallMsgFilterW" => {
            signature!(Generic, 2, 2; Pointer(lp_msg), Dword(n_code))
        }
        "CallNextHookEx" => {
            signature!(Generic, 4, 4; Pointer(hhk), Dword(n_code), Pointer(w_param), Pointer(l_param))
        }
        "CallWindowProcA" | "CallWindowProcW" => {
            signature!(Generic, 5, 5; Pointer(lp_prev_wnd_func), Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "CascadeWindows" | "TileWindows" => {
            signature!(Generic, 5, 5; Pointer(hwnd_parent), Dword(w_how), Pointer(lp_rect), Dword(c_kids), Pointer(lp_kids))
        }
        "ChangeClipboardChain" => {
            signature!(Generic, 2, 2; Pointer(h_wnd_remove), Pointer(h_wnd_new_next))
        }
        "ChangeDisplaySettingsA" | "ChangeDisplaySettingsW" => {
            signature!(Generic, 2, 2; Pointer(lp_dev_mode), Dword(dw_flags))
        }
        "ChangeDisplaySettingsExA" => {
            signature!(Generic, 5, 5; AnsiString(lpsz_device_name), Pointer(lp_dev_mode), Pointer(hwnd), Dword(dwflags), Pointer(l_param))
        }
        "ChangeDisplaySettingsExW" => {
            signature!(Generic, 5, 5; WideString(lpsz_device_name), Pointer(lp_dev_mode), Pointer(hwnd), Dword(dwflags), Pointer(l_param))
        }
        "ChangeMenuA" => {
            signature!(Generic, 5, 5; Pointer(h_menu), Dword(cmd), AnsiString(lpsz_new_item), Dword(cmd_insert), Dword(flags))
        }
        "ChangeMenuW" => {
            signature!(Generic, 5, 5; Pointer(h_menu), Dword(cmd), WideString(lpsz_new_item), Dword(cmd_insert), Dword(flags))
        }
        "ChangeWindowMessageFilter" => signature!(Generic, 2, 2; Dword(message), Dword(dw_flag)),
        "ChangeWindowMessageFilterEx" => {
            signature!(Generic, 4, 4; Pointer(hwnd), Dword(message), Dword(action), Pointer(p_change_filter_struct))
        }
        "CharLowerA" | "CharUpperA" | "CharUpperW" => signature!(Generic, 1, 1; Pointer(lpsz)),
        "CharToOemA" | "OemToCharA" | "OemToCharW" => {
            signature!(Generic, 2, 2; AnsiString(p_src), Pointer(p_dst))
        }
        "CharToOemBuffA" | "OemToCharBuffA" | "OemToCharBuffW" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_src), Pointer(lpsz_dst), Dword(cch_dst_length))
        }
        "CharToOemBuffW" => {
            signature!(Generic, 3, 3; WideString(lpsz_src), Pointer(lpsz_dst), Dword(cch_dst_length))
        }
        "CharToOemW" => signature!(Generic, 2, 2; WideString(p_src), Pointer(p_dst)),
        "CheckDlgButton" => {
            signature!(Generic, 3, 3; Pointer(h_dlg), Dword(n_id_button), Dword(u_check))
        }
        "CheckMenuItem" => {
            signature!(Generic, 3, 3; Pointer(h_menu), Dword(u_id_check_item), Dword(u_check))
        }
        "CheckMenuRadioItem" => {
            signature!(Generic, 5, 5; Pointer(hmenu), Dword(first), Dword(last), Dword(check), Dword(flags))
        }
        "CheckRadioButton" => {
            signature!(Generic, 4, 4; Pointer(h_dlg), Dword(n_id_first_button), Dword(n_id_last_button), Dword(n_id_check_button))
        }
        "ChildWindowFromPoint" | "DragDetect" | "MonitorFromPoint" | "RealChildWindowFromPoint" => {
            signature!(Generic, 2, 3)
        }
        "ChildWindowFromPointEx" | "MenuItemFromPoint" => signature!(Generic, 3, 4),
        "ClientToScreen"
        | "LogicalToPhysicalPoint"
        | "LogicalToPhysicalPointForPerMonitorDPI"
        | "PhysicalToLogicalPoint"
        | "PhysicalToLogicalPointForPerMonitorDPI"
        | "ScreenToClient" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lp_point)),
        "ClipCursor" | "GetClipCursor" => signature!(Generic, 1, 1; Pointer(lp_rect)),
        "ConsoleControl" => {
            signature!(NtStatus, 3, 3; Dword(command), Pointer(consoleinformation), Dword(consoleinformationlength))
        }
        "CopyAcceleratorTableA" | "CopyAcceleratorTableW" => {
            signature!(Generic, 3, 3; Pointer(h_accel_src), Pointer(lp_accel_dst), Dword(c_accel_entries))
        }
        "CopyIcon" => signature!(Generic, 1, 1; Pointer(h_icon)),
        "CopyImage" => {
            signature!(Generic, 5, 5; Pointer(h), Dword(type), Dword(cx), Dword(cy), Dword(flags))
        }
        "CreateAcceleratorTableA" | "CreateAcceleratorTableW" => {
            signature!(Generic, 2, 2; Pointer(paccel), Dword(c_accel))
        }
        "CreateCaret" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Pointer(h_bitmap), Dword(n_width), Dword(n_height))
        }
        "CreateCursor" => {
            signature!(Generic, 7, 7; Pointer(h_inst), Dword(x_hot_spot), Dword(y_hot_spot), Dword(n_width), Dword(n_height), Pointer(pv_and_plane), Pointer(pv_xor_plane))
        }
        "CreateDesktopA" => {
            signature!(Generic, 6, 6; AnsiString(lpsz_desktop), Pointer(lpsz_device), Pointer(p_devmode), Dword(dw_flags), Dword(dw_desired_access), Pointer(lpsa))
        }
        "CreateDesktopExA" => {
            signature!(Generic, 8, 8; AnsiString(lpsz_desktop), Pointer(lpsz_device), Pointer(p_devmode), Dword(dw_flags), Dword(dw_desired_access), Pointer(lpsa), Dword(ul_heap_size), Pointer(pvoid))
        }
        "CreateDesktopExW" => {
            signature!(Generic, 8, 8; WideString(lpsz_desktop), Pointer(lpsz_device), Pointer(p_devmode), Dword(dw_flags), Dword(dw_desired_access), Pointer(lpsa), Dword(ul_heap_size), Pointer(pvoid))
        }
        "CreateDesktopW" => {
            signature!(Generic, 6, 6; WideString(lpsz_desktop), Pointer(lpsz_device), Pointer(p_devmode), Dword(dw_flags), Dword(dw_desired_access), Pointer(lpsa))
        }
        "CreateDialogIndirectParamA" | "CreateDialogIndirectParamW" => {
            signature!(Generic, 5, 5; Pointer(h_instance), Pointer(lp_template), Pointer(h_wnd_parent), Pointer(lp_dialog_func), Pointer(dw_init_param))
        }
        "CreateDialogParamA" | "DialogBoxParamA" => {
            signature!(Generic, 5, 5; Pointer(h_instance), AnsiString(lp_template_name), Pointer(h_wnd_parent), Pointer(lp_dialog_func), Pointer(dw_init_param))
        }
        "CreateDialogParamW" | "DialogBoxParamW" => {
            signature!(Generic, 5, 5; Pointer(h_instance), WideString(lp_template_name), Pointer(h_wnd_parent), Pointer(lp_dialog_func), Pointer(dw_init_param))
        }
        "CreateIcon" => {
            signature!(Generic, 7, 7; Pointer(h_instance), Dword(n_width), Dword(n_height), Byte(c_planes), Byte(c_bits_pixel), Pointer(lpb_an_dbits), Pointer(lpb_xo_rbits))
        }
        "CreateIconFromResource" => {
            signature!(Generic, 4, 4; Pointer(presbits), Dword(dw_res_size), Dword(f_icon), Dword(dw_ver))
        }
        "CreateIconFromResourceEx" => {
            signature!(Generic, 7, 7; Pointer(presbits), Dword(dw_res_size), Dword(f_icon), Dword(dw_ver), Dword(cx_desired), Dword(cy_desired), Dword(flags))
        }
        "CreateIconIndirect" => signature!(Generic, 1, 1; Pointer(piconinfo)),
        "CreateMDIWindowA" => {
            signature!(Generic, 10, 10; AnsiString(lp_class_name), AnsiString(lp_window_name), Dword(dw_style), Dword(x), Dword(y), Dword(n_width), Dword(n_height), Pointer(h_wnd_parent), Pointer(h_instance), Pointer(l_param))
        }
        "CreateMDIWindowW" => {
            signature!(Generic, 10, 10; WideString(lp_class_name), WideString(lp_window_name), Dword(dw_style), Dword(x), Dword(y), Dword(n_width), Dword(n_height), Pointer(h_wnd_parent), Pointer(h_instance), Pointer(l_param))
        }
        "CreateSyntheticPointerDevice" => {
            signature!(Generic, 3, 3; Dword(pointer_type), Dword(max_count), Dword(mode))
        }
        "CreateWindowStationA" => {
            signature!(Generic, 4, 4; AnsiString(lpwinsta), Dword(dw_flags), Dword(dw_desired_access), Pointer(lpsa))
        }
        "CreateWindowStationW" => {
            signature!(Generic, 4, 4; WideString(lpwinsta), Dword(dw_flags), Dword(dw_desired_access), Pointer(lpsa))
        }
        "DdeAbandonTransaction" => {
            signature!(Generic, 3, 3; Dword(id_inst), Pointer(h_conv), Dword(id_transaction))
        }
        "DdeAccessData" => signature!(Generic, 2, 2; Pointer(h_data), Pointer(pcb_data_size)),
        "DdeAddData" => {
            signature!(Generic, 4, 4; Pointer(h_data), Pointer(p_src), Dword(cb), Dword(cb_off))
        }
        "DdeClientTransaction" => {
            signature!(Generic, 8, 8; Pointer(p_data), Dword(cb_data), Pointer(h_conv), Pointer(hsz_item), Dword(w_fmt), Dword(w_type), Dword(dw_timeout), Pointer(pdw_result))
        }
        "DdeCmpStringHandles" => signature!(Generic, 2, 2; Pointer(hsz1), Pointer(hsz2)),
        "DdeConnect" => {
            signature!(Generic, 4, 4; Dword(id_inst), Pointer(hsz_service), Pointer(hsz_topic), Pointer(p_cc))
        }
        "DdeConnectList" => {
            signature!(Generic, 5, 5; Dword(id_inst), Pointer(hsz_service), Pointer(hsz_topic), Pointer(h_conv_list), Pointer(p_cc))
        }
        "DdeCreateDataHandle" => {
            signature!(Generic, 7, 7; Dword(id_inst), Pointer(p_src), Dword(cb), Dword(cb_off), Pointer(hsz_item), Dword(w_fmt), Dword(af_cmd))
        }
        "DdeCreateStringHandleA" => {
            signature!(Generic, 3, 3; Dword(id_inst), AnsiString(psz), Dword(i_code_page))
        }
        "DdeCreateStringHandleW" => {
            signature!(Generic, 3, 3; Dword(id_inst), WideString(psz), Dword(i_code_page))
        }
        "DdeDisconnect" | "DdeImpersonateClient" | "DdeReconnect" => {
            signature!(Generic, 1, 1; Pointer(h_conv))
        }
        "DdeDisconnectList" => signature!(Generic, 1, 1; Pointer(h_conv_list)),
        "DdeEnableCallback" => {
            signature!(Generic, 3, 3; Dword(id_inst), Pointer(h_conv), Dword(w_cmd))
        }
        "DdeFreeDataHandle" | "DdeUnaccessData" => signature!(Generic, 1, 1; Pointer(h_data)),
        "DdeFreeStringHandle" | "DdeKeepStringHandle" => {
            signature!(Generic, 2, 2; Dword(id_inst), Pointer(hsz))
        }
        "DdeGetData" => {
            signature!(Generic, 4, 4; Pointer(h_data), Pointer(p_dst), Dword(cb_max), Dword(cb_off))
        }
        "DdeGetLastError" | "DdeUninitialize" => signature!(Generic, 1, 1; Dword(id_inst)),
        "DdeInitializeA" | "DdeInitializeW" => {
            signature!(Generic, 4, 4; Pointer(pid_inst), Pointer(pfn_callback), Dword(af_cmd), Dword(ul_res))
        }
        "DdeNameService" => {
            signature!(Generic, 4, 4; Dword(id_inst), Pointer(hsz1), Pointer(hsz2), Dword(af_cmd))
        }
        "DdePostAdvise" => {
            signature!(Generic, 3, 3; Dword(id_inst), Pointer(hsz_topic), Pointer(hsz_item))
        }
        "DdeQueryConvInfo" => {
            signature!(Generic, 3, 3; Pointer(h_conv), Dword(id_transaction), Pointer(p_conv_info))
        }
        "DdeQueryNextServer" => {
            signature!(Generic, 2, 2; Pointer(h_conv_list), Pointer(h_conv_prev))
        }
        "DdeQueryStringA" | "DdeQueryStringW" => {
            signature!(Generic, 5, 5; Dword(id_inst), Pointer(hsz), Pointer(psz), Dword(cch_max), Dword(i_code_page))
        }
        "DdeSetQualityOfService" => {
            signature!(Generic, 3, 3; Pointer(hwnd_client), Pointer(pqos_new), Pointer(pqos_prev))
        }
        "DdeSetUserHandle" => {
            signature!(Generic, 3, 3; Pointer(h_conv), Dword(id), Pointer(h_user))
        }
        "DefDlgProcA" | "DefDlgProcW" => {
            signature!(Generic, 4, 4; Pointer(h_dlg), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "DefFrameProcA" | "DefFrameProcW" => {
            signature!(Generic, 5, 5; Pointer(h_wnd), Pointer(h_wnd_mdi_client), Dword(u_msg), Pointer(w_param), Pointer(l_param))
        }
        "DefMDIChildProcA" | "DefMDIChildProcW" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(u_msg), Pointer(w_param), Pointer(l_param))
        }
        "DefRawInputProc" => {
            signature!(Generic, 3, 3; Pointer(pa_raw_input), Dword(n_input), Dword(cb_size_header))
        }
        "DeferWindowPos" => {
            signature!(Generic, 8, 8; Pointer(h_win_pos_info), Pointer(h_wnd), Pointer(h_wnd_insert_after), Dword(x), Dword(y), Dword(cx), Dword(cy), Dword(u_flags))
        }
        "DialogBoxIndirectParamA" | "DialogBoxIndirectParamW" => {
            signature!(Generic, 5, 5; Pointer(h_instance), Pointer(h_dialog_template), Pointer(h_wnd_parent), Pointer(lp_dialog_func), Pointer(dw_init_param))
        }
        "DisableProcessWindowsGhosting" => signature!(Void, 0, 0; ),
        "DisplayConfigGetDeviceInfo" => signature!(Generic, 1, 1; Pointer(request_packet)),
        "DisplayConfigSetDeviceInfo" => signature!(Generic, 1, 1; Pointer(set_packet)),
        "DlgDirListA" | "DlgDirListW" => {
            signature!(Generic, 5, 5; Pointer(h_dlg), Pointer(lp_path_spec), Dword(n_id_list_box), Dword(n_id_static_path), Dword(u_file_type))
        }
        "DlgDirListComboBoxA" | "DlgDirListComboBoxW" => {
            signature!(Generic, 5, 5; Pointer(h_dlg), Pointer(lp_path_spec), Dword(n_id_combo_box), Dword(n_id_static_path), Dword(u_filetype))
        }
        "DlgDirSelectComboBoxExA" | "DlgDirSelectComboBoxExW" => {
            signature!(Generic, 4, 4; Pointer(hwnd_dlg), Pointer(lp_string), Dword(cch_out), Dword(id_combo_box))
        }
        "DlgDirSelectExA" | "DlgDirSelectExW" => {
            signature!(Generic, 4, 4; Pointer(hwnd_dlg), Pointer(lp_string), Dword(ch_count), Dword(id_list_box))
        }
        "DragObject" => {
            signature!(Generic, 5, 5; Pointer(hwnd_parent), Pointer(hwnd_from), Dword(fmt), Pointer(data), Pointer(hcur))
        }
        "DrawAnimatedRects" => {
            signature!(Generic, 4, 4; Pointer(hwnd), Dword(id_ani), Pointer(lprc_from), Pointer(lprc_to))
        }
        "DrawCaption" => {
            signature!(Generic, 4, 4; Pointer(hwnd), Pointer(hdc), Pointer(lprect), Dword(flags))
        }
        "DrawEdge" => {
            signature!(Generic, 4, 4; Pointer(hdc), Pointer(qrc), Dword(edge), Dword(grf_flags))
        }
        "DrawFocusRect" | "InvertRect" => signature!(Generic, 2, 2; Pointer(h_dc), Pointer(lprc)),
        "DrawFrameControl" => {
            signature!(Generic, 4, 4; Pointer(hdc), Pointer(lprc), Dword(u_type), Dword(u_state))
        }
        "DrawIcon" => signature!(Generic, 4, 4; Pointer(h_dc), Dword(x), Dword(y), Pointer(h_icon)),
        "DrawIconEx" => {
            signature!(Generic, 9, 9; Pointer(hdc), Dword(x_left), Dword(y_top), Pointer(h_icon), Dword(cx_width), Dword(cy_width), Dword(istep_if_ani_cur), Pointer(hbr_flicker_free_draw), Dword(di_flags))
        }
        "DrawStateA" | "DrawStateW" => {
            signature!(Generic, 10, 10; Pointer(hdc), Pointer(hbr_fore), Pointer(qfn_call_back), Pointer(l_data), Pointer(w_data), Dword(x), Dword(y), Dword(cx), Dword(cy), Dword(u_flags))
        }
        "DrawTextExA" | "DrawTextExW" => {
            signature!(Generic, 6, 6; Pointer(hdc), Pointer(lpch_text), Dword(cch_text), Pointer(lprc), Dword(format), Pointer(lpdtp))
        }
        "EnableMenuItem" => {
            signature!(Generic, 3, 3; Pointer(h_menu), Dword(u_id_enable_item), Dword(u_enable))
        }
        "EnableMouseInPointer" => signature!(Generic, 1, 1; Dword(f_enable)),
        "EnableScrollBar" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Dword(w_s_bflags), Dword(w_arrows))
        }
        "EndDeferWindowPos" => signature!(Generic, 1, 1; Pointer(h_win_pos_info)),
        "EndDialog" => signature!(Generic, 2, 2; Pointer(h_dlg), Pointer(n_result)),
        "EnumChildWindows" => {
            signature!(Generic, 3, 3; Pointer(h_wnd_parent), Pointer(lp_enum_func), Pointer(l_param))
        }
        "EnumClipboardFormats" | "IsClipboardFormatAvailable" => {
            signature!(Generic, 1, 1; Dword(format))
        }
        "EnumDesktopWindows" => {
            signature!(Generic, 3, 3; Pointer(h_desktop), Pointer(lpfn), Pointer(l_param))
        }
        "EnumDesktopsA" | "EnumDesktopsW" => {
            signature!(Generic, 3, 3; Pointer(hwinsta), Pointer(lp_enum_func), Pointer(l_param))
        }
        "EnumDisplayDevicesA" => {
            signature!(Generic, 4, 4; AnsiString(lp_device), Dword(i_dev_num), Pointer(lp_display_device), Dword(dw_flags))
        }
        "EnumDisplayDevicesW" => {
            signature!(Generic, 4, 4; WideString(lp_device), Dword(i_dev_num), Pointer(lp_display_device), Dword(dw_flags))
        }
        "EnumDisplayMonitors" => {
            signature!(Generic, 4, 4; Pointer(hdc), Pointer(lprc_clip), Pointer(lpfn_enum), Pointer(dw_data))
        }
        "EnumDisplaySettingsA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_device_name), Dword(i_mode_num), Pointer(lp_dev_mode))
        }
        "EnumDisplaySettingsExA" => {
            signature!(Generic, 4, 4; AnsiString(lpsz_device_name), Dword(i_mode_num), Pointer(lp_dev_mode), Dword(dw_flags))
        }
        "EnumDisplaySettingsExW" => {
            signature!(Generic, 4, 4; WideString(lpsz_device_name), Dword(i_mode_num), Pointer(lp_dev_mode), Dword(dw_flags))
        }
        "EnumDisplaySettingsW" => {
            signature!(Generic, 3, 3; WideString(lpsz_device_name), Dword(i_mode_num), Pointer(lp_dev_mode))
        }
        "EnumPropsA" | "EnumPropsW" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lp_enum_func))
        }
        "EnumPropsExA" | "EnumPropsExW" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(lp_enum_func), Pointer(l_param))
        }
        "EnumThreadWindows" => {
            signature!(Generic, 3, 3; Dword(dw_thread_id), Pointer(lpfn), Pointer(l_param))
        }
        "EnumWindowStationsA" | "EnumWindowStationsW" | "EnumWindows" => {
            signature!(Generic, 2, 2; Pointer(lp_enum_func), Pointer(l_param))
        }
        "EvaluateProximityToPolygon" => {
            signature!(Generic, 4, 4; Dword(num_vertices), Pointer(control_polygon), Pointer(p_hit_testing_input), Pointer(p_proximity_eval))
        }
        "EvaluateProximityToRect" => {
            signature!(Generic, 3, 3; Pointer(control_bounding_box), Pointer(p_hit_testing_input), Pointer(p_proximity_eval))
        }
        "ExcludeUpdateRgn" => signature!(Generic, 2, 2; Pointer(h_dc), Pointer(h_wnd)),
        "ExitWindowsEx" => signature!(Generic, 2, 2; Dword(u_flags), Dword(dw_reason)),
        "FindWindowExA" => {
            signature!(Generic, 4, 4; Pointer(h_wnd_parent), Pointer(h_wnd_child_after), AnsiString(lpsz_class), AnsiString(lpsz_window))
        }
        "FindWindowExW" => {
            signature!(Generic, 4, 4; Pointer(h_wnd_parent), Pointer(h_wnd_child_after), WideString(lpsz_class), WideString(lpsz_window))
        }
        "FlashWindow" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(b_invert)),
        "FlashWindowEx" => signature!(Generic, 1, 1; Pointer(pfwi)),
        "FrameRect" => signature!(Generic, 3, 3; Pointer(h_dc), Pointer(lprc), Pointer(hbr)),
        "GetAltTabInfoA" | "GetAltTabInfoW" => {
            signature!(Generic, 5, 5; Pointer(hwnd), Dword(i_item), Pointer(pati), Pointer(psz_item_text), Dword(cch_item_text))
        }
        "GetAncestor" => signature!(Generic, 2, 2; Pointer(hwnd), Dword(ga_flags)),
        "GetAutoRotationState" => signature!(Generic, 1, 1; Pointer(p_state)),
        "GetAwarenessFromDpiAwarenessContext"
        | "GetDpiFromDpiAwarenessContext"
        | "IsValidDpiAwarenessContext"
        | "SetProcessDpiAwarenessContext" => signature!(Generic, 1, 1; Pointer(value)),
        "GetCIMSSM" | "GetCurrentInputMessageSource" => {
            signature!(Generic, 1, 1; Pointer(input_message_source))
        }
        "GetCaretPos" | "GetPhysicalCursorPos" => signature!(Generic, 1, 1; Pointer(lp_point)),
        "GetClassInfoA" => {
            signature!(Generic, 3, 3; Pointer(h_instance), AnsiString(lp_class_name), Pointer(lp_wnd_class))
        }
        "GetClassInfoW" => {
            signature!(Generic, 3, 3; Pointer(h_instance), WideString(lp_class_name), Pointer(lp_wnd_class))
        }
        "GetClassLongA" | "GetClassLongPtrA" | "GetClassLongPtrW" | "GetClassLongW"
        | "GetClassWord" | "GetWindowLongPtrA" | "GetWindowLongPtrW" | "GetWindowWord" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), Dword(n_index))
        }
        "GetClassNameA" | "GetClassNameW" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(lp_class_name), Dword(n_max_count))
        }
        "GetClipboardData" => signature!(Generic, 1, 1; Dword(u_format)),
        "GetClipboardFormatNameA" | "GetClipboardFormatNameW" => {
            signature!(Generic, 3, 3; Dword(format), Pointer(lpsz_format_name), Dword(cch_max_count))
        }
        "GetComboBoxInfo" => signature!(Generic, 2, 2; Pointer(hwnd_combo), Pointer(pcbi)),
        "GetCursorInfo" => signature!(Generic, 1, 1; Pointer(pci)),
        "GetDialogDpiChangeBehavior" => signature!(Generic, 1, 1; Pointer(h_dlg)),
        "GetDisplayAutoRotationPreferences" => signature!(Generic, 1, 1; Pointer(p_orientation)),
        "GetDisplayConfigBufferSizes" => {
            signature!(Win32Status, 3, 3; Dword(flags), Pointer(num_path_array_elements), Pointer(num_mode_info_array_elements))
        }
        "GetDlgItem" => signature!(Generic, 2, 2; Pointer(h_dlg), Dword(n_id_dlg_item)),
        "GetDlgItemInt" => {
            signature!(Generic, 4, 4; Pointer(h_dlg), Dword(n_id_dlg_item), Pointer(lp_translated), Dword(b_signed))
        }
        "GetDlgItemTextA" | "GetDlgItemTextW" => {
            signature!(Generic, 4, 4; Pointer(h_dlg), Dword(n_id_dlg_item), Pointer(lp_string), Dword(cch_max))
        }
        "GetDpiAwarenessContextForProcess" | "GetSystemDpiForProcess" | "IsImmersiveProcess" => {
            signature!(Generic, 1, 1; Pointer(h_process))
        }
        "GetGuiResources" => signature!(Generic, 2, 2; Pointer(h_process), Dword(ui_flags)),
        "GetIconInfoExA" | "GetIconInfoExW" => {
            signature!(Generic, 2, 2; Pointer(hicon), Pointer(piconinfo))
        }
        "GetKeyNameTextA" | "GetKeyNameTextW" => {
            signature!(Generic, 3, 3; Dword(l_param), Pointer(lp_string), Dword(cch_size))
        }
        "GetKeyboardLayoutList" => signature!(Generic, 2, 2; Dword(n_buff), Pointer(lp_list)),
        "GetKeyboardLayoutNameA" | "GetKeyboardLayoutNameW" => {
            signature!(Generic, 1, 1; Pointer(pwsz_klid))
        }
        "GetKeyboardState" | "SetKeyboardState" => signature!(Generic, 1, 1; Pointer(lp_key_state)),
        "GetLastInputInfo" => signature!(Generic, 1, 1; Pointer(plii)),
        "GetMenuContextHelpId" => signature!(Generic, 1, 1; Pointer(hmenu)),
        "GetMenuDefaultItem" => {
            signature!(Generic, 3, 3; Pointer(h_menu), Dword(f_by_pos), Dword(gmdi_flags))
        }
        "GetMenuItemCount" | "IsMenu" => signature!(Generic, 1, 1; Pointer(h_menu)),
        "GetMenuItemID" | "GetSubMenu" => signature!(Generic, 2, 2; Pointer(h_menu), Dword(n_pos)),
        "GetMenuItemInfoA" | "GetMenuItemInfoW" => {
            signature!(Generic, 4, 4; Pointer(hmenu), Dword(item), Dword(f_by_position), Pointer(lpmii))
        }
        "GetMenuState" => signature!(Generic, 3, 3; Pointer(h_menu), Dword(u_id), Dword(u_flags)),
        "GetMenuStringA" | "GetMenuStringW" => {
            signature!(Generic, 5, 5; Pointer(h_menu), Dword(u_id_item), Pointer(lp_string), Dword(cch_max), Dword(flags))
        }
        "GetMonitorInfoA" | "GetMonitorInfoW" => {
            signature!(Generic, 2, 2; Pointer(h_monitor), Pointer(lpmi))
        }
        "GetMouseMovePointsEx" => {
            signature!(Generic, 5, 5; Dword(cb_size), Pointer(lppt), Pointer(lppt_buf), Dword(n_buf_points), Dword(resolution))
        }
        "GetNextDlgGroupItem" | "GetNextDlgTabItem" => {
            signature!(Generic, 3, 3; Pointer(h_dlg), Pointer(h_ctl), Dword(b_previous))
        }
        "GetPriorityClipboardFormat" => {
            signature!(Generic, 2, 2; Pointer(pa_format_priority_list), Dword(c_formats))
        }
        "GetProcessDefaultLayout" => signature!(Generic, 1, 1; Pointer(pdw_default_layout)),
        "GetPropA" | "RemovePropA" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), AnsiString(lp_string))
        }
        "GetPropW" | "RemovePropW" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), WideString(lp_string))
        }
        "GetQueueStatus" => signature!(Generic, 1, 1; Dword(flags)),
        "GetRawInputBuffer" => {
            signature!(Generic, 3, 3; Pointer(p_data), Pointer(pcb_size), Dword(cb_size_header))
        }
        "GetRawInputData" => {
            signature!(Generic, 5, 5; Pointer(h_raw_input), Dword(ui_command), Pointer(p_data), Pointer(pcb_size), Dword(cb_size_header))
        }
        "GetRawInputDeviceInfoA" | "GetRawInputDeviceInfoW" => {
            signature!(Generic, 4, 4; Pointer(h_device), Dword(ui_command), Pointer(p_data), Pointer(pcb_size))
        }
        "GetRawInputDeviceList" => {
            signature!(Generic, 3, 3; Pointer(p_raw_input_device_list), Pointer(pui_num_devices), Dword(cb_size))
        }
        "GetRegisteredRawInputDevices" => {
            signature!(Generic, 3, 3; Pointer(p_raw_input_devices), Pointer(pui_num_devices), Dword(cb_size))
        }
        "GetScrollPos" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(n_bar)),
        "GetScrollRange" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(n_bar), Pointer(lp_min_pos), Pointer(lp_max_pos))
        }
        "GetSysColor" | "GetSysColorBrush" => signature!(Generic, 1, 1; Dword(n_index)),
        "GetSystemMenu" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(b_revert)),
        "GetSystemMetricsForDpi" => signature!(Generic, 2, 2; Dword(n_index), Dword(dpi)),
        "GetTabbedTextExtentA" | "GetTabbedTextExtentW" => {
            signature!(Generic, 5, 5; Pointer(hdc), Pointer(lp_string), Dword(ch_count), Dword(n_tab_positions), Pointer(lpn_tab_stop_positions))
        }
        "GetThreadDesktop" => signature!(Generic, 1, 1; Dword(dw_thread_id)),
        "GetUpdateRect" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(lp_rect), Dword(b_erase))
        }
        "GetUpdateRgn" | "InvalidateRgn" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(h_rgn), Dword(b_erase))
        }
        "GetUserObjectInformationA" => {
            signature!(Generic, 5, 5; Pointer(h_obj), Dword(n_index), Pointer(pv_info), Dword(n_length), Pointer(lpn_length_needed))
        }
        "GetWindow" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(u_cmd)),
        "GetWindowDisplayAffinity" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(pdw_affinity))
        }
        "GetWindowModuleFileNameA" | "GetWindowModuleFileNameW" => {
            signature!(Generic, 3, 3; Pointer(hwnd), Pointer(psz_file_name), Dword(cch_file_name_max))
        }
        "GetWindowPlacement" | "SetWindowPlacement" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lpwndpl))
        }
        "GetWindowRgn" | "ValidateRgn" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(h_rgn)),
        "GetWindowRgnBox" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lprc)),
        "GetWindowThreadProcessId" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lpdw_process_id))
        }
        "GrayStringA" | "GrayStringW" => {
            signature!(Generic, 9, 9; Pointer(h_dc), Pointer(h_brush), Pointer(lp_output_func), Pointer(lp_data), Dword(n_count), Dword(x), Dword(y), Dword(n_width), Dword(n_height))
        }
        "HiliteMenuItem" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Pointer(h_menu), Dword(u_id_hilite_item), Dword(u_hilite))
        }
        "IMPGetIMEA" | "IMPSetIMEA" => signature!(Generic, 2, 2; Pointer(hwnd), Pointer(lpimeproa)),
        "IMPGetIMEW" | "IMPSetIMEW" => signature!(Generic, 2, 2; Pointer(hwnd), Pointer(lpimeprow)),
        "IMPQueryIMEA" => signature!(Generic, 1, 1; Pointer(lpimeproa)),
        "IMPQueryIMEW" => signature!(Generic, 1, 1; Pointer(lpimeprow)),
        "ImpersonateDdeClientWindow" => {
            signature!(Generic, 2, 2; Pointer(h_wnd_client), Pointer(h_wnd_server))
        }
        "InSendMessageEx" => signature!(Generic, 1, 1; Pointer(lp_reserved)),
        "InheritWindowMonitor" => signature!(Generic, 2, 2; Pointer(hwnd), Pointer(hwnd_inherit)),
        "InitializeTouchInjection" => signature!(Generic, 2, 2; Dword(max_count), Dword(dw_mode)),
        "InjectSyntheticPointerInput" => {
            signature!(Generic, 3, 3; Pointer(device), Pointer(pointer_info), Dword(count))
        }
        "InjectTouchInput" => signature!(Generic, 2, 2; Dword(count), Pointer(contacts)),
        "InsertMenuA" => {
            signature!(Generic, 5, 5; Pointer(h_menu), Dword(u_position), Dword(u_flags), Pointer(u_id_new_item), AnsiString(lp_new_item))
        }
        "InsertMenuItemA" | "InsertMenuItemW" => {
            signature!(Generic, 4, 4; Pointer(hmenu), Dword(item), Dword(f_by_position), Pointer(lpmi))
        }
        "InsertMenuW" => {
            signature!(Generic, 5, 5; Pointer(h_menu), Dword(u_position), Dword(u_flags), Pointer(u_id_new_item), WideString(lp_new_item))
        }
        "InternalGetWindowText" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(p_string), Dword(cch_max_count))
        }
        "IsChild" => signature!(Generic, 2, 2; Pointer(h_wnd_parent), Pointer(h_wnd)),
        "IsDialogMessageA" | "IsDialogMessageW" => {
            signature!(Generic, 2, 2; Pointer(h_dlg), Pointer(lp_msg))
        }
        "IsDlgButtonChecked" => signature!(Generic, 2, 2; Pointer(h_dlg), Dword(n_id_button)),
        "IsGUIThread" => signature!(Generic, 1, 1; Dword(b_convert)),
        "IsTouchWindow" => signature!(Generic, 2, 2; Pointer(hwnd), Pointer(pul_flags)),
        "IsWinEventHookInstalled" => signature!(Generic, 1, 1; Dword(event)),
        "LoadKeyboardLayoutA" => signature!(Generic, 2, 2; AnsiString(pwsz_klid), Dword(flags)),
        "LoadKeyboardLayoutW" => signature!(Generic, 2, 2; WideString(pwsz_klid), Dword(flags)),
        "LoadMenuA" => signature!(Generic, 2, 2; Pointer(h_instance), AnsiString(lp_menu_name)),
        "LoadMenuIndirectA" | "LoadMenuIndirectW" => {
            signature!(Generic, 1, 1; Pointer(lp_menu_template))
        }
        "LoadMenuW" => signature!(Generic, 2, 2; Pointer(h_instance), WideString(lp_menu_name)),
        "LockSetForegroundWindow" => signature!(Generic, 1, 1; Dword(u_lock_code)),
        "LockWindowUpdate" => signature!(Generic, 1, 1; Pointer(h_wnd_lock)),
        "LookupIconIdFromDirectory" => signature!(Generic, 2, 2; Pointer(presbits), Dword(f_icon)),
        "LookupIconIdFromDirectoryEx" => {
            signature!(Generic, 5, 5; Pointer(presbits), Dword(f_icon), Dword(cx_desired), Dword(cy_desired), Dword(flags))
        }
        "MapDialogRect" => signature!(Generic, 2, 2; Pointer(h_dlg), Pointer(lp_rect)),
        "MapWindowPoints" => {
            signature!(Generic, 4, 4; Pointer(h_wnd_from), Pointer(h_wnd_to), Pointer(lp_points), Dword(c_points))
        }
        "MessageBeep" => signature!(Generic, 1, 1; Dword(u_type)),
        "MessageBoxExA" => {
            signature!(Generic, 5, 5; Pointer(h_wnd), AnsiString(lp_text), AnsiString(lp_caption), Dword(u_type), Word(w_language_id))
        }
        "MessageBoxExW" => {
            signature!(Generic, 5, 5; Pointer(h_wnd), WideString(lp_text), WideString(lp_caption), Dword(u_type), Word(w_language_id))
        }
        "MessageBoxIndirectA" | "MessageBoxIndirectW" => signature!(Generic, 1, 1; Pointer(lpmbp)),
        "ModifyMenuA" => {
            signature!(Generic, 5, 5; Pointer(h_mnu), Dword(u_position), Dword(u_flags), Pointer(u_id_new_item), AnsiString(lp_new_item))
        }
        "ModifyMenuW" => {
            signature!(Generic, 5, 5; Pointer(h_mnu), Dword(u_position), Dword(u_flags), Pointer(u_id_new_item), WideString(lp_new_item))
        }
        "MonitorFromRect" => signature!(Generic, 2, 2; Pointer(lprc), Dword(dw_flags)),
        "MonitorFromWindow" => signature!(Generic, 2, 2; Pointer(hwnd), Dword(dw_flags)),
        "MoveWindow" => {
            signature!(Generic, 6, 6; Pointer(h_wnd), Dword(x), Dword(y), Dword(n_width), Dword(n_height), Dword(b_repaint))
        }
        "MsgWaitForMultipleObjects" => {
            signature!(Generic, 5, 5; Dword(n_count), Pointer(p_handles), Dword(f_wait_all), Dword(dw_milliseconds), Dword(dw_wake_mask))
        }
        "MsgWaitForMultipleObjectsEx" => {
            signature!(Generic, 5, 5; Dword(n_count), Pointer(p_handles), Dword(dw_milliseconds), Dword(dw_wake_mask), Dword(dw_flags))
        }
        "NotifyWinEvent" => {
            signature!(Void, 4, 4; Dword(event), Pointer(hwnd), Dword(id_object), Dword(id_child))
        }
        "OemKeyScan" => signature!(Generic, 1, 1; Word(w_oem_char)),
        "OpenClipboard" => signature!(Generic, 1, 1; Pointer(h_wnd_new_owner)),
        "OpenDesktopA" => {
            signature!(Generic, 4, 4; AnsiString(lpsz_desktop), Dword(dw_flags), Dword(f_inherit), Dword(dw_desired_access))
        }
        "OpenDesktopW" => {
            signature!(Generic, 4, 4; WideString(lpsz_desktop), Dword(dw_flags), Dword(f_inherit), Dword(dw_desired_access))
        }
        "OpenInputDesktop" => {
            signature!(Generic, 3, 3; Dword(dw_flags), Dword(f_inherit), Dword(dw_desired_access))
        }
        "OpenWindowStationA" => {
            signature!(Generic, 3, 3; AnsiString(lpsz_win_sta), Dword(f_inherit), Dword(dw_desired_access))
        }
        "OpenWindowStationW" => {
            signature!(Generic, 3, 3; WideString(lpsz_win_sta), Dword(f_inherit), Dword(dw_desired_access))
        }
        "PackDDElParam" => signature!(Generic, 3, 3; Dword(msg), Pointer(ui_lo), Pointer(ui_hi)),
        "PackTouchHitTestingProximityEvaluation" => {
            signature!(Generic, 2, 2; Pointer(p_hit_testing_input), Pointer(p_proximity_eval))
        }
        "PaintDesktop" => signature!(Generic, 1, 1; Pointer(hdc)),
        "PostThreadMessageA" | "PostThreadMessageW" => {
            signature!(Generic, 4, 4; Dword(id_thread), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "PrintWindow" => signature!(Generic, 3, 3; Pointer(hwnd), Pointer(hdc_blt), Dword(n_flags)),
        "PrivateExtractIconsA" | "PrivateExtractIconsW" => {
            signature!(Generic, 8, 8; Pointer(sz_file_name), Dword(n_icon_index), Dword(cx_icon), Dword(cy_icon), Pointer(phicon), Pointer(piconid), Dword(n_icons), Dword(flags))
        }
        "QueryDisplayConfig" => {
            signature!(Win32Status, 6, 6; Dword(flags), Pointer(num_path_array_elements), Pointer(path_array), Pointer(num_mode_info_array_elements), Pointer(mode_info_array), Pointer(current_topology_id))
        }
        "RealGetWindowClassA" | "RealGetWindowClassW" => {
            signature!(Generic, 3, 3; Pointer(hwnd), Pointer(ptsz_class_name), Dword(cch_class_name_max))
        }
        "RedrawWindow" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Pointer(lprc_update), Pointer(hrgn_update), Dword(flags))
        }
        "RegisterClassExA" | "RegisterClassExW" => signature!(Generic, 1, 1; Pointer(wnd_class)),
        "RegisterClipboardFormatA" => signature!(Generic, 1, 1; AnsiString(lpsz_format)),
        "RegisterClipboardFormatW" => signature!(Generic, 1, 1; WideString(lpsz_format)),
        "RegisterDeviceNotificationA" | "RegisterDeviceNotificationW" => {
            signature!(Generic, 3, 3; Pointer(h_recipient), Pointer(notification_filter), Dword(flags))
        }
        "RegisterForTooltipDismissNotification" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), Dword(td_flags))
        }
        "RegisterHotKey" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(id), Dword(fs_modifiers), Dword(vk))
        }
        "RegisterPointerDeviceNotifications" => {
            signature!(Generic, 2, 2; Pointer(window), Dword(notify_range))
        }
        "RegisterPointerInputTarget"
        | "UnregisterPointerInputTarget"
        | "UnregisterPointerInputTargetEx" => {
            signature!(Generic, 2, 2; Pointer(hwnd), Dword(pointer_type))
        }
        "RegisterPointerInputTargetEx" => {
            signature!(Generic, 3, 3; Pointer(hwnd), Dword(pointer_type), Dword(f_observe))
        }
        "RegisterPowerSettingNotification" => {
            signature!(Generic, 3, 3; Pointer(h_recipient), Pointer(power_setting_guid), Dword(flags))
        }
        "RegisterRawInputDevices" => {
            signature!(Generic, 3, 3; Pointer(p_raw_input_devices), Dword(ui_num_devices), Dword(cb_size))
        }
        "RegisterSuspendResumeNotification" => {
            signature!(Generic, 2, 2; Pointer(h_recipient), Dword(flags))
        }
        "RegisterTouchHitTestingWindow" => signature!(Generic, 2, 2; Pointer(hwnd), Dword(value)),
        "RegisterTouchWindow" => signature!(Generic, 2, 2; Pointer(hwnd), Dword(ul_flags)),
        "RegisterWindowMessageA" => signature!(Generic, 1, 1; AnsiString(lp_string)),
        "RegisterWindowMessageW" => signature!(Generic, 1, 1; WideString(lp_string)),
        "RemoveMenu" => {
            signature!(Generic, 3, 3; Pointer(h_menu), Dword(u_position), Dword(u_flags))
        }
        "ReplyMessage" => signature!(Generic, 1, 1; Pointer(l_result)),
        "ReuseDDElParam" => {
            signature!(Generic, 5, 5; Pointer(l_param), Dword(msg_in), Dword(msg_out), Pointer(ui_lo), Pointer(ui_hi))
        }
        "ScrollDC" => {
            signature!(Generic, 7, 7; Pointer(h_dc), Dword(dx), Dword(dy), Pointer(lprc_scroll), Pointer(lprc_clip), Pointer(hrgn_update), Pointer(lprc_update))
        }
        "ScrollWindow" => {
            signature!(Generic, 5, 5; Pointer(h_wnd), Dword(x_amount), Dword(y_amount), Pointer(lp_rect), Pointer(lp_clip_rect))
        }
        "ScrollWindowEx" => {
            signature!(Generic, 8, 8; Pointer(h_wnd), Dword(dx), Dword(dy), Pointer(prc_scroll), Pointer(prc_clip), Pointer(hrgn_update), Pointer(prc_update), Dword(flags))
        }
        "SendDlgItemMessageA" | "SendDlgItemMessageW" => {
            signature!(Generic, 5, 5; Pointer(h_dlg), Dword(n_id_dlg_item), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "SendIMEMessageExA" | "SendIMEMessageExW" => {
            signature!(Generic, 2, 2; Pointer(hwnd), Pointer(lparam))
        }
        "SendInput" => {
            signature!(Generic, 3, 3; Dword(c_inputs), Pointer(p_inputs), Dword(cb_size))
        }
        "SendMessageCallbackA" | "SendMessageCallbackW" => {
            signature!(Generic, 6, 6; Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param), Pointer(lp_result_call_back), Pointer(dw_data))
        }
        "SendMessageTimeoutA" | "SendMessageTimeoutW" => {
            signature!(Generic, 7, 7; Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param), Dword(fu_flags), Dword(u_timeout), Pointer(lpdw_result))
        }
        "SendNotifyMessageA" | "SendNotifyMessageW" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "SetAdditionalForegroundBoostProcesses" => {
            signature!(Generic, 3, 3; Pointer(top_level_window), Dword(process_handle_count), Pointer(process_handle_array))
        }
        "SetCaretBlinkTime" => signature!(Generic, 1, 1; Dword(u_m_seconds)),
        "SetCaretPos" | "SetPhysicalCursorPos" => signature!(Generic, 2, 2; Dword(x), Dword(y)),
        "SetClassLongA" | "SetClassLongW" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Dword(n_index), Dword(dw_new_long))
        }
        "SetClassLongPtrA" | "SetClassLongPtrW" | "SetWindowLongPtrA" | "SetWindowLongPtrW" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Dword(n_index), Pointer(dw_new_long))
        }
        "SetClassWord" | "SetWindowWord" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Dword(n_index), Word(w_new_word))
        }
        "SetClipboardData" => signature!(Generic, 2, 2; Dword(u_format), Pointer(h_mem)),
        "SetClipboardViewer" => signature!(Generic, 1, 1; Pointer(h_wnd_new_viewer)),
        "SetCoalescableTimer" => {
            signature!(Generic, 5, 5; Pointer(h_wnd), Pointer(n_id_event), Dword(u_elapse), Pointer(lp_timer_func), Dword(u_tolerance_delay))
        }
        "SetDebugErrorLevel" => signature!(Void, 1, 1; Dword(dw_level)),
        "SetDialogControlDpiChangeBehavior" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Dword(mask), Dword(values))
        }
        "SetDialogDpiChangeBehavior" => {
            signature!(Generic, 3, 3; Pointer(h_dlg), Dword(mask), Dword(values))
        }
        "SetDisplayAutoRotationPreferences" => signature!(Generic, 1, 1; Dword(orientation)),
        "SetDisplayConfig" => {
            signature!(Generic, 5, 5; Dword(num_path_array_elements), Pointer(path_array), Dword(num_mode_info_array_elements), Pointer(mode_info_array), Dword(flags))
        }
        "SetDlgItemInt" => {
            signature!(Generic, 4, 4; Pointer(h_dlg), Dword(n_id_dlg_item), Dword(u_value), Dword(b_signed))
        }
        "SetDlgItemTextA" => {
            signature!(Generic, 3, 3; Pointer(h_dlg), Dword(n_id_dlg_item), AnsiString(lp_string))
        }
        "SetDlgItemTextW" => {
            signature!(Generic, 3, 3; Pointer(h_dlg), Dword(n_id_dlg_item), WideString(lp_string))
        }
        "SetDoubleClickTime" => signature!(Generic, 1, 1; Dword(uint)),
        "SetGestureConfig" => {
            signature!(Generic, 5, 5; Pointer(hwnd), Dword(dw_reserved), Dword(c_i_ds), Pointer(p_gesture_config), Dword(cb_size))
        }
        "SetLastErrorEx" => signature!(Void, 2, 2; Dword(dw_err_code), Dword(dw_type)),
        "SetLayeredWindowAttributes" => {
            signature!(Generic, 4, 4; Pointer(hwnd), Dword(cr_key), Byte(b_alpha), Dword(dw_flags))
        }
        "SetMenu" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(h_menu)),
        "SetMenuContextHelpId" => signature!(Generic, 2, 2; Pointer(hmenu), Dword(dword)),
        "SetMenuDefaultItem" => {
            signature!(Generic, 3, 3; Pointer(h_menu), Dword(u_item), Dword(f_by_pos))
        }
        "SetMenuInfo" => signature!(Generic, 2, 2; Pointer(hmenu), Pointer(lpcmenuinfo)),
        "SetMenuItemBitmaps" => {
            signature!(Generic, 5, 5; Pointer(h_menu), Dword(u_position), Dword(u_flags), Pointer(h_bitmap_unchecked), Pointer(h_bitmap_checked))
        }
        "SetMenuItemInfoA" | "SetMenuItemInfoW" => {
            signature!(Generic, 4, 4; Pointer(hmenu), Dword(item), Dword(f_by_positon), Pointer(lpmii))
        }
        "SetMessageExtraInfo" => signature!(Generic, 1, 1; Pointer(l_param)),
        "SetMessageQueue" => signature!(Generic, 1, 1; Dword(c_messages_max)),
        "SetParent" => signature!(Generic, 2, 2; Pointer(h_wnd_child), Pointer(h_wnd_new_parent)),
        "SetProcessDefaultLayout" => signature!(Generic, 1, 1; Dword(dw_default_layout)),
        "SetProcessRestrictionExemption" => signature!(Generic, 1, 1; Dword(f_enable_exemption)),
        "SetProcessWindowStation" => signature!(Generic, 1, 1; Pointer(h_win_sta)),
        "SetPropA" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), AnsiString(lp_string), Pointer(h_data))
        }
        "SetPropW" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), WideString(lp_string), Pointer(h_data))
        }
        "SetScrollInfo" => {
            signature!(Generic, 4, 4; Pointer(hwnd), Dword(n_bar), Pointer(lpsi), Dword(redraw))
        }
        "SetScrollPos" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(n_bar), Dword(n_pos), Dword(b_redraw))
        }
        "SetScrollRange" => {
            signature!(Generic, 5, 5; Pointer(h_wnd), Dword(n_bar), Dword(n_min_pos), Dword(n_max_pos), Dword(b_redraw))
        }
        "SetSysColors" => {
            signature!(Generic, 3, 3; Dword(c_elements), Pointer(lpa_elements), Pointer(lpa_rgb_values))
        }
        "SetSystemCursor" => signature!(Generic, 2, 2; Pointer(hcur), Dword(id)),
        "SetThreadDesktop" | "SwitchDesktop" => signature!(Generic, 1, 1; Pointer(h_desktop)),
        "SetThreadDpiAwarenessContext" => signature!(Generic, 1, 1; Pointer(dpi_context)),
        "SetThreadDpiHostingBehavior" => signature!(Generic, 1, 1; Dword(value)),
        "SetTimer" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Pointer(n_id_event), Dword(u_elapse), Pointer(lp_timer_func))
        }
        "SetUserObjectInformationA" | "SetUserObjectInformationW" => {
            signature!(Generic, 4, 4; Pointer(h_obj), Dword(n_index), Pointer(pv_info), Dword(n_length))
        }
        "SetUserObjectSecurity" => {
            signature!(Generic, 3, 3; Pointer(h_obj), Pointer(p_si_requested), Pointer(p_sid))
        }
        "SetWinEventHook" => {
            signature!(Generic, 7, 7; Dword(event_min), Dword(event_max), Pointer(hmod_win_event_proc), Pointer(pfn_win_event_proc), Dword(id_process), Dword(id_thread), Dword(dw_flags))
        }
        "SetWindowContextHelpId" => signature!(Generic, 2, 2; Pointer(hwnd), Dword(dword)),
        "SetWindowDisplayAffinity" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(dw_affinity)),
        "SetWindowFeedbackSetting" => {
            signature!(Generic, 5, 5; Pointer(hwnd), Dword(feedback), Dword(dw_flags), Dword(size), Pointer(configuration))
        }
        "SetWindowRgn" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(h_rgn), Dword(b_redraw))
        }
        "SetWindowsHookA" | "SetWindowsHookW" => {
            signature!(Generic, 2, 2; Dword(n_filter_type), Pointer(pfn_filter_proc))
        }
        "SetWindowsHookExW" => {
            signature!(Generic, 4, 4; Dword(id_hook), Pointer(lpfn), Pointer(hmod), Dword(dw_thread_id))
        }
        "ShowOwnedPopups" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(f_show)),
        "ShowScrollBar" => signature!(Generic, 3, 3; Pointer(h_wnd), Dword(w_bar), Dword(b_show)),
        "ShowWindowAsync" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(n_cmd_show)),
        "ShutdownBlockReasonCreate" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), WideString(pwsz_reason))
        }
        "ShutdownBlockReasonQuery" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(pwsz_buff), Pointer(pcch_buff))
        }
        "SkipPointerFrameMessages" => signature!(Generic, 1, 1; Dword(pointer_id)),
        "SwapMouseButton" => signature!(Generic, 1, 1; Dword(f_swap)),
        "SwitchToThisWindow" => signature!(Void, 2, 2; Pointer(hwnd), Dword(f_unknown)),
        "SystemParametersInfoForDpi" => {
            signature!(Generic, 5, 5; Dword(ui_action), Dword(ui_param), Pointer(pv_param), Dword(f_win_ini), Dword(dpi))
        }
        "SystemParametersInfoW" => {
            signature!(Generic, 4, 4; Dword(ui_action), Dword(ui_param), Pointer(pv_param), Dword(f_win_ini))
        }
        "TabbedTextOutA" | "TabbedTextOutW" => {
            signature!(Generic, 8, 8; Pointer(hdc), Dword(x), Dword(y), Pointer(lp_string), Dword(ch_count), Dword(n_tab_positions), Pointer(lpn_tab_stop_positions), Dword(n_tab_origin))
        }
        "ToAscii" => {
            signature!(Generic, 5, 5; Dword(u_virt_key), Dword(u_scan_code), Pointer(lp_key_state), Pointer(lp_char), Dword(u_flags))
        }
        "ToAsciiEx" => {
            signature!(Generic, 6, 6; Dword(u_virt_key), Dword(u_scan_code), Pointer(lp_key_state), Pointer(lp_char), Dword(u_flags), Pointer(dwhkl))
        }
        "ToUnicode" => {
            signature!(Generic, 6, 6; Dword(w_virt_key), Dword(w_scan_code), Pointer(lp_key_state), Pointer(pwsz_buff), Dword(cch_buff), Dword(w_flags))
        }
        "ToUnicodeEx" => {
            signature!(Generic, 7, 7; Dword(w_virt_key), Dword(w_scan_code), Pointer(lp_key_state), Pointer(pwsz_buff), Dword(cch_buff), Dword(w_flags), Pointer(dwhkl))
        }
        "TrackMouseEvent" => signature!(Generic, 1, 1; Pointer(lp_event_track)),
        "TrackPopupMenu" => {
            signature!(Generic, 7, 7; Pointer(h_menu), Dword(u_flags), Dword(x), Dword(y), Dword(n_reserved), Pointer(h_wnd), Pointer(prc_rect))
        }
        "TrackPopupMenuEx" => {
            signature!(Generic, 6, 6; Pointer(h_menu), Dword(u_flags), Dword(x), Dword(y), Pointer(hwnd), Pointer(lptpm))
        }
        "TranslateAcceleratorA" | "TranslateAcceleratorW" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(h_acc_table), Pointer(lp_msg))
        }
        "TranslateMDISysAccel" => signature!(Generic, 2, 2; Pointer(h_wnd_client), Pointer(lp_msg)),
        "UnhookWinEvent" => signature!(Generic, 1, 1; Pointer(h_win_event_hook)),
        "UnhookWindowsHook" => signature!(Generic, 2, 2; Dword(n_code), Pointer(pfn_filter_proc)),
        "UnloadKeyboardLayout" => signature!(Generic, 1, 1; Pointer(hkl)),
        "UnpackDDElParam" => {
            signature!(Generic, 4, 4; Dword(msg), Pointer(l_param), Pointer(pui_lo), Pointer(pui_hi))
        }
        "UnregisterClassA" => {
            signature!(Generic, 2, 2; AnsiString(lp_class_name), Pointer(h_instance))
        }
        "UnregisterClassW" => {
            signature!(Generic, 2, 2; WideString(lp_class_name), Pointer(h_instance))
        }
        "UnregisterDeviceNotification"
        | "UnregisterPowerSettingNotification"
        | "UnregisterSuspendResumeNotification" => signature!(Generic, 1, 1; Pointer(handle)),
        "UnregisterHotKey" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(id)),
        "UpdateLayeredWindow" => {
            signature!(Generic, 9, 9; Pointer(h_wnd), Pointer(hdc_dst), Pointer(ppt_dst), Pointer(psize), Pointer(hdc_src), Pointer(ppt_src), Dword(cr_key), Pointer(pblend), Dword(dw_flags))
        }
        "UpdateLayeredWindowIndirect" => {
            signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(p_ulw_info))
        }
        "UserHandleGrantAccess" => {
            signature!(Generic, 3, 3; Pointer(h_user_handle), Pointer(h_job), Dword(b_grant))
        }
        "ValidateRect" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lp_rect)),
        "VkKeyScanA" => signature!(Generic, 1, 1; Byte(ch)),
        "VkKeyScanExA" => signature!(Generic, 2, 2; Byte(ch), Pointer(dwhkl)),
        "VkKeyScanExW" => signature!(Generic, 2, 2; Word(ch), Pointer(dwhkl)),
        "VkKeyScanW" => signature!(Generic, 1, 1; Word(ch)),
        "WINNLSEnableIME" => signature!(Generic, 2, 2; Pointer(hwnd), Dword(bool)),
        "WaitForInputIdle" => signature!(Generic, 2, 2; Pointer(h_process), Dword(dw_milliseconds)),
        "WinHelpA" => {
            signature!(Generic, 4, 4; Pointer(h_wnd_main), AnsiString(lpsz_help), Dword(u_command), Pointer(dw_data))
        }
        "WinHelpW" => {
            signature!(Generic, 4, 4; Pointer(h_wnd_main), WideString(lpsz_help), Dword(u_command), Pointer(dw_data))
        }
        "WindowFromDC" => signature!(Generic, 1, 1; Pointer(h_dc)),
        "WindowFromPhysicalPoint" | "WindowFromPoint" => signature!(Generic, 1, 2),
        "keybd_event" => {
            signature!(Void, 4, 4; Byte(b_vk), Byte(b_scan), Dword(dw_flags), Pointer(dw_extra_info))
        }
        "mouse_event" => {
            signature!(Void, 5, 5; Dword(dw_flags), Dword(dx), Dword(dy), Dword(dw_data), Pointer(dw_extra_info))
        }
        "wvsprintfA" => {
            signature!(Generic, 3, 3; Pointer(lpstr), AnsiString(lpcstr), Pointer(arglist))
        }
        "wvsprintfW" => {
            signature!(Generic, 3, 3; Pointer(lpwstr), WideString(lpcwstr), Pointer(arglist))
        }
        "AdjustWindowRect" => {
            signature!(Win32Bool, 3, 3; Pointer(lp_rect), Dword(dw_style), Dword(b_menu))
        }
        "AdjustWindowRectEx" => {
            signature!(Win32Bool, 4, 4; Pointer(lp_rect), Dword(dw_style), Dword(b_menu), Dword(dw_ex_style))
        }
        "BeginPaint" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lp_paint)),
        "CharLowerBuffA" => signature!(Generic, 2, 2; Pointer(lpsz), Dword(cch_length)),
        "CharLowerBuffW" => signature!(Generic, 2, 2; Pointer(lpsz), Dword(cch_length)),
        "CharLowerW" => signature!(Generic, 1, 1; Pointer(lpsz)),
        "CharNextA" => signature!(Generic, 1, 1; AnsiString(lpsz)),
        "CharNextExA" => {
            signature!(Generic, 3, 3; Word(code_page), AnsiString(lp_current_char), Dword(dw_flags))
        }
        "CharNextW" => signature!(Generic, 1, 1; WideString(lpsz)),
        "CharPrevA" => signature!(Generic, 2, 2; AnsiString(lpsz_start), AnsiString(lpsz_current)),
        "CharPrevExA" => {
            signature!(Generic, 4, 4; Word(code_page), AnsiString(lp_start), AnsiString(lp_current_char), Dword(dw_flags))
        }
        "CharPrevW" => signature!(Generic, 2, 2; WideString(lpsz_start), WideString(lpsz_current)),
        "CharUpperBuffA" => signature!(Generic, 2, 2; Pointer(lpsz), Dword(cch_length)),
        "CharUpperBuffW" => signature!(Generic, 2, 2; Pointer(lpsz), Dword(cch_length)),
        "CloseClipboard" => signature!(Win32Bool, 0, 0),
        "CloseDesktop" => signature!(Win32Bool, 1, 1; Pointer(h_desktop)),
        "CloseGestureInfoHandle" => signature!(Win32Bool, 1, 1; Pointer(h_gesture_info)),
        "CloseTouchInputHandle" => signature!(Win32Bool, 1, 1; Pointer(h_touch_input)),
        "CloseWindow" => signature!(Win32Bool, 1, 1; Pointer(h_wnd)),
        "CloseWindowStation" => signature!(Win32Bool, 1, 1; Pointer(h_win_sta)),
        "CopyRect" => signature!(Generic, 2, 2; Pointer(lprc_dst), Pointer(lprc_src)),
        "CreateWindowExA" => {
            signature!(Generic, 12, 12; Dword(dw_ex_style), AnsiString(lp_class_name), AnsiString(lp_window_name), Dword(dw_style), Dword(x), Dword(y), Dword(n_width), Dword(n_height), Pointer(h_wnd_parent), Pointer(h_menu), Pointer(h_instance), Pointer(lp_param))
        }
        "CreateWindowExW" => {
            signature!(Generic, 12, 12; Dword(dw_ex_style), WideString(lp_class_name), WideString(lp_window_name), Dword(dw_style), Dword(x), Dword(y), Dword(n_width), Dword(n_height), Pointer(h_wnd_parent), Pointer(h_menu), Pointer(h_instance), Pointer(lp_param))
        }
        "DefWindowProcA" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "DefWindowProcW" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "DeleteMenu" => {
            signature!(Generic, 3, 3; Pointer(h_menu), Dword(u_position), Dword(u_flags))
        }
        "DestroyAcceleratorTable" => signature!(Win32Bool, 1, 1; Pointer(h_accel)),
        "DestroyCaret" => signature!(Win32Bool, 0, 0),
        "DestroyCursor" => signature!(Win32Bool, 1, 1; Pointer(h_cursor)),
        "DestroyIcon" => signature!(Win32Bool, 1, 1; Pointer(h_icon)),
        "DestroyMenu" => signature!(Win32Bool, 1, 1; Pointer(h_menu)),
        "DestroySyntheticPointerDevice" => signature!(Void, 1, 1; Pointer(device)),
        "DestroyWindow" => signature!(Win32Bool, 1, 1; Pointer(h_wnd)),
        "DispatchMessageA" => signature!(Generic, 1, 1; Pointer(lp_msg)),
        "DispatchMessageW" => signature!(Generic, 1, 1; Pointer(lp_msg)),
        "DrawTextA" => {
            signature!(Generic, 5, 5; Pointer(hdc), Pointer(lpch_text), Dword(cch_text), Pointer(lprc), Dword(format))
        }
        "DrawTextW" => {
            signature!(Generic, 5, 5; Pointer(hdc), Pointer(lpch_text), Dword(cch_text), Pointer(lprc), Dword(format))
        }
        "EnableWindow" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(b_enable)),
        "EndPaint" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lp_paint)),
        "EqualRect" => signature!(Generic, 2, 2; Pointer(lprc1), Pointer(lprc2)),
        "FillRect" => signature!(Generic, 3, 3; Pointer(h_dc), Pointer(lprc), Pointer(hbr)),
        "FindWindowA" => {
            signature!(Generic, 2, 2; AnsiString(lp_class_name), AnsiString(lp_window_name))
        }
        "FindWindowW" => {
            signature!(Generic, 2, 2; WideString(lp_class_name), WideString(lp_window_name))
        }
        "FreeDDElParam" => signature!(Win32Bool, 2, 2; Dword(msg), Pointer(l_param)),
        "GetAsyncKeyState" => signature!(Generic, 1, 1; Dword(v_key)),
        "GetClassInfoExA" => {
            signature!(Generic, 3, 3; Pointer(h_instance), AnsiString(lpsz_class), Pointer(lpwcx))
        }
        "GetClassInfoExW" => {
            signature!(Generic, 3, 3; Pointer(h_instance), WideString(lpsz_class), Pointer(lpwcx))
        }
        "GetClientRect" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lp_rect)),
        "GetCursorPos" => signature!(Generic, 1, 1; Pointer(lp_point)),
        "GetDC" => signature!(Generic, 1, 1; Pointer(h_wnd)),
        "GetDCEx" => signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(hrgn_clip), Dword(flags)),
        "GetDesktopWindow" => signature!(Generic, 0, 0),
        "GetFocus" => signature!(Generic, 0, 0),
        "GetForegroundWindow" => signature!(Generic, 0, 0),
        "GetKeyState" => signature!(Generic, 1, 1; Dword(n_virt_key)),
        "GetKeyboardLayout" => signature!(Generic, 1, 1; Dword(id_thread)),
        "GetKeyboardType" => signature!(Generic, 1, 1; Dword(n_type_flag)),
        "GetMessageA" => {
            signature!(GetMessage, 4, 4; Pointer(lp_msg), Pointer(h_wnd), Dword(w_msg_filter_min), Dword(w_msg_filter_max))
        }
        "GetMessageW" => {
            signature!(GetMessage, 4, 4; Pointer(lp_msg), Pointer(h_wnd), Dword(w_msg_filter_min), Dword(w_msg_filter_max))
        }
        "GetProcessWindowStation" => signature!(Handle, 0, 0),
        "GetSystemMetrics" => signature!(Generic, 1, 1; Dword(n_index)),
        "GetUserObjectInformationW" => {
            signature!(Generic, 5, 5; Pointer(h_obj), Dword(n_index), Pointer(pv_info), Dword(n_length), Pointer(lpn_length_needed))
        }
        "GetWindowDC" => signature!(Generic, 1, 1; Pointer(h_wnd)),
        "GetWindowLongA" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(n_index)),
        "GetWindowLongW" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(n_index)),
        "GetWindowRect" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(lp_rect)),
        "GetWindowTextA" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(lp_string), Dword(n_max_count))
        }
        "GetWindowTextW" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Pointer(lp_string), Dword(n_max_count))
        }
        "InflateRect" => signature!(Generic, 3, 3; Pointer(lprc), Dword(dx), Dword(dy)),
        "IntersectRect" => {
            signature!(Generic, 3, 3; Pointer(lprc_dst), Pointer(lprc_src1), Pointer(lprc_src2))
        }
        "InvalidateRect" => {
            signature!(Win32Bool, 3, 3; Pointer(h_wnd), Pointer(lp_rect), Dword(b_erase))
        }
        "IsCharAlphaA" => signature!(Generic, 1, 1; Byte(ch)),
        "IsCharAlphaNumericA" => signature!(Generic, 1, 1; Byte(ch)),
        "IsCharAlphaNumericW" => signature!(Generic, 1, 1; Word(ch)),
        "IsCharAlphaW" => signature!(Generic, 1, 1; Word(ch)),
        "IsCharLowerA" => signature!(Generic, 1, 1; Byte(ch)),
        "IsCharLowerW" => signature!(Generic, 1, 1; Word(ch)),
        "IsCharUpperA" => signature!(Generic, 1, 1; Byte(ch)),
        "IsCharUpperW" => signature!(Generic, 1, 1; Word(ch)),
        "IsRectEmpty" => signature!(Generic, 1, 1; Pointer(lprc)),
        "IsWindow" => signature!(Generic, 1, 1; Pointer(h_wnd)),
        "IsWindowVisible" => signature!(Generic, 1, 1; Pointer(h_wnd)),
        "LoadAcceleratorsA" => signature!(Handle, 2, 2; Pointer(instance), Pointer(resource_name)),
        "LoadAcceleratorsW" => signature!(Handle, 2, 2; Pointer(instance), Pointer(resource_name)),
        "LoadBitmapA" => signature!(Handle, 2, 2; Pointer(instance), Pointer(resource_name)),
        "LoadBitmapW" => signature!(Handle, 2, 2; Pointer(instance), Pointer(resource_name)),
        "LoadCursorA" => signature!(Handle, 2, 2; Pointer(instance), Pointer(resource_name)),
        "LoadCursorFromFileA" => signature!(Handle, 1, 1; AnsiString(lp_file_name)),
        "LoadCursorFromFileW" => signature!(Handle, 1, 1; WideString(lp_file_name)),
        "LoadCursorW" => signature!(Handle, 2, 2; Pointer(instance), Pointer(resource_name)),
        "LoadIconA" => signature!(Handle, 2, 2; Pointer(instance), Pointer(resource_name)),
        "LoadIconW" => signature!(Handle, 2, 2; Pointer(instance), Pointer(resource_name)),
        "LoadImageA" => {
            signature!(Generic, 6, 6; Pointer(instance), Pointer(resource_name), Dword(image_type), Dword(width), Dword(height), Dword(flags))
        }
        "LoadImageW" => {
            signature!(Generic, 6, 6; Pointer(instance), Pointer(resource_name), Dword(image_type), Dword(width), Dword(height), Dword(flags))
        }
        "LoadStringA" => {
            signature!(Generic, 4, 4; Pointer(h_instance), Dword(u_id), Pointer(lp_buffer), Dword(cch_buffer_max))
        }
        "LoadStringW" => {
            signature!(Generic, 4, 4; Pointer(h_instance), Dword(u_id), Pointer(lp_buffer), Dword(cch_buffer_max))
        }
        "MapVirtualKeyA" => signature!(Generic, 2, 2; Dword(u_code), Dword(u_map_type)),
        "MapVirtualKeyExA" => {
            signature!(Generic, 3, 3; Dword(u_code), Dword(u_map_type), Pointer(dwhkl))
        }
        "MapVirtualKeyExW" => {
            signature!(Generic, 3, 3; Dword(u_code), Dword(u_map_type), Pointer(dwhkl))
        }
        "MapVirtualKeyW" => signature!(Generic, 2, 2; Dword(u_code), Dword(u_map_type)),
        "MessageBoxA" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), AnsiString(lp_text), AnsiString(lp_caption), Dword(u_type))
        }
        "MessageBoxW" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), WideString(lp_text), WideString(lp_caption), Dword(u_type))
        }
        "OffsetRect" => signature!(Generic, 3, 3; Pointer(rc_limit), Dword(x), Dword(y)),
        "PeekMessageA" => {
            signature!(Generic, 5, 5; Pointer(lp_msg), Pointer(h_wnd), Dword(w_msg_filter_min), Dword(w_msg_filter_max), Dword(w_remove_msg))
        }
        "PeekMessageW" => {
            signature!(Generic, 5, 5; Pointer(lp_msg), Pointer(h_wnd), Dword(w_msg_filter_min), Dword(w_msg_filter_max), Dword(w_remove_msg))
        }
        "PostMessageA" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "PostMessageW" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "PostQuitMessage" => signature!(Void, 1, 1; Dword(n_exit_code)),
        "PtInRect" => signature!(Generic, 2, 3),
        "RegisterClassA" => signature!(Generic, 1, 1; Pointer(lp_wnd_class)),
        "RegisterClassW" => signature!(Generic, 1, 1; Pointer(lp_wnd_class)),
        "ReleaseCapture" => signature!(Win32Bool, 0, 0),
        "ReleaseDC" => signature!(Generic, 2, 2; Pointer(h_wnd), Pointer(h_dc)),
        "SendMessageA" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "SendMessageW" => {
            signature!(Generic, 4, 4; Pointer(h_wnd), Dword(msg), Pointer(w_param), Pointer(l_param))
        }
        "SetCursor" => signature!(Handle, 1, 1; Pointer(h_cursor)),
        "SetCursorPos" => signature!(Generic, 2, 2; Dword(x), Dword(y)),
        "SetFocus" => signature!(Generic, 1, 1; Pointer(h_wnd)),
        "SetForegroundWindow" => signature!(Generic, 1, 1; Pointer(h_wnd)),
        "SetRect" => {
            signature!(Generic, 5, 5; Pointer(lprc), Dword(x_left), Dword(y_top), Dword(x_right), Dword(y_bottom))
        }
        "SetRectEmpty" => signature!(Generic, 1, 1; Pointer(lprc)),
        "SetWindowLongA" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Dword(n_index), Dword(dw_new_long))
        }
        "SetWindowLongW" => {
            signature!(Generic, 3, 3; Pointer(h_wnd), Dword(n_index), Dword(dw_new_long))
        }
        "SetWindowPos" => {
            signature!(Generic, 7, 7; Pointer(h_wnd), Pointer(h_wnd_insert_after), Dword(x), Dword(y), Dword(cx), Dword(cy), Dword(u_flags))
        }
        "SetWindowTextA" => signature!(Generic, 2, 2; Pointer(h_wnd), AnsiString(lp_string)),
        "SetWindowTextW" => signature!(Generic, 2, 2; Pointer(h_wnd), WideString(lp_string)),
        "SetWindowsHookExA" => {
            signature!(Generic, 4, 4; Dword(id_hook), Pointer(lpfn), Pointer(hmod), Dword(dw_thread_id))
        }
        "ShowCursor" => signature!(Generic, 1, 1; Dword(b_show)),
        "ShowWindow" => signature!(Generic, 2, 2; Pointer(h_wnd), Dword(n_cmd_show)),
        "SubtractRect" => {
            signature!(Generic, 3, 3; Pointer(lprc_dst), Pointer(lprc_src1), Pointer(lprc_src2))
        }
        "SystemParametersInfoA" => {
            signature!(Generic, 4, 4; Dword(ui_action), Dword(ui_param), Pointer(pv_param), Dword(f_win_ini))
        }
        "TranslateMessage" => signature!(Generic, 1, 1; Pointer(lp_msg)),
        "UnhookWindowsHookEx" => signature!(Generic, 1, 1; Pointer(hhk)),
        "UnionRect" => {
            signature!(Generic, 3, 3; Pointer(lprc_dst), Pointer(lprc_src1), Pointer(lprc_src2))
        }
        "UpdateWindow" => signature!(Win32Bool, 1, 1; Pointer(h_wnd)),
        _ => None,
    }
}
