use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::serialization;
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
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!BindMoniker h:0x{:x} arg2:0x{:x}", handle, arg2);
            let handle = helper::handler_create("BindMoniker");
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
        "CLIPFORMAT_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserFreeExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserFreeExt arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserMarshalExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserMarshalExt arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserSizeExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserSizeExt arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "CLIPFORMAT_UserUnmarshalExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLIPFORMAT_UserUnmarshalExt arg1:0x{:x} arg2:0x{:x}",
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
        "CLSIDFromOle1Class" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLSIDFromOle1Class arg1:0x{:x} arg2:0x{:x}",
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
        "CLSIDFromProgID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLSIDFromProgID arg1:0x{:x} arg2:0x{:x}",
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
        "CLSIDFromProgIDEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CLSIDFromProgIDEx arg1:0x{:x} arg2:0x{:x}",
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
        "CLSIDFromString" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            let cleaned: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
            if cleaned.len() == 32 && clsid_ptr != 0 {
                let d1 = u32::from_str_radix(&cleaned[0..8], 16).unwrap_or(0);
                let d2 = u16::from_str_radix(&cleaned[8..12], 16).unwrap_or(0);
                let d3 = u16::from_str_radix(&cleaned[12..16], 16).unwrap_or(0);
                emu.maps.write_dword(clsid_ptr, d1);
                emu.maps.write_word(clsid_ptr + 4, d2);
                emu.maps.write_word(clsid_ptr + 6, d3);
                for i in 0..8usize {
                    let b = u8::from_str_radix(&cleaned[16 + i * 2..18 + i * 2], 16).unwrap_or(0);
                    emu.maps.write_byte(clsid_ptr + 8 + i as u64, b);
                }
                emu.regs_mut().rax = 0; // S_OK
            } else if clsid_ptr != 0 {
                emu.maps.write_dword(clsid_ptr, 0);
                emu.maps.write_dword(clsid_ptr + 4, 0);
                emu.maps.write_dword(clsid_ptr + 8, 0);
                emu.maps.write_dword(clsid_ptr + 12, 0);
                emu.regs_mut().rax = 0x800401F3; // CO_E_CLASSSTRING
            } else {
                emu.regs_mut().rax = 0x80070057; // E_INVALIDARG
            }
            log_red!(emu, "ole32!CLSIDFromString '{}'", s);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "CStdAsyncStubBuffer2_Connect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer2_Connect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CStdAsyncStubBuffer2_Connect");
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
        "CStdAsyncStubBuffer2_Disconnect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer2_Disconnect arg1:0x{:x} arg2:0x{:x}",
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
        "CStdAsyncStubBuffer2_Release" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer2_Release arg1:0x{:x} arg2:0x{:x}",
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
        "CStdAsyncStubBuffer_AddRef" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_AddRef arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CStdAsyncStubBuffer_AddRef");
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
        "CStdAsyncStubBuffer_Connect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Connect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CStdAsyncStubBuffer_Connect");
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
        "CStdAsyncStubBuffer_Disconnect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Disconnect arg1:0x{:x} arg2:0x{:x}",
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
        "CStdAsyncStubBuffer_Invoke" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Invoke arg1:0x{:x} arg2:0x{:x}",
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
        "CStdAsyncStubBuffer_QueryInterface" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_QueryInterface arg1:0x{:x} arg2:0x{:x}",
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
        "CStdAsyncStubBuffer_Release" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdAsyncStubBuffer_Release arg1:0x{:x} arg2:0x{:x}",
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
        "CStdStubBuffer2_Connect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_Connect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CStdStubBuffer2_Connect");
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
        "CStdStubBuffer2_CountRefs" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_CountRefs arg1:0x{:x} arg2:0x{:x}",
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
        "CStdStubBuffer2_Disconnect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_Disconnect arg1:0x{:x} arg2:0x{:x}",
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
        "CStdStubBuffer2_QueryInterface" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CStdStubBuffer2_QueryInterface arg1:0x{:x} arg2:0x{:x}",
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
        "CheckInitDde" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CheckInitDde arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CheckInitDde");
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
        "CleanROTForApartment" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CleanROTForApartment arg1:0x{:x} arg2:0x{:x}",
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
        "ClipboardProcessUninitialize" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ClipboardProcessUninitialize h:0x{:x} arg2:0x{:x}",
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
        "CoAddRefServerProcess" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoAddRefServerProcess arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoAddRefServerProcess");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoAicGetTokenForCOM" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoAicGetTokenForCOM arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoAllowSetForegroundWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoAllowSetForegroundWindow arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoAllowUnmarshalerCLSID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoAllowUnmarshalerCLSID arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoBuildVersion" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoBuildVersion arg1:0x{:x}", arg1);
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
        "CoCancelCall" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoCancelCall arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoCheckElevationEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoCheckElevationEnabled arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoCheckElevationEnabled");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoCopyProxy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoCopyProxy arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoCreateFreeThreadedMarshaler" => {
            let punk_outer = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let ppunk_marshal = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://IMarshal");
            if ppunk_marshal != 0 {
                emu.maps.write_dword(ppunk_marshal, hndl as u32);
            }
            log_red!(emu, "ole32!CoCreateFreeThreadedMarshaler =0x{:x}", hndl);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "CoCreateGuid" => {
            let pguid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoCreateGuid ptr: 0x{:x}", pguid);
            if pguid != 0 {
                let seed = emu.pos as u32;
                emu.maps
                    .write_dword(pguid, seed.wrapping_mul(0x6C078965).wrapping_add(1));
                emu.maps
                    .write_word(pguid + 4, ((seed & 0xFFFF) ^ 0x4000) as u16);
                emu.maps
                    .write_word(pguid + 6, (0x4000 | (seed & 0x0FFF)) as u16);
                emu.maps
                    .write_byte(pguid + 8, (0x80 | ((seed >> 16) & 0x3F)) as u8);
                emu.maps.write_byte(pguid + 9, (seed >> 24) as u8);
                for i in 0..6u64 {
                    emu.maps
                        .write_byte(pguid + 10 + i, ((seed.wrapping_add(i as u32)) & 0xFF) as u8);
                }
                emu.regs_mut().rax = 0;
            } else {
                emu.regs_mut().rax = 0x80070057; // E_INVALIDARG
            }
            emu.stack_pop32(false);
        }
        "CoCreateInstanceEx" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _outer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let ppv_addr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv_addr != 0 {
                emu.maps.write_dword(ppv_addr, 0);
            }
            log_red!(
                emu,
                "ole32!CoCreateInstanceEx clsid:0x{:x} ctx:0x{:x} => REGDB_E_CLASSNOTREG",
                clsid_ptr,
                ctx
            );
            emu.regs_mut().rax = 0x80040154;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CoCreateInstanceFromApp" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _outer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let ppv_addr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv_addr != 0 {
                emu.maps.write_dword(ppv_addr, 0);
            }
            log_red!(
                emu,
                "ole32!CoCreateInstanceFromApp clsid:0x{:x} ctx:0x{:x} => REGDB_E_CLASSNOTREG",
                clsid_ptr,
                ctx
            );
            emu.regs_mut().rax = 0x80040154;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CoCreateObjectInContext" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _outer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let ppv_addr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv_addr != 0 {
                emu.maps.write_dword(ppv_addr, 0);
            }
            log_red!(
                emu,
                "ole32!CoCreateObjectInContext clsid:0x{:x} ctx:0x{:x} => REGDB_E_CLASSNOTREG",
                clsid_ptr,
                ctx
            );
            emu.regs_mut().rax = 0x80040154;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CoDeactivateObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoDeactivateObject arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoDecodeProxy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoDecodeProxy arg1:0x{:x}", arg1);
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
        "CoDecrementMTAUsage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoDecrementMTAUsage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoDisableCallCancellation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoDisableCallCancellation arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoDisconnectContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoDisconnectContext arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoDisconnectObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoDisconnectObject arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoDosDateTimeToFileTime" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoDosDateTimeToFileTime arg1:0x{:x}", arg1);
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
        "CoEnableCallCancellation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoEnableCallCancellation arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoEnableCallCancellation");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoFileTimeNow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoFileTimeNow arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoFileTimeToDosDateTime" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoFileTimeToDosDateTime arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoFreeAllLibraries" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoFreeAllLibraries arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoFreeLibrary" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoFreeLibrary arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoFreeUnusedLibraries" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoFreeUnusedLibraries arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoFreeUnusedLibrariesEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoFreeUnusedLibrariesEx arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetActivationState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetActivationState arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetActivationState");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetApartmentID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetApartmentID arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetApartmentID");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetApartmentType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetApartmentType arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetApartmentType");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetCallContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetCallContext arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetCallContext");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetCallState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetCallState arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetCallState");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetCallerTID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetCallerTID arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetCallerTID");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetCancelObject" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let outer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

            let hndl = helper::handler_create("com_object");
            let ppv_addr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv_addr != 0 {
                emu.maps.write_dword(ppv_addr, hndl as u32);
            }
            log_red!(
                emu,
                "ole32!CoGetCancelObject ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CoGetClassObject" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let _server_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let ppv = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv != 0 {
                emu.maps.write_dword(ppv, 0);
            }
            log_red!(
                emu,
                "ole32!CoGetClassObject clsid:0x{:x} ctx:0x{:x} => REGDB_E_CLASSNOTREG",
                clsid_ptr,
                ctx
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x80040154; // REGDB_E_CLASSNOTREG
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
        }
        "CoGetClassVersion" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetClassVersion arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetClassVersion");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetComCatalog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetComCatalog arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetComCatalog");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetContextToken" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetContextToken arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetContextToken");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetCurrentLogicalThreadId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetCurrentLogicalThreadId arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetCurrentLogicalThreadId");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetCurrentProcess" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetCurrentProcess arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetCurrentProcess");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoImpersonateClient" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoImpersonateClient arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetDefaultContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetDefaultContext arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetDefaultContext");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetInstanceFromFile" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let outer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

            let hndl = helper::handler_create("com_object");
            let ppv_addr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv_addr != 0 {
                emu.maps.write_dword(ppv_addr, hndl as u32);
            }
            log_red!(
                emu,
                "ole32!CoGetInstanceFromFile ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CoGetInstanceFromIStorage" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let outer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

            let hndl = helper::handler_create("com_object");
            let ppv_addr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv_addr != 0 {
                emu.maps.write_dword(ppv_addr, hndl as u32);
            }
            log_red!(
                emu,
                "ole32!CoGetInstanceFromIStorage ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CoGetInterceptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetInterceptor arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetInterceptor");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetInterceptorForOle32" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetInterceptorForOle32 arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetInterceptorForOle32");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetInterceptorFromTypeInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetInterceptorFromTypeInfo arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetInterceptorFromTypeInfo");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetInterfaceAndReleaseStream" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CoGetInterfaceAndReleaseStream arg1:0x{:x}",
                arg1
            );
            let handle = helper::handler_create("CoGetInterfaceAndReleaseStream");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetMalloc" => {
            let _context = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let ppv = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hndl = helper::handler_create("com_malloc");
            if ppv != 0 {
                emu.maps.write_dword(ppv, hndl as u32);
            }
            log_red!(emu, "ole32!CoGetMalloc => 0x{:x}", hndl);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CoGetMarshalSizeMax" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetMarshalSizeMax arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetMarshalSizeMax");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetModuleType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetModuleType arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetModuleType");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetObject" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let outer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

            let hndl = helper::handler_create("com_object");
            let ppv_addr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv_addr != 0 {
                emu.maps.write_dword(ppv_addr, hndl as u32);
            }
            log_red!(emu, "ole32!CoGetObject ctx:0x{:x} => 0x{:x}", ctx, hndl);
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CoGetObjectContext" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let outer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

            let hndl = helper::handler_create("com_object");
            let ppv_addr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv_addr != 0 {
                emu.maps.write_dword(ppv_addr, hndl as u32);
            }
            log_red!(
                emu,
                "ole32!CoGetObjectContext ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CoGetPSClsid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetPSClsid arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetPSClsid");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetProcessIdentifier" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetProcessIdentifier arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetProcessIdentifier");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetStandardMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetStandardMarshal arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetStandardMarshal");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetStdMarshalEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetStdMarshalEx arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetStdMarshalEx");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetSystemSecurityPermissions" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CoGetSystemSecurityPermissions arg1:0x{:x}",
                arg1
            );
            let handle = helper::handler_create("CoGetSystemSecurityPermissions");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetSystemWow64DirectoryW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetSystemWow64DirectoryW arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetSystemWow64DirectoryW");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoGetTreatAsClass" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoGetTreatAsClass arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoGetTreatAsClass");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoHandlePriorityEventsFromMessagePump" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(
                emu,
                "ole32!CoHandlePriorityEventsFromMessagePump 0x{:x}",
                arg1
            );
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
        "CoIncrementMTAUsage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoIncrementMTAUsage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoInitializeSecurity" => {
            let sec_desc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let c_auth_svc = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let as_auth_svc = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _reserved1 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let authn_level = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let imp_level = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let auth_list = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
            let capabilities = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);
            let _reserved3 = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0);
            log_red!(
                emu,
                "ole32!CoInitializeSecurity sd: 0x{:x} authn: {} imp: {} caps: 0x{:x}",
                sec_desc,
                authn_level,
                imp_level,
                capabilities
            );
            let handle = helper::handler_create("CoInitializeSecurity");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..9u32 {
                emu.stack_pop32(false);
            }
        }
        "CoInitializeWOW" => {
            let reserved = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoInitializeWOW");
            let handle = helper::handler_create("CoInitializeWOW");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoInstall" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoInstall arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoInstall");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoInvalidateRemoteMachineBindings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CoInvalidateRemoteMachineBindings arg1:0x{:x}",
                arg1
            );
            let handle = helper::handler_create("CoInvalidateRemoteMachineBindings");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoIsHandlerConnected" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoIsHandlerConnected arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoIsOle1Class" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoIsOle1Class arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoLoadLibrary" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ole32!CoLoadLibrary arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("ole32://CoLoadLibrary");
            emu.regs_mut().rax = hndl;
        }
        "CoLockObjectExternal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ole32!CoLockObjectExternal 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CoMarshalHresult" => {
            let stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoMarshalHresult stream:0x{:x}", stream);
            let handle = helper::handler_create("CoMarshalHresult");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoMarshalInterThreadInterfaceInStream" => {
            let stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CoMarshalInterThreadInterfaceInStream stream:0x{:x}",
                stream
            );
            let handle = helper::handler_create("CoMarshalInterThreadInterfaceInStream");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoMarshalInterface" => {
            let stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoMarshalInterface stream:0x{:x}", stream);
            let handle = helper::handler_create("CoMarshalInterface");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoPopServiceDomain" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoPopServiceDomain arg1:0x{:x}", arg1);
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
        "CoPushServiceDomain" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoPushServiceDomain arg1:0x{:x}", arg1);
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
        "CoQueryAuthenticationServices" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoQueryAuthenticationServices arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoQueryClientBlanket" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoQueryClientBlanket arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoQueryProxyBlanket" => {
            let p_proxy = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p_authn_svc = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p_authz_svc = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p_server_princ = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p_authn_level = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p_imp_level = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p_auth_info = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p_capabilites = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoQueryProxyBlanket proxy:0x{:x}", p_proxy);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..8u32 {
                emu.stack_pop32(false);
            }
        }
        "CoQueryReleaseObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoQueryReleaseObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoReactivateObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoReactivateObject arg1:0x{:x}", arg1);
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
        "CoRegisterActivationFilter" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRegisterActivationFilter arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoRegisterActivationFilter");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRegisterChannelHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRegisterChannelHook arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoRegisterChannelHook");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRegisterClassObject" => {
            let rclsid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _punk = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let cls_ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let pdw_register = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
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
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CoRegisterInitializeSpy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRegisterInitializeSpy arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoRegisterInitializeSpy");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRegisterMallocSpy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRegisterMallocSpy arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoRegisterMallocSpy");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRegisterMessageFilter" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRegisterMessageFilter arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoRegisterMessageFilter");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRegisterPSClsid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRegisterPSClsid arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoRegisterPSClsid");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRegisterSurrogate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRegisterSurrogate arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoRegisterSurrogate");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRegisterSurrogateEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRegisterSurrogateEx arg1:0x{:x}", arg1);
            let handle = helper::handler_create("CoRegisterSurrogateEx");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoReleaseMarshalData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoReleaseMarshalData arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoReleaseServerProcess" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoReleaseServerProcess arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoResumeClassObjects" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoResumeClassObjects arg1:0x{:x}", arg1);
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
        "CoRetireServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRetireServer arg1:0x{:x}", arg1);
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
        "CoRevertToSelf" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRevertToSelf arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRevokeClassObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRevokeClassObject arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRevokeInitializeSpy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRevokeInitializeSpy arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoRevokeMallocSpy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoRevokeMallocSpy arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoSetCancelObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoSetCancelObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoSetMessageDispatcher" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoSetMessageDispatcher arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoSetProxyBlanket" => {
            let proxy = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let authn_svc = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let authz_svc = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let server_princ = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CoSetProxyBlanket proxy:0x{:x} authn:{} authz:{}",
                proxy,
                authn_svc,
                authz_svc
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
        }
        "CoSetState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoSetState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoSuspendClassObjects" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoSuspendClassObjects arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoSwitchCallContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoSwitchCallContext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoTaskMemRealloc" => {
            let old_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let new_size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CoTaskMemRealloc old:0x{:x} size:{}",
                old_ptr,
                new_size
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            if new_size == 0 {
                if old_ptr != 0 {
                    heap_engine::heap_free(emu, 0, old_ptr);
                }
                for _ in 0..2u32 {
                    emu.stack_pop32(false);
                }
                emu.regs_mut().rax = 0;
            } else {
                let new_ptr = heap_engine::heap_allocate(emu, 0, new_size).unwrap_or(0);
                if new_ptr != 0 && old_ptr != 0 {
                    let copy_size = std::cmp::min(new_size, 0x1000);
                    for i in 0..copy_size {
                        let b = emu.maps.read_byte(old_ptr + i).unwrap_or(0);
                        emu.maps.write_byte(new_ptr + i, b);
                    }
                    heap_engine::heap_free(emu, 0, old_ptr);
                }
                for _ in 0..2u32 {
                    emu.stack_pop32(false);
                }
                emu.regs_mut().rax = new_ptr;
            }
        }
        "CoTestCancel" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoTestCancel arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoTreatAsClass" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoTreatAsClass arg1:0x{:x}", arg1);
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
        "CoTryGetInterceptorForAppModelVirtualization" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!CoTryGetInterceptorForAppModelVirtualization arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoUnloadingWOW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoUnloadingWOW arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoUnmarshalHresult" => {
            let stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoUnmarshalHresult stream:0x{:x}", stream);
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
        "CoUnmarshalInterface" => {
            let stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoUnmarshalInterface stream:0x{:x}", stream);
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
        "CoVrfCheckThreadState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoVrfCheckThreadState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoVrfGetThreadState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoVrfGetThreadState arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoVrfReleaseThreadState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!CoVrfReleaseThreadState arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CoWaitForMultipleHandles" => {
            let dw_flags = arg(emu, 0);
            let dw_timeout = arg(emu, 1);
            let c_handles = arg(emu, 2);
            let p_handles = arg(emu, 3);
            let lpdw_index = arg(emu, 4);
            log_red!(
                emu,
                "ole32!CoWaitForMultipleHandles flags:0x{:x} timeout:{} count:{} handles:0x{:x}",
                dw_flags,
                dw_timeout,
                c_handles,
                p_handles
            );
            if lpdw_index != 0 {
                emu.maps.write_dword(lpdw_index as u64, 0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
        }
        "CoWaitForMultipleObjects" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ole32!CoWaitForMultipleObjects 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ComPs_NdrDllCanUnloadNow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!ComPs_NdrDllCanUnloadNow arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "ComPs_NdrDllGetClassObject" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let outer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

            let hndl = helper::handler_create("com_object");
            let ppv_addr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ppv_addr != 0 {
                emu.maps.write_dword(ppv_addr, hndl as u32);
            }
            log_red!(
                emu,
                "ole32!ComPs_NdrDllGetClassObject ctx:0x{:x} => 0x{:x}",
                ctx,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "ComPs_NdrDllRegisterProxy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!ComPs_NdrDllRegisterProxy arg1:0x{:x}", arg1);
            let handle = helper::handler_create("ComPs_NdrDllRegisterProxy");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "ComPs_NdrDllUnregisterProxy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!ComPs_NdrDllUnregisterProxy arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CreateAntiMoniker" => {
            let pp_moniker = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://AntiMoniker");
            if pp_moniker != 0 {
                emu.maps.write_dword(pp_moniker, hndl as u32);
            }
            log_red!(emu, "ole32!CreateAntiMoniker =0x{:x}", hndl);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateBindCtx" => {
            let reserved = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let ppbc = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://BindCtx");
            if ppbc != 0 {
                emu.maps.write_dword(ppbc, hndl as u32);
            }
            log_red!(emu, "ole32!CreateBindCtx =0x{:x}", hndl);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateClassMoniker" => {
            let rclsid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pp_moniker = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://ClassMoniker");
            if pp_moniker != 0 {
                emu.maps.write_dword(pp_moniker, hndl as u32);
            }
            log_red!(
                emu,
                "ole32!CreateClassMoniker clsid:0x{:x} =0x{:x}",
                rclsid,
                hndl
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateDataAdviseHolder" => {
            let pp_holder = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://DataAdviseHolder");
            if pp_holder != 0 {
                emu.maps.write_dword(pp_holder, hndl as u32);
            }
            log_red!(emu, "ole32!CreateDataAdviseHolder =0x{:x}", hndl);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateDataCache" => {
            let punk_outer = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let rclsid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let riid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let ppv = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://DataCache");
            if ppv != 0 {
                emu.maps.write_dword(ppv, hndl as u32);
            }
            log_red!(emu, "ole32!CreateDataCache =0x{:x}", hndl);
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateErrorInfo" => {
            let pp_errinfo = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://ErrorInfo");
            if pp_errinfo != 0 {
                emu.maps.write_dword(pp_errinfo, hndl as u32);
            }
            log_red!(emu, "ole32!CreateErrorInfo =0x{:x}", hndl);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateFileMoniker" => {
            let psz_path = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pp_moniker = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let path = emu.maps.read_wide_string(psz_path);
            let hndl = helper::handler_create(&format!("ole32://FileMoniker:{}", path));
            if pp_moniker != 0 {
                emu.maps.write_dword(pp_moniker, hndl as u32);
            }
            log_red!(emu, "ole32!CreateFileMoniker '{}' =0x{:x}", path, hndl);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateGenericComposite" => {
            let pmk_first = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pmk_rest = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pp_moniker = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://GenericComposite");
            if pp_moniker != 0 {
                emu.maps.write_dword(pp_moniker, hndl as u32);
            }
            log_red!(emu, "ole32!CreateGenericComposite =0x{:x}", hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateILockBytesOnHGlobal" => {
            let hglobal = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let f_delete = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pp_lkbyt = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://ILockBytes");
            if pp_lkbyt != 0 {
                emu.maps.write_dword(pp_lkbyt, hndl as u32);
            }
            log_red!(emu, "ole32!CreateILockBytesOnHGlobal =0x{:x}", hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateItemMoniker" => {
            let psz_delim = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let psz_item = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pp_moniker = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://ItemMoniker");
            if pp_moniker != 0 {
                emu.maps.write_dword(pp_moniker, hndl as u32);
            }
            log_red!(emu, "ole32!CreateItemMoniker =0x{:x}", hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateObjrefMoniker" => {
            let punk = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pp_moniker = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://ObjrefMoniker");
            if pp_moniker != 0 {
                emu.maps.write_dword(pp_moniker, hndl as u32);
            }
            log_red!(emu, "ole32!CreateObjrefMoniker =0x{:x}", hndl);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateOleAdviseHolder" => {
            let pp_holder = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://OleAdviseHolder");
            if pp_holder != 0 {
                emu.maps.write_dword(pp_holder, hndl as u32);
            }
            log_red!(emu, "ole32!CreateOleAdviseHolder =0x{:x}", hndl);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreatePointerMoniker" => {
            let punk = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pp_moniker = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://PointerMoniker");
            if pp_moniker != 0 {
                emu.maps.write_dword(pp_moniker, hndl as u32);
            }
            log_red!(emu, "ole32!CreatePointerMoniker =0x{:x}", hndl);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateStdProgressIndicator" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let psz_title = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let pibsc = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let pp_out = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://StdProgressIndicator");
            if pp_out != 0 {
                emu.maps.write_dword(pp_out, hndl as u32);
            }
            log_red!(emu, "ole32!CreateStdProgressIndicator =0x{:x}", hndl);
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "CreateStreamOnHGlobal" => {
            let hglobal = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let f_delete = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let ppstm = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://Stream");
            if ppstm != 0 {
                emu.maps.write_dword(ppstm, hndl as u32);
            }
            log_red!(emu, "ole32!CreateStreamOnHGlobal =0x{:x}", hndl);
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "DcomChannelSetHResult" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!DcomChannelSetHResult arg1:0x{:x} arg2:0x{:x}",
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
        "DdeBindToObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!DdeBindToObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("DdeBindToObject");
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
        "DeletePatternAndExtensionTables" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "ole32!DeletePatternAndExtensionTables handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DestroyRunningObjectTable" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "ole32!DestroyRunningObjectTable handle:0x{:x}", handle);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DllDebugObjectRPCHook" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!DllDebugObjectRPCHook h:0x{:x} arg2:0x{:x}",
                handle,
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
        "DllGetClassObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!DllGetClassObject h:0x{:x} arg2:0x{:x}",
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
        "DllGetClassObjectWOW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "ole32!DllGetClassObjectWOW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DllRegisterServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!DllRegisterServer arg1:0x{:x} arg2:0x{:x}",
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
        "DoDragDrop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!DoDragDrop arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
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
        "DragDropSetFDT" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!DragDropSetFDT arg1:0x{:x} arg2:0x{:x}",
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
        "EnableHookObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!EnableHookObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("EnableHookObject");
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
        "FindExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!FindExt arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
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
        "FmtIdToPropStgName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!FmtIdToPropStgName arg1:0x{:x} arg2:0x{:x}",
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
        "FreePropVariantArray" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "ole32!FreePropVariantArray handle:0x{:x}", handle);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetActiveObjectExt" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!GetActiveObjectExt h:0x{:x} arg2:0x{:x}",
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
        "GetClassFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!GetClassFile h:0x{:x} arg2:0x{:x}", handle, arg2);
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
        "GetConvertStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!GetConvertStg h:0x{:x} arg2:0x{:x}",
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
        "GetDocumentBitStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!GetDocumentBitStg h:0x{:x} arg2:0x{:x}",
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
            if pperrinfo != 0 {
                emu.maps.write_dword(pperrinfo, 0); // no error info
            }
            log_red!(emu, "ole32!GetErrorInfo => S_FALSE (no error info)");
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1; // S_FALSE
        }
        "GetHGlobalFromILockBytes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ole32!GetHGlobalFromILockBytes 0x{:x}", arg1);
            let handle = helper::handler_create("GetHGlobalFromILockBytes");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetHGlobalFromStream" => {
            let pstm = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let phglobal = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "GetHookInterface" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!GetHookInterface h:0x{:x} arg2:0x{:x}",
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
        "GetObjectFromRotByPath" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!GetObjectFromRotByPath h:0x{:x} arg2:0x{:x}",
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
        "GetRunningObjectTable" => {
            let reserved = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pp_rot = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hndl = helper::handler_create("ole32://IRunningObjectTable");
            if pp_rot != 0 {
                emu.maps.write_dword(pp_rot, hndl as u32);
            }
            log_red!(emu, "ole32!GetRunningObjectTable =0x{:x}", hndl);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "HACCEL_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HACCEL_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HACCEL_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HACCEL_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HACCEL_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HACCEL_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HACCEL_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HACCEL_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HACCEL_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HACCEL_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HACCEL_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HACCEL_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HACCEL_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HACCEL_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HACCEL_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HACCEL_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HBITMAP_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBITMAP_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HBITMAP_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBITMAP_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HBITMAP_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBITMAP_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HBITMAP_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBITMAP_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HBITMAP_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBITMAP_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HBITMAP_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBITMAP_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HBITMAP_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBITMAP_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HBITMAP_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBITMAP_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HBRUSH_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBRUSH_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HBRUSH_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBRUSH_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HBRUSH_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBRUSH_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HBRUSH_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBRUSH_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HBRUSH_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBRUSH_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HBRUSH_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBRUSH_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HBRUSH_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBRUSH_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HBRUSH_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HBRUSH_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HDC_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HDC_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HDC_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HDC_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HDC_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HDC_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HDC_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HDC_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HDC_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HDC_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HDC_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HDC_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HDC_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HDC_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HDC_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HDC_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HENHMETAFILE_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HENHMETAFILE_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HENHMETAFILE_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HENHMETAFILE_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HENHMETAFILE_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HENHMETAFILE_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HENHMETAFILE_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HENHMETAFILE_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HENHMETAFILE_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HGLOBAL_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HGLOBAL_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HGLOBAL_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HGLOBAL_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HGLOBAL_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HGLOBAL_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HGLOBAL_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HGLOBAL_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HGLOBAL_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HICON_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HICON_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HICON_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HICON_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HICON_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HICON_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HICON_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HICON_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HICON_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HICON_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HICON_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HICON_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HICON_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HICON_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HICON_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HICON_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMENU_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMENU_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HMENU_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMENU_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMENU_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMENU_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HMENU_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMENU_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMENU_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMENU_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HMENU_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMENU_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMENU_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMENU_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HMENU_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMENU_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILEPICT_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILEPICT_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILEPICT_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILEPICT_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILEPICT_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILEPICT_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILEPICT_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILEPICT_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILEPICT_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILE_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILE_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILE_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILE_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILE_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILE_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILE_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HMETAFILE_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMETAFILE_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMONITOR_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMONITOR_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HMONITOR_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMONITOR_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMONITOR_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMONITOR_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HMONITOR_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMONITOR_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMONITOR_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMONITOR_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HMONITOR_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMONITOR_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HMONITOR_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMONITOR_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HMONITOR_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HMONITOR_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserFreeExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserFreeExt arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserMarshalExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserMarshalExt arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserSizeExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserSizeExt arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HPALETTE_UserUnmarshalExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HPALETTE_UserUnmarshalExt arg1:0x{:x} arg2:0x{:x}",
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
        "HRGN_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HRGN_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HRGN_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HRGN_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HRGN_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HRGN_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HRGN_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HRGN_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserFree64Ext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserFree64Ext arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserFreeExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserFreeExt arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserMarshal64Ext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserMarshal64Ext arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserMarshalExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserMarshalExt arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserSize64Ext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserSize64Ext arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserSizeExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserSizeExt arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
                "ole32!HWND_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserUnmarshal64Ext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserUnmarshal64Ext arg1:0x{:x} arg2:0x{:x}",
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
        "HWND_UserUnmarshalExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HWND_UserUnmarshalExt arg1:0x{:x} arg2:0x{:x}",
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
        "HkOleRegisterObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!HkOleRegisterObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("HkOleRegisterObject");
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
        "IIDFromString" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "IsAccelerator" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!IsAccelerator arg1:0x{:x} arg2:0x{:x}",
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
        "IsEqualGUID" => {
            let guid1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let guid2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = equal as u64;
        }
        "IsRoInitializeASTAAllowedInDesktop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!IsRoInitializeASTAAllowedInDesktop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("IsRoInitializeASTAAllowedInDesktop");
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
        "IsValidIid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!IsValidIid arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
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
        "IsValidInterface" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!IsValidInterface arg1:0x{:x} arg2:0x{:x}",
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
        "IsValidPtrIn" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!IsValidPtrIn arg1:0x{:x} arg2:0x{:x}",
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
        "IsValidPtrOut" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!IsValidPtrOut arg1:0x{:x} arg2:0x{:x}",
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
        "MkParseDisplayName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!MkParseDisplayName arg1:0x{:x} arg2:0x{:x}",
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
        "MonikerCommonPrefixWith" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!MonikerCommonPrefixWith arg1:0x{:x} arg2:0x{:x}",
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
        "MonikerLoadTypeLib" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!MonikerLoadTypeLib arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("MonikerLoadTypeLib");
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
        "MonikerRelativePathTo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!MonikerRelativePathTo arg1:0x{:x} arg2:0x{:x}",
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
        "NdrOleInitializeExtension" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrOleInitializeExtension arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction10" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction10 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction11" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction11 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction12" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction12 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction13" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction13 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction14" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction14 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction15" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction15 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction16" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction16 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction17" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction17 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction18" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction18 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction19" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction19 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction20" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction20 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction21" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction21 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction22" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction22 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction23" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction23 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction24" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction24 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction25" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction25 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction26" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction26 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction27" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction27 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction28" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction28 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction29" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction29 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction3" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction3 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction30" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction30 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction31" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction31 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction32" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction32 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction4" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction4 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction5" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction5 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction6" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction6 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction7" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction7 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction8" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction8 arg1:0x{:x} arg2:0x{:x}",
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
        "NdrProxyForwardingFunction9" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!NdrProxyForwardingFunction9 arg1:0x{:x} arg2:0x{:x}",
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
        "ObjectStublessClient10" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient10 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient11" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient11 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient12" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient12 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient13" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient13 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient14" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient14 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient15" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient15 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient16" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient16 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient17" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient17 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient18" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient18 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient19" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient19 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient20" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient20 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient21" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient21 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient22" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient22 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient23" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient23 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient24" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient24 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient25" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient25 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient26" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient26 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient27" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient27 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient28" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient28 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient29" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient29 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient3" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient3 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient30" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient30 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient31" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient31 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient32" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient32 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient4" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient4 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient5" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient5 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient6" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient6 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient7" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient7 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient8" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient8 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "ObjectStublessClient9" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ObjectStublessClient9 h:0x{:x} arg2:0x{:x}",
                handle,
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
        "Ole32DllGetClassObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!Ole32DllGetClassObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleBuildVersion" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleBuildVersion arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleConvertIStorageToOLESTREAM" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleConvertIStorageToOLESTREAM arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleConvertIStorageToOLESTREAMEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!OleConvertIStorageToOLESTREAMEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleConvertOLESTREAMToIStorage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleConvertOLESTREAMToIStorage arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleConvertOLESTREAMToIStorage2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorage2 arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleConvertOLESTREAMToIStorageEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorageEx arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleConvertOLESTREAMToIStorageEx2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!OleConvertOLESTREAMToIStorageEx2 arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreate" => {
            let rclsid = arg(emu, 0) as u64;
            let riid = arg(emu, 1) as u64;
            let render_opt = arg(emu, 2);
            let p_format_etc = arg(emu, 3) as u64;
            let p_client_site = arg(emu, 4) as u64;
            let p_stg = arg(emu, 5) as u64;
            let ppv_obj = arg(emu, 6) as u64;
            log_red!(
                emu,
                "ole32!OleCreate clsid:0x{:x} iid:0x{:x} render:{} site:0x{:x} stg:0x{:x}",
                rclsid,
                riid,
                render_opt,
                p_client_site,
                p_stg
            );
            if ppv_obj != 0 {
                let h = helper::handler_create("ole_object");
                emu.maps.write_dword(ppv_obj, h as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateDefaultHandler" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateDefaultHandler arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateDefaultHandler");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateEmbeddingHelper" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateEmbeddingHelper arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateEmbeddingHelper");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateEx arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateEx");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateFontIndirectExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateFontIndirectExt arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateFontIndirectExt");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateFromData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateFromData arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateFromData");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateFromDataEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateFromDataEx arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateFromDataEx");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateFromFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateFromFile arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateFromFile");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateFromFileEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateFromFileEx arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateFromFileEx");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateLink" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateLink arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateLink");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateLinkEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateLinkEx arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateLinkEx");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateLinkFromData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateLinkFromData arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateLinkFromData");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateLinkFromDataEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateLinkFromDataEx arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateLinkFromDataEx");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateLinkToFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateLinkToFile arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateLinkToFile");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateLinkToFileEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateLinkToFileEx arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateLinkToFileEx");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateMenuDescriptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ole32!OleCreateMenuDescriptor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("ole32://OleCreateMenuDescriptor");
            emu.regs_mut().rax = hndl;
        }
        "OleCreatePictureIndirectExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreatePictureIndirectExt arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreatePictureIndirectExt");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreatePropertyFrameIndirectExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!OleCreatePropertyFrameIndirectExt arg1:0x{:x}",
                arg1
            );
            let handle = helper::handler_create("OleCreatePropertyFrameIndirectExt");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleCreateStaticFromData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleCreateStaticFromData arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleCreateStaticFromData");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleDestroyMenuDescriptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleDestroyMenuDescriptor arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleDoAutoConvert" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleDoAutoConvert arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleDraw" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleDraw arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleDuplicateData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "ole32!OleDuplicateData arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("ole32://OleDuplicateData");
            emu.regs_mut().rax = hndl;
        }
        "OleFlushClipboard" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleFlushClipboard arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleGetAutoConvert" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleGetAutoConvert arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleGetClipboard" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleGetClipboard arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleGetClipboardWithEnterpriseInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!OleGetClipboardWithEnterpriseInfo arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleGetIconOfClass" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "ole32!OleGetIconOfClass arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("ole32://OleGetIconOfClass");
            emu.regs_mut().rax = hndl;
        }
        "OleGetIconOfFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ole32!OleGetIconOfFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("ole32://OleGetIconOfFile");
            emu.regs_mut().rax = hndl;
        }
        "OleGetPackageClipboardOwner" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleGetPackageClipboardOwner arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleIconToCursorExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleIconToCursorExt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleInitializeWOW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleInitializeWOW arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleInitializeWOW");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleIsCurrentClipboard" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleIsCurrentClipboard arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleIsRunning" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleIsRunning arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleLoad" => {
            let p_stg = arg(emu, 0) as u64;
            let riid = arg(emu, 1) as u64;
            let p_client_site = arg(emu, 2) as u64;
            let ppv_obj = arg(emu, 3) as u64;
            log_red!(
                emu,
                "ole32!OleLoad stg:0x{:x} iid:0x{:x} site:0x{:x} out:0x{:x}",
                p_stg,
                riid,
                p_client_site,
                ppv_obj
            );
            if ppv_obj != 0 {
                let h = helper::handler_create("ole_object");
                emu.maps.write_dword(ppv_obj, h as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "OleLoadFromStream" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleLoadFromStream arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleLoadFromStream");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleLoadPictureExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleLoadPictureExt arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleLoadPictureExt");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleLoadPictureFileExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleLoadPictureFileExt arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleLoadPictureFileExt");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleLoadPicturePathExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleLoadPicturePathExt arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleLoadPicturePathExt");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleLockRunning" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ole32!OleLockRunning 0x{:x}", arg1);
            let handle = helper::handler_create("OleLockRunning");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "OleMetafilePictFromIconAndLabel" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "ole32!OleMetafilePictFromIconAndLabel arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("ole32://unknown");
            emu.regs_mut().rax = hndl;
        }
        "OleNoteObjectVisible" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleNoteObjectVisible arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleQueryCreateFromData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleQueryCreateFromData arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleQueryLinkFromData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleQueryLinkFromData arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleRegEnumFormatEtc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleRegEnumFormatEtc arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleRegEnumVerbs" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleRegEnumVerbs arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleRegGetMiscStatus" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleRegGetMiscStatus arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleRegGetUserType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleRegGetUserType arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleReleaseEnumVerbCache" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleReleaseEnumVerbCache arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleRun" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleRun arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleRun");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleSave" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleSave arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleSave");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleSavePictureFileExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleSavePictureFileExt arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleSavePictureFileExt");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleSaveToStream" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleSaveToStream arg1:0x{:x}", arg1);
            let handle = helper::handler_create("OleSaveToStream");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleSetAutoConvert" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleSetAutoConvert arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleSetClipboard" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleSetClipboard arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleSetContainedObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleSetContainedObject arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleSetMenuDescriptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleSetMenuDescriptor arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleTranslateAccelerator" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleTranslateAccelerator arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleTranslateColorExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!OleTranslateColorExt arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "OleUninitialize" => {
            log_red!(emu, "ole32!OleUninitialize");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
        }
        "OpenOrCreateStream" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("OpenOrCreateStream");
            log_red!(emu, "ole32!OpenOrCreateStream => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ProgIDFromCLSID" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let ppsz = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "PropStgNameToFmtId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!PropStgNameToFmtId arg1:0x{:x}", arg1);
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
        "PropSysAllocString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!PropSysAllocString arg1:0x{:x}", arg1);
            let handle = helper::handler_create("PropSysAllocString");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "PropSysFreeString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!PropSysFreeString arg1:0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "PropVariantChangeType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ole32!PropVariantChangeType arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "PropVariantClear" => {
            let pvar = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if pvar != 0 {
                emu.maps.write_word(pvar, 0); // VT_EMPTY
            }
            log_red!(emu, "ole32!PropVariantClear ptr:0x{:x}", pvar);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "PropVariantCopy" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if dst != 0 && src != 0 {
                for i in 0..4u64 {
                    let dw = emu.maps.read_dword(src + i * 4).unwrap_or(0);
                    emu.maps.write_dword(dst + i * 4, dw);
                }
            }
            log_red!(emu, "ole32!PropVariantCopy dst:0x{:x} src:0x{:x}", dst, src);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "ReadClassStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!ReadClassStg h:0x{:x} arg2:0x{:x}", handle, arg2);
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
        "ReadClassStm" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!ReadClassStm h:0x{:x} arg2:0x{:x}", handle, arg2);
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
        "ReadFmtUserTypeStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ReadFmtUserTypeStg h:0x{:x} arg2:0x{:x}",
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
        "ReadOleStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!ReadOleStg h:0x{:x} arg2:0x{:x}", handle, arg2);
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
        "ReadStorageProperties" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ReadStorageProperties h:0x{:x} arg2:0x{:x}",
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
        "ReadStringStream" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!ReadStringStream h:0x{:x} arg2:0x{:x}",
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
        "RegisterActiveObjectExt" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!RegisterActiveObjectExt h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("RegisterActiveObjectExt");
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
        "RegisterDragDrop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!RegisterDragDrop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterDragDrop");
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
        "ReleaseStgMedium" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "ole32!ReleaseStgMedium handle:0x{:x}", handle);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RevokeActiveObjectExt" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!RevokeActiveObjectExt h:0x{:x} arg2:0x{:x}",
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
        "RevokeDragDrop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!RevokeDragDrop arg1:0x{:x} arg2:0x{:x}",
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
        "RoGetAgileReference" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!RoGetAgileReference arg1:0x{:x} arg2:0x{:x}",
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
        "SNB_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SNB_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "SNB_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SNB_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "SNB_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SNB_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "SNB_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SNB_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "SNB_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SNB_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "SNB_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SNB_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "SNB_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SNB_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "SNB_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SNB_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserFreeExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserFreeExt arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserMarshalExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserMarshalExt arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserSizeExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserSizeExt arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "STGMEDIUM_UserUnmarshalExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!STGMEDIUM_UserUnmarshalExt arg1:0x{:x} arg2:0x{:x}",
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
        "SetConvertStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SetConvertStg h:0x{:x} arg2:0x{:x}",
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
        "SetDocumentBitStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SetDocumentBitStg h:0x{:x} arg2:0x{:x}",
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
        "SetErrorInfo" => {
            let _reserved = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let perrinfo = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!SetErrorInfo errinfo:0x{:x}", perrinfo);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0; // S_OK
        }
        "SetOleautModule" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SetOleautModule h:0x{:x} arg2:0x{:x}",
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
        "SetWOWThunkGlobalPtr" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!SetWOWThunkGlobalPtr h:0x{:x} arg2:0x{:x}",
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
        "StdTypesGetClassObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StdTypesGetClassObject h:0x{:x} arg2:0x{:x}",
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
        "StdTypesRegisterServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StdTypesRegisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("StdTypesRegisterServer");
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
        "StgConvertPropertyToVariant" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgConvertPropertyToVariant arg1:0x{:x} arg2:0x{:x}",
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
        "StgConvertVariantToProperty" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgConvertVariantToProperty arg1:0x{:x} arg2:0x{:x}",
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
        "StgCreateDocfile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgCreateDocfile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("StgCreateDocfile");
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
        "StgCreateDocfileOnILockBytes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ole32!StgCreateDocfileOnILockBytes 0x{:x}", arg1);
            let handle = helper::handler_create("StgCreateDocfileOnILockBytes");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "StgCreatePropSetStg" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgCreatePropSetStg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("StgCreatePropSetStg");
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
        "StgCreatePropStg" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgCreatePropStg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("StgCreatePropStg");
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
        "StgCreateStorageEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgCreateStorageEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("StgCreateStorageEx");
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
        "StgGetIFillLockBytesOnFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ole32!StgGetIFillLockBytesOnFile 0x{:x}", arg1);
            let handle = helper::handler_create("StgGetIFillLockBytesOnFile");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "StgGetIFillLockBytesOnILockBytes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ole32!StgGetIFillLockBytesOnILockBytes 0x{:x}", arg1);
            let handle = helper::handler_create("StgGetIFillLockBytesOnILockBytes");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "StgIsStorageFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgIsStorageFile h:0x{:x} arg2:0x{:x}",
                handle,
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
        "StgIsStorageILockBytes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ole32!StgIsStorageILockBytes 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "StgOpenAsyncDocfileOnIFillLockBytes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(
                emu,
                "ole32!StgOpenAsyncDocfileOnIFillLockBytes 0x{:x}",
                arg1
            );
            let handle = helper::handler_create("StgOpenAsyncDocfileOnIFillLockBytes");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "StgOpenPropStg" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgOpenPropStg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("StgOpenPropStg");
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
        "StgOpenStorage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgOpenStorage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("StgOpenStorage");
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
        "StgOpenStorageEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgOpenStorageEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("StgOpenStorageEx");
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
        "StgOpenStorageOnHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgOpenStorageOnHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("StgOpenStorageOnHandle");
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
        "StgOpenStorageOnILockBytes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ole32!StgOpenStorageOnILockBytes 0x{:x}", arg1);
            let handle = helper::handler_create("StgOpenStorageOnILockBytes");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "StgPropertyLengthAsVariant" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!StgPropertyLengthAsVariant arg1:0x{:x} arg2:0x{:x}",
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
        "StgSetTimes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!StgSetTimes arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
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
        "StringFromCLSID" => {
            let clsid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let guid_str = read_guid_string(emu, clsid_ptr);
            if out_ptr != 0 {
                let alloc_sz: u64 = 39 * 2 + 2;
                if let Some(buf) = emu.maps.alloc(alloc_sz) {
                    emu.maps.create_map(
                        &format!("guid_str_{:x}", buf),
                        buf,
                        alloc_sz,
                        crate::maps::mem64::Permission::READ_WRITE,
                    );
                    emu.maps.write_wide_string(buf, &guid_str);
                    emu.maps.write_dword(out_ptr, buf as u32);
                } else {
                    emu.maps.write_dword(out_ptr, 0);
                    emu.regs_mut().rax = 0x8007000E; // E_OUTOFMEMORY
                    for _ in 0..2 {
                        emu.stack_pop32(false);
                    }
                    return api.to_string();
                }
            }
            log_red!(emu, "ole32!StringFromCLSID => '{}'", guid_str);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "StringFromGUID2" => {
            let guid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let buf_size = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as i32;
            let guid_str = read_guid_string(emu, guid_ptr);
            let needed: i32 = 39;
            if buf_ptr != 0 && buf_size >= needed {
                emu.maps.write_wide_string(buf_ptr, &guid_str);
                emu.regs_mut().rax = needed as u64;
            } else {
                emu.regs_mut().rax = 0;
            }
            log_red!(emu, "ole32!StringFromGUID2 => '{}'", guid_str);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "StringFromIID" => {
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "UpdateDCOMSettings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!UpdateDCOMSettings arg1:0x{:x} arg2:0x{:x}",
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
        "UpdateProcessTracing" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!UpdateProcessTracing h:0x{:x} arg2:0x{:x}",
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
        "UtConvertDvtd16toDvtd32" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!UtConvertDvtd16toDvtd32 arg1:0x{:x} arg2:0x{:x}",
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
        "UtConvertDvtd32toDvtd16" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!UtConvertDvtd32toDvtd16 arg1:0x{:x} arg2:0x{:x}",
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
        "UtGetDvtd16Info" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!UtGetDvtd16Info arg1:0x{:x} arg2:0x{:x}",
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
        "UtGetDvtd32Info" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!UtGetDvtd32Info arg1:0x{:x} arg2:0x{:x}",
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
        "WdtpInterfacePointer_UserFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserFree arg1:0x{:x} arg2:0x{:x}",
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
        "WdtpInterfacePointer_UserFree64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserFree64 arg1:0x{:x} arg2:0x{:x}",
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
        "WdtpInterfacePointer_UserMarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserMarshal arg1:0x{:x} arg2:0x{:x}",
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
        "WdtpInterfacePointer_UserMarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserMarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "WdtpInterfacePointer_UserSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserSize arg1:0x{:x} arg2:0x{:x}",
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
        "WdtpInterfacePointer_UserSize64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserSize64 arg1:0x{:x} arg2:0x{:x}",
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
        "WdtpInterfacePointer_UserUnmarshal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserUnmarshal arg1:0x{:x} arg2:0x{:x}",
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
        "WdtpInterfacePointer_UserUnmarshal64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WdtpInterfacePointer_UserUnmarshal64 arg1:0x{:x} arg2:0x{:x}",
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
        "WriteClassStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WriteClassStg h:0x{:x} arg2:0x{:x}",
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
        "WriteClassStm" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WriteClassStm h:0x{:x} arg2:0x{:x}",
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
        "WriteFmtUserTypeStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WriteFmtUserTypeStg h:0x{:x} arg2:0x{:x}",
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
        "WriteOleStg" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ole32!WriteOleStg h:0x{:x} arg2:0x{:x}", handle, arg2);
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
        "WriteStorageProperties" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WriteStorageProperties h:0x{:x} arg2:0x{:x}",
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
        "WriteStringStream" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ole32!WriteStringStream h:0x{:x} arg2:0x{:x}",
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

        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
                );
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

pub fn OleInitialize(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "ole32!OleInitialize arg1:0x{:x}", arg1);
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 1);
}

