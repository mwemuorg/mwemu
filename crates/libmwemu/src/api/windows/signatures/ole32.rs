use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "WdtpInterfacePointer_UserSize" | "WdtpInterfacePointer_UserSize64" => {
            signature!(Generic, 5, 5; Pointer(flags), Dword(marshal_context), Dword(starting_offset), Pointer(interface), Pointer(iid))
        }
        "WdtpInterfacePointer_UserMarshal" | "WdtpInterfacePointer_UserMarshal64" => {
            signature!(Generic, 5, 5; Pointer(flags), Dword(marshal_context), Pointer(buffer), Pointer(interface), Pointer(iid))
        }
        "WdtpInterfacePointer_UserUnmarshal" | "WdtpInterfacePointer_UserUnmarshal64" => {
            signature!(Generic, 4, 4; Pointer(flags), Pointer(buffer), Pointer(interface_out), Pointer(iid))
        }
        "WdtpInterfacePointer_UserFree" | "WdtpInterfacePointer_UserFree64" => {
            signature!(Void, 1, 1; Pointer(interface))
        }
        "BindMoniker" => {
            signature!(Hresult, 4, 4; Pointer(pmk), Dword(grf_opt), Pointer(iid_result), Pointer(ppv_result))
        }
        "CLIPFORMAT_UserFree"
        | "HBRUSH_UserFree"
        | "HBRUSH_UserFree64"
        | "HENHMETAFILE_UserFree"
        | "HENHMETAFILE_UserFree64"
        | "HMETAFILEPICT_UserFree"
        | "HMETAFILEPICT_UserFree64"
        | "HMETAFILE_UserFree"
        | "HMETAFILE_UserFree64"
        | "CLIPFORMAT_UserFree64"
        | "HACCEL_UserFree"
        | "HACCEL_UserFree64"
        | "HBITMAP_UserFree"
        | "HBITMAP_UserFree64"
        | "HDC_UserFree"
        | "HDC_UserFree64"
        | "HGLOBAL_UserFree"
        | "HGLOBAL_UserFree64"
        | "HICON_UserFree"
        | "HICON_UserFree64"
        | "HMENU_UserFree"
        | "HMENU_UserFree64"
        | "HMONITOR_UserFree"
        | "HMONITOR_UserFree64"
        | "HPALETTE_UserFree"
        | "HPALETTE_UserFree64"
        | "HRGN_UserFree"
        | "HWND_UserFree"
        | "HWND_UserFree64"
        | "SNB_UserFree"
        | "SNB_UserFree64"
        | "STGMEDIUM_UserFree"
        | "STGMEDIUM_UserFree64" => signature!(Void, 2, 2; Pointer(flags), Pointer(object)),
        "CLIPFORMAT_UserMarshal"
        | "HBRUSH_UserMarshal"
        | "HBRUSH_UserMarshal64"
        | "HENHMETAFILE_UserMarshal"
        | "HENHMETAFILE_UserMarshal64"
        | "HMETAFILEPICT_UserMarshal"
        | "HMETAFILEPICT_UserMarshal64"
        | "HMETAFILE_UserMarshal"
        | "HMETAFILE_UserMarshal64"
        | "CLIPFORMAT_UserMarshal64"
        | "CLIPFORMAT_UserUnmarshal"
        | "HBRUSH_UserUnmarshal"
        | "HBRUSH_UserUnmarshal64"
        | "HENHMETAFILE_UserUnmarshal"
        | "HENHMETAFILE_UserUnmarshal64"
        | "HMETAFILEPICT_UserUnmarshal"
        | "HMETAFILEPICT_UserUnmarshal64"
        | "HMETAFILE_UserUnmarshal"
        | "HMETAFILE_UserUnmarshal64"
        | "CLIPFORMAT_UserUnmarshal64"
        | "HACCEL_UserMarshal"
        | "HACCEL_UserMarshal64"
        | "HACCEL_UserUnmarshal"
        | "HACCEL_UserUnmarshal64"
        | "HBITMAP_UserMarshal"
        | "HBITMAP_UserMarshal64"
        | "HBITMAP_UserUnmarshal"
        | "HBITMAP_UserUnmarshal64"
        | "HDC_UserMarshal"
        | "HDC_UserMarshal64"
        | "HDC_UserUnmarshal"
        | "HDC_UserUnmarshal64"
        | "HGLOBAL_UserMarshal"
        | "HGLOBAL_UserMarshal64"
        | "HGLOBAL_UserUnmarshal"
        | "HGLOBAL_UserUnmarshal64"
        | "HICON_UserMarshal"
        | "HICON_UserMarshal64"
        | "HICON_UserUnmarshal"
        | "HICON_UserUnmarshal64"
        | "HMENU_UserMarshal"
        | "HMENU_UserMarshal64"
        | "HMENU_UserUnmarshal"
        | "HMENU_UserUnmarshal64"
        | "HMONITOR_UserMarshal"
        | "HMONITOR_UserMarshal64"
        | "HMONITOR_UserUnmarshal"
        | "HMONITOR_UserUnmarshal64"
        | "HPALETTE_UserMarshal"
        | "HPALETTE_UserMarshal64"
        | "HPALETTE_UserUnmarshal"
        | "HPALETTE_UserUnmarshal64"
        | "HRGN_UserMarshal"
        | "HRGN_UserUnmarshal"
        | "HWND_UserMarshal"
        | "HWND_UserMarshal64"
        | "HWND_UserUnmarshal"
        | "HWND_UserUnmarshal64"
        | "SNB_UserMarshal"
        | "SNB_UserMarshal64"
        | "SNB_UserUnmarshal"
        | "SNB_UserUnmarshal64"
        | "STGMEDIUM_UserMarshal"
        | "STGMEDIUM_UserMarshal64"
        | "STGMEDIUM_UserUnmarshal"
        | "STGMEDIUM_UserUnmarshal64" => {
            signature!(Generic, 3, 3; Pointer(flags), Pointer(buffer), Pointer(object))
        }
        "CLIPFORMAT_UserSize"
        | "HBRUSH_UserSize"
        | "HBRUSH_UserSize64"
        | "HENHMETAFILE_UserSize"
        | "HENHMETAFILE_UserSize64"
        | "HMETAFILEPICT_UserSize"
        | "HMETAFILEPICT_UserSize64"
        | "HMETAFILE_UserSize"
        | "HMETAFILE_UserSize64"
        | "CLIPFORMAT_UserSize64"
        | "HACCEL_UserSize"
        | "HACCEL_UserSize64"
        | "HBITMAP_UserSize"
        | "HBITMAP_UserSize64"
        | "HDC_UserSize"
        | "HDC_UserSize64"
        | "HGLOBAL_UserSize"
        | "HGLOBAL_UserSize64"
        | "HICON_UserSize"
        | "HICON_UserSize64"
        | "HMENU_UserSize"
        | "HMENU_UserSize64"
        | "HMONITOR_UserSize"
        | "HMONITOR_UserSize64"
        | "HPALETTE_UserSize"
        | "HPALETTE_UserSize64"
        | "HRGN_UserSize"
        | "HWND_UserSize"
        | "HWND_UserSize64"
        | "SNB_UserSize"
        | "SNB_UserSize64"
        | "STGMEDIUM_UserSize"
        | "STGMEDIUM_UserSize64" => {
            signature!(Generic, 3, 3; Pointer(flags), Dword(starting_offset), Pointer(object))
        }
        "CLSIDFromProgID" | "CLSIDFromProgIDEx" => {
            signature!(Hresult, 2, 2; WideString(lpsz_prog_id), Pointer(lpclsid))
        }
        "CoAddRefServerProcess"
        | "CoBuildVersion"
        | "CoGetCurrentProcess"
        | "CoReleaseServerProcess"
        | "OleBuildVersion" => signature!(Generic, 0, 0; ),
        "CoAllowSetForegroundWindow" => {
            signature!(Hresult, 2, 2; Pointer(p_unk), Pointer(lpv_reserved))
        }
        "CoAllowUnmarshalerCLSID" => signature!(Hresult, 1, 1; Pointer(clsid)),
        "CoCancelCall" => signature!(Hresult, 2, 2; Dword(dw_thread_id), Dword(ul_timeout)),
        "CoCopyProxy" => signature!(Hresult, 2, 2; Pointer(p_proxy), Pointer(pp_copy)),
        "CoCreateFreeThreadedMarshaler" => {
            signature!(Hresult, 2, 2; Pointer(punk_outer), Pointer(ppunk_marshal))
        }
        "CoCreateGuid" | "CoGetCurrentLogicalThreadId" => signature!(Hresult, 1, 1; Pointer(pguid)),
        "CoDecrementMTAUsage" => signature!(Hresult, 1, 1; Pointer(cookie)),
        "CoDisableCallCancellation" | "CoEnableCallCancellation" => {
            signature!(Hresult, 1, 1; Pointer(p_reserved))
        }
        "CoDisconnectContext" => signature!(Hresult, 1, 1; Dword(dw_timeout)),
        "CoDisconnectObject" => signature!(Hresult, 2, 2; Pointer(p_unk), Dword(dw_reserved)),
        "CoDosDateTimeToFileTime" => {
            signature!(Generic, 3, 3; Word(n_dos_date), Word(n_dos_time), Pointer(lp_file_time))
        }
        "CoFileTimeNow" => signature!(Hresult, 1, 1; Pointer(lp_file_time)),
        "CoFileTimeToDosDateTime" => {
            signature!(Generic, 3, 3; Pointer(lp_file_time), Pointer(lp_dos_date), Pointer(lp_dos_time))
        }
        "CoFreeAllLibraries" | "CoFreeUnusedLibraries" | "OleUninitialize" => {
            signature!(Void, 0, 0; )
        }
        "CoFreeLibrary" => signature!(Void, 1, 1; Pointer(h_inst)),
        "CoFreeUnusedLibrariesEx" => {
            signature!(Void, 2, 2; Dword(dw_unload_delay), Dword(dw_reserved))
        }
        "CoGetApartmentType" => {
            signature!(Hresult, 2, 2; Pointer(p_apt_type), Pointer(p_apt_qualifier))
        }
        "CoGetCallContext" => signature!(Hresult, 2, 2; Pointer(riid), Pointer(pp_interface)),
        "CoGetCallerTID" => signature!(Hresult, 1, 1; Pointer(lpdw_tid)),
        "CoGetCancelObject" => {
            signature!(Hresult, 3, 3; Dword(dw_thread_id), Pointer(iid), Pointer(pp_unk))
        }
        "CoGetClassObject" => {
            signature!(Hresult, 5, 5; Pointer(rclsid), Dword(dw_cls_context), Pointer(pv_reserved), Pointer(riid), Pointer(ppv))
        }
        "CoGetContextToken" => signature!(Hresult, 1, 1; Pointer(p_token)),
        "CoGetDefaultContext" => {
            signature!(Hresult, 3, 3; Dword(apt_type), Pointer(riid), Pointer(ppv))
        }
        "CoGetInterfaceAndReleaseStream" => {
            signature!(Hresult, 3, 3; Pointer(p_stm), Pointer(iid), Pointer(ppv))
        }
        "CoGetMalloc" => signature!(Hresult, 2, 2; Dword(dw_mem_context), Pointer(pp_malloc)),
        "CoGetObject" => {
            signature!(Hresult, 4, 4; WideString(psz_name), Pointer(p_bind_options), Pointer(riid), Pointer(ppv))
        }
        "CoGetObjectContext" => signature!(Hresult, 2, 2; Pointer(riid), Pointer(ppv)),
        "CoGetPSClsid" => signature!(Hresult, 2, 2; Pointer(riid), Pointer(p_clsid)),
        "CoGetStandardMarshal" => {
            signature!(Hresult, 6, 6; Pointer(riid), Pointer(p_unk), Dword(dw_dest_context), Pointer(pv_dest_context), Dword(mshlflags), Pointer(pp_marshal))
        }
        "CoGetStdMarshalEx" => {
            signature!(Hresult, 3, 3; Pointer(p_unk_outer), Dword(smexflags), Pointer(pp_unk_inner))
        }
        "CoGetSystemSecurityPermissions" => {
            signature!(Hresult, 2, 2; Dword(com_sd_type), Pointer(pp_sd))
        }
        "CoGetTreatAsClass" | "OleGetAutoConvert" => {
            signature!(Hresult, 2, 2; Pointer(clsid_old), Pointer(p_clsid_new))
        }
        "CoImpersonateClient"
        | "CoResumeClassObjects"
        | "CoRevertToSelf"
        | "CoRevokeMallocSpy"
        | "CoSuspendClassObjects"
        | "CoTestCancel"
        | "OleFlushClipboard" => signature!(Hresult, 0, 0; ),
        "CoIncrementMTAUsage" => signature!(Hresult, 1, 1; Pointer(p_cookie)),
        "CoInitialize" | "OleInitialize" => signature!(Hresult, 1, 1; Pointer(pv_reserved)),
        "CoInitializeSecurity" => {
            signature!(Hresult, 9, 9; Pointer(p_sec_desc), Dword(c_auth_svc), Pointer(as_auth_svc), Pointer(p_reserved1), Dword(dw_authn_level), Dword(dw_imp_level), Pointer(p_auth_list), Dword(dw_capabilities), Pointer(p_reserved3))
        }
        "CoInstall" => {
            signature!(Hresult, 5, 5; Pointer(pbc), Dword(dw_flags), Pointer(p_class_spec), Pointer(p_query), WideString(psz_code_base))
        }
        "CoInvalidateRemoteMachineBindings" => {
            signature!(Hresult, 1, 1; WideString(psz_machine_name))
        }
        "CoIsHandlerConnected" => signature!(Generic, 1, 1; Pointer(p_unk)),
        "CoIsOle1Class" => signature!(Generic, 1, 1; Pointer(rclsid)),
        "CoLoadLibrary" => signature!(Generic, 2, 2; WideString(lpsz_lib_name), Dword(b_auto_free)),
        "CoLockObjectExternal" => {
            signature!(Hresult, 3, 3; Pointer(p_unk), Dword(f_lock), Dword(f_last_unlock_releases))
        }
        "CoMarshalHresult" => signature!(Hresult, 2, 2; Pointer(pstm), Dword(hresult)),
        "CoMarshalInterThreadInterfaceInStream" => {
            signature!(Hresult, 3, 3; Pointer(riid), Pointer(p_unk), Pointer(pp_stm))
        }
        "CoQueryAuthenticationServices" => {
            signature!(Hresult, 2, 2; Pointer(pc_auth_svc), Pointer(as_auth_svc))
        }
        "CoQueryClientBlanket" => {
            signature!(Hresult, 7, 7; Pointer(p_authn_svc), Pointer(p_authz_svc), Pointer(p_server_princ_name), Pointer(p_authn_level), Pointer(p_imp_level), Pointer(p_privs), Pointer(p_capabilities))
        }
        "CoQueryProxyBlanket" => {
            signature!(Hresult, 8, 8; Pointer(p_proxy), Pointer(pw_authn_svc), Pointer(p_authz_svc), Pointer(p_server_princ_name), Pointer(p_authn_level), Pointer(p_imp_level), Pointer(p_auth_info), Pointer(p_capabilites))
        }
        "CoRegisterActivationFilter" => signature!(Hresult, 1, 1; Pointer(p_activation_filter)),
        "CoRegisterChannelHook" => {
            signature!(Hresult, 2, 2; Pointer(extension_uuid), Pointer(p_channel_hook))
        }
        "CoRegisterClassObject" => {
            signature!(Hresult, 5, 5; Pointer(rclsid), Pointer(p_unk), Dword(dw_cls_context), Dword(flags), Pointer(lpdw_register))
        }
        "CoRegisterInitializeSpy" => {
            signature!(Hresult, 2, 2; Pointer(p_spy), Pointer(puli_cookie))
        }
        "CoRegisterMallocSpy" => signature!(Hresult, 1, 1; Pointer(p_malloc_spy)),
        "CoRegisterMessageFilter" => {
            signature!(Hresult, 2, 2; Pointer(lp_message_filter), Pointer(lplp_message_filter))
        }
        "CoRegisterPSClsid" => signature!(Hresult, 2, 2; Pointer(riid), Pointer(rclsid)),
        "CoRegisterSurrogate" => signature!(Hresult, 1, 1; Pointer(p_surrogate)),
        "CoReleaseMarshalData" => signature!(Hresult, 1, 1; Pointer(p_stm)),
        "CoRevokeClassObject" => signature!(Hresult, 1, 1; Dword(dw_register)),
        "CoRevokeInitializeSpy" => signature!(Hresult, 1, 2; Qword(uli_cookie)),
        "CoSetCancelObject" => signature!(Hresult, 1, 1; Pointer(p_unk)),
        "CoSetProxyBlanket" => {
            signature!(Hresult, 8, 8; Pointer(p_proxy), Dword(dw_authn_svc), Dword(dw_authz_svc), WideString(p_server_princ_name), Dword(dw_authn_level), Dword(dw_imp_level), Pointer(p_auth_info), Dword(dw_capabilities))
        }
        "CoSwitchCallContext" => {
            signature!(Hresult, 2, 2; Pointer(p_new_object), Pointer(pp_old_object))
        }
        "CoTreatAsClass" | "OleSetAutoConvert" => {
            signature!(Hresult, 2, 2; Pointer(clsid_old), Pointer(clsid_new))
        }
        "CoUnmarshalHresult" => signature!(Hresult, 2, 2; Pointer(pstm), Pointer(phresult)),
        "CoUnmarshalInterface" => {
            signature!(Hresult, 3, 3; Pointer(p_stm), Pointer(riid), Pointer(ppv))
        }
        "CoWaitForMultipleHandles" | "CoWaitForMultipleObjects" => {
            signature!(Hresult, 5, 5; Dword(dw_flags), Dword(dw_timeout), Dword(c_handles), Pointer(p_handles), Pointer(lpdwindex))
        }
        "CreateAntiMoniker" => signature!(Hresult, 1, 1; Pointer(ppmk)),
        "CreateBindCtx" => signature!(Hresult, 2, 2; Dword(reserved), Pointer(ppbc)),
        "CreateClassMoniker" => signature!(Hresult, 2, 2; Pointer(rclsid), Pointer(ppmk)),
        "CreateDataAdviseHolder" => signature!(Hresult, 1, 1; Pointer(pp_da_holder)),
        "CreateDataCache" => {
            signature!(Hresult, 4, 4; Pointer(p_unk_outer), Pointer(rclsid), Pointer(iid), Pointer(ppv))
        }
        "CreateErrorInfo" => signature!(Hresult, 1, 1; Pointer(pperrinfo)),
        "CreateFileMoniker" => signature!(Hresult, 2, 2; WideString(lpsz_path_name), Pointer(ppmk)),
        "CreateGenericComposite" => {
            signature!(Hresult, 3, 3; Pointer(pmk_first), Pointer(pmk_rest), Pointer(ppmk_composite))
        }
        "CreateILockBytesOnHGlobal" => {
            signature!(Hresult, 3, 3; Pointer(h_global), Dword(f_delete_on_release), Pointer(pplkbyt))
        }
        "CreateItemMoniker" => {
            signature!(Hresult, 3, 3; WideString(lpsz_delim), WideString(lpsz_item), Pointer(ppmk))
        }
        "CreateObjrefMoniker" | "CreatePointerMoniker" => {
            signature!(Hresult, 2, 2; Pointer(punk), Pointer(ppmk))
        }
        "CreateOleAdviseHolder" => signature!(Hresult, 1, 1; Pointer(pp_oa_holder)),
        "CreateStdProgressIndicator" => {
            signature!(Hresult, 4, 4; Pointer(hwnd_parent), WideString(psz_title), Pointer(p_ibsc_caller), Pointer(pp_ibsc))
        }
        "CreateStreamOnHGlobal" => {
            signature!(Hresult, 3, 3; Pointer(h_global), Dword(f_delete_on_release), Pointer(ppstm))
        }
        "DcomChannelSetHResult" => {
            signature!(Hresult, 3, 3; Pointer(pv_reserved), Pointer(pul_reserved), Dword(apps_hr))
        }
        "DoDragDrop" => {
            signature!(Hresult, 4, 4; Pointer(p_data_obj), Pointer(p_drop_source), Dword(dw_ok_effects), Pointer(pdw_effect))
        }
        "FmtIdToPropStgName" => signature!(Hresult, 2, 2; Pointer(pfmtid), Pointer(osz_name)),
        "FreePropVariantArray" => signature!(Hresult, 2, 2; Dword(c_variants), Pointer(rgvars)),
        "GetClassFile" => signature!(Hresult, 2, 2; WideString(sz_filename), Pointer(pclsid)),
        "GetConvertStg" => signature!(Hresult, 1, 1; Pointer(p_stg)),
        "GetErrorInfo" => signature!(Hresult, 2, 2; Dword(dw_reserved), Pointer(pperrinfo)),
        "GetHGlobalFromILockBytes" => signature!(Hresult, 2, 2; Pointer(plkbyt), Pointer(phglobal)),
        "GetHGlobalFromStream" => signature!(Hresult, 2, 2; Pointer(pstm), Pointer(phglobal)),
        "GetRunningObjectTable" => signature!(Hresult, 2, 2; Dword(reserved), Pointer(pprot)),
        "IsAccelerator" => {
            signature!(Generic, 4, 4; Pointer(h_accel), Dword(c_accel_entries), Pointer(lp_msg), Pointer(lpw_cmd))
        }
        "MkParseDisplayName" => {
            signature!(Hresult, 4, 4; Pointer(pbc), WideString(sz_user_name), Pointer(pch_eaten), Pointer(ppmk))
        }
        "MonikerCommonPrefixWith" => {
            signature!(Hresult, 3, 3; Pointer(pmk_this), Pointer(pmk_other), Pointer(ppmk_common))
        }
        "MonikerRelativePathTo" => {
            signature!(Hresult, 4, 4; Pointer(pmk_src), Pointer(pmk_dest), Pointer(ppmk_rel_path), Dword(dw_reserved))
        }
        "OleConvertIStorageToOLESTREAM" => {
            signature!(Hresult, 2, 2; Pointer(pstg), Pointer(lpolestream))
        }
        "OleConvertOLESTREAMToIStorage" => {
            signature!(Hresult, 3, 3; Pointer(lpolestream), Pointer(pstg), Pointer(ptd))
        }
        "OleConvertOLESTREAMToIStorage2" => {
            signature!(Hresult, 6, 6; Pointer(lpolestream), Pointer(pstg), Pointer(ptd), Dword(opt), Pointer(pv_callback_context), Pointer(p_query_convert_ole_link_callback))
        }
        "OleCreate" => {
            signature!(Hresult, 7, 7; Pointer(rclsid), Pointer(riid), Dword(renderopt), Pointer(p_format_etc), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateDefaultHandler" => {
            signature!(Hresult, 4, 4; Pointer(clsid), Pointer(p_unk_outer), Pointer(riid), Pointer(lplp_obj))
        }
        "OleCreateEmbeddingHelper" => {
            signature!(Hresult, 6, 6; Pointer(clsid), Pointer(p_unk_outer), Dword(flags), Pointer(p_cf), Pointer(riid), Pointer(lplp_obj))
        }
        "OleCreateEx" => {
            signature!(Hresult, 12, 12; Pointer(rclsid), Pointer(riid), Dword(dw_flags), Dword(renderopt), Dword(c_formats), Pointer(rg_advf), Pointer(rg_format_etc), Pointer(lp_advise_sink), Pointer(rgdw_connection), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateFromData" | "OleCreateLinkFromData" => {
            signature!(Hresult, 7, 7; Pointer(p_src_data_obj), Pointer(riid), Dword(renderopt), Pointer(p_format_etc), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateFromDataEx" | "OleCreateLinkFromDataEx" => {
            signature!(Hresult, 12, 12; Pointer(p_src_data_obj), Pointer(riid), Dword(dw_flags), Dword(renderopt), Dword(c_formats), Pointer(rg_advf), Pointer(rg_format_etc), Pointer(lp_advise_sink), Pointer(rgdw_connection), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateFromFile" => {
            signature!(Hresult, 8, 8; Pointer(rclsid), Pointer(lpsz_file_name), Pointer(riid), Dword(renderopt), Pointer(lp_format_etc), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateFromFileEx" => {
            signature!(Hresult, 13, 13; Pointer(rclsid), Pointer(lpsz_file_name), Pointer(riid), Dword(dw_flags), Dword(renderopt), Dword(c_formats), Pointer(rg_advf), Pointer(rg_format_etc), Pointer(lp_advise_sink), Pointer(rgdw_connection), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateLink" => {
            signature!(Hresult, 7, 7; Pointer(pmk_link_src), Pointer(riid), Dword(renderopt), Pointer(lp_format_etc), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateLinkEx" => {
            signature!(Hresult, 12, 12; Pointer(pmk_link_src), Pointer(riid), Dword(dw_flags), Dword(renderopt), Dword(c_formats), Pointer(rg_advf), Pointer(rg_format_etc), Pointer(lp_advise_sink), Pointer(rgdw_connection), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateLinkToFile" => {
            signature!(Hresult, 7, 7; Pointer(lpsz_file_name), Pointer(riid), Dword(renderopt), Pointer(lp_format_etc), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateLinkToFileEx" => {
            signature!(Hresult, 12, 12; Pointer(lpsz_file_name), Pointer(riid), Dword(dw_flags), Dword(renderopt), Dword(c_formats), Pointer(rg_advf), Pointer(rg_format_etc), Pointer(lp_advise_sink), Pointer(rgdw_connection), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleCreateMenuDescriptor" => {
            signature!(Generic, 2, 2; Pointer(hmenu_combined), Pointer(lp_menu_widths))
        }
        "OleCreateStaticFromData" => {
            signature!(Hresult, 7, 7; Pointer(p_src_data_obj), Pointer(iid), Dword(renderopt), Pointer(p_format_etc), Pointer(p_client_site), Pointer(p_stg), Pointer(ppv_obj))
        }
        "OleDestroyMenuDescriptor" => signature!(Hresult, 1, 1; Pointer(holemenu)),
        "OleDoAutoConvert" => signature!(Hresult, 2, 2; Pointer(p_stg), Pointer(p_clsid_new)),
        "OleDraw" => {
            signature!(Hresult, 4, 4; Pointer(p_unknown), Dword(dw_aspect), Pointer(hdc_draw), Pointer(lprc_bounds))
        }
        "OleDuplicateData" => {
            signature!(Generic, 3, 3; Pointer(h_src), Word(cf_format), Dword(ui_flags))
        }
        "OleGetClipboard" => signature!(Hresult, 1, 1; Pointer(pp_data_obj)),
        "OleGetClipboardWithEnterpriseInfo" => {
            signature!(Hresult, 5, 5; Pointer(data_object), Pointer(data_enterprise_id), Pointer(source_description), Pointer(target_description), Pointer(data_description))
        }
        "OleGetIconOfClass" => {
            signature!(Generic, 3, 3; Pointer(rclsid), WideString(lpsz_label), Dword(f_use_type_as_label))
        }
        "OleGetIconOfFile" => {
            signature!(Generic, 2, 2; WideString(lpsz_path), Dword(f_use_file_as_label))
        }
        "OleIsCurrentClipboard" | "OleSetClipboard" => {
            signature!(Hresult, 1, 1; Pointer(p_data_obj))
        }
        "OleIsRunning" => signature!(Generic, 1, 1; Pointer(p_object)),
        "OleLoad" => {
            signature!(Hresult, 4, 4; Pointer(p_stg), Pointer(riid), Pointer(p_client_site), Pointer(ppv_obj))
        }
        "OleLoadFromStream" => {
            signature!(Hresult, 3, 3; Pointer(p_stm), Pointer(iid_interface), Pointer(ppv_obj))
        }
        "OleLockRunning" => {
            signature!(Hresult, 3, 3; Pointer(p_unknown), Dword(f_lock), Dword(f_last_unlock_closes))
        }
        "OleMetafilePictFromIconAndLabel" => {
            signature!(Generic, 4, 4; Pointer(h_icon), WideString(lpsz_label), WideString(lpsz_source_file), Dword(i_icon_index))
        }
        "OleNoteObjectVisible" => signature!(Hresult, 2, 2; Pointer(p_unknown), Dword(f_visible)),
        "OleQueryCreateFromData" | "OleQueryLinkFromData" => {
            signature!(Hresult, 1, 1; Pointer(p_src_data_object))
        }
        "OleRegEnumFormatEtc" => {
            signature!(Hresult, 3, 3; Pointer(clsid), Dword(dw_direction), Pointer(ppenum))
        }
        "OleRegEnumVerbs" => signature!(Hresult, 2, 2; Pointer(clsid), Pointer(ppenum)),
        "OleRegGetMiscStatus" => {
            signature!(Hresult, 3, 3; Pointer(clsid), Dword(dw_aspect), Pointer(pdw_status))
        }
        "OleRegGetUserType" => {
            signature!(Hresult, 3, 3; Pointer(clsid), Dword(dw_form_of_type), Pointer(psz_user_type))
        }
        "OleRun" => signature!(Hresult, 1, 1; Pointer(p_unknown)),
        "OleSave" => {
            signature!(Hresult, 3, 3; Pointer(p_ps), Pointer(p_stg), Dword(f_same_as_load))
        }
        "OleSaveToStream" => signature!(Hresult, 2, 2; Pointer(p_p_stm), Pointer(p_stm)),
        "OleSetContainedObject" => {
            signature!(Hresult, 2, 2; Pointer(p_unknown), Dword(f_contained))
        }
        "OleSetMenuDescriptor" => {
            signature!(Hresult, 5, 5; Pointer(holemenu), Pointer(hwnd_frame), Pointer(hwnd_active_object), Pointer(lp_frame), Pointer(lp_active_obj))
        }
        "OleTranslateAccelerator" => {
            signature!(Hresult, 3, 3; Pointer(lp_frame), Pointer(lp_frame_info), Pointer(lpmsg))
        }
        "ProgIDFromCLSID" => signature!(Hresult, 2, 2; Pointer(clsid), Pointer(lplpsz_prog_id)),
        "PropStgNameToFmtId" => signature!(Hresult, 2, 2; WideString(osz_name), Pointer(pfmtid)),
        "PropVariantChangeType" => {
            signature!(Hresult, 4, 4; Pointer(ppropvar_dest), Pointer(propvar_src), Dword(flags), Word(vt))
        }
        "PropVariantClear" => signature!(Hresult, 1, 1; Pointer(pvar)),
        "PropVariantCopy" => signature!(Hresult, 2, 2; Pointer(pvar_dest), Pointer(pvar_src)),
        "ReadClassStg" => signature!(Hresult, 2, 2; Pointer(p_stg), Pointer(pclsid)),
        "ReadClassStm" => signature!(Hresult, 2, 2; Pointer(p_stm), Pointer(pclsid)),
        "ReadFmtUserTypeStg" => {
            signature!(Hresult, 3, 3; Pointer(pstg), Pointer(pcf), Pointer(lplpsz_user_type))
        }
        "RegisterDragDrop" => signature!(Hresult, 2, 2; Pointer(hwnd), Pointer(p_drop_target)),
        "ReleaseStgMedium" => signature!(Void, 1, 1; Pointer(lpstgmedium)),
        "RevokeDragDrop" => signature!(Hresult, 1, 1; Pointer(hwnd)),
        "SetConvertStg" => signature!(Hresult, 2, 2; Pointer(p_stg), Dword(f_convert)),
        "SetErrorInfo" => signature!(Hresult, 2, 2; Dword(dw_reserved), Pointer(perrinfo)),
        "StgConvertPropertyToVariant" => {
            signature!(Generic, 4, 4; Pointer(pprop), Word(code_page), Pointer(pvar), Pointer(pma))
        }
        "StgConvertVariantToProperty" => {
            signature!(Generic, 7, 7; Pointer(pvar), Word(code_page), Pointer(pprop), Pointer(pcb), Dword(pid), Byte(f_reserved), Pointer(pc_indirect))
        }
        "StgCreateDocfile" => {
            signature!(Hresult, 4, 4; WideString(pwcs_name), Dword(grf_mode), Dword(reserved), Pointer(ppstg_open))
        }
        "StgCreateDocfileOnILockBytes" => {
            signature!(Hresult, 4, 4; Pointer(plkbyt), Dword(grf_mode), Dword(reserved), Pointer(ppstg_open))
        }
        "StgCreatePropSetStg" => {
            signature!(Hresult, 3, 3; Pointer(p_storage), Dword(dw_reserved), Pointer(pp_prop_set_stg))
        }
        "StgCreatePropStg" => {
            signature!(Hresult, 6, 6; Pointer(p_unk), Pointer(fmtid), Pointer(pclsid), Dword(grf_flags), Dword(dw_reserved), Pointer(pp_prop_stg))
        }
        "StgCreateStorageEx" | "StgOpenStorageEx" => {
            signature!(Hresult, 8, 8; WideString(pwcs_name), Dword(grf_mode), Dword(stgfmt), Dword(grf_attrs), Pointer(p_stg_options), Pointer(p_security_descriptor), Pointer(riid), Pointer(pp_object_open))
        }
        "StgGetIFillLockBytesOnFile" => {
            signature!(Hresult, 2, 2; WideString(pwcs_name), Pointer(ppflb))
        }
        "StgGetIFillLockBytesOnILockBytes" => {
            signature!(Hresult, 2, 2; Pointer(pilb), Pointer(ppflb))
        }
        "StgIsStorageFile" => signature!(Hresult, 1, 1; WideString(pwcs_name)),
        "StgIsStorageILockBytes" => signature!(Hresult, 1, 1; Pointer(plkbyt)),
        "StgOpenAsyncDocfileOnIFillLockBytes" => {
            signature!(Hresult, 4, 4; Pointer(pflb), Dword(grf_mode), Dword(async_flags), Pointer(ppstg_open))
        }
        "StgOpenPropStg" => {
            signature!(Hresult, 5, 5; Pointer(p_unk), Pointer(fmtid), Dword(grf_flags), Dword(dw_reserved), Pointer(pp_prop_stg))
        }
        "StgOpenStorage" => {
            signature!(Hresult, 6, 6; WideString(pwcs_name), Pointer(pstg_priority), Dword(grf_mode), Pointer(snb_exclude), Dword(reserved), Pointer(ppstg_open))
        }
        "StgOpenStorageOnILockBytes" => {
            signature!(Hresult, 6, 6; Pointer(plkbyt), Pointer(pstg_priority), Dword(grf_mode), Pointer(snb_exclude), Dword(reserved), Pointer(ppstg_open))
        }
        "StgPropertyLengthAsVariant" => {
            signature!(Generic, 4, 4; Pointer(p_prop), Dword(cb_prop), Word(code_page), Byte(b_reserved))
        }
        "StgSetTimes" => {
            signature!(Hresult, 4, 4; WideString(lpsz_name), Pointer(pctime), Pointer(patime), Pointer(pmtime))
        }
        "WriteClassStg" => signature!(Hresult, 2, 2; Pointer(p_stg), Pointer(rclsid)),
        "WriteClassStm" => signature!(Hresult, 2, 2; Pointer(p_stm), Pointer(rclsid)),
        "WriteFmtUserTypeStg" => {
            signature!(Hresult, 3, 3; Pointer(pstg), Word(cf), WideString(lpsz_user_type))
        }
        _ => None,
    }
}
