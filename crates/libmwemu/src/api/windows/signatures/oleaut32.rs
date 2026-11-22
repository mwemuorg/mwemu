use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "BSTR_UserFree"
        | "BSTR_UserFree64"
        | "HWND_UserFree"
        | "HWND_UserFree64"
        | "LPSAFEARRAY_UserFree"
        | "LPSAFEARRAY_UserFree64"
        | "VARIANT_UserFree"
        | "VARIANT_UserFree64" => signature!(Void, 2, 2; Pointer(flags), Pointer(object)),
        "BSTR_UserMarshal"
        | "BSTR_UserMarshal64"
        | "BSTR_UserUnmarshal"
        | "BSTR_UserUnmarshal64"
        | "HWND_UserMarshal"
        | "HWND_UserMarshal64"
        | "HWND_UserUnmarshal"
        | "HWND_UserUnmarshal64"
        | "LPSAFEARRAY_UserMarshal"
        | "LPSAFEARRAY_UserMarshal64"
        | "LPSAFEARRAY_UserUnmarshal"
        | "LPSAFEARRAY_UserUnmarshal64"
        | "VARIANT_UserMarshal"
        | "VARIANT_UserMarshal64"
        | "VARIANT_UserUnmarshal"
        | "VARIANT_UserUnmarshal64" => {
            signature!(Generic, 3, 3; Pointer(flags), Pointer(buffer), Pointer(object))
        }
        "BSTR_UserSize"
        | "BSTR_UserSize64"
        | "HWND_UserSize"
        | "HWND_UserSize64"
        | "LPSAFEARRAY_UserSize"
        | "LPSAFEARRAY_UserSize64"
        | "VARIANT_UserSize"
        | "VARIANT_UserSize64" => {
            signature!(Generic, 3, 3; Pointer(flags), Dword(starting_offset), Pointer(object))
        }
        "ClearCustData" => signature!(Void, 1, 1; Pointer(p_cust_data)),
        "CreateDispTypeInfo" => {
            signature!(Hresult, 3, 3; Pointer(pidata), Dword(lcid), Pointer(pptinfo))
        }
        "CreateErrorInfo" => signature!(Hresult, 1, 1; Pointer(pperrinfo)),
        "CreateStdDispatch" => {
            signature!(Hresult, 4, 4; Pointer(punk_outer), Pointer(pv_this), Pointer(ptinfo), Pointer(ppunk_std_disp))
        }
        "CreateTypeLib" | "CreateTypeLib2" => {
            signature!(Hresult, 3, 3; Dword(syskind), Pointer(sz_file), Pointer(ppctlib))
        }
        "DispInvoke" => {
            signature!(Hresult, 8, 8; Pointer(this), Pointer(ptinfo), Dword(dispid_member), Word(w_flags), Pointer(pparams), Pointer(pvar_result), Pointer(pexcepinfo), Pointer(pu_arg_err))
        }
        "DosDateTimeToVariantTime" => {
            signature!(Generic, 3, 3; Word(w_dos_date), Word(w_dos_time), Pointer(pvtime))
        }
        "GetActiveObject" => {
            signature!(Hresult, 3, 3; Pointer(rclsid), Pointer(pv_reserved), Pointer(ppunk))
        }
        "GetErrorInfo" => signature!(Hresult, 2, 2; Dword(dw_reserved), Pointer(pperrinfo)),
        "LHashValOfNameSys" | "LHashValOfNameSysA" => {
            signature!(Generic, 3, 3; Dword(syskind), Dword(lcid), Pointer(sz_name))
        }
        "LoadRegTypeLib" => {
            signature!(Hresult, 5, 5; Pointer(rguid), Word(w_ver_major), Word(w_ver_minor), Dword(lcid), Pointer(pptlib))
        }
        "LoadTypeLib" => signature!(Hresult, 2, 2; WideString(sz_file), Pointer(pptlib)),
        "LoadTypeLibEx" => {
            signature!(Hresult, 3, 3; Pointer(sz_file), Dword(regkind), Pointer(pptlib))
        }
        "OaBuildVersion" => signature!(Generic, 0, 0; ),
        "OaEnablePerUserTLibRegistration" => signature!(Void, 0, 0; ),
        "OleCreatePropertyFrame" => {
            signature!(Hresult, 11, 11; Pointer(hwnd_owner), Dword(x), Dword(y), Pointer(lpsz_caption), Dword(c_objects), Pointer(pp_unk), Dword(c_pages), Pointer(p_page_cls_id), Dword(lcid), Dword(dw_reserved), Pointer(pv_reserved))
        }
        "OleCreatePropertyFrameIndirect" => signature!(Hresult, 1, 1; Pointer(lp_params)),
        "OleIconToCursor" => signature!(Generic, 2, 2; Pointer(hinst_exe), Pointer(h_icon)),
        "OleSavePictureFile" => signature!(Hresult, 2, 2),
        "RegisterTypeLib" | "RegisterTypeLibForUser" => {
            signature!(Hresult, 3, 3; Pointer(ptlib), WideString(sz_full_path), WideString(sz_help_dir))
        }
        "SafeArrayAccessData" => signature!(Hresult, 2, 2; Pointer(psa), Pointer(ppv_data)),
        "SafeArrayAllocData"
        | "SafeArrayDestroy"
        | "SafeArrayDestroyData"
        | "SafeArrayDestroyDescriptor"
        | "SafeArrayLock"
        | "SafeArrayUnaccessData"
        | "SafeArrayUnlock" => signature!(Hresult, 1, 1; Pointer(psa)),
        "SafeArrayAllocDescriptor" => signature!(Hresult, 2, 2; Dword(c_dims), Pointer(ppsa_out)),
        "SafeArrayAllocDescriptorEx" => {
            signature!(Hresult, 3, 3; Word(vt), Dword(c_dims), Pointer(ppsa_out))
        }
        "SafeArrayCopy" => signature!(Hresult, 2, 2; Pointer(psa), Pointer(ppsa_out)),
        "SafeArrayCopyData" => signature!(Hresult, 2, 2; Pointer(psa_source), Pointer(psa_target)),
        "SafeArrayCreate" => signature!(Generic, 3, 3; Word(vt), Dword(c_dims), Pointer(rgsabound)),
        "SafeArrayCreateEx" => {
            signature!(Generic, 4, 4; Word(vt), Dword(c_dims), Pointer(rgsabound), Pointer(pv_extra))
        }
        "SafeArrayCreateVector" => {
            signature!(Generic, 3, 3; Word(vt), Dword(l_lbound), Dword(c_elements))
        }
        "SafeArrayCreateVectorEx" => {
            signature!(Generic, 4, 4; Word(vt), Dword(l_lbound), Dword(c_elements), Pointer(pv_extra))
        }
        "SafeArrayGetDim" | "SafeArrayGetElemsize" => signature!(Generic, 1, 1; Pointer(psa)),
        "SafeArrayGetElement" | "SafeArrayPutElement" => {
            signature!(Hresult, 3, 3; Pointer(psa), Pointer(rg_indices), Pointer(pv))
        }
        "SafeArrayGetIID" => signature!(Hresult, 2, 2; Pointer(psa), Pointer(pguid)),
        "SafeArrayGetLBound" => {
            signature!(Hresult, 3, 3; Pointer(psa), Dword(n_dim), Pointer(pl_lbound))
        }
        "SafeArrayGetRecordInfo" | "SafeArraySetRecordInfo" => {
            signature!(Hresult, 2, 2; Pointer(psa), Pointer(prinfo))
        }
        "SafeArrayGetUBound" => {
            signature!(Hresult, 3, 3; Pointer(psa), Dword(n_dim), Pointer(pl_ubound))
        }
        "SafeArrayGetVartype" => signature!(Hresult, 2, 2; Pointer(psa), Pointer(pvt)),
        "SafeArrayPtrOfIndex" => {
            signature!(Hresult, 3, 3; Pointer(psa), Pointer(rg_indices), Pointer(ppv_data))
        }
        "SafeArrayReleaseData" => signature!(Void, 1, 1; Pointer(p_data)),
        "SafeArrayReleaseDescriptor" => signature!(Void, 1, 1; Pointer(psa)),
        "SafeArraySetIID" => signature!(Hresult, 2, 2; Pointer(psa), Pointer(guid)),
        "SetErrorInfo" => signature!(Hresult, 2, 2; Dword(dw_reserved), Pointer(perrinfo)),
        "SysAddRefString" => signature!(Hresult, 1, 1),
        "SysReleaseString" => signature!(Void, 1, 1),
        "SystemTimeToVariantTime" => {
            signature!(Generic, 2, 2; Pointer(lp_system_time), Pointer(pvtime))
        }
        "UnRegisterTypeLib" => {
            signature!(Hresult, 5, 5; Pointer(lib_id), Word(w_ver_major), Word(w_ver_minor), Dword(lcid), Dword(syskind))
        }
        "UnRegisterTypeLibForUser" => {
            signature!(Hresult, 5, 5; Pointer(lib_id), Word(w_major_ver_num), Word(w_minor_ver_num), Dword(lcid), Dword(syskind))
        }
        "VarBoolFromCy" | "VarDecFromCy" | "VarI1FromCy" | "VarI2FromCy" | "VarI4FromCy"
        | "VarI8FromCy" | "VarR4FromCy" | "VarR8FromCy" | "VarUI1FromCy" | "VarUI2FromCy"
        | "VarUI4FromCy" | "VarUI8FromCy" => signature!(Hresult, 2, 3),
        "VarBoolFromDec" => signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(pbool_out)),
        "VarBoolFromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pbool_out))
        }
        "VarBoolFromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pbool_out))
        }
        "VarBstrCmp" => signature!(Hresult, 4, 4),
        "VarBstrFromBool" => {
            signature!(Hresult, 4, 4; Word(bool_in), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromCy" => signature!(Hresult, 4, 5),
        "VarBstrFromDec" => {
            signature!(Hresult, 4, 4; Pointer(pdec_in), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromDisp" => {
            signature!(Hresult, 4, 4; Pointer(pdisp_in), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromI1" => {
            signature!(Hresult, 4, 4; Byte(c_in), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromI2" => {
            signature!(Hresult, 4, 4; Word(i_val), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromI4" => {
            signature!(Hresult, 4, 4; Dword(l_in), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromI8" => {
            signature!(Hresult, 4, 5; Qword(i64_in), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromUI1" => {
            signature!(Hresult, 4, 4; Byte(b_val), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromUI2" => {
            signature!(Hresult, 4, 4; Word(ui_in), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromUI4" => {
            signature!(Hresult, 4, 4; Dword(ul_in), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarBstrFromUI8" => {
            signature!(Hresult, 4, 5; Qword(ui64_in), Dword(lcid), Dword(dw_flags), Pointer(pbstr_out))
        }
        "VarCmp" => {
            signature!(Generic, 4, 4; Pointer(pvar_left), Pointer(pvar_right), Dword(lcid), Dword(dw_flags))
        }
        "VarCyCmp" => signature!(Generic, 2, 4),
        "VarDecCmp" => signature!(Generic, 2, 2; Pointer(pdec_left), Pointer(pdec_right)),
        "VarDecFromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pdec_out))
        }
        "VarDecFromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pdec_out))
        }
        "VarI1FromDec" => signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(pc_out)),
        "VarI1FromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pc_out))
        }
        "VarI1FromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pc_out))
        }
        "VarI2FromDec" => signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(ps_out)),
        "VarI2FromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(ps_out))
        }
        "VarI2FromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(ps_out))
        }
        "VarI4FromDec" => signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(pl_out)),
        "VarI4FromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pl_out))
        }
        "VarI4FromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pl_out))
        }
        "VarI8FromDec" | "VarUI8FromDec" => {
            signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(pi64_out))
        }
        "VarI8FromDisp" | "VarUI8FromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pi64_out))
        }
        "VarI8FromStr" | "VarUI8FromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pi64_out))
        }
        "VarR4FromDec" => signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(pflt_out)),
        "VarR4FromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pflt_out))
        }
        "VarR4FromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pflt_out))
        }
        "VarR8FromDec" => signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(pdbl_out)),
        "VarR8FromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pdbl_out))
        }
        "VarR8FromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pdbl_out))
        }
        "VarUI1FromDec" => signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(pb_out)),
        "VarUI1FromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pb_out))
        }
        "VarUI1FromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pb_out))
        }
        "VarUI2FromDec" => signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(pui_out)),
        "VarUI2FromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pui_out))
        }
        "VarUI2FromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pui_out))
        }
        "VarUI4FromDec" => signature!(Hresult, 2, 2; Pointer(pdec_in), Pointer(pul_out)),
        "VarUI4FromDisp" => {
            signature!(Hresult, 3, 3; Pointer(pdisp_in), Dword(lcid), Pointer(pul_out))
        }
        "VarUI4FromStr" => {
            signature!(Hresult, 4, 4; WideString(str_in), Dword(lcid), Dword(dw_flags), Pointer(pul_out))
        }
        "VariantChangeType" => {
            signature!(Hresult, 4, 4; Pointer(pvarg_dest), Pointer(pvar_src), Word(w_flags), Word(vt))
        }
        "VariantChangeTypeEx" => {
            signature!(Hresult, 5, 5; Pointer(pvarg_dest), Pointer(pvar_src), Dword(lcid), Word(w_flags), Word(vt))
        }
        "VariantCopy" => signature!(Hresult, 2, 2; Pointer(pvarg_dest), Pointer(pvarg_src)),
        "VariantCopyInd" => signature!(Hresult, 2, 2; Pointer(pvar_dest), Pointer(pvarg_src)),
        "VarBstrFromDate" => signature!(Hresult, 4, 5),
        "VarBstrFromR4" => signature!(Hresult, 4, 4),
        "VarBstrFromR8" => signature!(Hresult, 4, 5),
        "VarDecFromDate" => signature!(Hresult, 2, 3),
        "VarDecFromR4" => signature!(Hresult, 2, 2),
        "VarDecFromR8" => signature!(Hresult, 2, 3),
        "VarUdateFromDate" => signature!(Hresult, 3, 4),
        _ => None,
    }
}
