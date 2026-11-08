use crate::api::windows::common::fallback;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "SysAllocStringLen" => SysAllocStringLen(emu),
        "SysAllocString" => SysAllocString(emu),
        "SysFreeString" => SysFreeString(emu),
        "SysStringLen" => SysStringLen(emu),
        "SysStringByteLen" => SysStringByteLen(emu),
        "SysReAllocStringLen" => SysReAllocStringLen(emu),
        "VariantInit" => VariantInit(emu),
        "VariantClear" => VariantClear(emu),
        "BSTR_UserFree" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!BSTR_UserFree a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "BSTR_UserFree64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!BSTR_UserFree64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "BSTR_UserMarshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!BSTR_UserMarshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "BSTR_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!BSTR_UserMarshal64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "BSTR_UserSize" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!BSTR_UserSize a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "BSTR_UserSize64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!BSTR_UserSize64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "BSTR_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!BSTR_UserUnmarshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "BSTR_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!BSTR_UserUnmarshal64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "BstrFromVector" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!BstrFromVector a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "ClearCustData" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!ClearCustData a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CreateDispTypeInfo" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!CreateDispTypeInfo a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://CreateDispTypeInfo");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CreateErrorInfo" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!CreateErrorInfo a0:0x{:x}", _a0);
            let _h = helper::handler_create("oleaut32://CreateErrorInfo");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CreateStdDispatch" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!CreateStdDispatch a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://CreateStdDispatch");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "CreateTypeLib" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!CreateTypeLib a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("oleaut32://CreateTypeLib");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CreateTypeLib2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!CreateTypeLib2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("oleaut32://CreateTypeLib2");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "DispCallFunc" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!DispCallFunc a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "DispGetIDsOfNames" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!DispGetIDsOfNames a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "DispGetParam" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!DispGetParam a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "DispInvoke" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!DispInvoke a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DllCanUnloadNow" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!DllCanUnloadNow a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DllGetClassObject" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!DllGetClassObject a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DllRegisterServer" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!DllRegisterServer a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DllUnregisterServer" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!DllUnregisterServer a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DosDateTimeToVariantTime" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!DosDateTimeToVariantTime a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "GetActiveObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!GetActiveObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://GetActiveObject");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "GetAltMonthNames" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!GetAltMonthNames a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "GetErrorInfo" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!GetErrorInfo a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetRecordInfoFromGuids" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!GetRecordInfoFromGuids a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "GetRecordInfoFromTypeInfo" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!GetRecordInfoFromTypeInfo a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "GetVarConversionLocaleSetting" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!GetVarConversionLocaleSetting a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HWND_UserFree" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!HWND_UserFree a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HWND_UserFree64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!HWND_UserFree64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HWND_UserMarshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!HWND_UserMarshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HWND_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!HWND_UserMarshal64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HWND_UserSize" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!HWND_UserSize a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HWND_UserSize64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!HWND_UserSize64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HWND_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!HWND_UserUnmarshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HWND_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!HWND_UserUnmarshal64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "LHashValOfNameSys" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LHashValOfNameSys a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "LHashValOfNameSysA" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!LHashValOfNameSysA a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "LPSAFEARRAY_Marshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_Marshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_Size" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_Size a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_Unmarshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_Unmarshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_UserFree" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_UserFree a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_UserFree64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_UserFree64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_UserMarshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_UserMarshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_UserMarshal64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_UserSize" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_UserSize a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_UserSize64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_UserSize64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_UserUnmarshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LPSAFEARRAY_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!LPSAFEARRAY_UserUnmarshal64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "LoadRegTypeLib" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!LoadRegTypeLib a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("oleaut32://LoadRegTypeLib");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "LoadTypeLib" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!LoadTypeLib a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("oleaut32://LoadTypeLib");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "LoadTypeLibEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!LoadTypeLibEx a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("oleaut32://LoadTypeLibEx");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OACleanup" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OACleanup a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "OACreateTypeLib2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!OACreateTypeLib2 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://OACreateTypeLib2");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OaBuildVersion" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OaBuildVersion a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "OaEnablePerUserTLibRegistration" => {
            let _a0 = arg(emu, 0);
            log_red!(
                emu,
                "oleaut32!OaEnablePerUserTLibRegistration a0:0x{:x}",
                _a0
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "OleCreateFontIndirect" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleCreateFontIndirect a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleCreatePictureIndirect" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleCreatePictureIndirect a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleCreatePropertyFrame" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleCreatePropertyFrame a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleCreatePropertyFrameIndirect" => {
            let _a0 = arg(emu, 0);
            log_red!(
                emu,
                "oleaut32!OleCreatePropertyFrameIndirect a0:0x{:x}",
                _a0
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleIconToCursor" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleIconToCursor a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleLoadPicture" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleLoadPicture a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleLoadPictureEx" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleLoadPictureEx a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleLoadPictureFile" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleLoadPictureFile a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleLoadPictureFileEx" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleLoadPictureFileEx a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleLoadPicturePath" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleLoadPicturePath a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleSavePictureFile" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleSavePictureFile a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleTranslateColor" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!OleTranslateColor a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "QueryPathOfRegTypeLib" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!QueryPathOfRegTypeLib a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "RegisterActiveObject" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!RegisterActiveObject a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "RegisterTypeLib" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!RegisterTypeLib a0:0x{:x}", _a0);
            let _h = helper::handler_create("oleaut32://RegisterTypeLib");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "RegisterTypeLibForUser" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!RegisterTypeLibForUser a0:0x{:x}", _a0);
            let _h = helper::handler_create("oleaut32://RegisterTypeLibForUser");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "RevokeActiveObject" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!RevokeActiveObject a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SafeArrayAccessData" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!SafeArrayAccessData a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SafeArrayAddRef" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayAddRef a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArrayAllocData" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!SafeArrayAllocData a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://SafeArrayAllocData");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SafeArrayAllocDescriptor" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!SafeArrayAllocDescriptor a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://SafeArrayAllocDescriptor");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SafeArrayAllocDescriptorEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!SafeArrayAllocDescriptorEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://SafeArrayAllocDescriptorEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SafeArrayCopy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!SafeArrayCopy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("oleaut32://SafeArrayCopy");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SafeArrayCopyData" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!SafeArrayCopyData a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://SafeArrayCopyData");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SafeArrayCreate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayCreate a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://SafeArrayCreate");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayCreateEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayCreateEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "SafeArrayCreateVector" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayCreateVector a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://SafeArrayCreateVector");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayCreateVectorEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayCreateVectorEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "SafeArrayDestroy" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayDestroy a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArrayDestroyData" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayDestroyData a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArrayDestroyDescriptor" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayDestroyDescriptor a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArrayGetDim" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayGetDim a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayGetElement" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayGetElement a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayGetElemsize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayGetElemsize a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayGetIID" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayGetIID a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayGetLBound" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayGetLBound a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayGetRecordInfo" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayGetRecordInfo a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayGetUBound" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayGetUBound a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayGetVartype" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayGetVartype a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayLock" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayLock a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SafeArrayPtrOfIndex" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayPtrOfIndex a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayPutElement" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!SafeArrayPutElement a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "SafeArrayRedim" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayRedim a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArrayReleaseData" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayReleaseData a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArrayReleaseDescriptor" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayReleaseDescriptor a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArraySetIID" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArraySetIID a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArraySetRecordInfo" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArraySetRecordInfo a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArrayUnaccessData" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayUnaccessData a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "SafeArrayUnlock" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SafeArrayUnlock a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SetErrorInfo" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SetErrorInfo a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SetOaNoCache" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SetOaNoCache a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SetVarConversionLocaleSetting" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SetVarConversionLocaleSetting a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SysAddRefString" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SysAddRefString a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SysAllocStringByteLen" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!SysAllocStringByteLen a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SysReAllocString" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!SysReAllocString a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("oleaut32://SysReAllocString");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SysReleaseString" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SysReleaseString a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SystemTimeToVariantTime" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!SystemTimeToVariantTime a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "UnRegisterTypeLib" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!UnRegisterTypeLib a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "UnRegisterTypeLibForUser" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!UnRegisterTypeLibForUser a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VARIANT_UserFree" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VARIANT_UserFree a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VARIANT_UserFree64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VARIANT_UserFree64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VARIANT_UserMarshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VARIANT_UserMarshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VARIANT_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VARIANT_UserMarshal64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VARIANT_UserSize" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VARIANT_UserSize a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VARIANT_UserSize64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VARIANT_UserSize64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VARIANT_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VARIANT_UserUnmarshal a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VARIANT_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VARIANT_UserUnmarshal64 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarAbs" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarAbs a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarAdd" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarAdd a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarAnd" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarAnd a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarBoolFromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!VarBoolFromDate a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!VarBoolFromDisp a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBoolFromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarBoolFromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarBstrCat" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarBstrCat a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarBstrCmp" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarBstrCmp a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarBstrFromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!VarBstrFromBool a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!VarBstrFromDate a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!VarBstrFromDisp a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarBstrFromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarBstrFromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarCat" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCat a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarCmp" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCmp a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarCyAbs" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyAbs a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyAdd" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyAdd a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyCmp" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyCmp a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyCmpR8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyCmpR8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFix" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFix a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromBool" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromBool a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromDate" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromDate a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromDec" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromDec a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromDisp" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromDisp a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromI1" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromI1 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromI2" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromI2 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromI4" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromI4 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromI8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromI8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromR4" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromR4 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromR8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromR8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromStr" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromStr a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromUI1" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromUI1 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromUI2" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromUI2 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromUI4" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromUI4 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyFromUI8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyFromUI8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyInt" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyInt a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyMul" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyMul a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyMulI4" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyMulI4 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyMulI8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyMulI8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyNeg" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyNeg a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCyRound" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCyRound a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarCySub" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarCySub a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromBool" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromBool a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromCy" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromCy a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromDec" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromDec a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromDisp" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromDisp a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromI1" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromI1 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromI2" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromI2 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromI4" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromI4 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromI8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromI8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromR4" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromR4 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromR8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromR8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromStr" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromStr a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromUI1" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromUI1 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromUI2" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromUI2 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromUI4" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromUI4 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromUI8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromUI8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromUdate" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromUdate a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDateFromUdateEx" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDateFromUdateEx a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "VarDecAbs" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecAbs a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecAdd" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecAdd a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecCmp" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecCmp a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecCmpR8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecCmpR8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecDiv" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecDiv a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecFix" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecFix a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecFromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecFromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarDecFromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarDecInt" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecInt a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecMul" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecMul a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecNeg" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecNeg a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecRound" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecRound a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDecSub" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDecSub a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarDiv" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarDiv a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarEqv" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarEqv a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarFix" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarFix a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarFormat" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarFormat a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarFormatCurrency" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarFormatCurrency a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarFormatDateTime" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarFormatDateTime a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarFormatFromTokens" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarFormatFromTokens a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarFormatNumber" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarFormatNumber a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarFormatPercent" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarFormatPercent a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarI1FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI1FromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI1FromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI2FromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI2FromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarI4FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarI4FromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI4FromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI4FromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarI8FromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarI8FromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarIdiv" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarIdiv a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarImp" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarImp a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarInt" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarInt a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarMod" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarMod a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarMonthName" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarMonthName a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarMul" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarMul a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarNeg" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarNeg a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarNot" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarNot a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarNumFromParseNum" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarNumFromParseNum a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarOr" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarOr a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarParseNumFromStr" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarParseNumFromStr a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarPow" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarPow a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarR4CmpR8" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarR4CmpR8 a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarR4FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR4FromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR4FromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VarR8FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VarR8FromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8FromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarR8FromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarR8Pow" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarR8Pow a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarR8Round" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarR8Round a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarRound" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarRound a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarSub" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarSub a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarTokenizeFormatString" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarTokenizeFormatString a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarUI1FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI1FromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI1FromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI2FromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI2FromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI4FromUI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI4FromUI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromBool" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromCy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromCy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromDate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromDate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromDec" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromDec a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromDisp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromDisp a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromI8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromI8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromR4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromR4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromR8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromR8 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromStr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromStr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromUI1" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromUI1 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromUI2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromUI2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUI8FromUI4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VarUI8FromUI4 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VarUdateFromDate" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarUdateFromDate a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarWeekdayName" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarWeekdayName a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VarXor" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "oleaut32!VarXor a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VariantChangeType" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!VariantChangeType a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "VariantChangeTypeEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!VariantChangeTypeEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 5);
        }
        "VariantCopy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VariantCopy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VariantCopyInd" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "oleaut32!VariantCopyInd a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("oleaut32://VariantCopyInd");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VariantTimeToDosDateTime" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "oleaut32!VariantTimeToDosDateTime a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "VariantTimeToSystemTime" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "oleaut32!VariantTimeToSystemTime a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "VectorFromBstr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "oleaut32!VectorFromBstr a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }
    String::new()
}

