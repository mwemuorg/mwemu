use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32;
use crate::windows::constants;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "OleInitialize" => OleInitialize(emu),
        "CoInitialize" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoInitialize arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoInitializeEx" => CoInitializeEx(emu),
        "CoCreateInstance" => CoCreateInstance(emu),
        "CoTaskMemAlloc" => CoTaskMemAlloc(emu),
        "CoTaskMemFree" => CoTaskMemFree(emu),
        "CoUninitialize" => CoUninitialize(emu),
        "CoResumeClassObjects" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoResumeClassObjects arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoAddRefServerProcess" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoAddRefServerProcess arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoReleaseServerProcess" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoReleaseServerProcess arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateClassMoniker" => {
            let rclsid = emu.regs().rcx;
            let ppmk = emu.regs().rdx;
            let hndl = helper::handler_create("ole32://IMoniker");
            if ppmk != 0 {
                emu.maps.write_qword(ppmk, hndl);
            }
            log_red!(emu, "ole32!CreateClassMoniker =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "GetRunningObjectTable" => {
            let reserved = emu.regs().rcx;
            let pp_rot = emu.regs().rdx;
            let hndl = helper::handler_create("ole32://IRunningObjectTable");
            if pp_rot != 0 {
                emu.maps.write_qword(pp_rot, hndl);
            }
            log_red!(emu, "ole32!GetRunningObjectTable =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "BindMoniker" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!BindMoniker h:0x{:x} arg2:0x{:x}", handle, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CLIPFORMAT_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserFreeExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserFreeExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserMarshalExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserMarshalExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserSizeExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserSizeExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLIPFORMAT_UserUnmarshalExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserUnmarshalExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CLSIDFromOle1Class" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLSIDFromOle1Class arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CLSIDFromProgID" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLSIDFromProgID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CLSIDFromProgIDEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CLSIDFromProgIDEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CLSIDFromString" => CLSIDFromString(emu),
        "CStdAsyncStubBuffer2_Connect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer2_Connect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdAsyncStubBuffer2_Disconnect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer2_Disconnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdAsyncStubBuffer2_Release" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer2_Release arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdAsyncStubBuffer_AddRef" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_AddRef arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdAsyncStubBuffer_Connect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Connect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdAsyncStubBuffer_Disconnect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Disconnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdAsyncStubBuffer_Invoke" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Invoke arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdAsyncStubBuffer_QueryInterface" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_QueryInterface arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdAsyncStubBuffer_Release" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Release arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdStubBuffer2_Connect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_Connect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdStubBuffer2_CountRefs" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_CountRefs arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdStubBuffer2_Disconnect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_Disconnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CStdStubBuffer2_QueryInterface" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_QueryInterface arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckInitDde" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CheckInitDde arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CleanROTForApartment" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CleanROTForApartment arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClipboardProcessUninitialize" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ClipboardProcessUninitialize h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CoAicGetTokenForCOM" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoAicGetTokenForCOM arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoAllowSetForegroundWindow" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoAllowSetForegroundWindow arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoAllowUnmarshalerCLSID" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoAllowUnmarshalerCLSID arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoBuildVersion" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoBuildVersion arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoCancelCall" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoCancelCall arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoCheckElevationEnabled" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoCheckElevationEnabled arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoCopyProxy" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoCopyProxy arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoCreateFreeThreadedMarshaler" => {
            let punk_outer = emu.regs().rcx;
            let ppunk_marshal = emu.regs().rdx;
            let hndl = helper::handler_create("ole32://IMarshal");
            if ppunk_marshal != 0 {
                emu.maps.write_qword(ppunk_marshal, hndl);
            }
            log_red!(emu, "ole32!CoCreateFreeThreadedMarshaler =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoCreateGuid" => CoCreateGuid(emu),
        "CoCreateInstanceEx" => {
            let clsid_ptr = emu.regs().rcx;
            let outer = emu.regs().rdx;
            let ctx = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_object");
            let ppv_addr = ppv;
            if ppv_addr != 0 {
                emu.maps.write_qword(ppv_addr, hndl);
            }
            log_red!(
                emu,
                "ole32!CoCreateInstanceEx ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoCreateInstanceFromApp" => {
            let clsid_ptr = emu.regs().rcx;
            let outer = emu.regs().rdx;
            let ctx = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_object");
            let ppv_addr = ppv;
            if ppv_addr != 0 {
                emu.maps.write_qword(ppv_addr, hndl);
            }
            log_red!(
                emu,
                "ole32!CoCreateInstanceFromApp ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoCreateObjectInContext" => {
            let clsid_ptr = emu.regs().rcx;
            let outer = emu.regs().rdx;
            let ctx = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_object");
            let ppv_addr = ppv;
            if ppv_addr != 0 {
                emu.maps.write_qword(ppv_addr, hndl);
            }
            log_red!(
                emu,
                "ole32!CoCreateObjectInContext ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoDeactivateObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoDeactivateObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoDecodeProxy" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoDecodeProxy arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoDecrementMTAUsage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoDecrementMTAUsage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoDisableCallCancellation" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoDisableCallCancellation arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoDisconnectContext" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoDisconnectContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoDisconnectObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoDisconnectObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoDosDateTimeToFileTime" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoDosDateTimeToFileTime arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoEnableCallCancellation" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoEnableCallCancellation arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoFileTimeNow" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoFileTimeNow arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoFileTimeToDosDateTime" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoFileTimeToDosDateTime arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoFreeAllLibraries" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoFreeAllLibraries arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoFreeLibrary" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoFreeLibrary arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoFreeUnusedLibraries" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoFreeUnusedLibraries arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoFreeUnusedLibrariesEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoFreeUnusedLibrariesEx arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetActivationState" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetActivationState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetApartmentID" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetApartmentID arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetApartmentType" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetApartmentType arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetCallContext" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetCallContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetCallState" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetCallState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetCallerTID" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetCallerTID arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetCancelObject" => {
            let clsid_ptr = emu.regs().rcx;
            let outer = emu.regs().rdx;
            let ctx = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_object");
            let ppv_addr = ppv;
            if ppv_addr != 0 {
                emu.maps.write_qword(ppv_addr, hndl);
            }
            log_red!(
                emu,
                "ole32!CoGetCancelObject ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetClassObject" => {
            let clsid_ptr = emu.regs().rcx;
            let ctx = emu.regs().rdx;
            let server_info = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_class_factory");
            if ppv != 0 {
                emu.maps.write_qword(ppv, hndl);
            }
            log_red!(
                emu,
                "ole32!CoGetClassObject ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetClassVersion" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetClassVersion arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetComCatalog" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetComCatalog arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetContextToken" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetContextToken arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetCurrentLogicalThreadId" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetCurrentLogicalThreadId arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetCurrentProcess" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetCurrentProcess arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetDefaultContext" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetDefaultContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetInstanceFromFile" => {
            let clsid_ptr = emu.regs().rcx;
            let outer = emu.regs().rdx;
            let ctx = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_object");
            let ppv_addr = ppv;
            if ppv_addr != 0 {
                emu.maps.write_qword(ppv_addr, hndl);
            }
            log_red!(
                emu,
                "ole32!CoGetInstanceFromFile ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetInstanceFromIStorage" => {
            let clsid_ptr = emu.regs().rcx;
            let outer = emu.regs().rdx;
            let ctx = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_object");
            let ppv_addr = ppv;
            if ppv_addr != 0 {
                emu.maps.write_qword(ppv_addr, hndl);
            }
            log_red!(
                emu,
                "ole32!CoGetInstanceFromIStorage ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetInterceptor" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetInterceptor arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetInterceptorForOle32" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetInterceptorForOle32 arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetInterceptorFromTypeInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetInterceptorFromTypeInfo arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetInterfaceAndReleaseStream" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!CoGetInterfaceAndReleaseStream arg1:0x{:x}",
                arg1
            );
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetMalloc" => {
            let _context = emu.regs().rcx;
            let ppv = emu.regs().rdx;
            let hndl = helper::handler_create("com_malloc");
            if ppv != 0 {
                emu.maps.write_qword(ppv, hndl);
            }
            log_red!(emu, "ole32!CoGetMalloc => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetMarshalSizeMax" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetMarshalSizeMax arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetModuleType" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetModuleType arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetObject" => {
            let clsid_ptr = emu.regs().rcx;
            let outer = emu.regs().rdx;
            let ctx = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_object");
            let ppv_addr = ppv;
            if ppv_addr != 0 {
                emu.maps.write_qword(ppv_addr, hndl);
            }
            log_red!(emu, "ole32!CoGetObject ctx:0x{:x} => 0x{:x}", ctx, hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetObjectContext" => {
            let clsid_ptr = emu.regs().rcx;
            let outer = emu.regs().rdx;
            let ctx = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_object");
            let ppv_addr = ppv;
            if ppv_addr != 0 {
                emu.maps.write_qword(ppv_addr, hndl);
            }
            log_red!(
                emu,
                "ole32!CoGetObjectContext ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetPSClsid" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetPSClsid arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetProcessIdentifier" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetProcessIdentifier arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetStandardMarshal" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetStandardMarshal arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetStdMarshalEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetStdMarshalEx arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetSystemSecurityPermissions" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!CoGetSystemSecurityPermissions arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetSystemWow64DirectoryW" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetSystemWow64DirectoryW arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoGetTreatAsClass" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoGetTreatAsClass arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoHandlePriorityEventsFromMessagePump" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!CoHandlePriorityEventsFromMessagePump 0x{:x}",
                arg1
            );
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CoImpersonateClient" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoImpersonateClient arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoIncrementMTAUsage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoIncrementMTAUsage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoInitializeSecurity" => {
            let reserved = emu.regs().rcx;
            log_red!(emu, "ole32!CoInitializeSecurity");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoInitializeWOW" => {
            let reserved = emu.regs().rcx;
            log_red!(emu, "ole32!CoInitializeWOW");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoInstall" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoInstall arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoInvalidateRemoteMachineBindings" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!CoInvalidateRemoteMachineBindings arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoIsHandlerConnected" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoIsHandlerConnected arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoIsOle1Class" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoIsOle1Class arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoLoadLibrary" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CoLoadLibrary arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let hndl = helper::handler_create("ole32://CoLoadLibrary");
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CoLockObjectExternal" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoLockObjectExternal 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CoMarshalHresult" => {
            let stream = emu.regs().rcx;
            log_red!(emu, "ole32!CoMarshalHresult stream:0x{:x}", stream);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoMarshalInterThreadInterfaceInStream" => {
            let stream = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!CoMarshalInterThreadInterfaceInStream stream:0x{:x}",
                stream
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoMarshalInterface" => {
            let stream = emu.regs().rcx;
            log_red!(emu, "ole32!CoMarshalInterface stream:0x{:x}", stream);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoPopServiceDomain" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoPopServiceDomain arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoPushServiceDomain" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoPushServiceDomain arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoQueryAuthenticationServices" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoQueryAuthenticationServices arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoQueryClientBlanket" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoQueryClientBlanket arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoQueryProxyBlanket" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoQueryProxyBlanket arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoQueryReleaseObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoQueryReleaseObject arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoReactivateObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoReactivateObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRegisterActivationFilter" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRegisterActivationFilter arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRegisterChannelHook" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRegisterChannelHook arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRegisterClassObject" => {
            let rclsid = emu.regs().rcx;
            let punk = emu.regs().rdx;
            let cls_ctx = emu.regs().r8;
            let flags = emu.regs().r9;
            let pdw_register = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let hndl = helper::handler_create("com_class_registration");
            if pdw_register != 0 {
                emu.maps.write_dword(pdw_register, hndl as u32);
            }
            log_red!(
                emu,
                "ole32!CoRegisterClassObject clsid:0x{:x} ctx:{} => 0x{:x}",
                rclsid,
                cls_ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRegisterInitializeSpy" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRegisterInitializeSpy arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRegisterMallocSpy" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRegisterMallocSpy arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRegisterMessageFilter" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRegisterMessageFilter arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRegisterPSClsid" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRegisterPSClsid arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRegisterSurrogate" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRegisterSurrogate arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRegisterSurrogateEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRegisterSurrogateEx arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoReleaseMarshalData" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoReleaseMarshalData arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRetireServer" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRetireServer arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRevertToSelf" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRevertToSelf arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRevokeClassObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRevokeClassObject arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRevokeInitializeSpy" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRevokeInitializeSpy arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoRevokeMallocSpy" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoRevokeMallocSpy arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoSetCancelObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoSetCancelObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoSetMessageDispatcher" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoSetMessageDispatcher arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoSetProxyBlanket" => {
            let proxy = emu.regs().rcx;
            let authn_svc = emu.regs().rdx;
            let authz_svc = emu.regs().r8;
            let server_princ = emu.regs().r9;
            log_red!(
                emu,
                "ole32!CoSetProxyBlanket proxy:0x{:x} authn:{} authz:{}",
                proxy,
                authn_svc,
                authz_svc
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoSetState" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoSetState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoSuspendClassObjects" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoSuspendClassObjects arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoSwitchCallContext" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoSwitchCallContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoTaskMemRealloc" => {
            let old_ptr = emu.regs().rcx;
            let new_size = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!CoTaskMemRealloc old:0x{:x} size:{}",
                old_ptr,
                new_size
            );
            if new_size == 0 {
                if old_ptr != 0 {
                    let name = format!("cotaskmem_{:x}", old_ptr);
                    emu.maps.free(&name);
                }
                emu.set_last_error(0);
                emu.regs_mut().rax = 0;
            } else if let Some(base) = emu.maps.alloc(new_size) {
                let name = format!("cotaskmem_{:x}", base);
                emu.maps.create_map(
                    &name,
                    base,
                    new_size,
                    crate::maps::mem64::Permission::READ_WRITE,
                );
                if old_ptr != 0 {
                    let old_name = format!("cotaskmem_{:x}", old_ptr);
                    if let Some(old_map) = emu.maps.get_map_by_name(&old_name) {
                        let old_size = old_map.size() as u64;
                        let copy_size = std::cmp::min(old_size, new_size);
                        for i in 0..copy_size {
                            let b = emu.maps.read_byte(old_ptr + i).unwrap_or(0);
                            emu.maps.write_byte(base + i, b);
                        }
                    }
                    emu.maps.free(&old_name);
                }
                emu.set_last_error(0);
                emu.regs_mut().rax = base;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CoTestCancel" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoTestCancel arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoTreatAsClass" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoTreatAsClass arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoTryGetInterceptorForAppModelVirtualization" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!CoTryGetInterceptorForAppModelVirtualization arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoUnloadingWOW" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoUnloadingWOW arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoUnmarshalHresult" => {
            let stream = emu.regs().rcx;
            log_red!(emu, "ole32!CoUnmarshalHresult stream:0x{:x}", stream);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoUnmarshalInterface" => {
            let stream = emu.regs().rcx;
            log_red!(emu, "ole32!CoUnmarshalInterface stream:0x{:x}", stream);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoVrfCheckThreadState" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoVrfCheckThreadState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoVrfGetThreadState" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoVrfGetThreadState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoVrfReleaseThreadState" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoVrfReleaseThreadState arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CoWaitForMultipleHandles" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoWaitForMultipleHandles 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CoWaitForMultipleObjects" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!CoWaitForMultipleObjects 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ComPs_NdrDllCanUnloadNow" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!ComPs_NdrDllCanUnloadNow arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ComPs_NdrDllGetClassObject" => {
            let clsid_ptr = emu.regs().rcx;
            let outer = emu.regs().rdx;
            let ctx = emu.regs().r8;
            let iid_ptr = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("com_object");
            let ppv_addr = ppv;
            if ppv_addr != 0 {
                emu.maps.write_qword(ppv_addr, hndl);
            }
            log_red!(
                emu,
                "ole32!ComPs_NdrDllGetClassObject ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ComPs_NdrDllRegisterProxy" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!ComPs_NdrDllRegisterProxy arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ComPs_NdrDllUnregisterProxy" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!ComPs_NdrDllUnregisterProxy arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateAntiMoniker" => {
            let ppmk = emu.regs().rcx;
            let hndl = helper::handler_create("ole32://IMoniker");
            if ppmk != 0 {
                emu.maps.write_qword(ppmk, hndl);
            }
            log_red!(
                emu,
                "ole32!CreateAntiMoniker ppmk:0x{:x} =0x{:x}",
                ppmk,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateBindCtx" => {
            let reserved = emu.regs().rcx;
            let ppbc = emu.regs().rdx;
            let hndl = helper::handler_create("ole32://IBindCtx");
            if ppbc != 0 {
                emu.maps.write_qword(ppbc, hndl);
            }
            log_red!(emu, "ole32!CreateBindCtx =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateDataAdviseHolder" => {
            let pp_dah = emu.regs().rcx;
            let hndl = helper::handler_create("ole32://IDataAdviseHolder");
            if pp_dah != 0 {
                emu.maps.write_qword(pp_dah, hndl);
            }
            log_red!(emu, "ole32!CreateDataAdviseHolder =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateDataCache" => {
            let punk_outer = emu.regs().rcx;
            let rclsid = emu.regs().rdx;
            let riid = emu.regs().r8;
            let ppv = emu.regs().r9;
            let hndl = helper::handler_create("ole32://IOleCache");
            if ppv != 0 {
                emu.maps.write_qword(ppv, hndl);
            }
            log_red!(
                emu,
                "ole32!CreateDataCache outer:0x{:x} clsid:0x{:x} =0x{:x}",
                punk_outer,
                rclsid,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateErrorInfo" => {
            let pp_errinfo = emu.regs().rcx;
            let hndl = helper::handler_create("ole32://ICreateErrorInfo");
            if pp_errinfo != 0 {
                emu.maps.write_qword(pp_errinfo, hndl);
            }
            log_red!(emu, "ole32!CreateErrorInfo =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateFileMoniker" => {
            let lpsz_path_name = emu.regs().rcx;
            let ppmk = emu.regs().rdx;
            let hndl = helper::handler_create("ole32://IMoniker");
            if ppmk != 0 {
                emu.maps.write_qword(ppmk, hndl);
            }
            let path = emu.maps.read_wide_string(lpsz_path_name);
            log_red!(emu, "ole32!CreateFileMoniker path:'{}' =0x{:x}", path, hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateGenericComposite" => {
            let pmk_first = emu.regs().rcx;
            let pmk_rest = emu.regs().rdx;
            let ppmk_composite = emu.regs().r8;
            let hndl = helper::handler_create("ole32://IMoniker");
            if ppmk_composite != 0 {
                emu.maps.write_qword(ppmk_composite, hndl);
            }
            log_red!(emu, "ole32!CreateGenericComposite =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateILockBytesOnHGlobal" => {
            let hglobal = emu.regs().rcx;
            let f_delete = emu.regs().rdx;
            let pplkbyt = emu.regs().r8;
            let hndl = helper::handler_create("ole32://ILockBytes");
            if pplkbyt != 0 {
                emu.maps.write_qword(pplkbyt, hndl);
            }
            log_red!(emu, "ole32!CreateILockBytesOnHGlobal =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateItemMoniker" => {
            let lpsz_delim = emu.regs().rcx;
            let lpsz_item = emu.regs().rdx;
            let ppmk = emu.regs().r8;
            let hndl = helper::handler_create("ole32://IMoniker");
            if ppmk != 0 {
                emu.maps.write_qword(ppmk, hndl);
            }
            log_red!(emu, "ole32!CreateItemMoniker =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateObjrefMoniker" => {
            let punk = emu.regs().rcx;
            let ppmk = emu.regs().rdx;
            let hndl = helper::handler_create("ole32://IMoniker");
            if ppmk != 0 {
                emu.maps.write_qword(ppmk, hndl);
            }
            log_red!(emu, "ole32!CreateObjrefMoniker =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateOleAdviseHolder" => {
            let pp_oah = emu.regs().rcx;
            let hndl = helper::handler_create("ole32://IOleAdviseHolder");
            if pp_oah != 0 {
                emu.maps.write_qword(pp_oah, hndl);
            }
            log_red!(emu, "ole32!CreateOleAdviseHolder =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreatePointerMoniker" => {
            let punk = emu.regs().rcx;
            let ppmk = emu.regs().rdx;
            let hndl = helper::handler_create("ole32://IMoniker");
            if ppmk != 0 {
                emu.maps.write_qword(ppmk, hndl);
            }
            log_red!(emu, "ole32!CreatePointerMoniker =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateStdProgressIndicator" => {
            let hwnd_parent = emu.regs().rcx;
            let psz_title = emu.regs().rdx;
            let pp_ibsc = emu.regs().r8;
            let hndl = helper::handler_create("ole32://IBindStatusCallback");
            if pp_ibsc != 0 {
                emu.maps.write_qword(pp_ibsc, hndl);
            }
            log_red!(emu, "ole32!CreateStdProgressIndicator =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateStreamOnHGlobal" => {
            let hglobal = emu.regs().rcx;
            let f_delete = emu.regs().rdx;
            let ppstm = emu.regs().r8;
            let hndl = helper::handler_create("ole32://IStream");
            if ppstm != 0 {
                emu.maps.write_qword(ppstm, hndl);
            }
            log_red!(emu, "ole32!CreateStreamOnHGlobal =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "DcomChannelSetHResult" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!DcomChannelSetHResult arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeBindToObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!DdeBindToObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeletePatternAndExtensionTables" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(
                emu,
                "ole32!DeletePatternAndExtensionTables handle:0x{:x}",
                handle
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "DestroyRunningObjectTable" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "ole32!DestroyRunningObjectTable handle:0x{:x}", handle);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "DllDebugObjectRPCHook" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!DllDebugObjectRPCHook h:0x{:x} arg2:0x{:x}",
                handle,
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
                "ole32!DllGetClassObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllGetClassObjectWOW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "ole32!DllGetClassObjectWOW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "DllRegisterServer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!DllRegisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DoDragDrop" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!DoDragDrop arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DragDropSetFDT" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!DragDropSetFDT arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableHookObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!EnableHookObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!FindExt arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FmtIdToPropStgName" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!FmtIdToPropStgName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FreePropVariantArray" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "ole32!FreePropVariantArray handle:0x{:x}", handle);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetActiveObjectExt" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!GetActiveObjectExt h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClassFile" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!GetClassFile h:0x{:x} arg2:0x{:x}", handle, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConvertStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!GetConvertStg h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDocumentBitStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!GetDocumentBitStg h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetErrorInfo" => {
            let _reserved = emu.regs().rcx;
            let pperrinfo = emu.regs().rdx;
            if pperrinfo != 0 {
                emu.maps.write_dword(pperrinfo, 0); // no error info
            }
            log_red!(emu, "ole32!GetErrorInfo => S_FALSE (no error info)");
            emu.regs_mut().rax = 1; // S_FALSE
        }
        "GetHGlobalFromILockBytes" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!GetHGlobalFromILockBytes 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetHGlobalFromStream" => {
            let pstm = emu.regs().rcx;
            let phglobal = emu.regs().rdx;
            let hndl = helper::handler_create("ole32://HGlobal");
            if phglobal != 0 {
                emu.maps.write_dword(phglobal, hndl as u32);
            }
            log_red!(
                emu,
                "ole32!GetHGlobalFromStream pstm:0x{:x} => 0x{:x}",
                pstm,
                hndl
            );
            emu.regs_mut().rax = 0; // S_OK
        }
        "GetHookInterface" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!GetHookInterface h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetObjectFromRotByPath" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!GetObjectFromRotByPath h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HACCEL_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HACCEL_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HACCEL_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HACCEL_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HACCEL_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HACCEL_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HACCEL_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HACCEL_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HACCEL_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HACCEL_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HACCEL_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HACCEL_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HACCEL_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HACCEL_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HACCEL_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HACCEL_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBITMAP_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBITMAP_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBITMAP_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBITMAP_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBITMAP_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBITMAP_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBITMAP_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBITMAP_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBITMAP_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBITMAP_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBITMAP_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBITMAP_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBITMAP_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBITMAP_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBITMAP_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBITMAP_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBRUSH_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBRUSH_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBRUSH_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBRUSH_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBRUSH_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBRUSH_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBRUSH_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBRUSH_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBRUSH_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBRUSH_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBRUSH_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBRUSH_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBRUSH_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBRUSH_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HBRUSH_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HBRUSH_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HDC_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HDC_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HDC_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HDC_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HDC_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HDC_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HDC_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HDC_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HDC_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HDC_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HDC_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HDC_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HDC_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HDC_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HDC_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HDC_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HENHMETAFILE_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HENHMETAFILE_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HENHMETAFILE_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HENHMETAFILE_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HENHMETAFILE_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HENHMETAFILE_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HENHMETAFILE_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HENHMETAFILE_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HGLOBAL_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HGLOBAL_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HGLOBAL_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HGLOBAL_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HGLOBAL_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HGLOBAL_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HGLOBAL_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HGLOBAL_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HICON_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HICON_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HICON_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HICON_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HICON_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HICON_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HICON_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HICON_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HICON_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HICON_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HICON_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HICON_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HICON_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HICON_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HICON_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HICON_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMENU_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMENU_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMENU_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMENU_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMENU_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMENU_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMENU_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMENU_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMENU_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMENU_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMENU_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMENU_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMENU_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMENU_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMENU_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMENU_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMETAFILEPICT_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "HMETAFILEPICT_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "HMETAFILEPICT_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "HMETAFILEPICT_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "HMETAFILEPICT_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "HMETAFILEPICT_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "HMETAFILEPICT_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "HMETAFILEPICT_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "HMETAFILE_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMETAFILE_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMETAFILE_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMETAFILE_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMETAFILE_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMETAFILE_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMETAFILE_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMETAFILE_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMONITOR_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMONITOR_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMONITOR_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMONITOR_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMONITOR_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMONITOR_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMONITOR_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMONITOR_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMONITOR_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMONITOR_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMONITOR_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMONITOR_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMONITOR_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMONITOR_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HMONITOR_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HMONITOR_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserFreeExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserFreeExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserMarshalExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserMarshalExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserSizeExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserSizeExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HPALETTE_UserUnmarshalExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HPALETTE_UserUnmarshalExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HRGN_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HRGN_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HRGN_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HRGN_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HRGN_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HRGN_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HRGN_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HRGN_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
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
                "ole32!HWND_UserFree arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserFree64Ext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HWND_UserFree64Ext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserFreeExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HWND_UserFreeExt arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserMarshal64Ext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HWND_UserMarshal64Ext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserMarshalExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HWND_UserMarshalExt arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserSize arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserSize64Ext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HWND_UserSize64Ext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserSizeExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HWND_UserSizeExt arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserUnmarshal64Ext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HWND_UserUnmarshal64Ext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HWND_UserUnmarshalExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HWND_UserUnmarshalExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HkOleRegisterObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!HkOleRegisterObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IIDFromString" => {
            let str_ptr = emu.regs().rcx;
            let iid_ptr = emu.regs().rdx;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let cleaned: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
            if cleaned.len() == 32 && iid_ptr != 0 {
                let d1 = u32::from_str_radix(&cleaned[0..8], 16).unwrap_or(0);
                let d2 = u16::from_str_radix(&cleaned[8..12], 16).unwrap_or(0);
                let d3 = u16::from_str_radix(&cleaned[12..16], 16).unwrap_or(0);
                emu.maps.write_dword(iid_ptr, d1);
                emu.maps.write_word(iid_ptr + 4, d2);
                emu.maps.write_word(iid_ptr + 6, d3);
                for i in 0..8usize {
                    let b = u8::from_str_radix(&cleaned[16 + i * 2..18 + i * 2], 16).unwrap_or(0);
                    emu.maps.write_byte(iid_ptr + 8 + i as u64, b);
                }
                emu.regs_mut().rax = 0;
            } else if iid_ptr != 0 {
                emu.maps.write_dword(iid_ptr, 0);
                emu.maps.write_dword(iid_ptr + 4, 0);
                emu.maps.write_dword(iid_ptr + 8, 0);
                emu.maps.write_dword(iid_ptr + 12, 0);
                emu.regs_mut().rax = 0x800401F3; // CO_E_CLASSSTRING
            } else {
                emu.regs_mut().rax = 0x80070057; // E_INVALIDARG
            }
            log_red!(emu, "ole32!IIDFromString '{}'", s);
        }
        "IsAccelerator" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!IsAccelerator arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsEqualGUID" => {
            let guid1 = emu.regs().rcx;
            let guid2 = emu.regs().rdx;
            let equal = if guid1 != 0 && guid2 != 0 {
                let mut eq = true;
                for i in 0..4u64 {
                    let a = emu.maps.read_dword(guid1 + i * 4).unwrap_or(0);
                    let b = emu.maps.read_dword(guid2 + i * 4).unwrap_or(0);
                    if a != b {
                        eq = false;
                        break;
                    }
                }
                eq
            } else {
                false
            };
            log_red!(
                emu,
                "ole32!IsEqualGUID 0x{:x} 0x{:x} => {}",
                guid1,
                guid2,
                equal
            );
            emu.regs_mut().rax = equal as u64;
        }
        "IsRoInitializeASTAAllowedInDesktop" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!IsRoInitializeASTAAllowedInDesktop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsValidIid" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!IsValidIid arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsValidInterface" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!IsValidInterface arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsValidPtrIn" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!IsValidPtrIn arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsValidPtrOut" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!IsValidPtrOut arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MkParseDisplayName" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!MkParseDisplayName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MonikerCommonPrefixWith" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!MonikerCommonPrefixWith arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MonikerLoadTypeLib" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!MonikerLoadTypeLib arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MonikerRelativePathTo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!MonikerRelativePathTo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrOleInitializeExtension" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrOleInitializeExtension arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction10" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction10 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction11" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction11 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction12" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction12 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction13" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction13 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction14" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction14 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction15" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction15 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction16" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction16 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction17" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction17 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction18" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction18 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction19" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction19 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction20" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction20 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction21" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction21 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction22" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction22 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction23" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction23 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction24" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction24 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction25" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction25 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction26" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction26 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction27" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction27 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction28" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction28 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction29" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction29 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction3" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction3 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction30" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction30 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction31" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction31 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction32" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction32 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction4" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction4 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction5" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction5 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction6" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction6 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction7" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction7 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction8" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction8 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NdrProxyForwardingFunction9" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction9 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient10" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient10 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient11" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient11 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient12" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient12 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient13" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient13 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient14" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient14 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient15" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient15 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient16" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient16 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient17" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient17 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient18" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient18 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient19" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient19 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient20" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient20 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient21" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient21 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient22" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient22 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient23" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient23 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient24" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient24 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient25" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient25 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient26" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient26 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient27" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient27 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient28" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient28 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient29" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient29 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient3" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient3 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient30" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient30 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient31" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient31 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient32" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient32 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient4" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient4 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient5" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient5 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient6" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient6 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient7" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient7 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient8" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient8 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ObjectStublessClient9" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ObjectStublessClient9 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Ole32DllGetClassObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!Ole32DllGetClassObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleBuildVersion" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleBuildVersion arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleConvertIStorageToOLESTREAM" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleConvertIStorageToOLESTREAM arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleConvertIStorageToOLESTREAMEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!OleConvertIStorageToOLESTREAMEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleConvertOLESTREAMToIStorage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleConvertOLESTREAMToIStorage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleConvertOLESTREAMToIStorage2" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorage2 arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleConvertOLESTREAMToIStorageEx" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorageEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleConvertOLESTREAMToIStorageEx2" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorageEx2 arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleCreate" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!OleCreate arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreate");
            emu.regs_mut().rax = handle;
        }
        "OleCreateDefaultHandler" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateDefaultHandler arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatedefaulthandler");
            emu.regs_mut().rax = handle;
        }
        "OleCreateEmbeddingHelper" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateEmbeddingHelper arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreateembeddinghelper");
            emu.regs_mut().rax = handle;
        }
        "OleCreateEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!OleCreateEx arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreateex");
            emu.regs_mut().rax = handle;
        }
        "OleCreateFontIndirectExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateFontIndirectExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatefontindirectext");
            emu.regs_mut().rax = handle;
        }
        "OleCreateFromData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateFromData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatefromdata");
            emu.regs_mut().rax = handle;
        }
        "OleCreateFromDataEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateFromDataEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatefromdataex");
            emu.regs_mut().rax = handle;
        }
        "OleCreateFromFile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateFromFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatefromfile");
            emu.regs_mut().rax = handle;
        }
        "OleCreateFromFileEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateFromFileEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatefromfileex");
            emu.regs_mut().rax = handle;
        }
        "OleCreateLink" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateLink arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatelink");
            emu.regs_mut().rax = handle;
        }
        "OleCreateLinkEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateLinkEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatelinkex");
            emu.regs_mut().rax = handle;
        }
        "OleCreateLinkFromData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateLinkFromData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatelinkfromdata");
            emu.regs_mut().rax = handle;
        }
        "OleCreateLinkFromDataEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateLinkFromDataEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatelinkfromdataex");
            emu.regs_mut().rax = handle;
        }
        "OleCreateLinkToFile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateLinkToFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatelinktofile");
            emu.regs_mut().rax = handle;
        }
        "OleCreateLinkToFileEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateLinkToFileEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatelinktofileex");
            emu.regs_mut().rax = handle;
        }
        "OleCreateMenuDescriptor" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateMenuDescriptor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let hndl = helper::handler_create("ole32://OleCreateMenuDescriptor");
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "OleCreatePictureIndirectExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreatePictureIndirectExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatepictureindirectext");
            emu.regs_mut().rax = handle;
        }
        "OleCreatePropertyFrameIndirectExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreatePropertyFrameIndirectExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatepropertyframeindirectext");
            emu.regs_mut().rax = handle;
        }
        "OleCreateStaticFromData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleCreateStaticFromData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olecreatestaticfromdata");
            emu.regs_mut().rax = handle;
        }
        "OleDestroyMenuDescriptor" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleDestroyMenuDescriptor arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleDoAutoConvert" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleDoAutoConvert arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleDraw" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleDraw arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleDuplicateData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "ole32!OleDuplicateData arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let hndl = helper::handler_create("ole32://OleDuplicateData");
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "OleFlushClipboard" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleFlushClipboard arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleGetAutoConvert" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleGetAutoConvert arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleGetClipboard" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleGetClipboard arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleGetClipboardWithEnterpriseInfo" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!OleGetClipboardWithEnterpriseInfo arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleGetIconOfClass" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "ole32!OleGetIconOfClass arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let hndl = helper::handler_create("ole32://OleGetIconOfClass");
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "OleGetIconOfFile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleGetIconOfFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let hndl = helper::handler_create("ole32://OleGetIconOfFile");
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "OleGetPackageClipboardOwner" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleGetPackageClipboardOwner arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleIconToCursorExt" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleIconToCursorExt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleInitializeWOW" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleInitializeWOW arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleIsCurrentClipboard" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleIsCurrentClipboard arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleIsRunning" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleIsRunning arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoad" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleLoad arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadFromStream" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleLoadFromStream arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPictureExt" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleLoadPictureExt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPictureFileExt" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleLoadPictureFileExt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLoadPicturePathExt" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleLoadPicturePathExt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleLockRunning" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleLockRunning 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleMetafilePictFromIconAndLabel" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "ole32!OleMetafilePictFromIconAndLabel arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            let hndl = helper::handler_create("ole32://OleMetafilePictFromIconAndLabel");
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "OleNoteObjectVisible" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleNoteObjectVisible arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleQueryCreateFromData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!OleQueryCreateFromData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let handle = helper::handler_create("olequerycreatefromdata");
            emu.regs_mut().rax = handle;
        }
        "OleQueryLinkFromData" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleQueryLinkFromData arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleRegEnumFormatEtc" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleRegEnumFormatEtc arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleRegEnumVerbs" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleRegEnumVerbs arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleRegGetMiscStatus" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleRegGetMiscStatus arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleRegGetUserType" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleRegGetUserType arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleReleaseEnumVerbCache" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleReleaseEnumVerbCache arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleRun" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleRun arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleSave" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleSave arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleSavePictureFileExt" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleSavePictureFileExt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleSaveToStream" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleSaveToStream arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleSetAutoConvert" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleSetAutoConvert arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleSetClipboard" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleSetClipboard arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleSetContainedObject" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleSetContainedObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleSetMenuDescriptor" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleSetMenuDescriptor arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleTranslateAccelerator" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleTranslateAccelerator arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleTranslateColorExt" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleTranslateColorExt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OleUninitialize" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!OleUninitialize arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "OpenOrCreateStream" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("OpenOrCreateStream");
            log_red!(emu, "ole32!OpenOrCreateStream => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "ProgIDFromCLSID" => {
            let clsid_ptr = emu.regs().rcx;
            let ppsz = emu.regs().rdx;
            let guid_str = read_guid_string(emu, clsid_ptr);
            let prog_id = format!("Class.{}", &guid_str[1..9]);
            if ppsz != 0 {
                let alloc_sz = (prog_id.len() as u64 + 1) * 2;
                if let Some(buf) = emu.maps.alloc(alloc_sz) {
                    emu.maps.create_map(
                        &format!("progid_{:x}", buf),
                        buf,
                        alloc_sz,
                        crate::maps::mem64::Permission::READ_WRITE,
                    );
                    emu.maps.write_wide_string(buf, &prog_id);
                    emu.maps.write_dword(ppsz, buf as u32);
                }
            }
            log_red!(emu, "ole32!ProgIDFromCLSID {} => '{}'", guid_str, prog_id);
            emu.regs_mut().rax = 0;
        }
        "PropStgNameToFmtId" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!PropStgNameToFmtId arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "PropSysAllocString" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!PropSysAllocString arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "PropSysFreeString" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!PropSysFreeString arg1:0x{:x}", arg1);
            if arg1 != 0 {
                helper::handler_close(arg1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "PropVariantChangeType" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!PropVariantChangeType arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "PropVariantClear" => {
            let pvar = emu.regs().rcx;
            if pvar != 0 {
                emu.maps.write_word(pvar, 0); // VT_EMPTY
            }
            log_red!(emu, "ole32!PropVariantClear ptr:0x{:x}", pvar);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "PropVariantCopy" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
            if dst != 0 && src != 0 {
                for i in 0..4u64 {
                    let dw = emu.maps.read_dword(src + i * 4).unwrap_or(0);
                    emu.maps.write_dword(dst + i * 4, dw);
                }
            }
            log_red!(emu, "ole32!PropVariantCopy dst:0x{:x} src:0x{:x}", dst, src);
            emu.regs_mut().rax = 0;
        }
        "ReadClassStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!ReadClassStg h:0x{:x} arg2:0x{:x}", handle, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadClassStm" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!ReadClassStm h:0x{:x} arg2:0x{:x}", handle, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadFmtUserTypeStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ReadFmtUserTypeStg h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadOleStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!ReadOleStg h:0x{:x} arg2:0x{:x}", handle, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadStorageProperties" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ReadStorageProperties h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadStringStream" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!ReadStringStream h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterActiveObjectExt" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!RegisterActiveObjectExt h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterDragDrop" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!RegisterDragDrop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReleaseStgMedium" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "ole32!ReleaseStgMedium handle:0x{:x}", handle);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "RevokeActiveObjectExt" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!RevokeActiveObjectExt h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RevokeDragDrop" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!RevokeDragDrop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RoGetAgileReference" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!RoGetAgileReference arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SNB_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SNB_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SNB_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SNB_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SNB_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SNB_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SNB_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SNB_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SNB_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SNB_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SNB_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SNB_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SNB_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SNB_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SNB_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SNB_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserFreeExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserFreeExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserMarshalExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserMarshalExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserSizeExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserSizeExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "STGMEDIUM_UserUnmarshalExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserUnmarshalExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SetConvertStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SetConvertStg h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDocumentBitStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SetDocumentBitStg h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetErrorInfo" => {
            let _reserved = emu.regs().rcx;
            let perrinfo = emu.regs().rdx;
            log_red!(emu, "ole32!SetErrorInfo errinfo:0x{:x}", perrinfo);
            emu.regs_mut().rax = 0; // S_OK
        }
        "SetOleautModule" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SetOleautModule h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWOWThunkGlobalPtr" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!SetWOWThunkGlobalPtr h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StdTypesGetClassObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StdTypesGetClassObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StdTypesRegisterServer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StdTypesRegisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgConvertPropertyToVariant" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgConvertPropertyToVariant arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgConvertVariantToProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgConvertVariantToProperty arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgCreateDocfile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgCreateDocfile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgCreateDocfileOnILockBytes" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!StgCreateDocfileOnILockBytes 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "StgCreatePropSetStg" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgCreatePropSetStg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgCreatePropStg" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgCreatePropStg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgCreateStorageEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgCreateStorageEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgGetIFillLockBytesOnFile" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!StgGetIFillLockBytesOnFile 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "StgGetIFillLockBytesOnILockBytes" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!StgGetIFillLockBytesOnILockBytes 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "StgIsStorageFile" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgIsStorageFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgIsStorageILockBytes" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!StgIsStorageILockBytes 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "StgOpenAsyncDocfileOnIFillLockBytes" => {
            let arg1 = emu.regs().rcx;
            log_red!(
                emu,
                "ole32!StgOpenAsyncDocfileOnIFillLockBytes 0x{:x}",
                arg1
            );
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "StgOpenPropStg" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgOpenPropStg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgOpenStorage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgOpenStorage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgOpenStorageEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgOpenStorageEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgOpenStorageOnHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgOpenStorageOnHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgOpenStorageOnILockBytes" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ole32!StgOpenStorageOnILockBytes 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "StgPropertyLengthAsVariant" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!StgPropertyLengthAsVariant arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgSetTimes" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!StgSetTimes arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StringFromCLSID" => StringFromCLSID(emu),
        "StringFromGUID2" => StringFromGUID2(emu),
        "StringFromIID" => {
            let iid_ptr = emu.regs().rcx;
            let out_ptr = emu.regs().rdx;
            let guid_str = read_guid_string(emu, iid_ptr);
            if out_ptr != 0 {
                let alloc_sz: u64 = 39 * 2 + 2;
                if let Some(buf) = emu.maps.alloc(alloc_sz) {
                    emu.maps.create_map(
                        &format!("iid_str_{:x}", buf),
                        buf,
                        alloc_sz,
                        crate::maps::mem64::Permission::READ_WRITE,
                    );
                    emu.maps.write_wide_string(buf, &guid_str);
                    emu.maps.write_dword(out_ptr, buf as u32);
                } else {
                    emu.maps.write_dword(out_ptr, 0);
                }
            }
            log_red!(emu, "ole32!StringFromIID => '{}'", guid_str);
            emu.regs_mut().rax = 0;
        }
        "UpdateDCOMSettings" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!UpdateDCOMSettings arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateProcessTracing" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!UpdateProcessTracing h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UtConvertDvtd16toDvtd32" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!UtConvertDvtd16toDvtd32 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UtConvertDvtd32toDvtd16" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!UtConvertDvtd32toDvtd16 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UtGetDvtd16Info" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!UtGetDvtd16Info arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UtGetDvtd32Info" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!UtGetDvtd32Info arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WdtpInterfacePointer_UserFree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WdtpInterfacePointer_UserFree64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserFree64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WdtpInterfacePointer_UserMarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserMarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WdtpInterfacePointer_UserMarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WdtpInterfacePointer_UserSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WdtpInterfacePointer_UserSize64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserSize64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WdtpInterfacePointer_UserUnmarshal" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WdtpInterfacePointer_UserUnmarshal64" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WriteClassStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WriteClassStg h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WriteClassStm" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WriteClassStm h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WriteFmtUserTypeStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WriteFmtUserTypeStg h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WriteOleStg" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "ole32!WriteOleStg h:0x{:x} arg2:0x{:x}", handle, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WriteStorageProperties" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WriteStorageProperties h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WriteStringStream" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ole32!WriteStringStream h:0x{:x} arg2:0x{:x}",
                handle,
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

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
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

pub fn OleInitialize(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "ole32!OleInitialize arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::S_OK;
}

pub fn CoInitializeEx(emu: &mut emu::Emu) {
    let co_init = emu.regs().rdx;
    log_red!(emu, "ole32!CoInitializeEx dwCoInit: 0x{:x}", co_init);
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::S_OK;
}

pub fn CoCreateInstance(emu: &mut emu::Emu) {
    let rclsid = emu.regs().rcx;
    let _punk_outer = emu.regs().rdx;
    let dw_cls_context = emu.regs().r8;
    let riid = emu.regs().r9;
    let ppv = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let clsid_str = if rclsid != 0 {
        read_guid_string(emu, rclsid)
    } else {
        String::from("{NULL}")
    };
    let riid_str = if riid != 0 {
        read_guid_string(emu, riid)
    } else {
        String::from("{NULL}")
    };

    log_red!(
        emu,
        "ole32!CoCreateInstance clsid:{} ctx:0x{:x} riid:{} ppv:0x{:x}",
        clsid_str,
        dw_cls_context,
        riid_str,
        ppv
    );

    if ppv != 0 {
        emu.maps.write_qword(ppv, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80040154; // REGDB_E_CLASSNOTREG
}

pub fn CoTaskMemAlloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx;
    log_red!(emu, "ole32!CoTaskMemAlloc size: 0x{:x}", size);

    let addr = heap_engine::heap_allocate(emu, 0, size).unwrap_or(0);
    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}

pub fn CoTaskMemFree(emu: &mut emu::Emu) {
    let pv = emu.regs().rcx;
    log_red!(emu, "ole32!CoTaskMemFree pv: 0x{:x}", pv);

    if pv != 0 {
        heap_engine::heap_free(emu, 0, pv);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn CoUninitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoUninitialize");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

fn read_guid_string(emu: &emu::Emu, addr: u64) -> String {
    let d1 = emu.maps.read_dword(addr).unwrap_or(0);
    let d2 = emu.maps.read_word(addr + 4).unwrap_or(0);
    let d3 = emu.maps.read_word(addr + 6).unwrap_or(0);
    let mut d4 = [0u8; 8];
    for (i, byte) in d4.iter_mut().enumerate() {
        *byte = emu.maps.read_byte(addr + 8 + i as u64).unwrap_or(0);
    }
    format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        d1, d2, d3, d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]
    )
}

pub fn CoCreateGuid(emu: &mut emu::Emu) {
    let pguid = emu.regs().rcx;

    log_red!(emu, "ole32!CoCreateGuid ptr: 0x{:x}", pguid);

    if pguid == 0 {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0x80070057; // E_INVALIDARG
        return;
    }

    // Write a deterministic GUID (version 4 format)
    // Using emu.pos as seed for reproducibility
    let seed = emu.pos as u32;
    emu.maps
        .write_dword(pguid, seed.wrapping_mul(0x6C078965).wrapping_add(1));
    emu.maps
        .write_word(pguid + 4, ((seed & 0xFFFF) ^ 0x4000) as u16);
    emu.maps
        .write_word(pguid + 6, (0x4000 | (seed & 0x0FFF)) as u16); // version 4
    emu.maps
        .write_byte(pguid + 8, (0x80 | ((seed >> 16) & 0x3F)) as u8); // variant 1
    emu.maps.write_byte(pguid + 9, (seed >> 24) as u8);
    for i in 0..6u64 {
        emu.maps
            .write_byte(pguid + 10 + i, ((seed.wrapping_add(i as u32)) & 0xFF) as u8);
    }

    let guid_str = read_guid_string(emu, pguid);
    log_red!(emu, "CoCreateGuid -> {}", guid_str);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}

pub fn StringFromCLSID(emu: &mut emu::Emu) {
    let rclsid = emu.regs().rcx;
    let ppsz = emu.regs().rdx;

    let guid_str = read_guid_string(emu, rclsid);
    log_red!(emu, "ole32!StringFromCLSID {} ppsz: 0x{:x}", guid_str, ppsz);

    // Allocate wide string for the GUID "{...}\0" = 39 chars
    let alloc_sz = 39 * 2 + 2;
    let buf = match emu.maps.alloc(alloc_sz) {
        Some(a) => a,
        None => {
            if ppsz != 0 {
                emu.maps.write_qword(ppsz, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x8007000E; // E_OUTOFMEMORY
            return;
        }
    };
    emu.maps.create_map(
        &format!("guid_str_{:x}", buf),
        buf,
        alloc_sz,
        crate::maps::mem64::Permission::READ_WRITE,
    );

    emu.maps.write_wide_string(buf, &guid_str);

    if ppsz != 0 {
        emu.maps.write_qword(ppsz, buf);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}

pub fn StringFromGUID2(emu: &mut emu::Emu) {
    let rguid = emu.regs().rcx;
    let lpsz = emu.regs().rdx;
    let cch_max = emu.regs().r8 as i32;

    let guid_str = read_guid_string(emu, rguid);
    log_red!(
        emu,
        "ole32!StringFromGUID2 {} buf: 0x{:x} max: {}",
        guid_str,
        lpsz,
        cch_max
    );

    let needed = 39; // "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" = 38 + null = 39
    if cch_max < needed {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }

    if lpsz != 0 {
        emu.maps.write_wide_string(lpsz, &guid_str);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = needed as u64;
}

pub fn CLSIDFromString(emu: &mut emu::Emu) {
    let lpsz = emu.regs().rcx;
    let pclsid = emu.regs().rdx;

    let guid_text = if lpsz != 0 {
        emu.maps.read_wide_string(lpsz)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "ole32!CLSIDFromString '{}' -> 0x{:x}",
        guid_text,
        pclsid
    );

    // Parse "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}"
    let cleaned: String = guid_text
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();

    if cleaned.len() != 32 || pclsid == 0 {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0x800401F3; // CO_E_CLASSSTRING
        return;
    }

    let d1 = u32::from_str_radix(&cleaned[0..8], 16).unwrap_or(0);
    let d2 = u16::from_str_radix(&cleaned[8..12], 16).unwrap_or(0);
    let d3 = u16::from_str_radix(&cleaned[12..16], 16).unwrap_or(0);

    emu.maps.write_dword(pclsid, d1);
    emu.maps.write_word(pclsid + 4, d2);
    emu.maps.write_word(pclsid + 6, d3);

    for i in 0..8usize {
        let byte_val = u8::from_str_radix(&cleaned[16 + i * 2..18 + i * 2], 16).unwrap_or(0);
        emu.maps.write_byte(pclsid + 8 + i as u64, byte_val);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK (NOERROR)
}
