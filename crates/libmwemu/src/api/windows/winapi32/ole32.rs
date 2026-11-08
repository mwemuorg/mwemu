use crate::api::windows::common::fallback;
use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "OleInitialize" => OleInitialize(emu),
        "CoInitialize" => CoInitialize(emu),
        "CoInitializeEx" => CoInitializeEx(emu),
        "CoCreateInstance" => CoCreateInstance(emu),
        "CoTaskMemAlloc" => CoTaskMemAlloc(emu),
        "CoTaskMemFree" => CoTaskMemFree(emu),
        "CoUninitialize" => CoUninitialize(emu),
        "BindMoniker" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!BindMoniker a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://BindMoniker");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserFree a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserFreeExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserFreeExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserMarshalExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserMarshalExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserSize a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserSizeExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserSizeExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLIPFORMAT_UserUnmarshalExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserUnmarshalExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CLSIDFromOle1Class" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CLSIDFromOle1Class a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CLSIDFromProgID" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CLSIDFromProgID a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CLSIDFromProgIDEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CLSIDFromProgIDEx a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CLSIDFromString" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CLSIDFromString a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CStdAsyncStubBuffer2_Connect" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer2_Connect a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CStdAsyncStubBuffer2_Connect");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdAsyncStubBuffer2_Disconnect" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer2_Disconnect a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdAsyncStubBuffer2_Release" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer2_Release a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdAsyncStubBuffer_AddRef" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_AddRef a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CStdAsyncStubBuffer_AddRef");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdAsyncStubBuffer_Connect" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Connect a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CStdAsyncStubBuffer_Connect");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdAsyncStubBuffer_Disconnect" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Disconnect a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdAsyncStubBuffer_Invoke" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Invoke a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdAsyncStubBuffer_QueryInterface" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_QueryInterface a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdAsyncStubBuffer_Release" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Release a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdStubBuffer2_Connect" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_Connect a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CStdStubBuffer2_Connect");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdStubBuffer2_CountRefs" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_CountRefs a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdStubBuffer2_Disconnect" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_Disconnect a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CStdStubBuffer2_QueryInterface" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_QueryInterface a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CheckInitDde" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CheckInitDde a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CheckInitDde");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CleanROTForApartment" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CleanROTForApartment a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ClipboardProcessUninitialize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ClipboardProcessUninitialize a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CoAddRefServerProcess" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoAddRefServerProcess a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoAddRefServerProcess");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoAicGetTokenForCOM" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoAicGetTokenForCOM a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoAllowSetForegroundWindow" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoAllowSetForegroundWindow a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoAllowUnmarshalerCLSID" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoAllowUnmarshalerCLSID a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoBuildVersion" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoBuildVersion a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoCancelCall" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoCancelCall a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoCheckElevationEnabled" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoCheckElevationEnabled a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoCheckElevationEnabled");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoCopyProxy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoCopyProxy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoCreateFreeThreadedMarshaler" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoCreateFreeThreadedMarshaler a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoCreateFreeThreadedMarshaler");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoCreateGuid" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoCreateGuid a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoCreateInstanceEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoCreateInstanceEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoCreateInstanceEx");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoCreateInstanceFromApp" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoCreateInstanceFromApp a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoCreateInstanceFromApp");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoCreateObjectInContext" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoCreateObjectInContext a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoCreateObjectInContext");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoDeactivateObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoDeactivateObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoDecodeProxy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CoDecodeProxy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CoDecrementMTAUsage" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoDecrementMTAUsage a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoDisableCallCancellation" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoDisableCallCancellation a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoDisconnectContext" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoDisconnectContext a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoDisconnectObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoDisconnectObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoDosDateTimeToFileTime" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoDosDateTimeToFileTime a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CoEnableCallCancellation" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoEnableCallCancellation a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoEnableCallCancellation");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoFileTimeNow" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoFileTimeNow a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoFileTimeToDosDateTime" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoFileTimeToDosDateTime a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CoFreeAllLibraries" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoFreeAllLibraries a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoFreeLibrary" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoFreeLibrary a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoFreeUnusedLibraries" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoFreeUnusedLibraries a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoFreeUnusedLibrariesEx" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoFreeUnusedLibrariesEx a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoGetActivationState" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoGetActivationState a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetActivationState");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetApartmentID" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoGetApartmentID a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetApartmentID");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetApartmentType" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoGetApartmentType a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetApartmentType");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetCallContext" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoGetCallContext a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetCallContext");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetCallState" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoGetCallState a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetCallState");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetCallerTID" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoGetCallerTID a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoGetCallerTID");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoGetCancelObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CoGetCancelObject a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetCancelObject");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoGetClassObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CoGetClassObject a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetClassObject");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoGetClassVersion" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CoGetClassVersion a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetClassVersion");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "CoGetComCatalog" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoGetComCatalog a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetComCatalog");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetContextToken" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoGetContextToken a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoGetContextToken");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoGetCurrentLogicalThreadId" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoGetCurrentLogicalThreadId a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoGetCurrentLogicalThreadId");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoGetCurrentProcess" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoGetCurrentProcess a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoGetCurrentProcess");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoGetDefaultContext" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoGetDefaultContext a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetDefaultContext");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CoGetInstanceFromFile" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoGetInstanceFromFile a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetInstanceFromFile");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoGetInstanceFromIStorage" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoGetInstanceFromIStorage a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetInstanceFromIStorage");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoGetInterceptor" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CoGetInterceptor a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetInterceptor");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "CoGetInterceptorForOle32" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoGetInterceptorForOle32 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetInterceptorForOle32");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "CoGetInterceptorFromTypeInfo" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoGetInterceptorFromTypeInfo a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetInterceptorFromTypeInfo");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "CoGetInterfaceAndReleaseStream" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoGetInterfaceAndReleaseStream a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoGetMalloc" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoGetMalloc a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetMalloc");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetMarshalSizeMax" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoGetMarshalSizeMax a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetMarshalSizeMax");
            let out_ptr = arg(emu, 6) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 7);
        }
        "CoGetModuleType" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoGetModuleType a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetModuleType");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CoGetObject a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetObject");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoGetObjectContext" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoGetObjectContext a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetObjectContext");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoGetPSClsid" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoGetPSClsid a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetPSClsid");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetProcessIdentifier" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoGetProcessIdentifier a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoGetProcessIdentifier");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoGetStandardMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoGetStandardMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetStandardMarshal");
            let out_ptr = arg(emu, 6) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 7);
        }
        "CoGetStdMarshalEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CoGetStdMarshalEx a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetStdMarshalEx");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CoGetSystemSecurityPermissions" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoGetSystemSecurityPermissions a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetSystemSecurityPermissions");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetSystemWow64DirectoryW" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoGetSystemWow64DirectoryW a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoGetSystemWow64DirectoryW");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoGetTreatAsClass" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoGetTreatAsClass a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoGetTreatAsClass");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoHandlePriorityEventsFromMessagePump" => {
            let _a0 = arg(emu, 0);
            log_red!(
                emu,
                "ole32!CoHandlePriorityEventsFromMessagePump a0:0x{:x}",
                _a0
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CoImpersonateClient" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoImpersonateClient a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoIncrementMTAUsage" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoIncrementMTAUsage a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoIncrementMTAUsage");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoInitializeSecurity" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoInitializeSecurity a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoInitializeSecurity");
            let out_ptr = arg(emu, 6) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 7);
        }
        "CoInitializeWOW" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoInitializeWOW a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoInitializeWOW");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoInstall" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CoInstall a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoInstall");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoInvalidateRemoteMachineBindings" => {
            let _a0 = arg(emu, 0);
            log_red!(
                emu,
                "ole32!CoInvalidateRemoteMachineBindings a0:0x{:x}",
                _a0
            );
            let _h = helper::handler_create("ole32://CoInvalidateRemoteMachineBindings");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoIsHandlerConnected" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoIsHandlerConnected a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoIsOle1Class" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoIsOle1Class a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoLoadLibrary" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoLoadLibrary a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoLoadLibrary");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoLockObjectExternal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoLockObjectExternal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CoMarshalHresult" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoMarshalHresult a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoMarshalHresult");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoMarshalInterThreadInterfaceInStream" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoMarshalInterThreadInterfaceInStream a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoMarshalInterThreadInterfaceInStream");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CoMarshalInterface" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoMarshalInterface a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoMarshalInterface");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "CoPopServiceDomain" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoPopServiceDomain a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoPushServiceDomain" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoPushServiceDomain a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoQueryAuthenticationServices" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoQueryAuthenticationServices a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoQueryClientBlanket" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoQueryClientBlanket a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 7);
        }
        "CoQueryProxyBlanket" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoQueryProxyBlanket a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 7);
        }
        "CoQueryReleaseObject" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoQueryReleaseObject a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoReactivateObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoReactivateObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoRegisterActivationFilter" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoRegisterActivationFilter a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoRegisterActivationFilter");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoRegisterChannelHook" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoRegisterChannelHook a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoRegisterChannelHook");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoRegisterClassObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoRegisterClassObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoRegisterClassObject");
            let out_ptr = arg(emu, 4) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 5);
        }
        "CoRegisterInitializeSpy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoRegisterInitializeSpy a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoRegisterInitializeSpy");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoRegisterMallocSpy" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoRegisterMallocSpy a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoRegisterMallocSpy");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoRegisterMessageFilter" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoRegisterMessageFilter a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoRegisterMessageFilter");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoRegisterPSClsid" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoRegisterPSClsid a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CoRegisterPSClsid");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoRegisterSurrogate" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoRegisterSurrogate a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoRegisterSurrogate");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoRegisterSurrogateEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoRegisterSurrogateEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CoRegisterSurrogateEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoReleaseMarshalData" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoReleaseMarshalData a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoReleaseServerProcess" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoReleaseServerProcess a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoResumeClassObjects" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoResumeClassObjects a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoRetireServer" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoRetireServer a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoRevertToSelf" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoRevertToSelf a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoRevokeClassObject" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoRevokeClassObject a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoRevokeInitializeSpy" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoRevokeInitializeSpy a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoRevokeMallocSpy" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoRevokeMallocSpy a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoSetCancelObject" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoSetCancelObject a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoSetCancelObject");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoSetMessageDispatcher" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoSetMessageDispatcher a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoSetMessageDispatcher");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoSetProxyBlanket" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CoSetProxyBlanket a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "CoSetState" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoSetState a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CoSetState");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoSuspendClassObjects" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoSuspendClassObjects a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoSwitchCallContext" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoSwitchCallContext a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoTaskMemRealloc" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoTaskMemRealloc a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoTestCancel" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoTestCancel a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoTreatAsClass" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CoTreatAsClass a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoTryGetInterceptorForAppModelVirtualization" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoTryGetInterceptorForAppModelVirtualization a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "CoUnloadingWOW" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoUnloadingWOW a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoUnmarshalHresult" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CoUnmarshalHresult a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CoUnmarshalInterface" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoUnmarshalInterface a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CoVrfCheckThreadState" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoVrfCheckThreadState a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoVrfGetThreadState" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoVrfGetThreadState a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoVrfReleaseThreadState" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CoVrfReleaseThreadState a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CoWaitForMultipleHandles" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoWaitForMultipleHandles a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "CoWaitForMultipleObjects" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CoWaitForMultipleObjects a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "ComPs_NdrDllCanUnloadNow" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ComPs_NdrDllCanUnloadNow a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "ComPs_NdrDllGetClassObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!ComPs_NdrDllGetClassObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://ComPs_NdrDllGetClassObject");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "ComPs_NdrDllRegisterProxy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!ComPs_NdrDllRegisterProxy a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://ComPs_NdrDllRegisterProxy");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "ComPs_NdrDllUnregisterProxy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!ComPs_NdrDllUnregisterProxy a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CreateAntiMoniker" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CreateAntiMoniker a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CreateAntiMoniker");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CreateBindCtx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CreateBindCtx a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CreateBindCtx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CreateClassMoniker" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CreateClassMoniker a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CreateClassMoniker");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CreateDataAdviseHolder" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CreateDataAdviseHolder a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CreateDataAdviseHolder");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CreateDataCache" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CreateDataCache a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CreateDataCache");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "CreateErrorInfo" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CreateErrorInfo a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CreateErrorInfo");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CreateFileMoniker" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!CreateFileMoniker a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CreateFileMoniker");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CreateGenericComposite" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CreateGenericComposite a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CreateGenericComposite");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CreateILockBytesOnHGlobal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CreateILockBytesOnHGlobal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CreateILockBytesOnHGlobal");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CreateItemMoniker" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!CreateItemMoniker a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://CreateItemMoniker");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CreateObjrefMoniker" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CreateObjrefMoniker a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CreateObjrefMoniker");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CreateOleAdviseHolder" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!CreateOleAdviseHolder a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://CreateOleAdviseHolder");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "CreatePointerMoniker" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!CreatePointerMoniker a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CreatePointerMoniker");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CreateStdProgressIndicator" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CreateStdProgressIndicator a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CreateStdProgressIndicator");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "CreateStreamOnHGlobal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!CreateStreamOnHGlobal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://CreateStreamOnHGlobal");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "DcomChannelSetHResult" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!DcomChannelSetHResult a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeBindToObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!DdeBindToObject a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://DdeBindToObject");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DeletePatternAndExtensionTables" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!DeletePatternAndExtensionTables a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "DestroyRunningObjectTable" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!DestroyRunningObjectTable a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "DllDebugObjectRPCHook" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!DllDebugObjectRPCHook a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DllGetClassObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!DllGetClassObject a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DllGetClassObjectWOW" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!DllGetClassObjectWOW a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "DllRegisterServer" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!DllRegisterServer a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://DllRegisterServer");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DoDragDrop" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!DoDragDrop a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DragDropSetFDT" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!DragDropSetFDT a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EnableHookObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!EnableHookObject a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://EnableHookObject");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FindExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!FindExt a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FmtIdToPropStgName" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!FmtIdToPropStgName a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FreePropVariantArray" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!FreePropVariantArray a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "GetActiveObjectExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!GetActiveObjectExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClassFile" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!GetClassFile a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetConvertStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!GetConvertStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDocumentBitStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!GetDocumentBitStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetErrorInfo" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!GetErrorInfo a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetHGlobalFromILockBytes" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!GetHGlobalFromILockBytes a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://GetHGlobalFromILockBytes");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetHGlobalFromStream" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!GetHGlobalFromStream a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://GetHGlobalFromStream");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetHookInterface" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!GetHookInterface a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetObjectFromRotByPath" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!GetObjectFromRotByPath a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetRunningObjectTable" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!GetRunningObjectTable a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://GetRunningObjectTable");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "HACCEL_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HACCEL_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HACCEL_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HACCEL_UserFree64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HACCEL_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HACCEL_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HACCEL_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HACCEL_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HACCEL_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HACCEL_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HACCEL_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HACCEL_UserSize64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HACCEL_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HACCEL_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HACCEL_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HACCEL_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBITMAP_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HBITMAP_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBITMAP_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBITMAP_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBITMAP_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBITMAP_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBITMAP_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBITMAP_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBITMAP_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HBITMAP_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBITMAP_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBITMAP_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBITMAP_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBITMAP_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBITMAP_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBITMAP_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBRUSH_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HBRUSH_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBRUSH_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HBRUSH_UserFree64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBRUSH_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBRUSH_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBRUSH_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBRUSH_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBRUSH_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HBRUSH_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBRUSH_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HBRUSH_UserSize64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBRUSH_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBRUSH_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HBRUSH_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HBRUSH_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HDC_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HDC_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HDC_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HDC_UserFree64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HDC_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HDC_UserMarshal a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HDC_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HDC_UserMarshal64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HDC_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HDC_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HDC_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HDC_UserSize64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HDC_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HDC_UserUnmarshal a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HDC_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HDC_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HENHMETAFILE_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserFree a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HENHMETAFILE_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HENHMETAFILE_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HENHMETAFILE_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HENHMETAFILE_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserSize a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HENHMETAFILE_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HENHMETAFILE_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HENHMETAFILE_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HGLOBAL_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HGLOBAL_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HGLOBAL_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HGLOBAL_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HGLOBAL_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HGLOBAL_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HGLOBAL_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HGLOBAL_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HGLOBAL_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HGLOBAL_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HGLOBAL_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HGLOBAL_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HGLOBAL_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HGLOBAL_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HGLOBAL_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HGLOBAL_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HICON_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HICON_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HICON_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HICON_UserFree64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HICON_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HICON_UserMarshal a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HICON_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HICON_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HICON_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HICON_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HICON_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HICON_UserSize64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HICON_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HICON_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HICON_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HICON_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMENU_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HMENU_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMENU_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HMENU_UserFree64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMENU_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HMENU_UserMarshal a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMENU_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMENU_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMENU_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HMENU_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMENU_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HMENU_UserSize64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMENU_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMENU_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMENU_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMENU_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMETAFILEPICT_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserFree a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "HMETAFILEPICT_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "HMETAFILEPICT_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "HMETAFILEPICT_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "HMETAFILEPICT_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserSize a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "HMETAFILEPICT_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "HMETAFILEPICT_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "HMETAFILEPICT_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "HMETAFILE_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILE_UserFree a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMETAFILE_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILE_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMETAFILE_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILE_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMETAFILE_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILE_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMETAFILE_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILE_UserSize a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMETAFILE_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILE_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMETAFILE_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILE_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMETAFILE_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMETAFILE_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMONITOR_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HMONITOR_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMONITOR_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMONITOR_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMONITOR_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMONITOR_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMONITOR_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMONITOR_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMONITOR_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HMONITOR_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMONITOR_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMONITOR_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMONITOR_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMONITOR_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HMONITOR_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HMONITOR_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HPALETTE_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserFreeExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserFreeExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserMarshalExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserMarshalExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HPALETTE_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserSizeExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserSizeExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HPALETTE_UserUnmarshalExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HPALETTE_UserUnmarshalExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HRGN_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HRGN_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HRGN_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HRGN_UserMarshal a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HRGN_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HRGN_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HRGN_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HRGN_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HWND_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HWND_UserFree64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserFree64Ext" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HWND_UserFree64Ext a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserFreeExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HWND_UserFreeExt a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HWND_UserMarshal a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HWND_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserMarshal64Ext" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HWND_UserMarshal64Ext a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserMarshalExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HWND_UserMarshalExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HWND_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HWND_UserSize64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserSize64Ext" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HWND_UserSize64Ext a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserSizeExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!HWND_UserSizeExt a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HWND_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HWND_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserUnmarshal64Ext" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HWND_UserUnmarshal64Ext a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HWND_UserUnmarshalExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HWND_UserUnmarshalExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HkOleRegisterObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!HkOleRegisterObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://HkOleRegisterObject");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IIDFromString" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!IIDFromString a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "IsAccelerator" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!IsAccelerator a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsEqualGUID" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!IsEqualGUID a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsRoInitializeASTAAllowedInDesktop" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!IsRoInitializeASTAAllowedInDesktop a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://IsRoInitializeASTAAllowedInDesktop");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsValidIid" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!IsValidIid a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsValidInterface" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!IsValidInterface a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsValidPtrIn" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!IsValidPtrIn a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsValidPtrOut" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!IsValidPtrOut a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MkParseDisplayName" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!MkParseDisplayName a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MonikerCommonPrefixWith" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!MonikerCommonPrefixWith a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MonikerLoadTypeLib" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!MonikerLoadTypeLib a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://MonikerLoadTypeLib");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MonikerRelativePathTo" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!MonikerRelativePathTo a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrOleInitializeExtension" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrOleInitializeExtension a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction10" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction10 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction11" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction11 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction12" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction12 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction13" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction13 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction14" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction14 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction15" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction15 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction16" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction16 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction17" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction17 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction18" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction18 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction19" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction19 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction20" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction20 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction21" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction21 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction22" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction22 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction23" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction23 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction24" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction24 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction25" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction25 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction26" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction26 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction27" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction27 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction28" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction28 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction29" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction29 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction3" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction3 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction30" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction30 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction31" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction31 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction32" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction32 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction4 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction5" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction5 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction6" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction6 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction7" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction7 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction8 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NdrProxyForwardingFunction9" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction9 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient10" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient10 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient11" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient11 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient12" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient12 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient13" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient13 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient14" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient14 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient15" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient15 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient16" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient16 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient17" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient17 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient18" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient18 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient19" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient19 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient20" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient20 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient21" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient21 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient22" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient22 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient23" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient23 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient24" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient24 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient25" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient25 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient26" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient26 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient27" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient27 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient28" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient28 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient29" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient29 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient3" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient3 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient30" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient30 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient31" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient31 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient32" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient32 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient4" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient4 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient5" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient5 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient6" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient6 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient7" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient7 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient8" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient8 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ObjectStublessClient9" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ObjectStublessClient9 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "Ole32DllGetClassObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!Ole32DllGetClassObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleBuildVersion" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleBuildVersion a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleConvertIStorageToOLESTREAM" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleConvertIStorageToOLESTREAM a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleConvertIStorageToOLESTREAMEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleConvertIStorageToOLESTREAMEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 7);
        }
        "OleConvertOLESTREAMToIStorage" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorage a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleConvertOLESTREAMToIStorage2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorage2 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleConvertOLESTREAMToIStorageEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorageEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 7);
        }
        "OleConvertOLESTREAMToIStorageEx2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorageEx2 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 7);
        }
        "OleCreate" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleCreate a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleCreate");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateDefaultHandler" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateDefaultHandler a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateDefaultHandler");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateEmbeddingHelper" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateEmbeddingHelper a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateEmbeddingHelper");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleCreateEx a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleCreateEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateFontIndirectExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateFontIndirectExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateFontIndirectExt");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateFromData" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleCreateFromData a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleCreateFromData");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateFromDataEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateFromDataEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateFromDataEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateFromFile" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleCreateFromFile a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleCreateFromFile");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateFromFileEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateFromFileEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateFromFileEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateLink" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleCreateLink a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleCreateLink");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateLinkEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleCreateLinkEx a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleCreateLinkEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateLinkFromData" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateLinkFromData a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateLinkFromData");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateLinkFromDataEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateLinkFromDataEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateLinkFromDataEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateLinkToFile" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateLinkToFile a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateLinkToFile");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateLinkToFileEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateLinkToFileEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateLinkToFileEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateMenuDescriptor" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateMenuDescriptor a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateMenuDescriptor");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreatePictureIndirectExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreatePictureIndirectExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreatePictureIndirectExt");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreatePropertyFrameIndirectExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreatePropertyFrameIndirectExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreatePropertyFrameIndirectExt");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleCreateStaticFromData" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleCreateStaticFromData a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleCreateStaticFromData");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleDestroyMenuDescriptor" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleDestroyMenuDescriptor a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleDoAutoConvert" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleDoAutoConvert a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleDraw" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!OleDraw a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "OleDuplicateData" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!OleDuplicateData a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleDuplicateData");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleFlushClipboard" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleFlushClipboard a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleGetAutoConvert" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleGetAutoConvert a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleGetClipboard" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleGetClipboard a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://OleGetClipboard");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleGetClipboardWithEnterpriseInfo" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleGetClipboardWithEnterpriseInfo a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleGetIconOfClass" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!OleGetIconOfClass a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleGetIconOfClass");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleGetIconOfFile" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleGetIconOfFile a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleGetIconOfFile");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleGetPackageClipboardOwner" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleGetPackageClipboardOwner a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "OleIconToCursorExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleIconToCursorExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleInitializeWOW" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleInitializeWOW a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleInitializeWOW");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleIsCurrentClipboard" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleIsCurrentClipboard a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleIsRunning" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleIsRunning a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleLoad" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!OleLoad a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleLoad");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "OleLoadFromStream" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!OleLoadFromStream a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleLoadFromStream");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleLoadPictureExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!OleLoadPictureExt a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleLoadPictureExt");
            let out_ptr = arg(emu, 5) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "OleLoadPictureFileExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleLoadPictureFileExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleLoadPictureFileExt");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleLoadPicturePathExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleLoadPicturePathExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleLoadPicturePathExt");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleLockRunning" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!OleLockRunning a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleLockRunning");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleMetafilePictFromIconAndLabel" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleMetafilePictFromIconAndLabel a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleMetafilePictFromIconAndLabel");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "OleNoteObjectVisible" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleNoteObjectVisible a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleQueryCreateFromData" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleQueryCreateFromData a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleQueryCreateFromData");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleQueryLinkFromData" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleQueryLinkFromData a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "OleRegEnumFormatEtc" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleRegEnumFormatEtc a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleRegEnumVerbs" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleRegEnumVerbs a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleRegGetMiscStatus" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleRegGetMiscStatus a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleRegGetUserType" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!OleRegGetUserType a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleReleaseEnumVerbCache" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleReleaseEnumVerbCache a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleRun" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleRun a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://OleRun");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleSave" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!OleSave a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleSave");
            let out_ptr = arg(emu, 2) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleSavePictureFileExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleSavePictureFileExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://OleSavePictureFileExt");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleSaveToStream" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleSaveToStream a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://OleSaveToStream");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleSetAutoConvert" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!OleSetAutoConvert a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleSetClipboard" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleSetClipboard a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://OleSetClipboard");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OleSetContainedObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!OleSetContainedObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "OleSetMenuDescriptor" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleSetMenuDescriptor a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "OleTranslateAccelerator" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleTranslateAccelerator a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleTranslateColorExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!OleTranslateColorExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "OleUninitialize" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OleUninitialize a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "OpenOrCreateStream" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!OpenOrCreateStream a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://OpenOrCreateStream");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "ProgIDFromCLSID" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!ProgIDFromCLSID a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "PropStgNameToFmtId" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!PropStgNameToFmtId a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "PropSysAllocString" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!PropSysAllocString a0:0x{:x}", _a0);
            let _h = helper::handler_create("ole32://PropSysAllocString");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "PropSysFreeString" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!PropSysFreeString a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "PropVariantChangeType" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!PropVariantChangeType a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "PropVariantClear" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!PropVariantClear a0:0x{:x}", _a0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "PropVariantCopy" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!PropVariantCopy a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "ReadClassStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!ReadClassStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReadClassStm" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!ReadClassStm a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReadFmtUserTypeStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ReadFmtUserTypeStg a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReadOleStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!ReadOleStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReadStorageProperties" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!ReadStorageProperties a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReadStringStream" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!ReadStringStream a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RegisterActiveObjectExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!RegisterActiveObjectExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://RegisterActiveObjectExt");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RegisterDragDrop" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!RegisterDragDrop a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://RegisterDragDrop");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReleaseStgMedium" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!ReleaseStgMedium a0:0x{:x}", _a0);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "RevokeActiveObjectExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!RevokeActiveObjectExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RevokeDragDrop" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!RevokeDragDrop a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RoGetAgileReference" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!RoGetAgileReference a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SNB_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SNB_UserFree a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SNB_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SNB_UserFree64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SNB_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SNB_UserMarshal a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SNB_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SNB_UserMarshal64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SNB_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SNB_UserSize a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SNB_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SNB_UserSize64 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SNB_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SNB_UserUnmarshal a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SNB_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!SNB_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserFree a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserFreeExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserFreeExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserMarshalExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserMarshalExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserSize a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserSizeExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserSizeExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "STGMEDIUM_UserUnmarshalExt" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserUnmarshalExt a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SetConvertStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SetConvertStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetDocumentBitStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SetDocumentBitStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetErrorInfo" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SetErrorInfo a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "SetOleautModule" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!SetOleautModule a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWOWThunkGlobalPtr" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!SetWOWThunkGlobalPtr a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StdTypesGetClassObject" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StdTypesGetClassObject a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StdTypesRegisterServer" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StdTypesRegisterServer a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://StdTypesRegisterServer");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgConvertPropertyToVariant" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StgConvertPropertyToVariant a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgConvertVariantToProperty" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StgConvertVariantToProperty a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgCreateDocfile" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!StgCreateDocfile a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://StgCreateDocfile");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgCreateDocfileOnILockBytes" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!StgCreateDocfileOnILockBytes a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://StgCreateDocfileOnILockBytes");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "StgCreatePropSetStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StgCreatePropSetStg a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://StgCreatePropSetStg");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgCreatePropStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!StgCreatePropStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://StgCreatePropStg");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgCreateStorageEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StgCreateStorageEx a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://StgCreateStorageEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgGetIFillLockBytesOnFile" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StgGetIFillLockBytesOnFile a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://StgGetIFillLockBytesOnFile");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "StgGetIFillLockBytesOnILockBytes" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StgGetIFillLockBytesOnILockBytes a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://StgGetIFillLockBytesOnILockBytes");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "StgIsStorageFile" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!StgIsStorageFile a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgIsStorageILockBytes" => {
            let _a0 = arg(emu, 0);
            log_red!(emu, "ole32!StgIsStorageILockBytes a0:0x{:x}", _a0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "StgOpenAsyncDocfileOnIFillLockBytes" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!StgOpenAsyncDocfileOnIFillLockBytes a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://StgOpenAsyncDocfileOnIFillLockBytes");
            let out_ptr = arg(emu, 3) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "StgOpenPropStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!StgOpenPropStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://StgOpenPropStg");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgOpenStorage" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!StgOpenStorage a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://StgOpenStorage");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgOpenStorageEx" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!StgOpenStorageEx a0:0x{:x} a1:0x{:x}", _a0, _a1);
            let hndl = helper::handler_create("ole32://StgOpenStorageEx");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgOpenStorageOnHandle" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StgOpenStorageOnHandle a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://StgOpenStorageOnHandle");
            let out_ptr = arg(emu, 1) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgOpenStorageOnILockBytes" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(
                emu,
                "ole32!StgOpenStorageOnILockBytes a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let hndl = helper::handler_create("ole32://StgOpenStorageOnILockBytes");
            let out_ptr = arg(emu, 6) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, hndl as u32);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 7);
        }
        "StgPropertyLengthAsVariant" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!StgPropertyLengthAsVariant a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StgSetTimes" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!StgSetTimes a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "StringFromCLSID" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!StringFromCLSID a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "StringFromGUID2" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!StringFromGUID2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "StringFromIID" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!StringFromIID a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "UpdateDCOMSettings" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!UpdateDCOMSettings a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UpdateProcessTracing" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!UpdateProcessTracing a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UtConvertDvtd16toDvtd32" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!UtConvertDvtd16toDvtd32 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UtConvertDvtd32toDvtd16" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!UtConvertDvtd32toDvtd16 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UtGetDvtd16Info" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!UtGetDvtd16Info a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UtGetDvtd32Info" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!UtGetDvtd32Info a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WdtpInterfacePointer_UserFree" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserFree a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "WdtpInterfacePointer_UserFree64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserFree64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            let h = _a0 as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "WdtpInterfacePointer_UserMarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserMarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "WdtpInterfacePointer_UserMarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserMarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "WdtpInterfacePointer_UserSize" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserSize a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "WdtpInterfacePointer_UserSize64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserSize64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "WdtpInterfacePointer_UserUnmarshal" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserUnmarshal a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "WdtpInterfacePointer_UserUnmarshal64" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserUnmarshal64 a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "WriteClassStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!WriteClassStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WriteClassStm" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!WriteClassStm a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WriteFmtUserTypeStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WriteFmtUserTypeStg a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WriteOleStg" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(emu, "ole32!WriteOleStg a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WriteStorageProperties" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            log_red!(
                emu,
                "ole32!WriteStorageProperties a0:0x{:x} a1:0x{:x}",
                _a0,
                _a1
            );
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WriteStringStream" => {
            let _a0 = arg(emu, 0);
            let _a1 = arg(emu, 1);
            let _a2 = arg(emu, 2);
            log_red!(emu, "ole32!WriteStringStream a0:0x{:x} a1:0x{:x}", _a0, _a1);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 5);
        }
        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }
    String::new()
}