fn SysAllocStringLen(emu: &mut emu::Emu) {
    let str_ptr = arg(emu, 0) as u64;
    let mut size = arg(emu, 1) as u64;

    if size == 0xffffffff {
        size = 1024;
    }
    let alloc_size = size * 2 + 2 + 8;

    let base = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            log::error!("oleaut32!SysAllocStringLen out of memory");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
            return;
        }
    };
    let name = format!("alloc_{:x}", base + 8);
    emu.maps
        .create_map(&name, base, alloc_size, Permission::READ_WRITE);
    emu.maps.write_dword(base, (size * 2) as u32);
    if str_ptr != 0 {
        emu.maps.memcpy(base + 8, str_ptr, (size * 2) as usize);
    }

    log_red!(emu, "oleaut32!SysAllocStringLen ={} chars:{}", name, size);

    emu.regs_mut().rax = base + 8;
    stdcall_ret(emu, 2);
}

fn SysAllocString(emu: &mut emu::Emu) {
    let str_ptr = arg(emu, 0) as u64;

    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    let byte_len = (s.len() * 2) as u64;
    let alloc_size = byte_len + 2 + 8;

    let base = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            log::error!("oleaut32!SysAllocString out of memory");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
            return;
        }
    };
    let name = format!("alloc_{:x}", base + 8);
    emu.maps
        .create_map(&name, base, alloc_size, Permission::READ_WRITE);
    emu.maps.write_dword(base, byte_len as u32);
    if str_ptr != 0 {
        emu.maps.memcpy(base + 8, str_ptr, byte_len as usize);
    }

    log_red!(emu, "oleaut32!SysAllocString \'{}\'  ={}", s, name);

    emu.regs_mut().rax = base + 8;
    stdcall_ret(emu, 1);
}

