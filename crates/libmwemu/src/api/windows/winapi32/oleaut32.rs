use crate::api::windows::common::fallback;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "SysAllocStringLen" => SysAllocStringLen(emu),
        "SysFreeString" => SysFreeString(emu),
        "BSTR_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!BSTR_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!BSTR_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!BSTR_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!BSTR_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BstrFromVector" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!BstrFromVector arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClearCustData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!ClearCustData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateDispTypeInfo" => {
            let pidata = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pptinfo = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://ITypeInfo");
            if pptinfo != 0 {
                emu.maps.write_dword(pptinfo, hndl as u32);
            }
            log_red!(emu, "oleaut32!CreateDispTypeInfo =0x{:x}", hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateErrorInfo" => {
            let pp_errinfo = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://ICreateErrorInfo");
            if pp_errinfo != 0 {
                emu.maps.write_dword(pp_errinfo, hndl as u32);
            }
            log_red!(emu, "oleaut32!CreateErrorInfo =0x{:x}", hndl);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateStdDispatch" => {
            let punk_outer = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pv_this = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let ptinfo = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let ppunk = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://IDispatch");
            if ppunk != 0 {
                emu.maps.write_dword(ppunk, hndl as u32);
            }
            log_red!(emu, "oleaut32!CreateStdDispatch =0x{:x}", hndl);
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateTypeLib" => {
            let sys_kind = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let sz_file = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pp_ctlib = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://ICreateTypeLib");
            if pp_ctlib != 0 {
                emu.maps.write_dword(pp_ctlib, hndl as u32);
            }
            log_red!(emu, "oleaut32!CreateTypeLib =0x{:x}", hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateTypeLib2" => {
            let sys_kind = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let sz_file = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pp_ctlib = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://ICreateTypeLib2");
            if pp_ctlib != 0 {
                emu.maps.write_dword(pp_ctlib, hndl as u32);
            }
            log_red!(emu, "oleaut32!CreateTypeLib2 =0x{:x}", hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "DispCallFunc" => {
            let punk = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!DispCallFunc 0x{:x}", punk);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "DispGetIDsOfNames" => {
            let punk = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!DispGetIDsOfNames 0x{:x}", punk);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "DispGetParam" => {
            let punk = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!DispGetParam 0x{:x}", punk);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "DispInvoke" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "oleaut32!DispInvoke 0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DllCanUnloadNow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!DllCanUnloadNow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "DllGetClassObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!DllGetClassObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllRegisterServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!DllRegisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("DllRegisterServer");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DosDateTimeToVariantTime" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!DosDateTimeToVariantTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetActiveObject" => {
            let rclsid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pv_reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let ppunk = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://IUnknown");
            if ppunk != 0 {
                emu.maps.write_dword(ppunk, hndl as u32);
            }
            log_red!(emu, "oleaut32!GetActiveObject =0x{:x}", hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "GetAltMonthNames" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!GetAltMonthNames h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetErrorInfo" => {
            let _reserved = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pperrinfo = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!GetErrorInfo pperrinfo: 0x{:x}", pperrinfo);
            if pperrinfo != 0 {
                emu.maps.write_dword(pperrinfo, 0);
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1; // S_FALSE
        }
        "GetRecordInfoFromGuids" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!GetRecordInfoFromGuids h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRecordInfoFromTypeInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!GetRecordInfoFromTypeInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetVarConversionLocaleSetting" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!GetVarConversionLocaleSetting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!HWND_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!HWND_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!HWND_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LHashValOfNameSys" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LHashValOfNameSys arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "LHashValOfNameSysA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "oleaut32!LHashValOfNameSysA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LPSAFEARRAY_Marshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_Marshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_Size" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_Size arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_Unmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_Unmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LoadRegTypeLib" => {
            let rguid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let w_ver_major = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u16;
            let w_ver_minor = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u16;
            let lcid = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pptlib = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://ITypeLib");
            if pptlib != 0 {
                emu.maps.write_dword(pptlib, hndl as u32);
            }
            log_red!(emu, "oleaut32!LoadRegTypeLib =0x{:x}", hndl);
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "LoadTypeLib" => {
            let sz_file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pptlib = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://ITypeLib");
            if pptlib != 0 {
                emu.maps.write_dword(pptlib, hndl as u32);
            }
            let path = emu.maps.read_wide_string(sz_file);
            log_red!(emu, "oleaut32!LoadTypeLib '{}' =0x{:x}", path, hndl);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "LoadTypeLibEx" => {
            let sz_file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let reg_kind = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pptlib = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://ITypeLib");
            if pptlib != 0 {
                emu.maps.write_dword(pptlib, hndl as u32);
            }
            let path = emu.maps.read_wide_string(sz_file);
            log_red!(emu, "oleaut32!LoadTypeLibEx '{}' =0x{:x}", path, hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "OACleanup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OACleanup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OACreateTypeLib2" => {
            let sys_kind = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let sz_file = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pp_ctlib = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("oleaut32://ICreateTypeLib2");
            if pp_ctlib != 0 {
                emu.maps.write_dword(pp_ctlib, hndl as u32);
            }
            log_red!(emu, "oleaut32!OACreateTypeLib2 =0x{:x}", hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "OaBuildVersion" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OaBuildVersion arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OleCreateFontIndirect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleCreateFontIndirect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleCreateFontIndirect");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleCreatePictureIndirect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleCreatePictureIndirect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleCreatePictureIndirect");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleCreatePropertyFrame" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleCreatePropertyFrame arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleCreatePropertyFrame");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleCreatePropertyFrameIndirect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleCreatePropertyFrameIndirect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleCreatePropertyFrameIndirect");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleIconToCursor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleIconToCursor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPicture" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleLoadPicture arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleLoadPicture");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPictureEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleLoadPictureEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleLoadPictureEx");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPictureFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleLoadPictureFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleLoadPictureFile");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPictureFileEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleLoadPictureFileEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleLoadPictureFileEx");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPicturePath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleLoadPicturePath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleLoadPicturePath");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleSavePictureFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleSavePictureFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OleSavePictureFile");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleTranslateColor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OleTranslateColor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "QueryPathOfRegTypeLib" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!QueryPathOfRegTypeLib h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterActiveObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!RegisterActiveObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("RegisterActiveObject");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTypeLib" => {
            let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let path = emu.maps.read_string(path_ptr);
            let hndl = helper::handler_create(&format!("typelib_{}", path));
            log_red!(emu, "oleaut32!RegisterTypeLib '{}'  => 0x{:x}", path, hndl);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "RegisterTypeLibForUser" => {
            let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let path = emu.maps.read_string(path_ptr);
            let hndl = helper::handler_create(&format!("typelib_{}", path));
            log_red!(
                emu,
                "oleaut32!RegisterTypeLibForUser '{}'  => 0x{:x}",
                path,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "RevokeActiveObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!RevokeActiveObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SafeArrayAccessData" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if data_ptr != 0 {
                emu.maps.write_dword(data_ptr, (sa + 0x20) as u32);
            }
            log_red!(emu, "oleaut32!SafeArrayAccessData 0x{:x}", sa);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayAddRef" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayAddRef 0x{:x}", sa);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayAllocData" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayAllocData 0x{:x}", sa);
            let handle = helper::handler_create("SafeArrayAllocData");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayAllocDescriptor" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayAllocDescriptor 0x{:x}", sa);
            let handle = helper::handler_create("SafeArrayAllocDescriptor");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayAllocDescriptorEx" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayAllocDescriptorEx 0x{:x}", sa);
            let handle = helper::handler_create("SafeArrayAllocDescriptorEx");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayCopy" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayCopy 0x{:x}", sa);
            let handle = helper::handler_create("SafeArrayCopy");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayCopyData" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayCopyData 0x{:x}", sa);
            let handle = helper::handler_create("SafeArrayCopyData");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayCreate" => {
            let vt = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dims = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let bounds = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create(&format!("safearray_vt{}", vt));
            log_red!(
                emu,
                "oleaut32!SafeArrayCreate vt:{} dims:{} => 0x{:x}",
                vt,
                dims,
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayCreateEx" => {
            let vt = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u16;
            let dims = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let rgsabound = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _pvextra = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let elem_size: u32 = match vt {
                8 => 4,   // VT_BSTR -> 32-bit pointer
                12 => 16, // VT_VARIANT
                _ => 4,
            };
            let total_elements = if rgsabound != 0 && dims > 0 {
                emu.maps.read_dword(rgsabound).unwrap_or(1)
            } else {
                1
            };
            let data_size = total_elements as u64 * elem_size as u64;
            let alloc_size = 24 + data_size;
            let sa = match emu.maps.alloc(alloc_size) {
                Some(a) => a,
                None => {
                    for _ in 0..4u32 {
                        emu.stack_pop32(false);
                    }
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
            emu.maps.write_dword(sa + 4, elem_size);
            emu.maps.write_dword(sa + 8, 0);
            emu.maps.write_dword(sa + 12, (sa + 24) as u32);
            if rgsabound != 0 && dims > 0 {
                let count = emu.maps.read_dword(rgsabound).unwrap_or(0);
                let lbound = emu.maps.read_dword(rgsabound + 4).unwrap_or(0);
                emu.maps.write_dword(sa + 16, count);
                emu.maps.write_dword(sa + 20, lbound);
            }
            log_red!(
                emu,
                "oleaut32!SafeArrayCreateEx vt:{} dims:{} =0x{:x}",
                vt,
                dims,
                sa
            );
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = sa;
        }
        "SafeArrayCreateVector" => {
            let vt = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let lower = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create(&format!("safearray_vec_{}", count));
            log_red!(
                emu,
                "oleaut32!SafeArrayCreateVector vt:{} lower:{} count:{} => 0x{:x}",
                vt,
                lower,
                count,
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayCreateVectorEx" => {
            let vt = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u16;
            let lbound = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let celements = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _pvextra = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let elem_size: u32 = match vt {
                8 => 4,   // VT_BSTR
                12 => 16, // VT_VARIANT
                _ => 4,
            };
            let data_size = celements as u64 * elem_size as u64;
            let alloc_size = 24 + data_size;
            let sa = match emu.maps.alloc(alloc_size) {
                Some(a) => a,
                None => {
                    for _ in 0..4u32 {
                        emu.stack_pop32(false);
                    }
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
            emu.maps.write_word(sa, 1); // cDims = 1
            emu.maps.write_word(sa + 2, 0);
            emu.maps.write_dword(sa + 4, elem_size);
            emu.maps.write_dword(sa + 8, 0);
            emu.maps.write_dword(sa + 12, (sa + 24) as u32);
            emu.maps.write_dword(sa + 16, celements);
            emu.maps.write_dword(sa + 20, lbound);
            log_red!(
                emu,
                "oleaut32!SafeArrayCreateVectorEx vt:{} lbound:{} count:{} =0x{:x}",
                vt,
                lbound,
                celements,
                sa
            );
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = sa;
        }
        "SafeArrayDestroy" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(sa);
            log_red!(emu, "oleaut32!SafeArrayDestroy 0x{:x}", sa);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayDestroyData" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(sa);
            log_red!(emu, "oleaut32!SafeArrayDestroyData 0x{:x}", sa);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayDestroyDescriptor" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(sa);
            log_red!(emu, "oleaut32!SafeArrayDestroyDescriptor 0x{:x}", sa);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayGetDim" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayGetDim 0x{:x} => 1", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayGetElement" => {
            let psa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let rgindices = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pv = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let mut result: u64 = 0; // S_OK
            if psa != 0 && rgindices != 0 && pv != 0 {
                let elem_size = emu.maps.read_dword(psa + 4).unwrap_or(4) as u64;
                let pv_data = emu.maps.read_dword(psa + 12).unwrap_or(0) as u64;
                let lbound = emu.maps.read_dword(psa + 20).unwrap_or(0);
                let index = emu.maps.read_dword(rgindices).unwrap_or(0);
                let offset = (index.wrapping_sub(lbound)) as u64 * elem_size;
                if pv_data != 0 && elem_size <= 16 {
                    let data = emu
                        .maps
                        .read_bytes(pv_data + offset, elem_size as usize)
                        .to_vec();
                    emu.maps.write_bytes(pv, &data);
                }
            } else {
                result = 0x8002000B; // DISP_E_BADINDEX
            }
            log_red!(
                emu,
                "oleaut32!SafeArrayGetElement psa:0x{:x} idx_ptr:0x{:x} pv:0x{:x}",
                psa,
                rgindices,
                pv
            );
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = result;
        }
        "SafeArrayGetElemsize" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayGetElemsize 0x{:x} => 4", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 4;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayGetIID" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayGetIID 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayGetLBound" => {
            let psa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _ndim = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let plbound = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayGetLBound psa: 0x{:x}", psa);
            if psa != 0 && plbound != 0 {
                let lbound = emu.maps.read_dword(psa + 20).unwrap_or(0);
                emu.maps.write_dword(plbound, lbound);
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "SafeArrayGetRecordInfo" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayGetRecordInfo 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayGetUBound" => {
            let psa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _ndim = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pubound = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayGetUBound psa: 0x{:x}", psa);
            if psa != 0 && pubound != 0 {
                let count = emu.maps.read_dword(psa + 16).unwrap_or(0);
                let lbound = emu.maps.read_dword(psa + 20).unwrap_or(0);
                let ubound = if count > 0 {
                    lbound + count - 1
                } else {
                    lbound
                };
                emu.maps.write_dword(pubound, ubound);
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "SafeArrayGetVartype" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayGetVartype 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayLock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "oleaut32!SafeArrayLock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayPtrOfIndex" => {
            let psa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let rgindices = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let ppv_data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if psa != 0 && rgindices != 0 && ppv_data != 0 {
                let elem_size = emu.maps.read_dword(psa + 4).unwrap_or(4) as u64;
                let pv_data = emu.maps.read_dword(psa + 12).unwrap_or(0) as u64;
                let lbound = emu.maps.read_dword(psa + 20).unwrap_or(0);
                let index = emu.maps.read_dword(rgindices).unwrap_or(0);
                let offset = (index.wrapping_sub(lbound)) as u64 * elem_size;
                emu.maps.write_dword(ppv_data, (pv_data + offset) as u32);
            }
            log_red!(emu, "oleaut32!SafeArrayPtrOfIndex psa:0x{:x}", psa);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "SafeArrayPutElement" => {
            let psa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let rgindices = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let pv = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if psa != 0 && rgindices != 0 && pv != 0 {
                let elem_size = emu.maps.read_dword(psa + 4).unwrap_or(4) as u64;
                let pv_data = emu.maps.read_dword(psa + 12).unwrap_or(0) as u64;
                let lbound = emu.maps.read_dword(psa + 20).unwrap_or(0);
                let index = emu.maps.read_dword(rgindices).unwrap_or(0);
                let offset = (index.wrapping_sub(lbound)) as u64 * elem_size;
                if pv_data != 0 && elem_size <= 16 {
                    let data = emu.maps.read_bytes(pv, elem_size as usize).to_vec();
                    emu.maps.write_bytes(pv_data + offset, &data);
                }
            }
            log_red!(
                emu,
                "oleaut32!SafeArrayPutElement psa:0x{:x} idx_ptr:0x{:x} pv:0x{:x}",
                psa,
                rgindices,
                pv
            );
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "SafeArrayRedim" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayRedim 0x{:x}", sa);
            let handle = helper::handler_create("SafeArrayRedim");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayReleaseData" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayReleaseData 0x{:x}", sa);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArraySetIID" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArraySetIID 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArraySetRecordInfo" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArraySetRecordInfo 0x{:x}", sa);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayUnaccessData" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayUnaccessData 0x{:x}", sa);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SafeArrayUnlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "oleaut32!SafeArrayUnlock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetErrorInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!SetErrorInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetOaNoCache" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!SetOaNoCache h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetVarConversionLocaleSetting" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!SetVarConversionLocaleSetting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SysAddRefString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!SysAddRefString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SysAddRefString");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SysReAllocString" => {
            let bstr_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            let hndl = helper::handler_create(&format!("bstr_{}", s));
            if bstr_ptr != 0 {
                emu.maps.write_dword(bstr_ptr, hndl as u32);
            }
            log_red!(emu, "oleaut32!SysReAllocString '{}' => 0x{:x}", s, hndl);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SysReAllocStringLen" => {
            let bstr_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let hndl = helper::handler_create(&format!("bstr_{}", len));
            if bstr_ptr != 0 {
                emu.maps.write_dword(bstr_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "oleaut32!SysReAllocStringLen '{}' len:{} => 0x{:x}",
                s,
                len,
                hndl
            );
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "SysReleaseString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!SysReleaseString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SystemTimeToVariantTime" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!SystemTimeToVariantTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnRegisterTypeLib" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!UnRegisterTypeLib arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("UnRegisterTypeLib");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnRegisterTypeLibForUser" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!UnRegisterTypeLibForUser arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("UnRegisterTypeLibForUser");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarAbs" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarAbs arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarAdd" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarAdd arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarAnd" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarAnd arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarBoolFromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBoolFromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarBoolFromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBstrCat" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarBstrCat src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrCmp" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarBstrCmp src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrFromBool" => {
            let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as i16;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if val != 0 { "True" } else { "False" };
            let bstr = alloc_bstr32(emu, s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            log_red!(emu, "oleaut32!VarBstrFromBool val:{} => '{}'", val, s);
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromCy" => {
            let cy_lo = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let cy_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let cy = ((cy_hi << 32) | cy_lo) as i64;
            let s = format!("{}.{:04}", cy / 10000, (cy % 10000).unsigned_abs());
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            log_red!(emu, "oleaut32!VarBstrFromCy => '{}'", s);
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromDate" => {
            let date_lo = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let date_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let date = f64::from_bits(((date_hi as u64) << 32) | date_lo as u64);
            let s = format!("{:.6}", date);
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            log_red!(emu, "oleaut32!VarBstrFromDate => '{}'", s);
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromDec" => {
            let dec_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let lo = emu.maps.read_dword(dec_ptr + 8).unwrap_or(0);
            let s = format!("{}", lo);
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            log_red!(emu, "oleaut32!VarBstrFromDec => '{}'", s);
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromDisp" => {
            let pdisp = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = format!("[IDispatch:0x{:x}]", pdisp);
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            log_red!(emu, "oleaut32!VarBstrFromDisp => '{}'", s);
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromI1" => {
            let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as i8;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = format!("{}", val);
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            log_red!(emu, "oleaut32!VarBstrFromI1 val:{} => '{}'", val, s);
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrFromI2" => {
            let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as i16;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = format!("{}", val);
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            log_red!(emu, "oleaut32!VarBstrFromI2 val:{} => '{}'", val, s);
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarBstrFromI4" => {
            let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as i32;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromI4 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrFromI8" => {
            let val_lo = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let val_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let val = ((val_hi << 32) | val_lo) as i64;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromI8 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrFromR4" => {
            let val = f32::from_bits(emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0));
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromR4 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrFromR8" => {
            let bits_lo = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let bits_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let val = f64::from_bits((bits_hi << 32) | bits_lo);
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromR8 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrFromUI1" => {
            let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u8;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromUI1 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrFromUI2" => {
            let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u16;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromUI2 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrFromUI4" => {
            let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromUI4 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "VarBstrFromUI8" => {
            let val_lo = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let val_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let val = (val_hi << 32) | val_lo;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pbstr_out = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = format!("{}", val);
            log_red!(
                emu,
                "oleaut32!VarBstrFromUI8 val:{} out:0x{:x}",
                val,
                pbstr_out
            );
            let bstr = alloc_bstr32(emu, &s);
            if pbstr_out != 0 {
                emu.maps.write_dword(pbstr_out, bstr as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "VarCat" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCat arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarCmp" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCmp arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "VarCyAbs" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyAbs src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyAdd" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyAdd src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyCmp" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyCmp src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyCmpR8" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyCmpR8 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFix" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFix src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromBool" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromBool src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromDate" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromDate src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromDec" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromDec src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromDisp" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromDisp src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromI1" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromI1 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromI2" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromI2 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromI4" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromI4 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromI8" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromI8 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromR4" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromR4 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromR8" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromR8 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromStr" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromStr src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromUI1" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromUI1 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromUI2" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromUI2 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromUI4" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromUI4 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyFromUI8" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyFromUI8 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyInt" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyInt src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyMul" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyMul src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyMulI4" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyMulI4 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyMulI8" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyMulI8 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyNeg" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyNeg src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCyRound" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCyRound src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarCySub" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarCySub src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromBool" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromBool src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromCy" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromCy src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromDec" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromDec src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromDisp" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromDisp src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromI1" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromI1 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromI2" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromI2 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromI4" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromI4 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromI8" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromI8 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromR4" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromR4 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromR8" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromR8 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromStr" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromStr src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromUI1" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromUI1 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromUI2" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromUI2 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromUI4" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromUI4 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromUI8" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromUI8 src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromUdate" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromUdate src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDateFromUdateEx" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDateFromUdateEx src:0x{:x}", src);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VarDecAbs" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecAbs arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecAdd" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecAdd arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecCmp" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecCmp arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecCmpR8" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecCmpR8 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecDiv" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecDiv arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecFix" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFix arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecFromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecFromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecFromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarDecInt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecInt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecMul" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecMul arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecNeg" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecNeg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecRound" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecRound arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDecSub" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarDecSub arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarDiv" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarDiv arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarEqv" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarEqv arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFix" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarFix arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormat" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarFormat arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatCurrency" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarFormatCurrency arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatDateTime" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarFormatDateTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatFromTokens" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarFormatFromTokens h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatNumber" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarFormatNumber arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarFormatPercent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarFormatPercent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarI1FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI1FromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI1FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI2FromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI2FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromStr" => {
            let str_in = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pl_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI4FromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI4FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarI8FromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarI8FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarIdiv" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarIdiv arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarImp" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarImp arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarInt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarInt arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarMod" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarMod arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarMonthName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarMonthName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarMul" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarMul arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarNeg" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarNeg arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarNot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarNot arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarNumFromParseNum" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarNumFromParseNum arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarOr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarOr arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarParseNumFromStr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarParseNumFromStr arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarPow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarPow arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarR4CmpR8" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4CmpR8 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarR4FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR4FromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR4FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromStr" => {
            let str_in = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pdbl_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8FromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarR8Pow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarR8Pow arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarR8Round" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarR8Round arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarRound" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarRound arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarSub" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarSub arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarTokenizeFormatString" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarTokenizeFormatString h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarUI1FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI1FromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI1FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI2FromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI2FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI4FromUI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI4FromUI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromBool" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromBool src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromCy" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromCy src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromDate" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromDate src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromDec" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromDec src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromDisp" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromDisp src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromI8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromI8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromR4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromR4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromR8" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromR8 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromStr" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromStr src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromUI1" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromUI1 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromUI2" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromUI2 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUI8FromUI4" => {
            let src_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUI8FromUI4 src:0x{:x} dst:0x{:x}",
                src_val,
                dst_ptr
            );
            if dst_ptr != 0 {
                emu.maps.write_dword(dst_ptr, src_val as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VarUdateFromDate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarUdateFromDate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarWeekdayName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VarWeekdayName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VarXor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!VarXor arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VariantChangeType" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let _lcid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let vt_new = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if dst != 0 {
                emu.maps.write_word(dst, vt_new as u16);
            }
            log_red!(
                emu,
                "oleaut32!VariantChangeType vt:{} dst:0x{:x}",
                vt_new,
                dst
            );
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "VariantChangeTypeEx" => {
            let pvarg_dest = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pvarg_src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let vt = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u16;
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
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VariantClear" => {
            let pvar = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if pvar != 0 {
                for i in 0..16u64 {
                    emu.maps.write_byte(pvar + i, 0);
                }
            }
            log_red!(emu, "oleaut32!VariantClear ptr:0x{:x}", pvar);
            emu.stack_pop32(false);
            emu.regs_mut().rax = 0;
        }
        "VariantCopy" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if dst != 0 && src != 0 {
                for i in 0..16u64 {
                    let b = emu.maps.read_byte(src + i).unwrap_or(0);
                    emu.maps.write_byte(dst + i, b);
                }
            }
            log_red!(emu, "oleaut32!VariantCopy dst:0x{:x} src:0x{:x}", dst, src);
            let handle = helper::handler_create("VariantCopy");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VariantCopyInd" => {
            let pvarg_dest = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pvarg_src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VariantCopyInd dest: 0x{:x} src: 0x{:x}",
                pvarg_dest,
                pvarg_src
            );
            let handle = helper::handler_create("VariantCopyInd");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            if pvarg_dest != 0 && pvarg_src != 0 {
                for i in 0..16u64 {
                    let b = emu.maps.read_byte(pvarg_src + i).unwrap_or(0);
                    emu.maps.write_byte(pvarg_dest + i, b);
                }
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "VariantInit" => VariantInit32(emu),
        "VariantTimeToDosDateTime" => {
            let pvar = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if pvar != 0 {
                emu.maps.write_word(pvar, 0); // VT_EMPTY
            }
            log_red!(emu, "oleaut32!VariantTimeToDosDateTime ptr:0x{:x}", pvar);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "VariantTimeToSystemTime" => {
            let pvar = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if pvar != 0 {
                emu.maps.write_word(pvar, 0); // VT_EMPTY
            }
            log_red!(emu, "oleaut32!VariantTimeToSystemTime ptr:0x{:x}", pvar);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "VectorFromBstr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VectorFromBstr arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BSTR_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!BSTR_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!BSTR_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!BSTR_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BSTR_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!BSTR_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "HWND_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!HWND_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!HWND_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VARIANT_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!VARIANT_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }

        "SysAllocString" => SysAllocString(emu),
        "SysAllocStringByteLen" => SysAllocStringByteLen(emu),
        "SysStringLen" => SysStringLen(emu),
        "SysStringByteLen" => SysStringByteLen(emu),
        "DllUnregisterServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!DllUnregisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!HWND_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!HWND_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!HWND_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LPSAFEARRAY_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "LPSAFEARRAY_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!LPSAFEARRAY_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OaEnablePerUserTLibRegistration" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "oleaut32!OaEnablePerUserTLibRegistration arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OaEnablePerUserTLibRegistration");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SafeArrayReleaseDescriptor" => {
            let sa = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "oleaut32!SafeArrayReleaseDescriptor 0x{:x}", sa);
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn alloc_bstr32(emu: &mut emu::Emu, s: &str) -> u64 {
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

fn SysAllocStringLen(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let mut size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    if size == 0xffffffff {
        size = 1024;
    }
    size += 1; // null byte
    size += 8; // metadata

    let base = match emu.maps.alloc(size + 8) {
        Some(a) => a,
        None => {
            log::error!("oleaut32!SysAllocStringLen out of memory");
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let name = format!("alloc_{:x}", base + 8);
    emu.maps
        .create_map(&name, base, size, Permission::READ_WRITE);
    emu.maps.memcpy(base + 8, str_ptr, size as usize - 1);

    log_red!(emu, "oleaut32!SysAllocStringLen  ={} {}", name, size - 8);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = base + 8;
}

fn SysFreeString(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "oleaut32!SysFreeString  0x{:x}", str_ptr);

    //emu.maps.free(&format!("alloc_{:x}", str_ptr));

    emu.stack_pop32(false);
}

fn SysAllocString32(emu: &mut emu::Emu) {
    let psz = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    if psz == 0 {
        log_red!(emu, "oleaut32!SysAllocString NULL -> 0");
        emu.stack_pop32(false);
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
            emu.stack_pop32(false);
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
    emu.stack_pop32(false);
    emu.regs_mut().rax = bstr;
}

fn SysAllocStringByteLen32(emu: &mut emu::Emu) {
    let psz = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

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
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = bstr;
}

fn SysStringLen32(emu: &mut emu::Emu) {
    let bstr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    if bstr == 0 {
        log_red!(emu, "oleaut32!SysStringLen NULL -> 0");
        emu.stack_pop32(false);
        emu.regs_mut().rax = 0;
        return;
    }

    let len_bytes = emu.maps.read_dword(bstr - 4).unwrap_or(0);
    let len_chars = len_bytes / 2;

    log_red!(emu, "oleaut32!SysStringLen 0x{:x} -> {}", bstr, len_chars);
    emu.stack_pop32(false);
    emu.regs_mut().rax = len_chars as u64;
}

fn SysStringByteLen32(emu: &mut emu::Emu) {
    let bstr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    if bstr == 0 {
        log_red!(emu, "oleaut32!SysStringByteLen NULL -> 0");
        emu.stack_pop32(false);
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
    emu.stack_pop32(false);
    emu.regs_mut().rax = len_bytes as u64;
}

fn VariantInit32(emu: &mut emu::Emu) {
    let pvarg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "oleaut32!VariantInit 0x{:x}", pvarg);

    if pvarg != 0 {
        for i in 0..16u64 {
            emu.maps.write_byte(pvarg + i, 0);
        }
    }
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn SysAllocString(emu: &mut emu::Emu) {
    let psz = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    emu.stack_pop32(false);

    if psz == 0 {
        log_red!(emu, "oleaut32!SysAllocString NULL -> 0");
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
    emu.regs_mut().rax = bstr;
}

fn SysAllocStringByteLen(emu: &mut emu::Emu) {
    let psz = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(
        emu,
        "oleaut32!SysAllocStringByteLen psz: 0x{:x} len: {}",
        psz,
        len
    );

    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }

    let total = 4 + len + 2 + 16;

    let base = match emu.maps.alloc(total) {
        Some(a) => a,
        None => {
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
    emu.regs_mut().rax = bstr;
}

fn SysStringLen(emu: &mut emu::Emu) {
    let bstr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    emu.stack_pop32(false);

    if bstr == 0 {
        log_red!(emu, "oleaut32!SysStringLen NULL -> 0");
        emu.regs_mut().rax = 0;
        return;
    }

    let len_bytes = emu.maps.read_dword(bstr - 4).unwrap_or(0);
    let len_chars = len_bytes / 2;

    log_red!(emu, "oleaut32!SysStringLen 0x{:x} -> {}", bstr, len_chars);
    emu.regs_mut().rax = len_chars as u64;
}

fn SysStringByteLen(emu: &mut emu::Emu) {
    let bstr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    emu.stack_pop32(false);

    if bstr == 0 {
        log_red!(emu, "oleaut32!SysStringByteLen NULL -> 0");
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
    emu.regs_mut().rax = len_bytes as u64;
}
