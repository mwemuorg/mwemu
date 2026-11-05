// This module is unwrap/expect-free: fallible ops on guest data must degrade
// gracefully (log + return / default), never panic. `unimplemented!`/`todo!`
// stay — they're deliberate coverage markers.
#![deny(clippy::unwrap_used, clippy::expect_used)]

use crate::emu;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32;
use crate::windows::constants;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "SysAllocStringLen" => SysAllocStringLen(emu),
        "SysReAllocStringLen" => SysReAllocStringLen(emu),
        "SysFreeString" => SysFreeString(emu),
        "VariantClear" => VariantClear(emu),
        "BSTR_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!BSTR_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!BSTR_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!BSTR_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!BSTR_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!BSTR_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!BSTR_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!BSTR_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!BSTR_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BstrFromVector" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!BstrFromVector arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClearCustData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!ClearCustData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateDispTypeInfo" => {
            let pidata = emu.regs().rcx;
            let lcid = emu.regs().rdx;
            let pptinfo = emu.regs().r8;
            let hndl = helper::handler_create("oleaut32://ITypeInfo");
            if pptinfo != 0 {
                emu.maps.write_qword(pptinfo, hndl);
            }
            log_red!(emu, "oleaut32!CreateDispTypeInfo =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateErrorInfo" => {
            let pp_errinfo = emu.regs().rcx;
            let hndl = helper::handler_create("oleaut32://ICreateErrorInfo");
            if pp_errinfo != 0 {
                emu.maps.write_qword(pp_errinfo, hndl);
            }
            log_red!(emu, "oleaut32!CreateErrorInfo =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateStdDispatch" => {
            let punk_outer = emu.regs().rcx;
            let pv_this = emu.regs().rdx;
            let ptinfo = emu.regs().r8;
            let ppunk_std_disp = emu.regs().r9;
            let hndl = helper::handler_create("oleaut32://IDispatch");
            if ppunk_std_disp != 0 {
                emu.maps.write_qword(ppunk_std_disp, hndl);
            }
            log_red!(emu, "oleaut32!CreateStdDispatch =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateTypeLib" => {
            let sys_kind = emu.regs().rcx;
            let sz_file = emu.regs().rdx;
            let pp_ctlib = emu.regs().r8;
            let hndl = helper::handler_create("oleaut32://ICreateTypeLib");
            if pp_ctlib != 0 {
                emu.maps.write_qword(pp_ctlib, hndl);
            }
            log_red!(emu, "oleaut32!CreateTypeLib =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateTypeLib2" => {
            let sys_kind = emu.regs().rcx;
            let sz_file = emu.regs().rdx;
            let pp_ctlib = emu.regs().r8;
            let hndl = helper::handler_create("oleaut32://ICreateTypeLib2");
            if pp_ctlib != 0 {
                emu.maps.write_qword(pp_ctlib, hndl);
            }
            log_red!(emu, "oleaut32!CreateTypeLib2 =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "DispCallFunc" => {
            let punk = emu.regs().rcx;
            log_red!(emu, "oleaut32!DispCallFunc 0x{:x}", punk);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "DispGetIDsOfNames" => {
            let punk = emu.regs().rcx;
            log_red!(emu, "oleaut32!DispGetIDsOfNames 0x{:x}", punk);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "DispGetParam" => {
            let punk = emu.regs().rcx;
            log_red!(emu, "oleaut32!DispGetParam 0x{:x}", punk);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "DispInvoke" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "oleaut32!DispInvoke 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllCanUnloadNow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!DllCanUnloadNow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllGetClassObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!DllGetClassObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllRegisterServer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!DllRegisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllUnregisterServer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!DllUnregisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DosDateTimeToVariantTime" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!DosDateTimeToVariantTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetActiveObject" => {
            let rclsid = emu.regs().rcx;
            let pv_reserved = emu.regs().rdx;
            let ppunk = emu.regs().r8;
            let hndl = helper::handler_create("oleaut32://IUnknown");
            if ppunk != 0 {
                emu.maps.write_qword(ppunk, hndl);
            }
            log_red!(emu, "oleaut32!GetActiveObject =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "GetAltMonthNames" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!GetAltMonthNames h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetErrorInfo" => {
            let reserved = emu.regs().rcx;
            let pperrinfo = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!GetErrorInfo reserved: {} pperrinfo: 0x{:x}",
                reserved,
                pperrinfo
            );
            if pperrinfo != 0 {
                emu.maps.write_qword(pperrinfo, 0); // no error info
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // S_FALSE (no error info available)
        }
        "GetRecordInfoFromGuids" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!GetRecordInfoFromGuids h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRecordInfoFromTypeInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!GetRecordInfoFromTypeInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetVarConversionLocaleSetting" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!GetVarConversionLocaleSetting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!HWND_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!HWND_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!HWND_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!HWND_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!HWND_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!HWND_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!HWND_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!HWND_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LHashValOfNameSys" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LHashValOfNameSys arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LHashValOfNameSysA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "oleaut32!LHashValOfNameSysA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LPSAFEARRAY_Marshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_Marshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_Size" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_Size arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_Unmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_Unmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LoadRegTypeLib" => {
            let rguid = emu.regs().rcx;
            let w_ver_major = emu.regs().rdx as u16;
            let w_ver_minor = emu.regs().r8 as u16;
            let lcid = emu.regs().r9 as u32;
            let pptlib = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("oleaut32://ITypeLib");
            if pptlib != 0 {
                emu.maps.write_qword(pptlib, hndl);
            }
            log_red!(emu, "oleaut32!LoadRegTypeLib =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "LoadTypeLib" => {
            let sz_file = emu.regs().rcx;
            let pptlib = emu.regs().rdx;
            let hndl = helper::handler_create("oleaut32://ITypeLib");
            if pptlib != 0 {
                emu.maps.write_qword(pptlib, hndl);
            }
            let path = emu.maps.read_wide_string(sz_file);
            log_red!(emu, "oleaut32!LoadTypeLib '{}' =0x{:x}", path, hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "LoadTypeLibEx" => {
            let sz_file = emu.regs().rcx;
            let reg_kind = emu.regs().rdx;
            let pptlib = emu.regs().r8;
            let hndl = helper::handler_create("oleaut32://ITypeLib");
            if pptlib != 0 {
                emu.maps.write_qword(pptlib, hndl);
            }
            let path = emu.maps.read_wide_string(sz_file);
            log_red!(emu, "oleaut32!LoadTypeLibEx '{}' =0x{:x}", path, hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "OACleanup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OACleanup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OACreateTypeLib2" => {
            let sys_kind = emu.regs().rcx;
            let sz_file = emu.regs().rdx;
            let pp_ctlib = emu.regs().r8;
            let hndl = helper::handler_create("oleaut32://ICreateTypeLib2");
            if pp_ctlib != 0 {
                emu.maps.write_qword(pp_ctlib, hndl);
            }
            log_red!(emu, "oleaut32!OACreateTypeLib2 =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "OaBuildVersion" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OaBuildVersion arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OaEnablePerUserTLibRegistration" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OaEnablePerUserTLibRegistration arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OleCreateFontIndirect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleCreateFontIndirect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleCreatePictureIndirect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleCreatePictureIndirect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleCreatePropertyFrame" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleCreatePropertyFrame arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleCreatePropertyFrameIndirect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleCreatePropertyFrameIndirect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleIconToCursor" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleIconToCursor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPicture" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleLoadPicture arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPictureEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleLoadPictureEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPictureFile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleLoadPictureFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPictureFileEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleLoadPictureFileEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPicturePath" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleLoadPicturePath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleSavePictureFile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleSavePictureFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleTranslateColor" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!OleTranslateColor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "QueryPathOfRegTypeLib" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!QueryPathOfRegTypeLib h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterActiveObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!RegisterActiveObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTypeLib" => {
            let path_ptr = emu.regs().rcx;
            let path = emu.maps.read_string(path_ptr);
            let hndl = helper::handler_create(&format!("typelib_{}", path));
            log_red!(emu, "oleaut32!RegisterTypeLib '{}'  => 0x{:x}", path, hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "RegisterTypeLibForUser" => {
            let path_ptr = emu.regs().rcx;
            let path = emu.maps.read_string(path_ptr);
            let hndl = helper::handler_create(&format!("typelib_{}", path));
            log_red!(
                emu,
                "oleaut32!RegisterTypeLibForUser '{}'  => 0x{:x}",
                path,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "RevokeActiveObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!RevokeActiveObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SafeArrayAccessData" => {
            let sa = emu.regs().rcx;
            let data_ptr = emu.regs().rdx;
            if data_ptr != 0 {
                emu.maps.write_qword(data_ptr, sa + 0x20);
            }
            log_red!(emu, "oleaut32!SafeArrayAccessData 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayAddRef" => {
            let sa = emu.regs().rcx;
            log_red!(emu, "oleaut32!SafeArrayAddRef 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayAllocData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SafeArrayAllocData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("safearrayallocdata");
            emu.regs_mut().rax = handle;
        }
        "SafeArrayAllocDescriptor" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SafeArrayAllocDescriptor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("safearrayallocdescriptor");
            emu.regs_mut().rax = handle;
        }
        "SafeArrayAllocDescriptorEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SafeArrayAllocDescriptorEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("safearrayallocdescriptorex");
            emu.regs_mut().rax = handle;
        }
        "SafeArrayCopy" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SafeArrayCopy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("safearraycopy");
            emu.regs_mut().rax = handle;
        }
        "SafeArrayCopyData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SafeArrayCopyData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("safearraycopydata");
            emu.regs_mut().rax = handle;
        }
        "SafeArrayCreate" => {
            let vt = emu.regs().rcx;
            let dims = emu.regs().rdx;
            let bounds = emu.regs().r8;
            let hndl = helper::handler_create(&format!("safearray_vt{}", vt));
            log_red!(
                emu,
                "oleaut32!SafeArrayCreate vt:{} dims:{} => 0x{:x}",
                vt,
                dims,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "SafeArrayCreateEx" => {
            let vt = emu.regs().rcx as u16;
            let dims = emu.regs().rdx as u32;
            let rgsabound = emu.regs().r8;
            let pvextra = emu.regs().r9;
            let elem_size: u64 = match vt {
                8 => 8,   // VT_BSTR
                12 => 16, // VT_VARIANT
                _ => 4,
            };
            let total_elements = if rgsabound != 0 && dims > 0 {
                emu.maps.read_dword(rgsabound).unwrap_or(1) as u64
            } else {
                1
            };
            let data_size = total_elements * elem_size;
            let alloc_size = 32 + data_size;
            let sa = match emu.maps.alloc(alloc_size) {
                Some(a) => a,
                None => {
                    emu.set_last_error(0);
                    emu.regs_mut().rax = 0;
                    return String::new();
                }
            };
            emu.maps.create_map(
                &format!("safearray_{:x}", sa),
                sa,
                alloc_size,
                Permission::READ_WRITE,
            );
            emu.maps.write_word(sa, dims as u16);
            emu.maps.write_word(sa + 2, 0);
            emu.maps.write_dword(sa + 4, elem_size as u32);
            emu.maps.write_dword(sa + 8, 0);
            emu.maps.write_qword(sa + 16, sa + 32);
            if rgsabound != 0 && dims > 0 {
                let count = emu.maps.read_dword(rgsabound).unwrap_or(0);
                let lbound = emu.maps.read_dword(rgsabound + 4).unwrap_or(0);
                emu.maps.write_dword(sa + 24, count);
                emu.maps.write_dword(sa + 28, lbound);
            }
            log_red!(
                emu,
                "oleaut32!SafeArrayCreateEx vt:{} dims:{} =0x{:x}",
                vt,
                dims,
                sa
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = sa;
        }
        "SafeArrayCreateVector" => {
            let vt = emu.regs().rcx;
            let lower = emu.regs().rdx;
            let count = emu.regs().r8;
            let hndl = helper::handler_create(&format!("safearray_vec_{}", count));
            log_red!(
                emu,
                "oleaut32!SafeArrayCreateVector vt:{} lower:{} count:{} => 0x{:x}",
                vt,
                lower,
                count,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "SafeArrayCreateVectorEx" => {
            let vt = emu.regs().rcx as u16;
            let lbound = emu.regs().rdx as u32;
            let celements = emu.regs().r8 as u32;
            let pvextra = emu.regs().r9;
            let elem_size: u64 = match vt {
                8 => 8,
                12 => 16,
                _ => 4,
            };
            let data_size = celements as u64 * elem_size;
            let alloc_size = 32 + data_size;
            let sa = match emu.maps.alloc(alloc_size) {
                Some(a) => a,
                None => {
                    emu.set_last_error(0);
                    emu.regs_mut().rax = 0;
                    return String::new();
                }
            };
            emu.maps.create_map(
                &format!("safearray_{:x}", sa),
                sa,
                alloc_size,
                Permission::READ_WRITE,
            );
            emu.maps.write_word(sa, 1);
            emu.maps.write_word(sa + 2, 0);
            emu.maps.write_dword(sa + 4, elem_size as u32);
            emu.maps.write_dword(sa + 8, 0);
            emu.maps.write_qword(sa + 16, sa + 32);
            emu.maps.write_dword(sa + 24, celements);
            emu.maps.write_dword(sa + 28, lbound);
            log_red!(
                emu,
                "oleaut32!SafeArrayCreateVectorEx vt:{} count:{} =0x{:x}",
                vt,
                celements,
                sa
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = sa;
        }
        "SafeArrayDestroy" => {
            let sa = emu.regs().rcx;
            helper::handler_close(sa);
            log_red!(emu, "oleaut32!SafeArrayDestroy 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayDestroyData" => {
            let sa = emu.regs().rcx;
            helper::handler_close(sa);
            log_red!(emu, "oleaut32!SafeArrayDestroyData 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayDestroyDescriptor" => {
            let sa = emu.regs().rcx;
            helper::handler_close(sa);
            log_red!(emu, "oleaut32!SafeArrayDestroyDescriptor 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayGetDim" => {
            let sa_ptr = emu.regs().rcx;
            let index = emu.regs().rdx;
            let out_ptr = emu.regs().r8;
            log_red!(
                emu,
                "oleaut32!SafeArrayGetDim sa:0x{:x} idx:0x{:x}",
                sa_ptr,
                index
            );
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayGetElement" => {
            let sa_ptr = emu.regs().rcx;
            let index = emu.regs().rdx;
            let out_ptr = emu.regs().r8;
            log_red!(
                emu,
                "oleaut32!SafeArrayGetElement sa:0x{:x} idx:0x{:x}",
                sa_ptr,
                index
            );
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayGetElemsize" => {
            let sa_ptr = emu.regs().rcx;
            let index = emu.regs().rdx;
            let out_ptr = emu.regs().r8;
            log_red!(
                emu,
                "oleaut32!SafeArrayGetElemsize sa:0x{:x} idx:0x{:x}",
                sa_ptr,
                index
            );
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayGetIID" => {
            let sa_ptr = emu.regs().rcx;
            let index = emu.regs().rdx;
            let out_ptr = emu.regs().r8;
            log_red!(
                emu,
                "oleaut32!SafeArrayGetIID sa:0x{:x} idx:0x{:x}",
                sa_ptr,
                index
            );
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayGetLBound" => {
            let psa = emu.regs().rcx;
            let ndim = emu.regs().rdx as u32;
            let plbound = emu.regs().r8;
            log_red!(
                emu,
                "oleaut32!SafeArrayGetLBound psa: 0x{:x} dim: {}",
                psa,
                ndim
            );
            if psa != 0 && plbound != 0 {
                let lbound = emu.maps.read_dword(psa + 28).unwrap_or(0);
                emu.maps.write_dword(plbound, lbound);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "SafeArrayGetRecordInfo" => {
            let sa_ptr = emu.regs().rcx;
            let index = emu.regs().rdx;
            let out_ptr = emu.regs().r8;
            log_red!(
                emu,
                "oleaut32!SafeArrayGetRecordInfo sa:0x{:x} idx:0x{:x}",
                sa_ptr,
                index
            );
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayGetUBound" => {
            let psa = emu.regs().rcx;
            let ndim = emu.regs().rdx as u32;
            let pubound = emu.regs().r8;
            log_red!(
                emu,
                "oleaut32!SafeArrayGetUBound psa: 0x{:x} dim: {}",
                psa,
                ndim
            );
            if psa != 0 && pubound != 0 {
                let count = emu.maps.read_dword(psa + 24).unwrap_or(0);
                let lbound = emu.maps.read_dword(psa + 28).unwrap_or(0);
                let ubound = if count > 0 {
                    lbound + count - 1
                } else {
                    lbound
                };
                emu.maps.write_dword(pubound, ubound);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "SafeArrayGetVartype" => {
            let sa_ptr = emu.regs().rcx;
            let index = emu.regs().rdx;
            let out_ptr = emu.regs().r8;
            log_red!(
                emu,
                "oleaut32!SafeArrayGetVartype sa:0x{:x} idx:0x{:x}",
                sa_ptr,
                index
            );
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayLock" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "oleaut32!SafeArrayLock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SafeArrayPtrOfIndex" => {
            let sa_ptr = emu.regs().rcx;
            let index = emu.regs().rdx;
            let out_ptr = emu.regs().r8;
            log_red!(
                emu,
                "oleaut32!SafeArrayPtrOfIndex sa:0x{:x} idx:0x{:x}",
                sa_ptr,
                index
            );
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayPutElement" => {
            let sa_ptr = emu.regs().rcx;
            let rgindices = emu.regs().rdx;
            let pv = emu.regs().r8;
            if sa_ptr != 0 && rgindices != 0 && pv != 0 {
                let elem_size = emu.maps.read_dword(sa_ptr + 4).unwrap_or(4) as u64;
                let pv_data = emu.maps.read_qword(sa_ptr + 16).unwrap_or(0);
                let lbound = emu.maps.read_dword(sa_ptr + 28).unwrap_or(0);
                let index = emu.maps.read_dword(rgindices).unwrap_or(0);
                let offset = (index.wrapping_sub(lbound)) as u64 * elem_size;
                if pv_data != 0 && elem_size <= 16 {
                    let data = emu.maps.read_bytes(pv, elem_size as usize).to_vec();
                    emu.maps.write_bytes(pv_data + offset, &data);
                }
            }
            log_red!(
                emu,
                "oleaut32!SafeArrayPutElement sa:0x{:x} idx_ptr:0x{:x} pv:0x{:x}",
                sa_ptr,
                rgindices,
                pv
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayRedim" => {
            let sa = emu.regs().rcx;
            log_red!(emu, "oleaut32!SafeArrayRedim 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayReleaseData" => {
            let sa = emu.regs().rcx;
            log_red!(emu, "oleaut32!SafeArrayReleaseData 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayReleaseDescriptor" => {
            let sa = emu.regs().rcx;
            log_red!(emu, "oleaut32!SafeArrayReleaseDescriptor 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArraySetIID" => {
            let sa = emu.regs().rcx;
            log_red!(emu, "oleaut32!SafeArraySetIID 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArraySetRecordInfo" => {
            let sa = emu.regs().rcx;
            log_red!(emu, "oleaut32!SafeArraySetRecordInfo 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayUnaccessData" => {
            let sa = emu.regs().rcx;
            log_red!(emu, "oleaut32!SafeArrayUnaccessData 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SafeArrayUnlock" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "oleaut32!SafeArrayUnlock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetErrorInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SetErrorInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetOaNoCache" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SetOaNoCache h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetVarConversionLocaleSetting" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SetVarConversionLocaleSetting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SysAddRefString" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SysAddRefString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SysAllocString" => SysAllocString(emu),
        "SysAllocStringByteLen" => SysAllocStringByteLen(emu),
        "SysReAllocString" => {
            let bstr_ptr = emu.regs().rcx;
            let str_ptr = emu.regs().rdx;
            let s = emu.maps.read_wide_string(str_ptr);
            let hndl = helper::handler_create(&format!("bstr_{}", s));
            if bstr_ptr != 0 {
                emu.maps.write_qword(bstr_ptr, hndl);
            }
            log_red!(emu, "oleaut32!SysReAllocString '{}' => 0x{:x}", s, hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SysReleaseString" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SysReleaseString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SysStringByteLen" => SysStringByteLen(emu),
        "SysStringLen" => SysStringLen(emu),
        "SystemTimeToVariantTime" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!SystemTimeToVariantTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnRegisterTypeLib" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!UnRegisterTypeLib arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnRegisterTypeLibForUser" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!UnRegisterTypeLibForUser arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarAbs" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarAbs arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarAdd" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarAdd arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarAnd" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarAnd arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarBoolFromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarBoolFromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrCat" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarBstrCat src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrCmp" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarBstrCmp src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromBool" => {
            let val = emu.regs().rcx as i16;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = if val == 0 { "False" } else { "True" };
            log_red!(
                emu,
                "oleaut32!VarBstrFromBool val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromCy" => {
            let cy = emu.regs().rcx as i64;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}.{:04}", cy / 10000, (cy % 10000).unsigned_abs());
            log_red!(
                emu,
                "oleaut32!VarBstrFromCy val:{} out:0x{:x}",
                s,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromDate" => {
            let _date_bits = emu.regs().rcx;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = "1/1/2024";
            log_red!(emu, "oleaut32!VarBstrFromDate out:0x{:x}", pbstr_out);
            let bstr = alloc_bstr(emu, s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromDec" => {
            let _dec_ptr = emu.regs().rcx;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = "0";
            log_red!(emu, "oleaut32!VarBstrFromDec out:0x{:x}", pbstr_out);
            let bstr = alloc_bstr(emu, s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromDisp" => {
            let _pdisp = emu.regs().rcx;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = "[object]";
            log_red!(emu, "oleaut32!VarBstrFromDisp out:0x{:x}", pbstr_out);
            let bstr = alloc_bstr(emu, s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromI1" => {
            let val = emu.regs().rcx as i8;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromI1 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromI2" => {
            let val = emu.regs().rcx as i16;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromI2 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromI4" => {
            let val = emu.regs().rcx as i32;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromI4 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromI8" => {
            let val = emu.regs().rcx as i64;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromI8 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromR4" => {
            let val = f32::from_bits(emu.regs().rcx as u32);
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromR4 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromR8" => {
            let val = f64::from_bits(emu.regs().rcx);
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromR8 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromUI1" => {
            let val = emu.regs().rcx as u8;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromUI1 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromUI2" => {
            let val = emu.regs().rcx as u16;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromUI2 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromUI4" => {
            let val = emu.regs().rcx as u32;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromUI4 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromUI8" => {
            let val = emu.regs().rcx;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pbstr_out = emu.regs().r9;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromUI8 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_qword(pbstr_out, bstr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCat" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCat arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarCmp" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCmp arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarCyAbs" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyAbs src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyAdd" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyAdd src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyCmp" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyCmp src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyCmpR8" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyCmpR8 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFix" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFix src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromBool" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromBool src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromDate" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromDate src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromDec" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromDec src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromDisp" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromDisp src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromI1" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromI1 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromI2" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromI2 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromI4" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromI4 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromI8" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromI8 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromR4" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromR4 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromR8" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromR8 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromStr" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromStr src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromUI1" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromUI1 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromUI2" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromUI2 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromUI4" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromUI4 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyFromUI8" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyFromUI8 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyInt" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyInt src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyMul" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyMul src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyMulI4" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyMulI4 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyMulI8" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyMulI8 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyNeg" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyNeg src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCyRound" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCyRound src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarCySub" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarCySub src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromBool" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromBool src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromCy" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromCy src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromDec" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromDec src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromDisp" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromDisp src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromI1" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromI1 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromI2" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromI2 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromI4" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromI4 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromI8" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromI8 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromR4" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromR4 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromR8" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromR8 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromStr" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromStr src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromUI1" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromUI1 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromUI2" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromUI2 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromUI4" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromUI4 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromUI8" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromUI8 src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromUdate" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromUdate src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDateFromUdateEx" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDateFromUdateEx src:0x{:x}", src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecAbs" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecAbs arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecAdd" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecAdd arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecCmp" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecCmp arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecCmpR8" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecCmpR8 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecDiv" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecDiv arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecFix" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFix arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecFromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecFromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecFromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarDecInt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecInt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecMul" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecMul arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecNeg" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecNeg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecRound" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecRound arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecSub" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarDecSub arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDiv" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarDiv arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarEqv" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarEqv arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFix" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarFix arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormat" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarFormat arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatCurrency" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarFormatCurrency arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatDateTime" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarFormatDateTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatFromTokens" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarFormatFromTokens h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatNumber" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarFormatNumber arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatPercent" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarFormatPercent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarI1FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI1FromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI1FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI2FromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI2FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromStr" => {
            let str_in = emu.regs().rcx;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pl_out = emu.regs().r9;
            let s = if str_in != 0 {
                emu.maps.read_wide_string(str_in)
            } else {
                String::new()
            };
            let val = s.trim().parse::<i32>().unwrap_or(0);
            log_red!(
                emu,
                "oleaut32!VarI4FromStr '{}' => {} out:0x{:x}",
                s,
                val,
                pl_out
            );
            if pl_out != 0 {
                emu.maps.write_dword(pl_out, val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI4FromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI4FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarI8FromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarI8FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarIdiv" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarIdiv arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarImp" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarImp arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarInt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarInt arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarMod" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarMod arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarMonthName" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarMonthName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarMul" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarMul arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarNeg" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarNeg arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarNot" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarNot arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarNumFromParseNum" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarNumFromParseNum arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarOr" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarOr arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarParseNumFromStr" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarParseNumFromStr arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarPow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarPow arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarR4CmpR8" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4CmpR8 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarR4FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR4FromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR4FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromStr" => {
            let str_in = emu.regs().rcx;
            let _lcid = emu.regs().rdx as u32;
            let _flags = emu.regs().r8 as u32;
            let pdbl_out = emu.regs().r9;
            let s = if str_in != 0 {
                emu.maps.read_wide_string(str_in)
            } else {
                String::new()
            };
            let val = s.trim().parse::<f64>().unwrap_or(0.0);
            log_red!(
                emu,
                "oleaut32!VarR8FromStr '{}' => {} out:0x{:x}",
                s,
                val,
                pdbl_out
            );
            if pdbl_out != 0 {
                emu.maps.write_qword(pdbl_out, val.to_bits());
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8FromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarR8Pow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarR8Pow arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarR8Round" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarR8Round arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarRound" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarRound arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarSub" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarSub arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarTokenizeFormatString" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarTokenizeFormatString h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarUI1FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI1FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI2FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromUI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI4FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromBool" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromCy" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromDate" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromDec" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromDisp" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromI8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromR4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromR8" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromStr" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromUI1" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromUI2" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromUI4" => {
            let src_val = emu.regs().rcx;
            let dst_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUI8FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VarUdateFromDate" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarUdateFromDate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarWeekdayName" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VarWeekdayName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarXor" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "oleaut32!VarXor arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VariantChangeType" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            let _lcid = emu.regs().r8;
            let vt_new = emu.regs().r9;
            if dst != 0 {
                emu.maps.write_word(dst, vt_new as u16);
            }
            log_red!(
                emu,
                "oleaut32!VariantChangeType vt:{} dst:0x{:x}",
                vt_new,
                dst
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VariantChangeTypeEx" => {
            let pvarg_dest = emu.regs().rcx;
            let pvarg_src = emu.regs().rdx;
            let vt = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as u16;
            log_red!(
                emu,
                "oleaut32!VariantChangeTypeEx dest: 0x{:x} src: 0x{:x} vt: {}",
                pvarg_dest,
                pvarg_src,
                vt
            );
            if pvarg_dest != 0 && pvarg_src != 0 {
                for i in 0..16u64 {
                    let b = emu.maps.read_byte(pvarg_src + i).unwrap_or(0);
                    emu.maps.write_byte(pvarg_dest + i, b);
                }
                emu.maps.write_word(pvarg_dest, vt);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VariantCopy" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            if dst != 0 && src != 0 {
                let vt = emu.maps.read_word(src).unwrap_or(0);
                let val = emu.maps.read_qword(src + 8).unwrap_or(0);
                emu.maps.write_word(dst, vt);
                emu.maps.write_qword(dst + 8, val);
            }
            log_red!(emu, "oleaut32!VariantCopy dst:0x{:x} src:0x{:x}", dst, src);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VariantCopyInd" => {
            let pvarg_dest = emu.regs().rcx;
            let pvarg_src = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VariantCopyInd dest: 0x{:x} src: 0x{:x}",
                pvarg_dest,
                pvarg_src
            );
            if pvarg_dest != 0 && pvarg_src != 0 {
                for i in 0..16u64 {
                    let b = emu.maps.read_byte(pvarg_src + i).unwrap_or(0);
                    emu.maps.write_byte(pvarg_dest + i, b);
                }
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VariantInit" => VariantInit(emu),
        "VariantTimeToDosDateTime" => {
            let pvar = emu.regs().rcx;
            if pvar != 0 {
                emu.maps.write_word(pvar, 0); // VT_EMPTY
            }
            log_red!(emu, "oleaut32!VariantTimeToDosDateTime ptr:0x{:x}", pvar);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VariantTimeToSystemTime" => {
            let pvar = emu.regs().rcx;
            if pvar != 0 {
                emu.maps.write_word(pvar, 0); // VT_EMPTY
            }
            log_red!(emu, "oleaut32!VariantTimeToSystemTime ptr:0x{:x}", pvar);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "VectorFromBstr" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "oleaut32!VectorFromBstr arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }

        _ => {
            if emu.cfg.skip_unimplemented {
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} at 0x{:x}",
                    addr,
                    api,
                    emu.regs().rip
                );
                emu.set_last_error(0);
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit
                && let Some(dump_file) = emu.cfg.dump_filename.as_ref()
            {
                serialization::Serialization::dump(emu, dump_file);
            }

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

fn alloc_bstr(emu: &mut emu::Emu, s: &str) -> u64 {
    let char_count = s.encode_utf16().count() as u64;
    let byte_len = char_count * 2;
    let total = 4 + byte_len + 2 + 16;
    let base = match emu.maps.alloc(total) {
        Some(a) => a,
        None => return 0,
    };
    let name = format!("bstr_alloc_{:x}", base);
    emu.maps
        .create_map(&name, base, total, Permission::READ_WRITE);
    emu.maps.write_dword(base, byte_len as u32);
    for (i, ch) in s.encode_utf16().enumerate() {
        emu.maps.write_word(base + 4 + (i as u64 * 2), ch);
    }
    emu.maps.write_word(base + 4 + byte_len, 0);
    base + 4
}

pub fn SysAllocStringLen(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let char_count = emu.regs().rdx;

    log_red!(
        emu,
        ":{:x} oleaut32!SysAllocStringLen str_ptr: 0x{:x} size: {}",
        emu.regs().rip,
        str_ptr,
        char_count
    );

    // Calculate exact sizes like the real API
    let string_bytes = char_count * 2; // Requested characters in bytes
    let total_alloc_size = 4 + string_bytes + 2 + 16; // Length prefix + string + null terminator
    // the extra 16 byes are not allocated on the
    // real winapi, but it's needed to allo some
    // optimizations

    // Allocate memory (no extra padding needed)
    let bstr = match emu.maps.alloc(total_alloc_size) {
        Some(a) => a,
        None => {
            // A hostile length can request an absurd allocation; degrade to a
            // null BSTR instead of aborting the whole run.
            log::warn!(
                "oleaut32!SysAllocStringLen: out of memory for {} bytes",
                total_alloc_size
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let name = format!("bstr_alloc_{:x}", bstr);
    emu.maps
        .create_map(&name, bstr, total_alloc_size, Permission::READ_WRITE);

    // Write length prefix (byte count of string data, excluding null terminator)
    emu.maps.write_dword(bstr, string_bytes as u32);

    if str_ptr == 0 {
        // Handle null input - zero out the string area
        for i in 0..char_count {
            emu.maps.write_word(bstr + 4 + (i * 2), 0);
        }
    } else {
        // Copy exactly char_count characters from input. A hostile/short source
        // pointer can leave a char unmapped: truncate there (like a real fault)
        // instead of panicking and killing the whole analysis.
        for i in 0..char_count {
            let char_addr = str_ptr + (i * 2);
            let wide_char = match emu.maps.read_word(char_addr) {
                Some(w) => w,
                None => {
                    log::warn!(
                        "oleaut32!SysAllocStringLen: unreadable source at 0x{:x}, truncating",
                        char_addr
                    );
                    break;
                }
            };
            emu.maps.write_word(bstr + 4 + (i * 2), wide_char);
        }
    }

    // Always write null terminator after the copied characters
    emu.maps.write_word(bstr + 4 + (char_count * 2), 0);

    let return_ptr = bstr + 4; // Return pointer to string data (after length prefix)

    log_red!(
        emu,
        "SysAllocStringLen returning: 0x{:x} (base: 0x{:x}, length_prefix: {})",
        return_ptr,
        bstr,
        string_bytes
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = return_ptr;
}

pub fn SysFreeString(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;

    log_red!(emu, "oleaut32!SysFreeString  0x{:x}", str_ptr);

    if str_ptr == 0 {
        // NULL pointer - nothing to free (this is valid behavior)
        return;
    }

    // BSTR pointer points to string data, but allocation starts 4 bytes earlier (length prefix)
    let alloc_base = str_ptr - 4;

    // Read the length from the prefix to know how much to zero out
    if let Some(length_bytes) = emu.maps.read_dword(alloc_base) {
        let total_size = 4 + length_bytes as u64 + 2; // prefix + string + null terminator
        let string_length = length_bytes / 2; // Convert bytes to characters

        log_red!(
            emu,
            "SysFreeString zeroing {} bytes starting at 0x{:x} (string data was {} bytes, {} chars)",
            total_size,
            // Total allocation size
            alloc_base,
            // Base address
            length_bytes,
            // String data in bytes
            string_length
        );

        // Zero out the entire BSTR allocation
        for i in 0..total_size {
            emu.maps.write_byte(alloc_base + i, 0);
        }
    } else {
        // Guest handed us a BSTR pointer whose length prefix we can't read — a
        // garbage/hostile handle. Warn and no-op instead of crashing.
        log::warn!(
            "{} oleaut32!SysFreeString: unreadable length prefix at 0x{:x}, ignoring",
            emu.pos,
            alloc_base,
        );
    }

    // Optionally, you could also try to free the map by name:
    // emu.maps.free(&format!("alloc_{:x}", alloc_base));
    // or
    // emu.maps.free(&format!("bstr_{:x}", alloc_base));
}

/*
INT SysReAllocStringLen(
  [in, out]      BSTR          *pbstr,
  [in, optional] const OLECHAR *psz,
  [in]           unsigned int  len
);
*/
fn SysReAllocStringLen(emu: &mut emu::Emu) {
    let pbstr_ptr = emu.regs().rcx; // Pointer to BSTR*
    let psz = emu.regs().rdx; // Source string (can be NULL)
    let len = emu.regs().r8; // Length in characters

    log_red!(
        emu,
        "oleaut32!SysReAllocStringLen pbstr_ptr: 0x{:x} psz: 0x{:x} len: {}",
        pbstr_ptr,
        psz,
        len
    );

    // Check if pbstr_ptr is NULL
    if pbstr_ptr == 0 {
        emu.set_last_error(0);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read the current BSTR pointer (might be NULL for first allocation)
    let old_bstr = emu.maps.read_qword(pbstr_ptr).unwrap_or(0);

    // Log old content if it exists
    if old_bstr != 0 {
        let old_alloc_base = old_bstr - 4;
        let old_len_bytes = emu.maps.read_dword(old_alloc_base).unwrap_or(0);
        let old_len_chars = old_len_bytes / 2;
        if old_len_chars > 0 {
            let old_string = emu
                .maps
                .read_wide_string_n(old_bstr, old_len_chars as usize);
            log_red!(
                emu,
                "Old BSTR content: \"{}\" (length: {} chars)",
                old_string,
                old_len_chars
            );
        }
    }

    // Log new source string if provided
    if psz != 0 && len > 0 {
        let new_string = emu.maps.read_wide_string_n(psz, len as usize);
        log_red!(
            emu,
            "New source string: \"{}\" (length: {} chars)",
            new_string,
            len
        );
    }

    // Calculate allocation size
    let byte_len = len * 2; // Length in bytes (UTF-16)
    let total_alloc_size = 4 + byte_len + 2; // 4-byte prefix + string + null terminator

    // Always allocate new memory (simpler than trying to realloc)
    let new_base = match emu.maps.alloc(total_alloc_size + 100) {
        Some(a) => a,
        None => {
            log::warn!(
                "oleaut32!SysReAllocStringLen: out of memory for {} bytes",
                total_alloc_size + 100
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let name = format!("bstr_{:x}", new_base);
    emu.maps.create_map(
        &name,
        new_base,
        total_alloc_size + 100,
        Permission::READ_WRITE,
    );

    // Write length prefix (in bytes, not including null terminator)
    emu.maps.write_dword(new_base, byte_len as u32);

    // Copy data from source if provided
    if psz != 0 && len > 0 {
        emu.maps.memcpy(new_base + 4, psz, len as usize * 2);
    } else if old_bstr != 0 && len > 0 {
        // No new source provided, preserve existing data (truncated to new length)
        let old_alloc_base = old_bstr - 4;
        let old_len_bytes = emu.maps.read_dword(old_alloc_base).unwrap_or(0);
        let old_len_chars = old_len_bytes / 2;
        let copy_len = std::cmp::min(len, old_len_chars as u64);

        if copy_len > 0 {
            emu.maps
                .memcpy(new_base + 4, old_bstr, copy_len as usize * 2);
        }

        // Zero out any remaining space if new length is longer than old length
        for i in copy_len..len {
            emu.maps.write_word(new_base + 4 + (i * 2), 0);
        }
    } else {
        // Initialize to zeros (empty string)
        for i in 0..len {
            emu.maps.write_word(new_base + 4 + (i * 2), 0);
        }
    }

    // Write null terminator
    emu.maps.write_word(new_base + 4 + byte_len, 0);

    // Update the BSTR pointer to point to the string data (skip the 4-byte length prefix)
    let new_bstr = new_base + 4;
    emu.maps.write_qword(pbstr_ptr, new_bstr);

    // Log the final result
    if len > 0 {
        let final_string = emu.maps.read_wide_string_n(new_bstr, len as usize);
        log_red!(
            emu,
            "Final BSTR content: \"{}\" (length: {} chars)",
            final_string,
            len
        );
    } else {
        log_red!(emu, "Created empty BSTR");
    }

    log_red!(
        emu,
        "oleaut32!SysReAllocStringLen allocated new string at 0x{:x} size: {} (base: 0x{:x})",
        new_bstr,
        byte_len,
        new_base
    );

    // Note: In a real implementation, you'd free the old BSTR here
    // but in an emulator, we might want to keep it for debugging

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::TRUE;
}

/*
HRESULT VariantClear(
  [in, out] VARIANTARG *pvarg
);
*/
pub fn VariantClear(emu: &mut emu::Emu) {
    let pvarg = emu.regs().rcx;

    log_red!(emu, "oleaut32!VariantClear pvarg: 0x{:x}", pvarg);

    // Basic validation
    if pvarg == 0 || !emu.maps.is_mapped(pvarg) {
        log_red!(emu, "VariantClear: Invalid pvarg pointer");
        emu.set_last_error(0);
        emu.regs_mut().rax = constants::HRESULT_E_INVALID_ARG;
        return;
    }

    // Clear the variant by setting vt field to VT_EMPTY (0)
    // The vt field is typically at offset 0 in the VARIANT structure
    emu.maps.write_word(pvarg, 0); // VT_EMPTY = 0

    log_red!(emu, "VariantClear: Cleared variant (set vt to VT_EMPTY)");

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}

pub fn SysAllocString(emu: &mut emu::Emu) {
    let psz = emu.regs().rcx;

    if psz == 0 {
        log_red!(emu, "oleaut32!SysAllocString NULL -> 0");
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }

    let s = emu.maps.read_wide_string(psz);
    let char_count = s.encode_utf16().count() as u64;

    log_red!(
        emu,
        "oleaut32!SysAllocString '{}' ({} chars)",
        s,
        char_count
    );

    let byte_len = char_count * 2;
    let total = 4 + byte_len + 2 + 16;

    let base = match emu.maps.alloc(total) {
        Some(a) => a,
        None => {
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let name = format!("bstr_alloc_{:x}", base);
    emu.maps
        .create_map(&name, base, total, Permission::READ_WRITE);

    emu.maps.write_dword(base, byte_len as u32);

    for (i, ch) in s.encode_utf16().enumerate() {
        emu.maps.write_word(base + 4 + (i as u64 * 2), ch);
    }
    emu.maps.write_word(base + 4 + byte_len, 0);

    let bstr = base + 4;
    log_red!(emu, "SysAllocString -> 0x{:x}", bstr);
    emu.set_last_error(0);
    emu.regs_mut().rax = bstr;
}

fn SysAllocStringByteLen(emu: &mut emu::Emu) {
    let psz = emu.regs().rcx;
    let len = emu.regs().rdx;

    log_red!(
        emu,
        "oleaut32!SysAllocStringByteLen psz: 0x{:x} len: {}",
        psz,
        len
    );

    let total = 4 + len + 2 + 16;

    let base = match emu.maps.alloc(total) {
        Some(a) => a,
        None => {
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let name = format!("bstr_alloc_{:x}", base);
    emu.maps
        .create_map(&name, base, total, Permission::READ_WRITE);

    emu.maps.write_dword(base, len as u32);

    if psz != 0 {
        for i in 0..len {
            let b = emu.maps.read_byte(psz + i).unwrap_or(0);
            emu.maps.write_byte(base + 4 + i, b);
        }
    }
    emu.maps.write_word(base + 4 + len, 0);

    let bstr = base + 4;
    emu.set_last_error(0);
    emu.regs_mut().rax = bstr;
}

pub fn SysStringLen(emu: &mut emu::Emu) {
    let bstr = emu.regs().rcx;

    if bstr == 0 {
        log_red!(emu, "oleaut32!SysStringLen NULL -> 0");
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }

    let len_bytes = emu.maps.read_dword(bstr - 4).unwrap_or(0);
    let len_chars = len_bytes / 2;

    log_red!(emu, "oleaut32!SysStringLen 0x{:x} -> {}", bstr, len_chars);
    emu.set_last_error(0);
    emu.regs_mut().rax = len_chars as u64;
}

pub fn SysStringByteLen(emu: &mut emu::Emu) {
    let bstr = emu.regs().rcx;

    if bstr == 0 {
        log_red!(emu, "oleaut32!SysStringByteLen NULL -> 0");
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }

    let len_bytes = emu.maps.read_dword(bstr - 4).unwrap_or(0);

    log_red!(
        emu,
        "oleaut32!SysStringByteLen 0x{:x} -> {}",
        bstr,
        len_bytes
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = len_bytes as u64;
}

pub fn VariantInit(emu: &mut emu::Emu) {
    let pvarg = emu.regs().rcx;

    log_red!(emu, "oleaut32!VariantInit 0x{:x}", pvarg);

    if pvarg != 0 {
        // Zero out the 16-byte VARIANT structure
        for i in 0..16u64 {
            emu.maps.write_byte(pvarg + i, 0);
        }
    }
    // VariantInit has no return value (void), but we set rax=0 for safety
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
