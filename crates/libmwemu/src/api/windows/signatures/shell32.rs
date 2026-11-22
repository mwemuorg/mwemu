use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "SHRestricted" => signature!(Generic, 1, 1; Dword(restriction)),
        "ILIsEqual" | "ILFindChild" => {
            signature!(Generic, 2, 2; Pointer(first_pidl), Pointer(second_pidl))
        }
        "ILIsParent" => {
            signature!(Generic, 3, 3; Pointer(parent), Pointer(child), Dword(immediate))
        }
        "AssocCreateForClasses" => {
            signature!(Hresult, 4, 4; Pointer(rg_classes), Dword(c_classes), Pointer(riid), Pointer(ppv))
        }
        "CDefFolderMenu_Create2" => {
            signature!(Hresult, 9, 9; Pointer(pidl_folder), Pointer(hwnd), Dword(cidl), Pointer(apidl), Pointer(psf), Pointer(pfn), Dword(n_keys), Pointer(ahkeys), Pointer(ppcm))
        }
        "CIDLData_CreateFromIDArray" => {
            signature!(Hresult, 4, 4; Pointer(pidl_folder), Dword(cidl), Pointer(apidl), Pointer(ppdtobj))
        }
        "DAD_DragLeave" => signature!(Generic, 0, 0; ),
        "DAD_SetDragImage" => signature!(Generic, 2, 2; Pointer(him), Pointer(ppt_offset)),
        "DoEnvironmentSubstA" | "DoEnvironmentSubstW" => {
            signature!(Generic, 2, 2; Pointer(psz_src), Dword(cch_src))
        }
        "DuplicateIcon" => signature!(Generic, 2, 2; Pointer(h_inst), Pointer(h_icon)),
        "ExtractAssociatedIconA" | "ExtractAssociatedIconW" => {
            signature!(Generic, 3, 3; Pointer(h_inst), Pointer(psz_icon_path), Pointer(pi_icon))
        }
        "ExtractAssociatedIconExA" | "ExtractAssociatedIconExW" => {
            signature!(Generic, 4, 4; Pointer(h_inst), Pointer(psz_icon_path), Pointer(pi_icon_index), Pointer(pi_icon_id))
        }
        "ExtractIconA" => {
            signature!(Generic, 3, 3; Pointer(h_inst), AnsiString(psz_exe_file_name), Dword(n_icon_index))
        }
        "ExtractIconW" => {
            signature!(Generic, 3, 3; Pointer(h_inst), WideString(psz_exe_file_name), Dword(n_icon_index))
        }
        "FindExecutableA" => {
            signature!(Generic, 3, 3; AnsiString(lp_file), AnsiString(lp_directory), Pointer(lp_result))
        }
        "FindExecutableW" => {
            signature!(Generic, 3, 3; WideString(lp_file), WideString(lp_directory), Pointer(lp_result))
        }
        "ILClone" | "ILCloneFirst" | "ILRemoveLastID" => signature!(Generic, 1, 1; Pointer(pidl)),
        "ILCombine" => signature!(Generic, 2, 2; Pointer(pidl1), Pointer(pidl2)),
        "ILCreateFromPathA" => signature!(Generic, 1, 1; AnsiString(psz_path)),
        "ILCreateFromPathW" | "SHSimpleIDListFromPath" | "Win32DeleteFile" => {
            signature!(Generic, 1, 1; WideString(psz_path))
        }
        "ILSaveToStream" => signature!(Hresult, 2, 2; Pointer(pstm), Pointer(pidl)),
        "OpenRegStream" => {
            signature!(Generic, 4, 4; Pointer(hkey), WideString(psz_subkey), WideString(psz_value), Dword(grf_mode))
        }
        "PathMakeUniqueName" => {
            signature!(Generic, 5, 5; Pointer(psz_unique_name), Dword(cch_max), WideString(psz_template), WideString(psz_long_plate), WideString(psz_dir))
        }
        "PifMgr_CloseProperties" => signature!(Generic, 2, 2; Pointer(h_props), Dword(fl_opt)),
        "PifMgr_SetProperties" => {
            signature!(Generic, 5, 5; Pointer(h_props), AnsiString(psz_group), Pointer(lp_props), Dword(cb_props), Dword(fl_opt))
        }
        "SHAlloc" => signature!(Generic, 1, 1; Pointer(cb)),
        "SHAssocEnumHandlers" => {
            signature!(Hresult, 3, 3; WideString(psz_extra), Dword(af_filter), Pointer(pp_enum_handler))
        }
        "SHAssocEnumHandlersForProtocolByApplication" => {
            signature!(Hresult, 3, 3; WideString(protocol), Pointer(riid), Pointer(enum_handlers))
        }
        "SHBindToFolderIDListParent" => {
            signature!(Hresult, 5, 5; Pointer(psf_root), Pointer(pidl), Pointer(riid), Pointer(ppv), Pointer(ppidl_last))
        }
        "SHBindToFolderIDListParentEx" => {
            signature!(Hresult, 6, 6; Pointer(psf_root), Pointer(pidl), Pointer(ppbc), Pointer(riid), Pointer(ppv), Pointer(ppidl_last))
        }
        "SHBindToObject" => {
            signature!(Hresult, 5, 5; Pointer(psf), Pointer(pidl), Pointer(pbc), Pointer(riid), Pointer(ppv))
        }
        "SHBindToParent" => {
            signature!(Hresult, 4, 4; Pointer(pidl), Pointer(riid), Pointer(ppv), Pointer(ppidl_last))
        }
        "SHBrowseForFolderA" | "SHBrowseForFolderW" => signature!(Generic, 1, 1; Pointer(lpbi)),
        "SHChangeNotification_Unlock" => signature!(Generic, 1, 1; Pointer(h_lock)),
        "SHChangeNotify" => {
            signature!(Void, 4, 4; Dword(w_event_id), Dword(u_flags), Pointer(dw_item1), Pointer(dw_item2))
        }
        "SHChangeNotifyRegister" => {
            signature!(Generic, 6, 6; Pointer(hwnd), Dword(f_sources), Dword(f_events), Dword(w_msg), Dword(c_entries), Pointer(pshcne))
        }
        "SHCoCreateInstance" => {
            signature!(Hresult, 5, 5; WideString(psz_clsid), Pointer(pclsid), Pointer(p_unk_outer), Pointer(riid), Pointer(ppv))
        }
        "SHCreateAssociationRegistration" | "SHCreateDefaultExtractIcon" => {
            signature!(Hresult, 2, 2; Pointer(riid), Pointer(ppv))
        }
        "SHCreateDataObject" => {
            signature!(Hresult, 6, 6; Pointer(pidl_folder), Dword(cidl), Pointer(apidl), Pointer(pdt_inner), Pointer(riid), Pointer(ppv))
        }
        "SHCreateDefaultContextMenu" => {
            signature!(Hresult, 3, 3; Pointer(pdcm), Pointer(riid), Pointer(ppv))
        }
        "SHCreateDefaultPropertiesOp" => {
            signature!(Hresult, 2, 2; Pointer(psi), Pointer(pp_file_op))
        }
        "SHCreateDirectory" => signature!(Generic, 2, 2; Pointer(hwnd), WideString(psz_path)),
        "SHCreateDirectoryExA" => {
            signature!(Generic, 3, 3; Pointer(hwnd), AnsiString(psz_path), Pointer(psa))
        }
        "SHCreateDirectoryExW" => {
            signature!(Generic, 3, 3; Pointer(hwnd), WideString(psz_path), Pointer(psa))
        }
        "SHCreateFileExtractIconW" => {
            signature!(Hresult, 4, 4; WideString(psz_file), Dword(dw_file_attributes), Pointer(riid), Pointer(ppv))
        }
        "SHCreateItemFromIDList" => {
            signature!(Hresult, 3, 3; Pointer(pidl), Pointer(riid), Pointer(ppv))
        }
        "SHCreateItemFromParsingName" => {
            signature!(Hresult, 4, 4; WideString(psz_path), Pointer(pbc), Pointer(riid), Pointer(ppv))
        }
        "SHCreateItemFromRelativeName" => {
            signature!(Hresult, 5, 5; Pointer(psi_parent), WideString(psz_name), Pointer(pbc), Pointer(riid), Pointer(ppv))
        }
        "SHCreateItemInKnownFolder" => {
            signature!(Hresult, 5, 5; Pointer(kfid), Dword(dw_kf_flags), WideString(psz_item), Pointer(riid), Pointer(ppv))
        }
        "SHCreateItemWithParent" => {
            signature!(Hresult, 5, 5; Pointer(pidl_parent), Pointer(psf_parent), Pointer(pidl), Pointer(riid), Pointer(ppv_item))
        }
        "SHCreatePropSheetExtArray" => {
            signature!(Generic, 3, 3; Pointer(h_key), WideString(psz_sub_key), Dword(max_iface))
        }
        "SHCreateQueryCancelAutoPlayMoniker" => signature!(Hresult, 1, 1; Pointer(ppmoniker)),
        "SHCreateShellFolderView" | "SHCreateShellFolderViewEx" => {
            signature!(Hresult, 2, 2; Pointer(pcsfv), Pointer(ppsv))
        }
        "SHCreateShellItem" => {
            signature!(Hresult, 4, 4; Pointer(pidl_parent), Pointer(psf_parent), Pointer(pidl), Pointer(ppsi))
        }
        "SHCreateShellItemArray" => {
            signature!(Hresult, 5, 5; Pointer(pidl_parent), Pointer(psf), Dword(cidl), Pointer(ppidl), Pointer(ppsi_item_array))
        }
        "SHCreateShellItemArrayFromDataObject" => {
            signature!(Hresult, 3, 3; Pointer(pdo), Pointer(riid), Pointer(ppv))
        }
        "SHCreateShellItemArrayFromIDLists" => {
            signature!(Hresult, 3, 3; Dword(cidl), Pointer(rgpidl), Pointer(ppsi_item_array))
        }
        "SHCreateShellItemArrayFromShellItem" => {
            signature!(Hresult, 3, 3; Pointer(psi), Pointer(riid), Pointer(ppv))
        }
        "SHCreateStdEnumFmtEtc" => {
            signature!(Hresult, 3, 3; Dword(cfmt), Pointer(afmt), Pointer(ppenum_format_etc))
        }
        "SHDefExtractIconA" => {
            signature!(Hresult, 6, 6; AnsiString(psz_icon_file), Dword(i_index), Dword(u_flags), Pointer(phicon_large), Pointer(phicon_small), Dword(n_icon_size))
        }
        "SHDefExtractIconW" => {
            signature!(Hresult, 6, 6; WideString(psz_icon_file), Dword(i_index), Dword(u_flags), Pointer(phicon_large), Pointer(phicon_small), Dword(n_icon_size))
        }
        "SHEnumerateUnreadMailAccountsW" => {
            signature!(Hresult, 4, 4; Pointer(h_key_user), Dword(dw_index), Pointer(psz_mail_address), Dword(cch_mail_address))
        }
        "SHFind_InitMenuPopup" => {
            signature!(Generic, 4, 4; Pointer(hmenu), Pointer(hwnd_owner), Dword(id_cmd_first), Dword(id_cmd_last))
        }
        "SHFlushSFCache" => signature!(Void, 0, 0; ),
        "SHFormatDrive" => {
            signature!(Generic, 4, 4; Pointer(hwnd), Dword(drive), Dword(fmt_id), Dword(options))
        }
        "SHFree" => signature!(Void, 1, 1; Pointer(pv)),
        "SHGetAttributesFromDataObject" => {
            signature!(Hresult, 4, 4; Pointer(pdo), Dword(dw_attribute_mask), Pointer(pdw_attributes), Pointer(pc_items))
        }
        "SHGetDataFromIDListA" | "SHGetDataFromIDListW" => {
            signature!(Hresult, 5, 5; Pointer(psf), Pointer(pidl), Dword(n_format), Pointer(pv), Dword(cb))
        }
        "SHGetDesktopFolder" => signature!(Hresult, 1, 1; Pointer(ppshf)),
        "SHGetDiskFreeSpaceExA" => {
            signature!(Generic, 4, 4; AnsiString(psz_directory_name), Pointer(pul_free_bytes_available_to_caller), Pointer(pul_total_number_of_bytes), Pointer(pul_total_number_of_free_bytes))
        }
        "SHGetDiskFreeSpaceExW" => {
            signature!(Generic, 4, 4; WideString(psz_directory_name), Pointer(pul_free_bytes_available_to_caller), Pointer(pul_total_number_of_bytes), Pointer(pul_total_number_of_free_bytes))
        }
        "SHGetFileInfoA" => {
            signature!(Generic, 5, 5; AnsiString(psz_path), Dword(dw_file_attributes), Pointer(psfi), Dword(cb_file_info), Dword(u_flags))
        }
        "SHGetFileInfoW" => {
            signature!(Generic, 5, 5; WideString(psz_path), Dword(dw_file_attributes), Pointer(psfi), Dword(cb_file_info), Dword(u_flags))
        }
        "SHGetFolderLocation" => {
            signature!(Hresult, 5, 5; Pointer(hwnd), Dword(csidl), Pointer(h_token), Dword(dw_flags), Pointer(ppidl))
        }
        "SHGetIDListFromObject" => signature!(Hresult, 2, 2; Pointer(punk), Pointer(ppidl)),
        "SHGetImageList" => {
            signature!(Hresult, 3, 3; Dword(i_image_list), Pointer(riid), Pointer(ppv_obj))
        }
        "SHGetItemFromDataObject" => {
            signature!(Hresult, 4, 4; Pointer(pdtobj), Dword(dw_flags), Pointer(riid), Pointer(ppv))
        }
        "SHGetItemFromObject" => {
            signature!(Hresult, 3, 3; Pointer(punk), Pointer(riid), Pointer(ppv))
        }
        "SHGetKnownFolderIDList" => {
            signature!(Hresult, 4, 4; Pointer(rfid), Dword(dw_flags), Pointer(h_token), Pointer(ppidl))
        }
        "SHGetKnownFolderItem" => {
            signature!(Hresult, 5, 5; Pointer(rfid), Dword(flags), Pointer(h_token), Pointer(riid), Pointer(ppv))
        }
        "SHGetNameFromIDList" => {
            signature!(Hresult, 3, 3; Pointer(pidl), Dword(sigdn_name), Pointer(ppsz_name))
        }
        "SHGetPropertyStoreForWindow" => {
            signature!(Hresult, 3, 3; Pointer(hwnd), Pointer(riid), Pointer(ppv))
        }
        "SHGetPropertyStoreFromIDList" => {
            signature!(Hresult, 4, 4; Pointer(pidl), Dword(flags), Pointer(riid), Pointer(ppv))
        }
        "SHGetPropertyStoreFromParsingName" => {
            signature!(Hresult, 5, 5; WideString(psz_path), Pointer(pbc), Dword(flags), Pointer(riid), Pointer(ppv))
        }
        "SHGetRealIDL" => {
            signature!(Hresult, 3, 3; Pointer(psf), Pointer(pidl_simple), Pointer(ppidl_real))
        }
        "SHGetSpecialFolderLocation" => {
            signature!(Hresult, 3, 3; Pointer(hwnd), Dword(csidl), Pointer(ppidl))
        }
        "SHGetStockIconInfo" => {
            signature!(Hresult, 3, 3; Dword(siid), Dword(u_flags), Pointer(psii))
        }
        "SHHandleUpdateImage" => signature!(Generic, 1, 1; Pointer(pidl_extra)),
        "SHILCreateFromPath" => {
            signature!(Hresult, 3, 3; WideString(psz_path), Pointer(ppidl), Pointer(rgf_in_out))
        }
        "SHLimitInputEdit" => signature!(Hresult, 2, 2; Pointer(hwnd_edit), Pointer(psf)),
        "SHMapPIDLToSystemImageListIndex" => {
            signature!(Generic, 3, 3; Pointer(pshf), Pointer(pidl), Pointer(pi_index_sel))
        }
        "SHMultiFileProperties" => signature!(Hresult, 2, 2; Pointer(pdtobj), Dword(dw_flags)),
        "SHObjectProperties" => {
            signature!(Generic, 4, 4; Pointer(hwnd), Dword(shop_object_type), WideString(psz_object_name), WideString(psz_property_page))
        }
        "SHOpenFolderAndSelectItems" => {
            signature!(Hresult, 4, 4; Pointer(pidl_folder), Dword(cidl), Pointer(apidl), Dword(dw_flags))
        }
        "SHOpenPropSheetW" => {
            signature!(Generic, 7, 7; WideString(psz_caption), Pointer(ahkeys), Dword(ckeys), Pointer(pclsid_default), Pointer(pdtobj), Pointer(psb), WideString(p_start_page))
        }
        "SHParseDisplayName" => {
            signature!(Hresult, 5, 5; WideString(psz_name), Pointer(pbc), Pointer(ppidl), Dword(sfgao_in), Pointer(psfgao_out))
        }
        "SHPathPrepareForWriteA" => {
            signature!(Hresult, 4, 4; Pointer(hwnd), Pointer(punk_enable_modless), AnsiString(psz_path), Dword(dw_flags))
        }
        "SHPathPrepareForWriteW" => {
            signature!(Hresult, 4, 4; Pointer(hwnd), Pointer(punk_enable_modless), WideString(psz_path), Dword(dw_flags))
        }
        "SHPropStgWriteMultiple" => {
            signature!(Hresult, 6, 6; Pointer(pps), Pointer(pu_code_page), Dword(cpspec), Pointer(rgpspec), Pointer(rgvar), Dword(propid_name_first))
        }
        "SHRemoveLocalizedName" => signature!(Hresult, 1, 1; WideString(psz_path)),
        "SHReplaceFromPropSheetExtArray" => {
            signature!(Generic, 4, 4; Pointer(hpsxa), Dword(u_page_id), Pointer(lpfn_replace_with), Pointer(l_param))
        }
        "SHSetDefaultProperties" => {
            signature!(Hresult, 4, 4; Pointer(hwnd), Pointer(psi), Dword(dw_file_op_flags), Pointer(pfops))
        }
        "SHSetInstanceExplorer" => signature!(Void, 1, 1; Pointer(punk)),
        "SHSetKnownFolderPath" => {
            signature!(Hresult, 4, 4; Pointer(rfid), Dword(dw_flags), Pointer(h_token), WideString(psz_path))
        }
        "SHSetLocalizedName" => {
            signature!(Hresult, 3, 3; WideString(psz_path), WideString(psz_res_module), Dword(ids_res))
        }
        "SHSetTemporaryPropertyForItem" => {
            signature!(Hresult, 3, 3; Pointer(psi), Pointer(propkey), Pointer(propvar))
        }
        "SHStartNetConnectionDialogW" => {
            signature!(Hresult, 3, 3; Pointer(hwnd), WideString(psz_remote_name), Dword(dw_type))
        }
        "SHUpdateImageA" => {
            signature!(Void, 4, 4; AnsiString(psz_hash_item), Dword(i_index), Dword(u_flags), Dword(i_image_index))
        }
        "SHUpdateImageW" => {
            signature!(Void, 4, 4; WideString(psz_hash_item), Dword(i_index), Dword(u_flags), Dword(i_image_index))
        }
        "ShellExecuteExA" | "ShellExecuteExW" => signature!(Generic, 1, 1; Pointer(p_exec_info)),
        "Shell_GetCachedImageIndex" => {
            signature!(Generic, 3, 3; WideString(pwsz_icon_path), Dword(i_icon_index), Dword(u_icon_flags))
        }
        "Shell_GetCachedImageIndexA" => {
            signature!(Generic, 3, 3; AnsiString(psz_icon_path), Dword(i_icon_index), Dword(u_icon_flags))
        }
        "Shell_GetCachedImageIndexW" => {
            signature!(Generic, 3, 3; WideString(psz_icon_path), Dword(i_icon_index), Dword(u_icon_flags))
        }
        "StrChrA" | "StrChrIA" => signature!(Generic, 2, 2; AnsiString(psz_start), Word(w_match)),
        "StrChrIW" | "StrChrW" => signature!(Generic, 2, 2; WideString(psz_start), Word(w_match)),
        "StrCmpNA" | "StrCmpNIA" => {
            signature!(Generic, 3, 3; AnsiString(psz1), AnsiString(psz2), Dword(n_char))
        }
        "StrCmpNIW" | "StrCmpNW" => {
            signature!(Generic, 3, 3; WideString(psz1), WideString(psz2), Dword(n_char))
        }
        "StrRChrA" | "StrRChrIA" => {
            signature!(Generic, 3, 3; AnsiString(psz_start), AnsiString(psz_end), Word(w_match))
        }
        "StrRChrIW" | "StrRChrW" => {
            signature!(Generic, 3, 3; WideString(psz_start), WideString(psz_end), Word(w_match))
        }
        "StrRStrIA" => {
            signature!(Generic, 3, 3; AnsiString(psz_source), AnsiString(psz_last), AnsiString(psz_srch))
        }
        "StrRStrIW" => {
            signature!(Generic, 3, 3; WideString(psz_source), WideString(psz_last), WideString(psz_srch))
        }
        "StrStrA" | "StrStrIA" => {
            signature!(Generic, 2, 2; AnsiString(psz_first), AnsiString(psz_srch))
        }
        "StrStrIW" | "StrStrW" => {
            signature!(Generic, 2, 2; WideString(psz_first), WideString(psz_srch))
        }
        "DragFinish" => signature!(Void, 1, 1; Pointer(h_drop)),
        "DragQueryFileA" | "DragQueryFileW" => {
            signature!(Generic, 4, 4; Pointer(h_drop), Dword(i_file), Pointer(lpsz_file), Dword(cch))
        }
        "DragQueryPoint" => signature!(Generic, 2, 2; Pointer(h_drop), Pointer(ppt)),
        "ExtractIconExA" => {
            signature!(Generic, 5, 5; AnsiString(lpsz_file), Dword(n_icon_index), Pointer(phicon_large), Pointer(phicon_small), Dword(n_icons))
        }
        "ExtractIconExW" => {
            signature!(Generic, 5, 5; WideString(lpsz_file), Dword(n_icon_index), Pointer(phicon_large), Pointer(phicon_small), Dword(n_icons))
        }
        "SHChangeNotifyDeregister" => signature!(Generic, 1, 1; Dword(ul_id)),
        "SHDestroyPropSheetExtArray" => signature!(Void, 1, 1; Pointer(hpsxa)),
        "SHFileOperationA" | "SHFileOperationW" => signature!(Generic, 1, 1; Pointer(lp_file_op)),
        "SHFreeNameMappings" => signature!(Void, 1, 1; Pointer(h_name_mappings)),
        "SHGetPathFromIDListEx" => {
            signature!(Generic, 4, 4; Pointer(pidl), Pointer(psz_path), Dword(cch_path), Dword(u_opts))
        }
        "ShellExecuteA" => {
            signature!(Generic, 6, 6; Pointer(hwnd), AnsiString(lp_operation), AnsiString(lp_file), AnsiString(lp_parameters), AnsiString(lp_directory), Dword(n_show_cmd))
        }
        "ShellExecuteW" => {
            signature!(Generic, 6, 6; Pointer(hwnd), WideString(lp_operation), WideString(lp_file), WideString(lp_parameters), WideString(lp_directory), Dword(n_show_cmd))
        }
        "Shell_NotifyIconA" | "Shell_NotifyIconW" => {
            signature!(Generic, 2, 2; Dword(dw_message), Pointer(lp_data))
        }
        "IsUserAnAdmin" => signature!(Generic, 0, 0),
        _ => None,
    }
}