fn SysFreeString(emu: &mut emu::Emu) {
    let str_ptr = arg(emu, 0) as u64;

    log_red!(emu, "oleaut32!SysFreeString 0x{:x}", str_ptr);

    stdcall_ret(emu, 1);
}

fn SysStringLen(emu: &mut emu::Emu) {
    let bstr = arg(emu, 0) as u64;

    let len = if bstr >= 8 {
        let byte_len = emu.maps.read_dword(bstr - 4).unwrap_or(0);
        byte_len / 2
    } else {
        0
    };

    log_red!(emu, "oleaut32!SysStringLen 0x{:x} = {}", bstr, len);

    emu.regs_mut().rax = len as u64;
    stdcall_ret(emu, 1);
}

fn SysStringByteLen(emu: &mut emu::Emu) {
    let bstr = arg(emu, 0) as u64;

    let len = if bstr >= 8 {
        emu.maps.read_dword(bstr - 4).unwrap_or(0)
    } else {
        0
    };

    log_red!(emu, "oleaut32!SysStringByteLen 0x{:x} = {}", bstr, len);

    emu.regs_mut().rax = len as u64;
    stdcall_ret(emu, 1);
}

fn SysReAllocStringLen(emu: &mut emu::Emu) {
    let bstr_ptr = arg(emu, 0) as u64;
    let new_str = arg(emu, 1) as u64;
    let new_len = arg(emu, 2) as u64;

    log_red!(
        emu,
        "oleaut32!SysReAllocStringLen ptr:0x{:x} chars:{}",
        bstr_ptr,
        new_len
    );

    let alloc_size = new_len * 2 + 2 + 8;
    let base = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
            return;
        }
    };
    let name = format!("alloc_{:x}", base + 8);
    emu.maps
        .create_map(&name, base, alloc_size, Permission::READ_WRITE);
    emu.maps.write_dword(base, (new_len * 2) as u32);
    if new_str != 0 {
        emu.maps.memcpy(base + 8, new_str, (new_len * 2) as usize);
    }
    if bstr_ptr != 0 {
        emu.maps.write_dword(bstr_ptr, (base + 8) as u32);
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn VariantInit(emu: &mut emu::Emu) {
    let variant_ptr = arg(emu, 0) as u64;

    log_red!(emu, "oleaut32!VariantInit ptr:0x{:x}", variant_ptr);

    if variant_ptr != 0 {
        for i in 0..4u64 {
            emu.maps.write_dword(variant_ptr + i * 4, 0);
        }
    }

    stdcall_ret(emu, 1);
}

fn VariantClear(emu: &mut emu::Emu) {
    let variant_ptr = arg(emu, 0) as u64;

    log_red!(emu, "oleaut32!VariantClear ptr:0x{:x}", variant_ptr);

    if variant_ptr != 0 {
        for i in 0..4u64 {
            emu.maps.write_dword(variant_ptr + i * 4, 0);
        }
    }

    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