fn OleInitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!OleInitialize");
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 1);
}

fn CoInitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoInitialize");
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 1);
}

fn CoInitializeEx(emu: &mut emu::Emu) {
    let co_init = arg(emu, 1);
    log_red!(emu, "ole32!CoInitializeEx dwCoInit: 0x{:x}", co_init);
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 2);
}

fn CoCreateInstance(emu: &mut emu::Emu) {
    let ppv = arg(emu, 4) as u64;
    log_red!(emu, "ole32!CoCreateInstance ppv: 0x{:x}", ppv);
    if ppv != 0 {
        emu.maps.write_dword(ppv, 0);
    }
    emu.regs_mut().rax = constants::REGDB_E_CLASSNOTREG;
    stdcall_ret(emu, 5);
}

fn CoTaskMemAlloc(emu: &mut emu::Emu) {
    let size = arg(emu, 0) as u64;
    log_red!(emu, "ole32!CoTaskMemAlloc size: 0x{:x}", size);
    emu.regs_mut().rax = heap_engine::heap_allocate(emu, 0, size).unwrap_or(0);
    stdcall_ret(emu, 1);
}

fn CoTaskMemFree(emu: &mut emu::Emu) {
    let pv = arg(emu, 0) as u64;
    log_red!(emu, "ole32!CoTaskMemFree pv: 0x{:x}", pv);
    if pv != 0 {
        heap_engine::heap_free(emu, 0, pv);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn CoUninitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoUninitialize");
    emu.regs_mut().rax = 0;
}