pub fn CoInitialize(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "ole32!CoInitialize arg1:0x{:x}", arg1);
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 1);
}

pub fn CoInitializeEx(emu: &mut emu::Emu) {
    let co_init = arg(emu, 1);
    log_red!(emu, "ole32!CoInitializeEx dwCoInit: 0x{:x}", co_init);
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 2);
}

pub fn CoCreateInstance(emu: &mut emu::Emu) {
    let clsid_ptr = arg(emu, 0) as u64;
    let _outer = arg(emu, 1);
    let ctx = arg(emu, 2);
    let _iid_ptr = arg(emu, 3) as u64;
    let ppv = arg(emu, 4) as u64;

    if ppv != 0 {
        emu.maps.write_dword(ppv, 0);
    }

    log_red!(
        emu,
        "ole32!CoCreateInstance clsid:0x{:x} ctx:0x{:x} ppv:0x{:x} => REGDB_E_CLASSNOTREG",
        clsid_ptr,
        ctx,
        ppv
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80040154; // REGDB_E_CLASSNOTREG
    stdcall_ret(emu, 5);
}

pub fn CoTaskMemAlloc(emu: &mut emu::Emu) {
    let size = arg(emu, 0) as u64;
    log_red!(emu, "ole32!CoTaskMemAlloc size: 0x{:x}", size);
    emu.regs_mut().rax = heap_engine::heap_allocate(emu, 0, size).unwrap_or(0);
    stdcall_ret(emu, 1);
}

pub fn CoTaskMemFree(emu: &mut emu::Emu) {
    let pv = arg(emu, 0) as u64;
    log_red!(emu, "ole32!CoTaskMemFree pv: 0x{:x}", pv);
    if pv != 0 {
        heap_engine::heap_free(emu, 0, pv);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

pub fn CoUninitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoUninitialize");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
