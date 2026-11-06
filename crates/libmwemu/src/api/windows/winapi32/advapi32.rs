use crate::api::windows::common::crypto_state;
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants::*;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "StartServiceCtrlDispatcherA" => StartServiceCtrlDispatcherA(emu),
        "StartServiceCtrlDispatcherW" => StartServiceCtrlDispatcherW(emu),
        "LookupPrivilegeValueW" => LookupPrivilegeValueW(emu),
        "CryptAcquireContextA" => CryptAcquireContextA(emu),
        "CryptAcquireContextW" => CryptAcquireContextW(emu),
        "CryptEncrypt" => CryptEncrypt(emu),
        "CryptDecrypt" => CryptDecrypt(emu),
        "CryptCreateHash" => CryptCreateHash(emu),
        "CryptGenKey" => CryptGenKey(emu),
        "CryptGetHashParam" => CryptGetHashParam(emu),
        "CryptGetKeyParam" => CryptGetKeyParam(emu),
        "CryptImportKey" => CryptImportKey(emu),
        "CryptSignHashA" => CryptSignHashA(emu),
        "CryptSignHashW" => CryptSignHashW(emu),
        "CryptReleaseContext" => CryptReleaseContext(emu),
        "CryptHashData" => CryptHashData(emu),
        "CryptDeriveKey" => CryptDeriveKey(emu),

        "RegOpenKeyExA" => RegOpenKeyExA(emu),
        "RegQueryValueExA" => RegQueryValueExA(emu),
        "RegQueryValueExW" => RegQueryValueExW(emu),
        "GetUserNameA" => GetUserNameA(emu),
        "GetUserNameW" => GetUserNameW(emu),
        "CryptGenRandom" => CryptGenRandom(emu),
        "AdjustTokenPrivileges" => AdjustTokenPrivileges(emu),

        "A_SHAFinal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!A_SHAFinal arg1:0x{:x} arg2:0x{:x}",
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
        "A_SHAInit" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!A_SHAInit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("A_SHAInit");
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
        "A_SHAUpdate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!A_SHAUpdate arg1:0x{:x} arg2:0x{:x}",
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
        "AbortSystemShutdownA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!AbortSystemShutdownA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AbortSystemShutdownW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!AbortSystemShutdownW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AccessCheck" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AccessCheck arg1:0x{:x} arg2:0x{:x}",
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
        "AccessCheckAndAuditAlarmA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AccessCheckAndAuditAlarmA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AccessCheckAndAuditAlarmW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AccessCheckAndAuditAlarmW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AccessCheckByType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AccessCheckByType arg1:0x{:x} arg2:0x{:x}",
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
        "AccessCheckByTypeAndAuditAlarmA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AccessCheckByTypeAndAuditAlarmA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AccessCheckByTypeAndAuditAlarmW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AccessCheckByTypeAndAuditAlarmW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AccessCheckByTypeResultList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AccessCheckByTypeResultList arg1:0x{:x} arg2:0x{:x}",
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
        "AccessCheckByTypeResultListAndAuditAlarmA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AccessCheckByTypeResultListAndAuditAlarmA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AccessCheckByTypeResultListAndAuditAlarmByHandleA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "advapi32!AccessCheckByTypeResultListAndAuditAlarmByHandleA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AccessCheckByTypeResultListAndAuditAlarmByHandleW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "advapi32!AccessCheckByTypeResultListAndAuditAlarmByHandleW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AccessCheckByTypeResultListAndAuditAlarmW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AccessCheckByTypeResultListAndAuditAlarmW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AddAccessAllowedAce" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddAccessAllowedAce p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let handle = helper::handler_create("AddAccessAllowedAce");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddAccessAllowedAceEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddAccessAllowedAceEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("AddAccessAllowedAceEx");
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
        "AddAccessAllowedObjectAce" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddAccessAllowedObjectAce h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("AddAccessAllowedObjectAce");
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
        "AddAccessDeniedAce" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddAccessDeniedAce p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let handle = helper::handler_create("AddAccessDeniedAce");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddAccessDeniedAceEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddAccessDeniedAceEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("AddAccessDeniedAceEx");
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
        "AddAccessDeniedObjectAce" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddAccessDeniedObjectAce h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("AddAccessDeniedObjectAce");
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
        "AddAce" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!AddAce arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("AddAce");
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
        "AddAuditAccessAce" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddAuditAccessAce arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("AddAuditAccessAce");
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
        "AddAuditAccessAceEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddAuditAccessAceEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("AddAuditAccessAceEx");
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
        "AddAuditAccessObjectAce" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddAuditAccessObjectAce h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("AddAuditAccessObjectAce");
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
        "AddConditionalAce" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddConditionalAce arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("AddConditionalAce");
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
        "AddMandatoryAce" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddMandatoryAce arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("AddMandatoryAce");
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
        "AddUsersToEncryptedFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddUsersToEncryptedFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("AddUsersToEncryptedFile");
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
        "AddUsersToEncryptedFileEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AddUsersToEncryptedFileEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("AddUsersToEncryptedFileEx");
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
        "AdjustTokenGroups" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AdjustTokenGroups h:0x{:x} arg2:0x{:x}",
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
        "AllocateAndInitializeSid" => {
            let sia_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let sub_auth_count = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let sa0 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let sa1 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let sa2 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let sa3 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let sa4 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
            let sa5 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);
            let sa6 = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0);
            let sa7 = emu.maps.read_dword(emu.regs().get_esp() + 36).unwrap_or(0);
            let pp_sid = emu.maps.read_dword(emu.regs().get_esp() + 40).unwrap_or(0) as u64;

            // SID: revision(1) + sub_auth_count(1) + authority(6) + sub_auths(4*count)
            let count = std::cmp::min(sub_auth_count, 8) as usize;
            let sid_size = 8 + count * 4;
            if let Some(sid_addr) = emu.maps.alloc(sid_size as u64) {
                // revision=1, sub_auth_count
                emu.maps.write_byte(sid_addr, 1);
                emu.maps.write_byte(sid_addr + 1, count as u8);
                // Copy SIA (6 bytes)
                if sia_ptr != 0 {
                    for i in 0..6 {
                        let b = emu.maps.read_byte(sia_ptr + i).unwrap_or(0);
                        emu.maps.write_byte(sid_addr + 2 + i, b);
                    }
                }
                // Write sub-authorities
                let sas = [sa0, sa1, sa2, sa3, sa4, sa5, sa6, sa7];
                for (i, &sa) in sas.iter().enumerate().take(count) {
                    emu.maps.write_dword(sid_addr + 8 + (i as u64) * 4, sa);
                }
                if pp_sid != 0 {
                    emu.maps.write_dword(pp_sid, sid_addr as u32);
                }
                log_red!(
                    emu,
                    "advapi32!AllocateAndInitializeSid count:{} => 0x{:x}",
                    count,
                    sid_addr
                );
            } else {
                log_red!(emu, "advapi32!AllocateAndInitializeSid alloc failed");
            }
            for _ in 0..11 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AllocateLocallyUniqueId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AllocateLocallyUniqueId arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("AllocateLocallyUniqueId");
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
        "AreAllAccessesGranted" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AreAllAccessesGranted arg1:0x{:x} arg2:0x{:x}",
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
        "AreAnyAccessesGranted" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AreAnyAccessesGranted arg1:0x{:x} arg2:0x{:x}",
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
        "AuditComputeEffectivePolicyBySid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditComputeEffectivePolicyBySid arg1:0x{:x} arg2:0x{:x}",
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
        "AuditComputeEffectivePolicyByToken" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditComputeEffectivePolicyByToken h:0x{:x} arg2:0x{:x}",
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
        "AuditEnumerateCategories" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditEnumerateCategories arg1:0x{:x} arg2:0x{:x}",
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
        "AuditEnumeratePerUserPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditEnumeratePerUserPolicy arg1:0x{:x} arg2:0x{:x}",
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
        "AuditEnumerateSubCategories" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditEnumerateSubCategories arg1:0x{:x} arg2:0x{:x}",
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
        "AuditFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditFree arg1:0x{:x} arg2:0x{:x}",
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
        "AuditLookupCategoryGuidFromCategoryId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditLookupCategoryGuidFromCategoryId arg1:0x{:x} arg2:0x{:x}",
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
        "AuditLookupCategoryIdFromCategoryGuid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditLookupCategoryIdFromCategoryGuid arg1:0x{:x} arg2:0x{:x}",
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
        "AuditLookupCategoryNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AuditLookupCategoryNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AuditLookupCategoryNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AuditLookupCategoryNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AuditLookupSubCategoryNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AuditLookupSubCategoryNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AuditLookupSubCategoryNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AuditLookupSubCategoryNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AuditQueryGlobalSaclA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AuditQueryGlobalSaclA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AuditQueryGlobalSaclW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AuditQueryGlobalSaclW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AuditQueryPerUserPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditQueryPerUserPolicy arg1:0x{:x} arg2:0x{:x}",
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
        "AuditQuerySecurity" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditQuerySecurity arg1:0x{:x} arg2:0x{:x}",
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
        "AuditQuerySystemPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditQuerySystemPolicy arg1:0x{:x} arg2:0x{:x}",
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
        "AuditSetGlobalSaclA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AuditSetGlobalSaclA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AuditSetGlobalSaclW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!AuditSetGlobalSaclW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AuditSetPerUserPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditSetPerUserPolicy arg1:0x{:x} arg2:0x{:x}",
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
        "AuditSetSecurity" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditSetSecurity arg1:0x{:x} arg2:0x{:x}",
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
        "AuditSetSystemPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!AuditSetSystemPolicy arg1:0x{:x} arg2:0x{:x}",
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
        "BackupEventLogA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!BackupEventLogA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BackupEventLogW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!BackupEventLogW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BaseRegCloseKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegCloseKey h:0x{:x} arg2:0x{:x}",
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
        "BaseRegCreateKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegCreateKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("BaseRegCreateKey");
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
        "BaseRegDeleteKeyEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegDeleteKeyEx h:0x{:x} arg2:0x{:x}",
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
        "BaseRegDeleteValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegDeleteValue arg1:0x{:x} arg2:0x{:x}",
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
        "BaseRegFlushKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegFlushKey h:0x{:x} arg2:0x{:x}",
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
        "BaseRegGetVersion" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegGetVersion arg1:0x{:x} arg2:0x{:x}",
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
        "BaseRegLoadKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegLoadKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("BaseRegLoadKey");
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
        "BaseRegOpenKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegOpenKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("BaseRegOpenKey");
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
        "BaseRegRestoreKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegRestoreKey h:0x{:x} arg2:0x{:x}",
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
        "BaseRegSaveKeyEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegSaveKeyEx h:0x{:x} arg2:0x{:x}",
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
        "BaseRegSetKeySecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegSetKeySecurity h:0x{:x} arg2:0x{:x}",
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
        "BaseRegSetValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegSetValue arg1:0x{:x} arg2:0x{:x}",
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
        "BaseRegUnLoadKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!BaseRegUnLoadKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("BaseRegUnLoadKey");
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
        "BuildExplicitAccessWithNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildExplicitAccessWithNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildExplicitAccessWithNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildExplicitAccessWithNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildImpersonateExplicitAccessWithNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildImpersonateExplicitAccessWithNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildImpersonateExplicitAccessWithNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildImpersonateExplicitAccessWithNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildImpersonateTrusteeA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildImpersonateTrusteeA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildImpersonateTrusteeW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildImpersonateTrusteeW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildSecurityDescriptorA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildSecurityDescriptorA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildSecurityDescriptorW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildSecurityDescriptorW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildTrusteeWithNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildTrusteeWithNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildTrusteeWithNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildTrusteeWithNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildTrusteeWithObjectsAndNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildTrusteeWithObjectsAndNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildTrusteeWithObjectsAndNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildTrusteeWithObjectsAndNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildTrusteeWithObjectsAndSidA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildTrusteeWithObjectsAndSidA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildTrusteeWithObjectsAndSidW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildTrusteeWithObjectsAndSidW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildTrusteeWithSidA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildTrusteeWithSidA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildTrusteeWithSidW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!BuildTrusteeWithSidW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CancelOverlappedAccess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CancelOverlappedAccess h:0x{:x} arg2:0x{:x}",
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
        "ChangeServiceConfig2A" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ChangeServiceConfig2A '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ChangeServiceConfig2W" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ChangeServiceConfig2W '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ChangeServiceConfigA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ChangeServiceConfigA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..11 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ChangeServiceConfigW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ChangeServiceConfigW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..11 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CheckForHiberboot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CheckForHiberboot arg1:0x{:x} arg2:0x{:x}",
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
        "CheckTokenMembership" => {
            let token = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let sid_to_check = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let is_member_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if is_member_ptr != 0 {
                emu.maps.write_dword(is_member_ptr, 1); // TRUE - user is member
            }
            log_red!(
                emu,
                "advapi32!CheckTokenMembership token:0x{:x} sid:0x{:x} => TRUE",
                token,
                sid_to_check
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ClearEventLogA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!ClearEventLogA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ClearEventLogW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!ClearEventLogW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CloseCodeAuthzLevel" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "advapi32!CloseCodeAuthzLevel handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseEncryptedFileRaw" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "advapi32!CloseEncryptedFileRaw handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseEventLog" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "advapi32!CloseEventLog handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseServiceHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "advapi32!CloseServiceHandle 0x{:x}", handle);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CloseThreadWaitChainSession" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "advapi32!CloseThreadWaitChainSession handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseTrace" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "advapi32!CloseTrace handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CommandLineFromMsiDescriptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CommandLineFromMsiDescriptor arg1:0x{:x} arg2:0x{:x}",
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
        "ComputeAccessTokenFromCodeAuthzLevel" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!ComputeAccessTokenFromCodeAuthzLevel arg1:0x{:x} arg2:0x{:x}",
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
        "ControlService" => {
            let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let control = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let status_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if status_ptr != 0 {
                emu.maps.write_dword(status_ptr, 0x10);
                emu.maps.write_dword(status_ptr + 4, 1);
            }
            log_red!(
                emu,
                "advapi32!ControlService svc:0x{:x} ctrl:{}",
                hsvc,
                control
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ControlServiceExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!ControlServiceExA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ControlServiceExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!ControlServiceExW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ControlTraceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!ControlTraceA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ControlTraceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!ControlTraceW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertAccessToSecurityDescriptorA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertAccessToSecurityDescriptorA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertAccessToSecurityDescriptorW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertAccessToSecurityDescriptorW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSDToStringSDDomainW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertSDToStringSDDomainW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSDToStringSDRootDomainA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertSDToStringSDRootDomainA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSDToStringSDRootDomainW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertSDToStringSDRootDomainW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSecurityDescriptorToAccessA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertSecurityDescriptorToAccessA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSecurityDescriptorToAccessNamedA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertSecurityDescriptorToAccessNamedA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSecurityDescriptorToAccessNamedW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertSecurityDescriptorToAccessNamedW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSecurityDescriptorToAccessW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertSecurityDescriptorToAccessW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSecurityDescriptorToStringSecurityDescriptorA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertSecurityDescriptorToStringSecurityDescriptorA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSecurityDescriptorToStringSecurityDescriptorW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertSecurityDescriptorToStringSecurityDescriptorW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSidToStringSidA" => {
            let str_sid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let sid_str = "S-1-5-21-1234567890-1234567890-1234567890-1001";
            log_red!(emu, "advapi32!ConvertSidToStringSidA -> '{}'", sid_str);
            if str_sid_ptr != 0 {
                let alloc_size = (sid_str.len() + 1) as u64;
                if let Some(base) = emu.maps.alloc(alloc_size) {
                    let name = format!("sid_str_{:x}", base);
                    emu.maps.create_map(
                        &name,
                        base,
                        alloc_size,
                        crate::maps::mem64::Permission::READ_WRITE,
                    );
                    emu.maps.write_string(base, sid_str);
                    emu.maps.write_dword(str_sid_ptr, base as u32);
                }
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertSidToStringSidW" => {
            let str_sid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let sid_str = "S-1-5-21-1234567890-1234567890-1234567890-1001";
            log_red!(emu, "advapi32!ConvertSidToStringSidW -> '{}'", sid_str);
            if str_sid_ptr != 0 {
                let wlen: Vec<u16> = sid_str.encode_utf16().chain(std::iter::once(0)).collect();
                let alloc_size = (wlen.len() * 2) as u64;
                if let Some(base) = emu.maps.alloc(alloc_size) {
                    let name = format!("sid_strw_{:x}", base);
                    emu.maps.create_map(
                        &name,
                        base,
                        alloc_size,
                        crate::maps::mem64::Permission::READ_WRITE,
                    );
                    for (i, ch) in wlen.iter().enumerate() {
                        emu.maps.write_word(base + (i as u64) * 2, *ch);
                    }
                    emu.maps.write_dword(str_sid_ptr, base as u32);
                }
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertStringSDToSDDomainA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertStringSDToSDDomainA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertStringSDToSDDomainW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertStringSDToSDDomainW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertStringSDToSDRootDomainA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertStringSDToSDRootDomainA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertStringSDToSDRootDomainW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertStringSDToSDRootDomainW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertStringSecurityDescriptorToSecurityDescriptorA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertStringSecurityDescriptorToSecurityDescriptorA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertStringSecurityDescriptorToSecurityDescriptorW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ConvertStringSecurityDescriptorToSecurityDescriptorW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertStringSidToSidA" => {
            let str_sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pp_sid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let sid_str = if str_sid_ptr != 0 && str_sid_ptr > 0xffff {
                emu.maps.read_string(str_sid_ptr)
            } else {
                String::new()
            };
            // Allocate a mock SID (S-1-5-21-... -> 28 bytes)
            let sid_size = 28u64;
            if let Some(sid_addr) = emu.maps.alloc(sid_size) {
                // Revision=1, SubAuthorityCount=5
                emu.maps.write_byte(sid_addr, 1);
                emu.maps.write_byte(sid_addr + 1, 5);
                // NT Authority {0,0,0,0,0,5}
                emu.maps.write_byte(sid_addr + 7, 5);
                // Sub-authorities: 21, placeholder values
                emu.maps.write_dword(sid_addr + 8, 21);
                emu.maps.write_dword(sid_addr + 12, 1234567890);
                emu.maps.write_dword(sid_addr + 16, 987654321);
                emu.maps.write_dword(sid_addr + 20, 111111111);
                emu.maps.write_dword(sid_addr + 24, 500);
                if pp_sid != 0 {
                    emu.maps.write_dword(pp_sid, sid_addr as u32);
                }
            }
            log_red!(emu, "advapi32!ConvertStringSidToSidA '{}'", sid_str);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertStringSidToSidW" => {
            let str_sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pp_sid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let sid_str = if str_sid_ptr != 0 && str_sid_ptr > 0xffff {
                emu.maps.read_wide_string(str_sid_ptr)
            } else {
                String::new()
            };
            let sid_size = 28u64;
            if let Some(sid_addr) = emu.maps.alloc(sid_size) {
                emu.maps.write_byte(sid_addr, 1);
                emu.maps.write_byte(sid_addr + 1, 5);
                emu.maps.write_byte(sid_addr + 7, 5);
                emu.maps.write_dword(sid_addr + 8, 21);
                emu.maps.write_dword(sid_addr + 12, 1234567890);
                emu.maps.write_dword(sid_addr + 16, 987654321);
                emu.maps.write_dword(sid_addr + 20, 111111111);
                emu.maps.write_dword(sid_addr + 24, 500);
                if pp_sid != 0 {
                    emu.maps.write_dword(pp_sid, sid_addr as u32);
                }
            }
            log_red!(emu, "advapi32!ConvertStringSidToSidW '{}'", sid_str);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ConvertToAutoInheritPrivateObjectSecurity" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!ConvertToAutoInheritPrivateObjectSecurity arg1:0x{:x} arg2:0x{:x}",
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
            emu.regs_mut().rax = 0;
        }
        "CopySid" => {
            let dest_len = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dest_sid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let src_sid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if src_sid != 0 && dest_sid != 0 {
                let sub_count = emu.maps.read_byte(src_sid + 1).unwrap_or(1) as u64;
                let sid_len = 8 + sub_count * 4;
                let copy_len = std::cmp::min(sid_len, dest_len);
                for i in 0..copy_len {
                    let b = emu.maps.read_byte(src_sid + i).unwrap_or(0);
                    emu.maps.write_byte(dest_sid + i, b);
                }
            }
            log_red!(
                emu,
                "advapi32!CopySid dst:0x{:x} src:0x{:x} len:{}",
                dest_sid,
                src_sid,
                dest_len
            );
            let handle = helper::handler_create("CopySid");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateCodeAuthzLevel" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateCodeAuthzLevel");
            log_red!(emu, "advapi32!CreateCodeAuthzLevel => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreatePrivateObjectSecurity" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreatePrivateObjectSecurity");
            log_red!(emu, "advapi32!CreatePrivateObjectSecurity => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreatePrivateObjectSecurityEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreatePrivateObjectSecurityEx");
            log_red!(
                emu,
                "advapi32!CreatePrivateObjectSecurityEx => 0x{:x}",
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreatePrivateObjectSecurityWithMultipleInheritance" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CreatePrivateObjectSecurityWithMultipleInheritance h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle =
                helper::handler_create("CreatePrivateObjectSecurityWithMultipleInheritance");
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
        "CreateProcessAsUserA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CreateProcessAsUserA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..11 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateProcessAsUserW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CreateProcessAsUserW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..11 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateProcessWithLogonW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CreateProcessWithLogonW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..11 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateProcessWithTokenW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CreateProcessWithTokenW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateRestrictedToken" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateRestrictedToken");
            log_red!(emu, "advapi32!CreateRestrictedToken => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateServiceA" => {
            let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let display_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let access = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let name = emu.maps.read_string(name_ptr);
            let display = emu.maps.read_string(display_ptr);
            let hsvc = helper::handler_create(&name);
            log_red!(
                emu,
                "advapi32!CreateServiceA '{}' display:'{}' => 0x{:x}",
                name,
                display,
                hsvc
            );
            for _ in 0..13 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hsvc;
        }
        "CreateServiceEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateServiceEx");
            log_red!(emu, "advapi32!CreateServiceEx => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateServiceW" => {
            let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let display_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let access = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let name = emu.maps.read_wide_string(name_ptr);
            let display = emu.maps.read_wide_string(display_ptr);
            let hsvc = helper::handler_create(&name);
            log_red!(
                emu,
                "advapi32!CreateServiceW '{}' display:'{}' => 0x{:x}",
                name,
                display,
                hsvc
            );
            for _ in 0..13 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hsvc;
        }
        "CreateTraceInstanceId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateTraceInstanceId");
            log_red!(emu, "advapi32!CreateTraceInstanceId => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateWellKnownSid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateWellKnownSid");
            log_red!(emu, "advapi32!CreateWellKnownSid => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CredBackupCredentials" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredBackupCredentials arg1:0x{:x} arg2:0x{:x}",
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
        "CredDeleteA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredDeleteA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredDeleteW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredDeleteW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredEncryptAndMarshalBinaryBlob" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredEncryptAndMarshalBinaryBlob arg1:0x{:x} arg2:0x{:x}",
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
        "CredEnumerateA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredEnumerateA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredEnumerateW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredEnumerateW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredFindBestCredentialA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredFindBestCredentialA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredFindBestCredentialW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredFindBestCredentialW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!CredFree arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
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
        "CredGetSessionTypes" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredGetSessionTypes arg1:0x{:x} arg2:0x{:x}",
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
        "CredGetTargetInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredGetTargetInfoA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredGetTargetInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredGetTargetInfoW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredIsMarshaledCredentialA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredIsMarshaledCredentialA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredIsMarshaledCredentialW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredIsMarshaledCredentialW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredIsProtectedA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredIsProtectedA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredIsProtectedW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredIsProtectedW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredMarshalCredentialA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredMarshalCredentialA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredMarshalCredentialW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredMarshalCredentialW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredProfileLoaded" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredProfileLoaded arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CredProfileLoaded");
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
        "CredProfileLoadedEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredProfileLoadedEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CredProfileLoadedEx");
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
        "CredProfileUnloaded" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredProfileUnloaded arg1:0x{:x} arg2:0x{:x}",
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
        "CredProtectA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredProtectA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredProtectW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredProtectW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredReadA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredReadA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredReadByTokenHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredReadByTokenHandle h:0x{:x} arg2:0x{:x}",
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
        "CredReadDomainCredentialsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredReadDomainCredentialsA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredReadDomainCredentialsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredReadDomainCredentialsW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredReadW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredReadW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredRenameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredRenameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredRenameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredRenameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredRestoreCredentials" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredRestoreCredentials arg1:0x{:x} arg2:0x{:x}",
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
        "CredUnmarshalCredentialA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredUnmarshalCredentialA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredUnmarshalCredentialW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredUnmarshalCredentialW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredUnprotectA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredUnprotectA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredUnprotectW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredUnprotectW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredWriteA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!CredWriteA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredWriteDomainCredentialsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredWriteDomainCredentialsA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredWriteDomainCredentialsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!CredWriteDomainCredentialsW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredWriteW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!CredWriteW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CredpConvertCredential" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredpConvertCredential arg1:0x{:x} arg2:0x{:x}",
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
        "CredpConvertOneCredentialSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredpConvertOneCredentialSize arg1:0x{:x} arg2:0x{:x}",
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
        "CredpConvertTargetInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredpConvertTargetInfo arg1:0x{:x} arg2:0x{:x}",
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
        "CredpDecodeCredential" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredpDecodeCredential arg1:0x{:x} arg2:0x{:x}",
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
        "CredpEncodeCredential" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredpEncodeCredential arg1:0x{:x} arg2:0x{:x}",
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
        "CredpEncodeSecret" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CredpEncodeSecret arg1:0x{:x} arg2:0x{:x}",
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
        "CryptContextAddRef" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CryptContextAddRef arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CryptContextAddRef");
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
        "CryptDestroyHash" => {
            let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!CryptDestroyHash hash: 0x{:x}", hhash);
            crypto_state::destroy_hash(hhash);
            helper::handler_close(hhash);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptDestroyKey" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!CryptDestroyKey key: 0x{:x}", hkey);
            crypto_state::destroy_key(hkey);
            helper::handler_close(hkey);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptDuplicateHash" => {
            let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let ph_hash = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let new_handle = helper::handler_create("crypt_hash_dup");
            log_red!(
                emu,
                "advapi32!CryptDuplicateHash src:0x{:x} => 0x{:x}",
                hhash,
                new_handle
            );
            if ph_hash != 0 {
                emu.maps.write_dword(ph_hash, new_handle as u32);
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptDuplicateKey" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let ph_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let new_handle = helper::handler_create("crypt_key_dup");
            crypto_state::create_key(new_handle, 0x6801, 128);
            log_red!(
                emu,
                "advapi32!CryptDuplicateKey src:0x{:x} => 0x{:x}",
                hkey,
                new_handle
            );
            if ph_key != 0 {
                emu.maps.write_dword(ph_key, new_handle as u32);
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptEnumProviderTypesA" => {
            let index = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let prov_type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let name_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!CryptEnumProviderTypesA index:{}", index);
            if index == 0 {
                if prov_type_ptr != 0 {
                    emu.maps.write_dword(prov_type_ptr, 1);
                }
                let name = "RSA Full (Signature and Key Exchange)";
                if name_ptr != 0 {
                    emu.maps.write_string(name_ptr, name);
                }
                if name_len_ptr != 0 {
                    emu.maps.write_dword(name_len_ptr, name.len() as u32 + 1);
                }
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptEnumProviderTypesW" => {
            let index = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let prov_type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let name_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!CryptEnumProviderTypesW index:{}", index);
            if index == 0 {
                if prov_type_ptr != 0 {
                    emu.maps.write_dword(prov_type_ptr, 1);
                }
                let name = "RSA Full (Signature and Key Exchange)";
                if name_ptr != 0 {
                    emu.maps.write_wide_string(name_ptr, name);
                }
                if name_len_ptr != 0 {
                    emu.maps
                        .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
                }
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptEnumProvidersA" => {
            let index = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let prov_type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let name_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!CryptEnumProvidersA index:{}", index);
            if index == 0 {
                if prov_type_ptr != 0 {
                    emu.maps.write_dword(prov_type_ptr, 1);
                }
                let name = "Microsoft Base Cryptographic Provider v1.0";
                if name_ptr != 0 {
                    emu.maps.write_string(name_ptr, name);
                }
                if name_len_ptr != 0 {
                    emu.maps.write_dword(name_len_ptr, name.len() as u32 + 1);
                }
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptEnumProvidersW" => {
            let index = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let prov_type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let name_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!CryptEnumProvidersW index:{}", index);
            if index == 0 {
                if prov_type_ptr != 0 {
                    emu.maps.write_dword(prov_type_ptr, 1);
                }
                let name = "Microsoft Base Cryptographic Provider v1.0";
                if name_ptr != 0 {
                    emu.maps.write_wide_string(name_ptr, name);
                }
                if name_len_ptr != 0 {
                    emu.maps
                        .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
                }
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
        }
        "CryptExportKey" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _hexp_key = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let blob_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CryptExportKey key: 0x{:x} type: {}",
                hkey,
                blob_type
            );
            if data_ptr == 0 && data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, 256);
            } else if data_len_ptr != 0 {
                let len = emu.maps.read_dword(data_len_ptr).unwrap_or(256);
                let write_len = std::cmp::min(len, 256) as u64;
                for i in 0..write_len {
                    emu.maps.write_byte(data_ptr + i, 0xAA);
                }
                emu.maps.write_dword(data_len_ptr, write_len as u32);
            }
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CryptGetDefaultProviderA" => {
            let _prov_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let name = "Microsoft Base Cryptographic Provider v1.0";
            if name_ptr != 0 {
                emu.maps.write_string(name_ptr, name);
            }
            if name_len_ptr != 0 {
                emu.maps.write_dword(name_len_ptr, name.len() as u32 + 1);
            }
            log_red!(emu, "advapi32!CryptGetDefaultProviderA => '{}'", name);
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetDefaultProviderW" => {
            let _prov_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let name = "Microsoft Base Cryptographic Provider v1.0";
            if name_ptr != 0 {
                emu.maps.write_wide_string(name_ptr, name);
            }
            if name_len_ptr != 0 {
                emu.maps
                    .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
            }
            log_red!(emu, "advapi32!CryptGetDefaultProviderW => '{}'", name);
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetProvParam" => {
            let _hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            log_red!(emu, "advapi32!CryptGetProvParam param:{}", param);
            match param {
                4 => {
                    let name = "Microsoft Base Cryptographic Provider v1.0";
                    if data_ptr != 0 {
                        emu.maps.write_string(data_ptr, name);
                    }
                    if data_len_ptr != 0 {
                        emu.maps.write_dword(data_len_ptr, name.len() as u32 + 1);
                    }
                }
                6 => {
                    let name = "default";
                    if data_ptr != 0 {
                        emu.maps.write_string(data_ptr, name);
                    }
                    if data_len_ptr != 0 {
                        emu.maps.write_dword(data_len_ptr, name.len() as u32 + 1);
                    }
                }
                _ => {
                    if data_ptr != 0 {
                        emu.maps.write_dword(data_ptr, 0);
                    }
                    if data_len_ptr != 0 {
                        emu.maps.write_dword(data_len_ptr, 4);
                    }
                }
            }
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptGetUserKey" => {
            let _hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let ph_user_key = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let handle = helper::handler_create("crypt_user_key");
            crypto_state::create_key(handle, 0x6801, 128);
            log_red!(
                emu,
                "advapi32!CryptGetUserKey spec:{} => 0x{:x}",
                key_spec,
                handle
            );
            if ph_user_key != 0 {
                emu.maps.write_dword(ph_user_key, handle as u32);
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptHashSessionKey" => {
            let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hkey = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let placeholder = vec![0x41u8; 16];
            crypto_state::hash_data(hhash, &placeholder);
            log_red!(
                emu,
                "advapi32!CryptHashSessionKey hash:0x{:x} key:0x{:x}",
                hhash,
                hkey
            );
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetHashParam" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CryptSetHashParam p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetKeyParam" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CryptSetKeyParam p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetProvParam" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!CryptSetProvParam p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CryptSetProviderA" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let name = emu.maps.read_string(name_ptr);
            log_red!(
                emu,
                "advapi32!CryptSetProviderA '{}' type:{}",
                name,
                prov_type
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptSetProviderExA" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let name = emu.maps.read_string(name_ptr);
            log_red!(
                emu,
                "advapi32!CryptSetProviderExA '{}' type:{} flags:0x{:x}",
                name,
                prov_type,
                flags
            );
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptSetProviderExW" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(
                emu,
                "advapi32!CryptSetProviderExW '{}' type:{} flags:0x{:x}",
                name,
                prov_type,
                flags
            );
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptSetProviderW" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(
                emu,
                "advapi32!CryptSetProviderW '{}' type:{}",
                name,
                prov_type
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptVerifySignatureA" => {
            let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let sig_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let hpub_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let desc = if desc_ptr != 0 {
                emu.maps.read_string(desc_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "advapi32!CryptVerifySignatureA hash:0x{:x} sig:0x{:x} len:{} key:0x{:x} desc:'{}'",
                hhash,
                sig_ptr,
                sig_len,
                hpub_key,
                desc
            );
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CryptVerifySignatureW" => {
            let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let sig_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let hpub_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let desc = if desc_ptr != 0 {
                emu.maps.read_wide_string(desc_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "advapi32!CryptVerifySignatureW hash:0x{:x} sig:0x{:x} len:{} key:0x{:x} desc:'{}'",
                hhash,
                sig_ptr,
                sig_len,
                hpub_key,
                desc
            );
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CveEventWrite" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!CveEventWrite 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DecryptFileA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!DecryptFileA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DecryptFileW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!DecryptFileW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DeleteAce" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "advapi32!DeleteAce handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteService" => {
            let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(hsvc);
            log_red!(emu, "advapi32!DeleteService 0x{:x}", hsvc);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DeregisterEventSource" => {
            let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(hndl) {
                helper::handler_close(hndl);
            }
            log_red!(emu, "advapi32!DeregisterEventSource 0x{:x}", hndl);
            emu.stack_pop32(false);
            emu.regs_mut().rax = 1;
        }
        "DestroyPrivateObjectSecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "advapi32!DestroyPrivateObjectSecurity handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DuplicateEncryptionInfoFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!DuplicateEncryptionInfoFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("DuplicateEncryptionInfoFile");
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
        "DuplicateToken" => {
            let existing_token = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let imp_level = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let new_token_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let new_hndl = helper::handler_create("dup_token");
            if new_token_ptr != 0 {
                emu.maps.write_dword(new_token_ptr, new_hndl as u32);
            }
            log_red!(
                emu,
                "advapi32!DuplicateToken src:0x{:x} level:{} => 0x{:x}",
                existing_token,
                imp_level,
                new_hndl
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DuplicateTokenEx" => {
            let existing_token = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let desired_access = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _lp_attrs = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let imp_level = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let token_type = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let new_token_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let new_hndl = helper::handler_create("dup_token_ex");
            if new_token_ptr != 0 {
                emu.maps.write_dword(new_token_ptr, new_hndl as u32);
            }
            log_red!(
                emu,
                "advapi32!DuplicateTokenEx src:0x{:x} access:0x{:x} level:{} type:{} => 0x{:x}",
                existing_token,
                desired_access,
                imp_level,
                token_type,
                new_hndl
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnableTrace" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EnableTrace arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnableTrace");
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
        "EnableTraceEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EnableTraceEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnableTraceEx");
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
        "EnableTraceEx2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EnableTraceEx2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnableTraceEx2");
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
        "EncryptFileA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!EncryptFileA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EncryptFileW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!EncryptFileW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EncryptedFileKeyInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EncryptedFileKeyInfo h:0x{:x} arg2:0x{:x}",
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
        "EncryptionDisable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EncryptionDisable arg1:0x{:x} arg2:0x{:x}",
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
        "EnumDependentServicesA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!EnumDependentServicesA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDependentServicesW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!EnumDependentServicesW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDynamicTimeZoneInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EnumDynamicTimeZoneInformation arg1:0x{:x} arg2:0x{:x}",
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
        "EnumServiceGroupW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!EnumServiceGroupW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumServicesStatusA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!EnumServicesStatusA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumServicesStatusExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!EnumServicesStatusExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..10 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumServicesStatusExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!EnumServicesStatusExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..10 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumServicesStatusW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!EnumServicesStatusW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumerateTraceGuids" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EnumerateTraceGuids arg1:0x{:x} arg2:0x{:x}",
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
        "EnumerateTraceGuidsEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EnumerateTraceGuidsEx arg1:0x{:x} arg2:0x{:x}",
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
        "EqualDomainSid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EqualDomainSid arg1:0x{:x} arg2:0x{:x}",
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
        "EqualPrefixSid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!EqualPrefixSid arg1:0x{:x} arg2:0x{:x}",
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
        "EqualSid" => {
            let sid1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let sid2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let equal = if sid1 != 0 && sid2 != 0 {
                let count1 = emu.maps.read_byte(sid1 + 1).unwrap_or(0) as u64;
                let count2 = emu.maps.read_byte(sid2 + 1).unwrap_or(0) as u64;
                if count1 == count2 {
                    let len = 8 + count1 * 4;
                    let mut eq = true;
                    for i in 0..len {
                        let b1 = emu.maps.read_byte(sid1 + i).unwrap_or(0);
                        let b2 = emu.maps.read_byte(sid2 + i).unwrap_or(0);
                        if b1 != b2 {
                            eq = false;
                            break;
                        }
                    }
                    eq
                } else {
                    false
                }
            } else {
                false
            };
            log_red!(
                emu,
                "advapi32!EqualSid 0x{:x} 0x{:x} => {}",
                sid1,
                sid2,
                equal as u32
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
            emu.set_last_error(0);
            emu.regs_mut().rax = equal as u64;
        }
        "EventAccessControl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventAccessControl 0x{:x}", arg1);
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
        "EventAccessQuery" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventAccessQuery 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventAccessRemove" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventAccessRemove 0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventActivityIdControl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventActivityIdControl 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventEnabled 0x{:x}", arg1);
            let handle = helper::handler_create("EventEnabled");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventProviderEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventProviderEnabled 0x{:x}", arg1);
            let handle = helper::handler_create("EventProviderEnabled");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventRegister" => {
            let provider_id = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let enable_callback = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let callback_ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let reg_handle = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!EventRegister prov: 0x{:x} cb: 0x{:x} ctx: 0x{:x} out: 0x{:x}",
                provider_id,
                enable_callback,
                callback_ctx,
                reg_handle
            );
            if reg_handle != 0 {
                let h = helper::handler_create("event_provider");
                emu.maps.write_dword(reg_handle as u64, h as u32);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "EventSetInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventSetInformation 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventUnregister" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventUnregister 0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventWrite" => {
            let reg_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let event_desc = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let user_data_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let user_data = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let _activity_id = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!EventWrite handle: 0x{:x} desc: 0x{:x} count: {} data: 0x{:x}",
                reg_handle,
                event_desc,
                user_data_count,
                user_data
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
        }
        "EventWriteEndScenario" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventWriteEndScenario 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventWriteEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventWriteEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventWriteStartScenario" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventWriteStartScenario 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventWriteString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!EventWriteString 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EventWriteTransfer" => {
            let reg_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let event_desc = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let activity_id = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let related_id = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let user_data_count = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let user_data = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!EventWriteTransfer handle: 0x{:x} desc: 0x{:x} activity: 0x{:x} count: {}",
                reg_handle,
                event_desc,
                activity_id,
                user_data_count
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
        }
        "FileEncryptionStatusA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!FileEncryptionStatusA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FileEncryptionStatusW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!FileEncryptionStatusW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FindFirstFreeAce" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!FindFirstFreeAce arg1:0x{:x} arg2:0x{:x}",
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
        "FlushEfsCache" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!FlushEfsCache h:0x{:x} arg2:0x{:x}",
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
        "FlushTraceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!FlushTraceA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FlushTraceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!FlushTraceW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FreeEncryptedFileKeyInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "advapi32!FreeEncryptedFileKeyInfo handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FreeEncryptedFileMetadata" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "advapi32!FreeEncryptedFileMetadata handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FreeEncryptionCertificateHashList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "advapi32!FreeEncryptionCertificateHashList handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FreeInheritedFromArray" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "advapi32!FreeInheritedFromArray handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FreeSid" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "advapi32!FreeSid handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetAccessPermissionsForObjectA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetAccessPermissionsForObjectA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetAccessPermissionsForObjectW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetAccessPermissionsForObjectW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetAce" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!GetAce p1:0x{:x} p2:0x{:x}", p1, p2);
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAclInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetAclInformation h:0x{:x} arg2:0x{:x}",
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
        "GetAuditedPermissionsFromAclA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetAuditedPermissionsFromAclA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetAuditedPermissionsFromAclW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetAuditedPermissionsFromAclW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetCurrentHwProfileA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetCurrentHwProfileA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetCurrentHwProfileW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetCurrentHwProfileW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetDynamicTimeZoneInformationEffectiveYears" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetDynamicTimeZoneInformationEffectiveYears h:0x{:x} arg2:0x{:x}",
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
        "GetEffectiveRightsFromAclA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetEffectiveRightsFromAclA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetEffectiveRightsFromAclW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetEffectiveRightsFromAclW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetEncryptedFileMetadata" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetEncryptedFileMetadata h:0x{:x} arg2:0x{:x}",
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
        "GetEventLogInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!GetEventLogInformation 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetExplicitEntriesFromAclA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetExplicitEntriesFromAclA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetExplicitEntriesFromAclW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetExplicitEntriesFromAclW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetFileSecurityA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetFileSecurityA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetFileSecurityW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetFileSecurityW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetInformationCodeAuthzLevelW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetInformationCodeAuthzLevelW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetInformationCodeAuthzPolicyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetInformationCodeAuthzPolicyW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetInheritanceSourceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetInheritanceSourceA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetInheritanceSourceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetInheritanceSourceW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetKernelObjectSecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetKernelObjectSecurity h:0x{:x} arg2:0x{:x}",
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
        "GetLengthSid" => {
            let sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let sub_count = if sid_ptr != 0 {
                emu.maps.read_byte(sid_ptr + 1).unwrap_or(1) as u64
            } else {
                1
            };
            let len = 8 + sub_count * 4;
            log_red!(emu, "advapi32!GetLengthSid 0x{:x} => {}", sid_ptr, len);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = len;
        }
        "GetLocalManagedApplicationData" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetLocalManagedApplicationData h:0x{:x} arg2:0x{:x}",
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
        "GetLocalManagedApplications" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetLocalManagedApplications h:0x{:x} arg2:0x{:x}",
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
        "GetManagedApplicationCategories" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetManagedApplicationCategories h:0x{:x} arg2:0x{:x}",
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
        "GetManagedApplications" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetManagedApplications h:0x{:x} arg2:0x{:x}",
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
        "GetMultipleTrusteeA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetMultipleTrusteeA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMultipleTrusteeOperationA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetMultipleTrusteeOperationA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMultipleTrusteeOperationW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetMultipleTrusteeOperationW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMultipleTrusteeW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetMultipleTrusteeW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetNamedSecurityInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetNamedSecurityInfoA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetNamedSecurityInfoExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetNamedSecurityInfoExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetNamedSecurityInfoExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetNamedSecurityInfoExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetNamedSecurityInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetNamedSecurityInfoW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetNumberOfEventLogRecords" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!GetNumberOfEventLogRecords 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetOldestEventLogRecord" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!GetOldestEventLogRecord 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetOverlappedAccessResults" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetOverlappedAccessResults h:0x{:x} arg2:0x{:x}",
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
        "GetPrivateObjectSecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetPrivateObjectSecurity h:0x{:x} arg2:0x{:x}",
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
        "GetSecurityDescriptorControl" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSecurityDescriptorControl h:0x{:x} arg2:0x{:x}",
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
        "GetSecurityDescriptorDacl" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSecurityDescriptorDacl p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSecurityDescriptorGroup" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSecurityDescriptorGroup h:0x{:x} arg2:0x{:x}",
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
        "GetSecurityDescriptorLength" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSecurityDescriptorLength h:0x{:x} arg2:0x{:x}",
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
        "GetSecurityDescriptorOwner" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSecurityDescriptorOwner h:0x{:x} arg2:0x{:x}",
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
        "GetSecurityDescriptorRMControl" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSecurityDescriptorRMControl h:0x{:x} arg2:0x{:x}",
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
        "GetSecurityDescriptorSacl" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSecurityDescriptorSacl h:0x{:x} arg2:0x{:x}",
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
        "GetSecurityInfo" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSecurityInfo p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSecurityInfoExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetSecurityInfoExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetSecurityInfoExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetSecurityInfoExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetServiceDisplayNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetServiceDisplayNameA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetServiceDisplayNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetServiceDisplayNameW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetServiceKeyNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetServiceKeyNameA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetServiceKeyNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetServiceKeyNameW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetSidIdentifierAuthority" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSidIdentifierAuthority h:0x{:x} arg2:0x{:x}",
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
        "GetSidLengthRequired" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetSidLengthRequired h:0x{:x} arg2:0x{:x}",
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
        "GetSidSubAuthority" => {
            let sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            // Returns pointer to the sub-authority DWORD at offset 8 + index*4
            let result = sid_ptr + 8 + index * 4;
            log_red!(
                emu,
                "advapi32!GetSidSubAuthority sid:0x{:x} idx:{} => 0x{:x}",
                sid_ptr,
                index,
                result
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "GetSidSubAuthorityCount" => {
            let sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            // Returns pointer to the sub-authority count byte at offset 1
            let result = sid_ptr + 1;
            log_red!(
                emu,
                "advapi32!GetSidSubAuthorityCount sid:0x{:x} => 0x{:x}",
                sid_ptr,
                result
            );
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "GetStringConditionFromBinary" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetStringConditionFromBinary h:0x{:x} arg2:0x{:x}",
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
        "GetThreadWaitChain" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!GetThreadWaitChain 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetTokenInformation" => {
            let token_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let info_class = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let info_buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let ret_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if ret_len_ptr != 0 {
                emu.maps.write_dword(ret_len_ptr, 4);
            }
            if info_buf != 0 && buf_len >= 4 {
                emu.maps.write_dword(info_buf, 1);
            }
            log_red!(
                emu,
                "advapi32!GetTokenInformation token:0x{:x} class:{}",
                token_handle,
                info_class
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTraceEnableFlags" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetTraceEnableFlags h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("GetTraceEnableFlags");
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
        "GetTraceEnableLevel" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetTraceEnableLevel h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("GetTraceEnableLevel");
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
        "GetTraceLoggerHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetTraceLoggerHandle h:0x{:x} arg2:0x{:x}",
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
        "GetTrusteeFormA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetTrusteeFormA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTrusteeFormW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetTrusteeFormW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTrusteeNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetTrusteeNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTrusteeNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetTrusteeNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTrusteeTypeA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetTrusteeTypeA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTrusteeTypeW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!GetTrusteeTypeW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetWindowsAccountDomainSid" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!GetWindowsAccountDomainSid h:0x{:x} arg2:0x{:x}",
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
        "I_QueryTagInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!I_QueryTagInformation arg1:0x{:x} arg2:0x{:x}",
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
        "I_ScGetCurrentGroupStateW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!I_ScGetCurrentGroupStateW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "I_ScIsSecurityProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!I_ScIsSecurityProcess h:0x{:x} arg2:0x{:x}",
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
        "I_ScPnPGetServiceName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!I_ScPnPGetServiceName arg1:0x{:x} arg2:0x{:x}",
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
        "I_ScQueryServiceConfig" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!I_ScQueryServiceConfig arg1:0x{:x} arg2:0x{:x}",
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
        "I_ScRegisterPreshutdownRestart" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!I_ScRegisterPreshutdownRestart arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("I_ScRegisterPreshutdownRestart");
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
        "I_ScReparseServiceDatabase" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!I_ScReparseServiceDatabase arg1:0x{:x} arg2:0x{:x}",
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
        "I_ScSendPnPMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!I_ScSendPnPMessage arg1:0x{:x} arg2:0x{:x}",
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
        "I_ScSendTSMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!I_ScSendTSMessage arg1:0x{:x} arg2:0x{:x}",
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
        "I_ScSetServiceBitsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!I_ScSetServiceBitsA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "I_ScSetServiceBitsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!I_ScSetServiceBitsW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "I_ScValidatePnPService" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!I_ScValidatePnPService arg1:0x{:x} arg2:0x{:x}",
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
        "IdentifyCodeAuthzLevelW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!IdentifyCodeAuthzLevelW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ImpersonateAnonymousToken" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!ImpersonateAnonymousToken h:0x{:x} arg2:0x{:x}",
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
        "ImpersonateLoggedOnUser" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!ImpersonateLoggedOnUser");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ImpersonateNamedPipeClient" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!ImpersonateNamedPipeClient h:0x{:x} arg2:0x{:x}",
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
        "ImpersonateSelf" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!ImpersonateSelf arg1:0x{:x} arg2:0x{:x}",
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
        "InitializeAcl" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!InitializeAcl p1:0x{:x} p2:0x{:x}", p1, p2);
            let handle = helper::handler_create("InitializeAcl");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeSecurityDescriptor" => {
            let sd_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let revision = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            if sd_ptr != 0 {
                emu.maps.write_byte(sd_ptr, revision as u8);
                emu.maps.write_byte(sd_ptr + 1, 0); // Sbz1
                emu.maps.write_word(sd_ptr + 2, 0); // Control
                emu.maps.write_dword(sd_ptr + 4, 0); // Owner
                emu.maps.write_dword(sd_ptr + 8, 0); // Group
                emu.maps.write_dword(sd_ptr + 12, 0); // Sacl
                emu.maps.write_dword(sd_ptr + 16, 0); // Dacl
            }
            log_red!(
                emu,
                "advapi32!InitializeSecurityDescriptor sd:0x{:x} rev:{}",
                sd_ptr,
                revision
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeSid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!InitializeSid arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("InitializeSid");
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
        "InitiateShutdownA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!InitiateShutdownA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InitiateShutdownW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!InitiateShutdownW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InitiateSystemShutdownA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!InitiateSystemShutdownA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InitiateSystemShutdownExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!InitiateSystemShutdownExA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InitiateSystemShutdownExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!InitiateSystemShutdownExW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InitiateSystemShutdownW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!InitiateSystemShutdownW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InstallApplication" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!InstallApplication arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("InstallApplication");
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
        "IsTextUnicode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!IsTextUnicode arg1:0x{:x} arg2:0x{:x}",
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
        "IsTokenRestricted" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!IsTokenRestricted h:0x{:x} arg2:0x{:x}",
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
        "IsTokenUntrusted" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!IsTokenUntrusted h:0x{:x} arg2:0x{:x}",
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
        "IsValidAcl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!IsValidAcl arg1:0x{:x} arg2:0x{:x}",
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
        "IsValidRelativeSecurityDescriptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!IsValidRelativeSecurityDescriptor arg1:0x{:x} arg2:0x{:x}",
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
        "IsValidSecurityDescriptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!IsValidSecurityDescriptor arg1:0x{:x} arg2:0x{:x}",
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
        "IsValidSid" => {
            let sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let valid = if sid_ptr != 0 {
                let revision = emu.maps.read_byte(sid_ptr).unwrap_or(0);
                let sub_count = emu.maps.read_byte(sid_ptr + 1).unwrap_or(0);
                revision == 1 && sub_count <= 15
            } else {
                false
            };
            log_red!(
                emu,
                "advapi32!IsValidSid 0x{:x} => {}",
                sid_ptr,
                valid as u32
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = valid as u64;
        }
        "IsWellKnownSid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!IsWellKnownSid arg1:0x{:x} arg2:0x{:x}",
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
        "LockServiceDatabase" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!LockServiceDatabase 0x{:x}", arg1);
            let handle = helper::handler_create("LockServiceDatabase");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LogonSecondaryUserIntoSessionW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!LogonSecondaryUserIntoSessionW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LogonUserA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!LogonUserA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LogonUserExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!LogonUserExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LogonUserExExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!LogonUserExExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LogonUserExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!LogonUserExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LogonUserW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!LogonUserW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LookupAccountNameA" => {
            let _system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let sid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let sid_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let domain_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let domain_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let use_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            // Write a mock SID (28 bytes = S-1-5-21-...)
            if sid_ptr != 0 {
                emu.maps.write_byte(sid_ptr, 1); // revision
                emu.maps.write_byte(sid_ptr + 1, 5); // sub-auth count
                emu.maps.write_byte(sid_ptr + 7, 5); // NT Authority
                emu.maps.write_dword(sid_ptr + 8, 21);
                emu.maps.write_dword(sid_ptr + 12, 1234567890);
                emu.maps.write_dword(sid_ptr + 16, 987654321);
                emu.maps.write_dword(sid_ptr + 20, 111111111);
                emu.maps.write_dword(sid_ptr + 24, 500);
            }
            if sid_sz_ptr != 0 {
                emu.maps.write_dword(sid_sz_ptr, 28);
            }
            if domain_ptr != 0 {
                emu.maps.write_string(domain_ptr, "DESKTOP");
            }
            if domain_sz_ptr != 0 {
                emu.maps.write_dword(domain_sz_ptr, 7);
            }
            if use_ptr != 0 {
                emu.maps.write_dword(use_ptr, 1); // SidTypeUser
            }
            log_red!(emu, "advapi32!LookupAccountNameA '{}'", name);
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LookupAccountNameW" => {
            let _system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let sid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let sid_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let domain_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let domain_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let use_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            if sid_ptr != 0 {
                emu.maps.write_byte(sid_ptr, 1);
                emu.maps.write_byte(sid_ptr + 1, 5);
                emu.maps.write_byte(sid_ptr + 7, 5);
                emu.maps.write_dword(sid_ptr + 8, 21);
                emu.maps.write_dword(sid_ptr + 12, 1234567890);
                emu.maps.write_dword(sid_ptr + 16, 987654321);
                emu.maps.write_dword(sid_ptr + 20, 111111111);
                emu.maps.write_dword(sid_ptr + 24, 500);
            }
            if sid_sz_ptr != 0 {
                emu.maps.write_dword(sid_sz_ptr, 28);
            }
            if domain_ptr != 0 {
                emu.maps.write_wide_string(domain_ptr, "DESKTOP");
            }
            if domain_sz_ptr != 0 {
                emu.maps.write_dword(domain_sz_ptr, 7);
            }
            if use_ptr != 0 {
                emu.maps.write_dword(use_ptr, 1); // SidTypeUser
            }
            log_red!(emu, "advapi32!LookupAccountNameW '{}'", name);
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LookupAccountSidA" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let name_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let domain_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let domain_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let use_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!LookupAccountSidA -> user/DESKTOP");
            let user = "user";
            let domain = "DESKTOP";
            if name_ptr != 0 {
                emu.maps.write_string(name_ptr, user);
            }
            if name_sz_ptr != 0 {
                emu.maps.write_dword(name_sz_ptr, (user.len() + 1) as u32);
            }
            if domain_ptr != 0 {
                emu.maps.write_string(domain_ptr, domain);
            }
            if domain_sz_ptr != 0 {
                emu.maps
                    .write_dword(domain_sz_ptr, (domain.len() + 1) as u32);
            }
            if use_ptr != 0 {
                emu.maps.write_dword(use_ptr, 1); // SidTypeUser
            }
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LookupAccountSidW" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let name_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let domain_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let domain_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let use_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!LookupAccountSidW -> user/DESKTOP");
            let user = "user";
            let domain = "DESKTOP";
            if name_ptr != 0 {
                for (i, ch) in user.encode_utf16().enumerate() {
                    emu.maps.write_word(name_ptr + (i as u64) * 2, ch);
                }
                emu.maps.write_word(name_ptr + (user.len() as u64) * 2, 0);
            }
            if name_sz_ptr != 0 {
                emu.maps.write_dword(name_sz_ptr, (user.len() + 1) as u32);
            }
            if domain_ptr != 0 {
                for (i, ch) in domain.encode_utf16().enumerate() {
                    emu.maps.write_word(domain_ptr + (i as u64) * 2, ch);
                }
                emu.maps
                    .write_word(domain_ptr + (domain.len() as u64) * 2, 0);
            }
            if domain_sz_ptr != 0 {
                emu.maps
                    .write_dword(domain_sz_ptr, (domain.len() + 1) as u32);
            }
            if use_ptr != 0 {
                emu.maps.write_dword(use_ptr, 1);
            }
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LookupPrivilegeDisplayNameA" => {
            let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            log_red!(emu, "advapi32!LookupPrivilegeDisplayNameA '{}'", name);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "LookupPrivilegeDisplayNameW" => {
            let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(emu, "advapi32!LookupPrivilegeDisplayNameW '{}'", name);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "LookupPrivilegeNameA" => {
            let _system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let luid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name_buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let size_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let priv_name = "SeDebugPrivilege";
            if name_buf != 0 {
                emu.maps.write_string(name_buf, priv_name);
            }
            if size_ptr != 0 {
                emu.maps.write_dword(size_ptr, priv_name.len() as u32);
            }
            log_red!(
                emu,
                "advapi32!LookupPrivilegeNameA luid:0x{:x} => '{}'",
                luid_ptr,
                priv_name
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LookupPrivilegeNameW" => {
            let _system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let luid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name_buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let size_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let priv_name = "SeDebugPrivilege";
            if name_buf != 0 {
                emu.maps.write_wide_string(name_buf, priv_name);
            }
            if size_ptr != 0 {
                emu.maps.write_dword(size_ptr, priv_name.len() as u32);
            }
            log_red!(
                emu,
                "advapi32!LookupPrivilegeNameW luid:0x{:x} => '{}'",
                luid_ptr,
                priv_name
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LookupPrivilegeValueA" => {
            let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let luid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            if luid_ptr != 0 {
                emu.maps.write_dword(luid_ptr, 20);
                emu.maps.write_dword(luid_ptr + 4, 0);
            }
            log_red!(emu, "advapi32!LookupPrivilegeValueA name:'{}'", name);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LookupSecurityDescriptorPartsA" => {
            let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            log_red!(emu, "advapi32!LookupSecurityDescriptorPartsA '{}'", name);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "LookupSecurityDescriptorPartsW" => {
            let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(emu, "advapi32!LookupSecurityDescriptorPartsW '{}'", name);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "LsaAddAccountRights" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaAddAccountRights arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaAddAccountRights");
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
        "LsaAddPrivilegesToAccount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaAddPrivilegesToAccount arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaAddPrivilegesToAccount");
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
        "LsaClearAuditLog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaClearAuditLog arg1:0x{:x} arg2:0x{:x}",
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
        "LsaClose" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!LsaClose arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
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
        "LsaConfigureAutoLogonCredentials" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaConfigureAutoLogonCredentials arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaConfigureAutoLogonCredentials");
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
        "LsaCreateAccount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaCreateAccount arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaCreateAccount");
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
        "LsaCreateSecret" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaCreateSecret arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaCreateSecret");
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
        "LsaCreateTrustedDomain" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaCreateTrustedDomain arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaCreateTrustedDomain");
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
        "LsaCreateTrustedDomainEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaCreateTrustedDomainEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaCreateTrustedDomainEx");
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
        "LsaDelete" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaDelete arg1:0x{:x} arg2:0x{:x}",
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
        "LsaDeleteTrustedDomain" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaDeleteTrustedDomain arg1:0x{:x} arg2:0x{:x}",
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
        "LsaDisableUserArso" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaDisableUserArso arg1:0x{:x} arg2:0x{:x}",
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
        "LsaEnableUserArso" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaEnableUserArso arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaEnableUserArso");
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
        "LsaEnumerateAccountRights" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaEnumerateAccountRights arg1:0x{:x} arg2:0x{:x}",
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
        "LsaEnumerateAccounts" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaEnumerateAccounts arg1:0x{:x} arg2:0x{:x}",
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
        "LsaEnumerateAccountsWithUserRight" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaEnumerateAccountsWithUserRight arg1:0x{:x} arg2:0x{:x}",
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
        "LsaEnumeratePrivileges" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaEnumeratePrivileges arg1:0x{:x} arg2:0x{:x}",
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
        "LsaEnumeratePrivilegesOfAccount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaEnumeratePrivilegesOfAccount arg1:0x{:x} arg2:0x{:x}",
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
        "LsaEnumerateTrustedDomains" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaEnumerateTrustedDomains arg1:0x{:x} arg2:0x{:x}",
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
        "LsaEnumerateTrustedDomainsEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaEnumerateTrustedDomainsEx arg1:0x{:x} arg2:0x{:x}",
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
        "LsaFreeMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaFreeMemory arg1:0x{:x} arg2:0x{:x}",
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
        "LsaGetAppliedCAPIDs" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaGetAppliedCAPIDs arg1:0x{:x} arg2:0x{:x}",
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
        "LsaGetDeviceRegistrationInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaGetDeviceRegistrationInfo arg1:0x{:x} arg2:0x{:x}",
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
        "LsaGetQuotasForAccount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaGetQuotasForAccount arg1:0x{:x} arg2:0x{:x}",
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
        "LsaGetRemoteUserName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaGetRemoteUserName arg1:0x{:x} arg2:0x{:x}",
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
        "LsaGetSystemAccessAccount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaGetSystemAccessAccount arg1:0x{:x} arg2:0x{:x}",
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
        "LsaGetUserName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaGetUserName arg1:0x{:x} arg2:0x{:x}",
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
        "LsaICLookupNames" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaICLookupNames arg1:0x{:x} arg2:0x{:x}",
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
        "LsaICLookupNamesWithCreds" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaICLookupNamesWithCreds arg1:0x{:x} arg2:0x{:x}",
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
        "LsaICLookupSids" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaICLookupSids arg1:0x{:x} arg2:0x{:x}",
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
        "LsaICLookupSidsWithCreds" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaICLookupSidsWithCreds arg1:0x{:x} arg2:0x{:x}",
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
        "LsaInvokeTrustScanner" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaInvokeTrustScanner arg1:0x{:x} arg2:0x{:x}",
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
        "LsaIsUserArsoAllowed" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaIsUserArsoAllowed arg1:0x{:x} arg2:0x{:x}",
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
        "LsaIsUserArsoEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaIsUserArsoEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaIsUserArsoEnabled");
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
        "LsaLookupNames" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaLookupNames arg1:0x{:x} arg2:0x{:x}",
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
        "LsaLookupNames2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaLookupNames2 arg1:0x{:x} arg2:0x{:x}",
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
        "LsaLookupPrivilegeDisplayName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaLookupPrivilegeDisplayName arg1:0x{:x} arg2:0x{:x}",
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
        "LsaLookupPrivilegeName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaLookupPrivilegeName arg1:0x{:x} arg2:0x{:x}",
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
        "LsaLookupPrivilegeValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaLookupPrivilegeValue arg1:0x{:x} arg2:0x{:x}",
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
        "LsaLookupSids" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaLookupSids arg1:0x{:x} arg2:0x{:x}",
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
        "LsaLookupSids2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaLookupSids2 arg1:0x{:x} arg2:0x{:x}",
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
        "LsaManageSidNameMapping" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaManageSidNameMapping arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaManageSidNameMapping");
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
        "LsaNtStatusToWinError" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaNtStatusToWinError arg1:0x{:x} arg2:0x{:x}",
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
        "LsaOpenAccount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaOpenAccount arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaOpenAccount");
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
        "LsaOpenPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaOpenPolicy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaOpenPolicy");
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
        "LsaOpenPolicySce" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaOpenPolicySce arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaOpenPolicySce");
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
        "LsaOpenSecret" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaOpenSecret arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaOpenSecret");
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
        "LsaOpenTrustedDomain" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaOpenTrustedDomain arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaOpenTrustedDomain");
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
        "LsaOpenTrustedDomainByName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaOpenTrustedDomainByName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LsaOpenTrustedDomainByName");
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
        "LsaProfileDeleted" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaProfileDeleted arg1:0x{:x} arg2:0x{:x}",
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
        "LsaPurgeLocalSystemAccessTable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaPurgeLocalSystemAccessTable arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQueryCAPs" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryCAPs arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQueryDomainInformationPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryDomainInformationPolicy arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQueryForestTrustInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryForestTrustInformation arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQueryForestTrustInformation2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryForestTrustInformation2 arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQueryInfoTrustedDomain" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryInfoTrustedDomain arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQueryInformationPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryInformationPolicy arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQueryLocalSystemAccess" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryLocalSystemAccess arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQueryLocalSystemAccessAll" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryLocalSystemAccessAll arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQuerySecret" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQuerySecret arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQuerySecurityObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQuerySecurityObject h:0x{:x} arg2:0x{:x}",
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
        "LsaQueryTrustedDomainInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryTrustedDomainInfo arg1:0x{:x} arg2:0x{:x}",
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
        "LsaQueryTrustedDomainInfoByName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaQueryTrustedDomainInfoByName arg1:0x{:x} arg2:0x{:x}",
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
        "LsaRemoveAccountRights" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaRemoveAccountRights arg1:0x{:x} arg2:0x{:x}",
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
        "LsaRemovePrivilegesFromAccount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaRemovePrivilegesFromAccount arg1:0x{:x} arg2:0x{:x}",
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
        "LsaRetrievePrivateData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaRetrievePrivateData arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetCAPs" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetCAPs arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetDomainInformationPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetDomainInformationPolicy arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetForestTrustInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetForestTrustInformation arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetForestTrustInformation2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetForestTrustInformation2 arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetInformationPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetInformationPolicy arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetInformationTrustedDomain" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetInformationTrustedDomain arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetLocalSystemAccess" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetLocalSystemAccess arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetQuotasForAccount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetQuotasForAccount arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetSecret" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetSecret arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetSecurityObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetSecurityObject h:0x{:x} arg2:0x{:x}",
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
        "LsaSetSystemAccessAccount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetSystemAccessAccount arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetTrustedDomainInfoByName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetTrustedDomainInfoByName arg1:0x{:x} arg2:0x{:x}",
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
        "LsaSetTrustedDomainInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaSetTrustedDomainInformation arg1:0x{:x} arg2:0x{:x}",
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
        "LsaStorePrivateData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaStorePrivateData arg1:0x{:x} arg2:0x{:x}",
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
        "LsaValidateProcUniqueLuid" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!LsaValidateProcUniqueLuid arg1:0x{:x} arg2:0x{:x}",
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
        "MD4Final" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!MD4Final arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
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
        "MD4Init" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!MD4Init arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("MD4Init");
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
        "MD4Update" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!MD4Update arg1:0x{:x} arg2:0x{:x}",
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
        "MD5Final" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!MD5Final arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
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
        "MD5Init" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!MD5Init arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("MD5Init");
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
        "MD5Update" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!MD5Update arg1:0x{:x} arg2:0x{:x}",
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
        "MIDL_user_free_Ext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!MIDL_user_free_Ext arg1:0x{:x} arg2:0x{:x}",
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
        "MSChapSrvChangePassword" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!MSChapSrvChangePassword arg1:0x{:x} arg2:0x{:x}",
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
        "MSChapSrvChangePassword2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!MSChapSrvChangePassword2 arg1:0x{:x} arg2:0x{:x}",
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
        "MakeAbsoluteSD" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!MakeAbsoluteSD arg1:0x{:x} arg2:0x{:x}",
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
        "MakeAbsoluteSD2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!MakeAbsoluteSD2 arg1:0x{:x} arg2:0x{:x}",
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
        "MakeSelfRelativeSD" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!MakeSelfRelativeSD arg1:0x{:x} arg2:0x{:x}",
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
        "MapGenericMask" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!MapGenericMask arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("MapGenericMask");
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
        "NotifyBootConfigStatus" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!NotifyBootConfigStatus 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "NotifyChangeEventLog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!NotifyChangeEventLog 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "NotifyServiceStatusChange" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!NotifyServiceStatusChange 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "NotifyServiceStatusChangeA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!NotifyServiceStatusChangeA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "NotifyServiceStatusChangeW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!NotifyServiceStatusChangeW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "NpGetUserName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!NpGetUserName arg1:0x{:x} arg2:0x{:x}",
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
        "ObjectCloseAuditAlarmA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ObjectCloseAuditAlarmA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ObjectCloseAuditAlarmW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ObjectCloseAuditAlarmW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ObjectDeleteAuditAlarmA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ObjectDeleteAuditAlarmA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ObjectDeleteAuditAlarmW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ObjectDeleteAuditAlarmW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ObjectOpenAuditAlarmA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ObjectOpenAuditAlarmA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ObjectOpenAuditAlarmW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ObjectOpenAuditAlarmW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ObjectPrivilegeAuditAlarmA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ObjectPrivilegeAuditAlarmA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ObjectPrivilegeAuditAlarmW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ObjectPrivilegeAuditAlarmW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenBackupEventLogA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!OpenBackupEventLogA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenBackupEventLogW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!OpenBackupEventLogW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenEncryptedFileRawA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!OpenEncryptedFileRawA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenEncryptedFileRawW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!OpenEncryptedFileRawW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenEventLogA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!OpenEventLogA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("advapi32://OpenEventLogA");
            emu.regs_mut().rax = hndl;
        }
        "OpenEventLogW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!OpenEventLogW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("advapi32://OpenEventLogW");
            emu.regs_mut().rax = hndl;
        }
        "OpenProcessToken" => {
            let process_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let desired_access = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let token_handle_ptr =
                emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let token = helper::handler_create("token");
            if token_handle_ptr != 0 {
                emu.maps.write_dword(token_handle_ptr, token as u32);
            }
            log_red!(
                emu,
                "advapi32!OpenProcessToken proc:0x{:x} access:0x{:x} => 0x{:x}",
                process_handle,
                desired_access,
                token
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenSCManagerA" => {
            let machine_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let database_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let machine = if machine_ptr != 0 {
                emu.maps.read_string(machine_ptr)
            } else {
                String::new()
            };
            let hsc = helper::handler_create("scmanager");
            log_red!(
                emu,
                "advapi32!OpenSCManagerA machine:'{}' access:0x{:x} => 0x{:x}",
                machine,
                access,
                hsc
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hsc;
        }
        "OpenSCManagerW" => {
            let machine_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let database_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let machine = if machine_ptr != 0 {
                emu.maps.read_wide_string(machine_ptr)
            } else {
                String::new()
            };
            let hsc = helper::handler_create("scmanager");
            log_red!(
                emu,
                "advapi32!OpenSCManagerW machine:'{}' access:0x{:x} => 0x{:x}",
                machine,
                access,
                hsc
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hsc;
        }
        "OpenServiceA" => {
            let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let name = emu.maps.read_string(name_ptr);
            let hsvc = helper::handler_create(&name);
            log_red!(
                emu,
                "advapi32!OpenServiceA scm:0x{:x} '{}' access:0x{:x} => 0x{:x}",
                hsc,
                name,
                access,
                hsvc
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hsvc;
        }
        "OpenServiceW" => {
            let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let name = emu.maps.read_wide_string(name_ptr);
            let hsvc = helper::handler_create(&name);
            log_red!(
                emu,
                "advapi32!OpenServiceW scm:0x{:x} '{}' access:0x{:x} => 0x{:x}",
                hsc,
                name,
                access,
                hsvc
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hsvc;
        }
        "OpenThreadToken" => {
            let thread_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let desired_access = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let open_as_self = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) != 0;
            let token_handle_ptr =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let token = helper::handler_create("token");
            if token_handle_ptr != 0 {
                emu.maps.write_dword(token_handle_ptr, token as u32);
            }
            log_red!(
                emu,
                "advapi32!OpenThreadToken thread:0x{:x} access:0x{:x} => 0x{:x}",
                thread_handle,
                desired_access,
                token
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenThreadWaitChainSession" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("OpenThreadWaitChainSession");
            log_red!(emu, "advapi32!OpenThreadWaitChainSession => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "OpenTraceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!OpenTraceA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenTraceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!OpenTraceW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OperationEnd" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!OperationEnd arg1:0x{:x} arg2:0x{:x}",
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
        "OperationStart" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!OperationStart arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("OperationStart");
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
        "PerfAddCounters" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfAddCounters arg1:0x{:x} arg2:0x{:x}",
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
        "PerfCloseQueryHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfCloseQueryHandle h:0x{:x} arg2:0x{:x}",
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
        "PerfCreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfCreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("PerfCreateInstance");
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
        "PerfDecrementULongCounterValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfDecrementULongCounterValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfDecrementULongLongCounterValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfDecrementULongLongCounterValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfDeleteCounters" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfDeleteCounters arg1:0x{:x} arg2:0x{:x}",
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
        "PerfDeleteInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfDeleteInstance arg1:0x{:x} arg2:0x{:x}",
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
        "PerfEnumerateCounterSet" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfEnumerateCounterSet arg1:0x{:x} arg2:0x{:x}",
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
        "PerfEnumerateCounterSetInstances" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfEnumerateCounterSetInstances arg1:0x{:x} arg2:0x{:x}",
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
        "PerfIncrementULongCounterValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfIncrementULongCounterValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfIncrementULongLongCounterValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfIncrementULongLongCounterValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfOpenQueryHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfOpenQueryHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("PerfOpenQueryHandle");
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
        "PerfQueryCounterData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfQueryCounterData arg1:0x{:x} arg2:0x{:x}",
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
        "PerfQueryCounterInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfQueryCounterInfo arg1:0x{:x} arg2:0x{:x}",
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
        "PerfQueryCounterSetRegistrationInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfQueryCounterSetRegistrationInfo arg1:0x{:x} arg2:0x{:x}",
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
        "PerfQueryInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfQueryInstance arg1:0x{:x} arg2:0x{:x}",
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
        "PerfRegCloseKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfRegCloseKey h:0x{:x} arg2:0x{:x}",
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
        "PerfRegEnumKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfRegEnumKey h:0x{:x} arg2:0x{:x}",
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
        "PerfRegEnumValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfRegEnumValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfRegQueryInfoKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfRegQueryInfoKey h:0x{:x} arg2:0x{:x}",
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
        "PerfRegQueryValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfRegQueryValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfRegSetValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfRegSetValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfSetCounterRefValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfSetCounterRefValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfSetCounterSetInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfSetCounterSetInfo arg1:0x{:x} arg2:0x{:x}",
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
        "PerfSetULongCounterValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfSetULongCounterValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfSetULongLongCounterValue" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfSetULongLongCounterValue arg1:0x{:x} arg2:0x{:x}",
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
        "PerfStartProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfStartProvider arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("PerfStartProvider");
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
        "PerfStartProviderEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfStartProviderEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("PerfStartProviderEx");
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
        "PerfStopProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PerfStopProvider arg1:0x{:x} arg2:0x{:x}",
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
        "PrivilegeCheck" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!PrivilegeCheck arg1:0x{:x} arg2:0x{:x}",
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
        "PrivilegedServiceAuditAlarmA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!PrivilegedServiceAuditAlarmA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PrivilegedServiceAuditAlarmW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!PrivilegedServiceAuditAlarmW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ProcessIdleTasks" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!ProcessIdleTasks h:0x{:x} arg2:0x{:x}",
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
        "ProcessIdleTasksW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!ProcessIdleTasksW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ProcessTrace" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!ProcessTrace h:0x{:x} arg2:0x{:x}",
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
        "QueryAllTracesA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!QueryAllTracesA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryAllTracesW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!QueryAllTracesW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryLocalUserServiceName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!QueryLocalUserServiceName h:0x{:x} arg2:0x{:x}",
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
        "QueryRecoveryAgentsOnEncryptedFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!QueryRecoveryAgentsOnEncryptedFile h:0x{:x} arg2:0x{:x}",
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
        "QuerySecurityAccessMask" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!QuerySecurityAccessMask h:0x{:x} arg2:0x{:x}",
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
        "QueryServiceConfig2A" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!QueryServiceConfig2A '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryServiceConfig2W" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!QueryServiceConfig2W '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryServiceConfigA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!QueryServiceConfigA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryServiceConfigW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!QueryServiceConfigW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryServiceDynamicInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!QueryServiceDynamicInformation h:0x{:x} arg2:0x{:x}",
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
        "QueryServiceLockStatusA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!QueryServiceLockStatusA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryServiceLockStatusW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!QueryServiceLockStatusW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryServiceObjectSecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!QueryServiceObjectSecurity h:0x{:x} arg2:0x{:x}",
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
        "QueryServiceStatus" => {
            let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let status_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if status_ptr != 0 {
                emu.maps.write_dword(status_ptr, 0x10);
                emu.maps.write_dword(status_ptr + 4, 4);
                emu.maps.write_dword(status_ptr + 8, 0x1f);
                emu.maps.write_dword(status_ptr + 12, 0);
                emu.maps.write_dword(status_ptr + 16, 0);
                emu.maps.write_dword(status_ptr + 20, 0);
                emu.maps.write_dword(status_ptr + 24, 0);
            }
            log_red!(emu, "advapi32!QueryServiceStatus svc:0x{:x}", hsvc);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryServiceStatusEx" => {
            let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let info_level = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let bytes_needed_ptr =
                emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            // SERVICE_STATUS_PROCESS is 36 bytes
            if buf_ptr != 0 && buf_size >= 36 {
                emu.maps.write_dword(buf_ptr, 0x10); // dwServiceType = SERVICE_WIN32_OWN_PROCESS
                emu.maps.write_dword(buf_ptr + 4, 4); // dwCurrentState = SERVICE_RUNNING
                emu.maps.write_dword(buf_ptr + 8, 0x1f); // dwControlsAccepted
                emu.maps.write_dword(buf_ptr + 12, 0);
                emu.maps.write_dword(buf_ptr + 16, 0);
                emu.maps.write_dword(buf_ptr + 20, 0);
                emu.maps.write_dword(buf_ptr + 24, 0);
                emu.maps.write_dword(buf_ptr + 28, 1234); // dwProcessId
                emu.maps.write_dword(buf_ptr + 32, 0); // dwServiceFlags
            }
            if bytes_needed_ptr != 0 {
                emu.maps.write_dword(bytes_needed_ptr, 36);
            }
            log_red!(
                emu,
                "advapi32!QueryServiceStatusEx svc:0x{:x} level:{}",
                hsvc,
                info_level
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryTraceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!QueryTraceA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryTraceProcessingHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!QueryTraceProcessingHandle h:0x{:x} arg2:0x{:x}",
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
        "QueryTraceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!QueryTraceW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryUserServiceName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!QueryUserServiceName h:0x{:x} arg2:0x{:x}",
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
        "QueryUserServiceNameForContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!QueryUserServiceNameForContext h:0x{:x} arg2:0x{:x}",
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
        "QueryUsersOnEncryptedFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!QueryUsersOnEncryptedFile h:0x{:x} arg2:0x{:x}",
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
        "ReadEncryptedFileRaw" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!ReadEncryptedFileRaw h:0x{:x} arg2:0x{:x}",
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
        "ReadEventLogA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ReadEventLogA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadEventLogW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!ReadEventLogW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegCloseKey" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(hkey);
            log_red!(emu, "advapi32!RegCloseKey 0x{:x}", hkey);
            emu.regs_mut().rax = 0;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "RegConnectRegistryA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegConnectRegistryA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegConnectRegistryExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegConnectRegistryExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegConnectRegistryExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegConnectRegistryExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegConnectRegistryW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegConnectRegistryW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegCopyTreeA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegCopyTreeA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegCopyTreeW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegCopyTreeW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegCreateKeyA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let phk_result = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let subkey = if subkey_ptr != 0 {
                emu.maps.read_string(subkey_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "advapi32!RegCreateKeyA key:0x{:x} sub:'{}'",
                hkey,
                subkey
            );
            let new_key = helper::handler_create("reg_key");
            if phk_result != 0 {
                emu.maps.write_dword(phk_result, new_key as u32);
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "RegCreateKeyExA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let class_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let options = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let sam = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let security = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let disposition_ptr =
                emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0) as u64;
            let subkey = emu.maps.read_string(subkey_ptr);
            let new_handle = helper::handler_create(&subkey);
            if result_ptr != 0 {
                emu.maps.write_dword(result_ptr, new_handle as u32);
            }
            if disposition_ptr != 0 {
                emu.maps.write_dword(disposition_ptr, 2);
            }
            log_red!(
                emu,
                "advapi32!RegCreateKeyExA hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                new_handle
            );
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "RegCreateKeyExW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let class_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let options = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let sam = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let security = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let disposition_ptr =
                emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0) as u64;
            let subkey = emu.maps.read_wide_string(subkey_ptr);
            let new_handle = helper::handler_create(&subkey);
            if result_ptr != 0 {
                emu.maps.write_dword(result_ptr, new_handle as u32);
            }
            if disposition_ptr != 0 {
                emu.maps.write_dword(disposition_ptr, 2);
            }
            log_red!(
                emu,
                "advapi32!RegCreateKeyExW hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                new_handle
            );
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "RegCreateKeyTransactedA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let subkey = emu.maps.read_string(subkey_ptr);
            let hndl = helper::handler_create(&format!("regkey_{}", subkey));
            log_red!(
                emu,
                "advapi32!RegCreateKeyTransactedA hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
        }
        "RegCreateKeyTransactedW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let subkey = emu.maps.read_wide_string(subkey_ptr);
            let hndl = helper::handler_create(&format!("regkey_{}", subkey));
            log_red!(
                emu,
                "advapi32!RegCreateKeyTransactedW hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
        }
        "RegCreateKeyW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let subkey = emu.maps.read_wide_string(subkey_ptr);
            let hndl = helper::handler_create(&format!("regkey_{}", subkey));
            log_red!(
                emu,
                "advapi32!RegCreateKeyW hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteKeyA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let subkey = emu.maps.read_string(subkey_ptr);
            log_red!(emu, "advapi32!RegDeleteKeyA hkey:0x{:x} '{}'", hkey, subkey);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "RegDeleteKeyExA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            log_red!(emu, "advapi32!RegDeleteKeyExA hkey:0x{:x} '{}'", hkey, name);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteKeyExW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(emu, "advapi32!RegDeleteKeyExW hkey:0x{:x} '{}'", hkey, name);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteKeyTransactedA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            log_red!(
                emu,
                "advapi32!RegDeleteKeyTransactedA hkey:0x{:x} '{}'",
                hkey,
                name
            );
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteKeyTransactedW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(
                emu,
                "advapi32!RegDeleteKeyTransactedW hkey:0x{:x} '{}'",
                hkey,
                name
            );
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteKeyValueA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            log_red!(
                emu,
                "advapi32!RegDeleteKeyValueA hkey:0x{:x} '{}'",
                hkey,
                name
            );
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteKeyValueW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(
                emu,
                "advapi32!RegDeleteKeyValueW hkey:0x{:x} '{}'",
                hkey,
                name
            );
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteKeyW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let subkey = emu.maps.read_wide_string(subkey_ptr);
            log_red!(emu, "advapi32!RegDeleteKeyW hkey:0x{:x} '{}'", hkey, subkey);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "RegDeleteTreeA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            log_red!(emu, "advapi32!RegDeleteTreeA hkey:0x{:x} '{}'", hkey, name);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteTreeW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(emu, "advapi32!RegDeleteTreeW hkey:0x{:x} '{}'", hkey, name);
            emu.regs_mut().rax = 0;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteValueA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            log_red!(emu, "advapi32!RegDeleteValueA hkey:0x{:x} '{}'", hkey, name);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "RegDeleteValueW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(emu, "advapi32!RegDeleteValueW hkey:0x{:x} '{}'", hkey, name);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "RegDisablePredefinedCache" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegDisablePredefinedCache arg1:0x{:x} arg2:0x{:x}",
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
        "RegDisablePredefinedCacheEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegDisablePredefinedCacheEx arg1:0x{:x} arg2:0x{:x}",
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
        "RegDisableReflectionKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegDisableReflectionKey h:0x{:x} arg2:0x{:x}",
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
        "RegEnableReflectionKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegEnableReflectionKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("RegEnableReflectionKey");
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
        "RegEnumKeyA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegEnumKeyA hkey:0x{:x} index:{}",
                hkey,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 259;
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
        }
        "RegEnumKeyExA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegEnumKeyExA hkey:0x{:x} idx:{}",
                hkey,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 259;
        }
        "RegEnumKeyExW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegEnumKeyExW hkey:0x{:x} idx:{}",
                hkey,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 259;
        }
        "RegEnumKeyW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegEnumKeyW hkey:0x{:x} index:{}",
                hkey,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 259;
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
        }
        "RegEnumValueA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegEnumValueA hkey:0x{:x} idx:{}",
                hkey,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 259;
        }
        "RegEnumValueW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegEnumValueW hkey:0x{:x} idx:{}",
                hkey,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 259;
        }
        "RegFlushKey" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!RegFlushKey");
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegGetKeySecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegGetKeySecurity h:0x{:x} arg2:0x{:x}",
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
        "RegGetValueA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegGetValueA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegGetValueW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegGetValueW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegLoadAppKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegLoadAppKeyA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegLoadAppKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegLoadAppKeyW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegLoadKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegLoadKeyA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegLoadKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegLoadKeyW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegLoadMUIStringA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegLoadMUIStringA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegLoadMUIStringW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegLoadMUIStringW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegNotifyChangeKeyValue" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let watch_subtree = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) != 0;
            let filter = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let event = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let async_ = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) != 0;
            log_red!(
                emu,
                "advapi32!RegNotifyChangeKeyValue hkey:0x{:x} subtree:{} filter:0x{:x}",
                hkey,
                watch_subtree,
                filter
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "RegOpenCurrentUser" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegOpenCurrentUser arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegOpenCurrentUser");
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
        "RegOpenKeyA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let sam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let subkey = emu.maps.read_string(subkey_ptr);
            let hndl = helper::handler_create(&format!("regkey_{}", subkey));
            if result_ptr != 0 {
                emu.maps.write_dword(result_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "advapi32!RegOpenKeyA hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "RegOpenKeyExW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let sam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let subkey = emu.maps.read_wide_string(subkey_ptr);
            let hndl = helper::handler_create(&format!("regkey_{}", subkey));
            if result_ptr != 0 {
                emu.maps.write_dword(result_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "advapi32!RegOpenKeyExW hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "RegOpenKeyTransactedA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let sam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let subkey = emu.maps.read_string(subkey_ptr);
            let hndl = helper::handler_create(&format!("regkey_{}", subkey));
            if result_ptr != 0 {
                emu.maps.write_dword(result_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "advapi32!RegOpenKeyTransactedA hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "RegOpenKeyTransactedW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let sam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let subkey = emu.maps.read_wide_string(subkey_ptr);
            let hndl = helper::handler_create(&format!("regkey_{}", subkey));
            if result_ptr != 0 {
                emu.maps.write_dword(result_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "advapi32!RegOpenKeyTransactedW hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "RegOpenKeyW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let sam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let subkey = emu.maps.read_wide_string(subkey_ptr);
            let hndl = helper::handler_create(&format!("regkey_{}", subkey));
            if result_ptr != 0 {
                emu.maps.write_dword(result_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "advapi32!RegOpenKeyW hkey:0x{:x} '{}' => 0x{:x}",
                hkey,
                subkey,
                hndl
            );
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "RegOpenUserClassesRoot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegOpenUserClassesRoot arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegOpenUserClassesRoot");
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
        "RegOverridePredefKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegOverridePredefKey h:0x{:x} arg2:0x{:x}",
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
        "RegQueryInfoKeyA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            if type_ptr != 0 {
                emu.maps.write_dword(type_ptr, 1); // REG_SZ
            }
            log_red!(
                emu,
                "advapi32!RegQueryInfoKeyA hkey:0x{:x} '{}'",
                hkey,
                name
            );
            emu.regs_mut().rax = 2;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegQueryInfoKeyW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            if type_ptr != 0 {
                emu.maps.write_dword(type_ptr, 1); // REG_SZ
            }
            log_red!(
                emu,
                "advapi32!RegQueryInfoKeyW hkey:0x{:x} '{}'",
                hkey,
                name
            );
            emu.regs_mut().rax = 2;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegQueryMultipleValuesA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            if type_ptr != 0 {
                emu.maps.write_dword(type_ptr, 1); // REG_SZ
            }
            log_red!(
                emu,
                "advapi32!RegQueryMultipleValuesA hkey:0x{:x} '{}'",
                hkey,
                name
            );
            emu.regs_mut().rax = 2;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegQueryMultipleValuesW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            if type_ptr != 0 {
                emu.maps.write_dword(type_ptr, 1); // REG_SZ
            }
            log_red!(
                emu,
                "advapi32!RegQueryMultipleValuesW hkey:0x{:x} '{}'",
                hkey,
                name
            );
            emu.regs_mut().rax = 2;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegQueryReflectionKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegQueryReflectionKey h:0x{:x} arg2:0x{:x}",
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
        "RegQueryValueA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            if type_ptr != 0 {
                emu.maps.write_dword(type_ptr, 1); // REG_SZ
            }
            log_red!(emu, "advapi32!RegQueryValueA hkey:0x{:x} '{}'", hkey, name);
            emu.regs_mut().rax = 2;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegQueryValueW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            if type_ptr != 0 {
                emu.maps.write_dword(type_ptr, 1); // REG_SZ
            }
            log_red!(emu, "advapi32!RegQueryValueW hkey:0x{:x} '{}'", hkey, name);
            emu.regs_mut().rax = 2;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegRenameKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegRenameKey h:0x{:x} arg2:0x{:x}",
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
        "RegReplaceKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegReplaceKeyA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "RegReplaceKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegReplaceKeyW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "RegRestoreKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegRestoreKeyA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegRestoreKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegRestoreKeyW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegSaveKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegSaveKeyA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegSaveKeyExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegSaveKeyExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegSaveKeyExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegSaveKeyExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegSaveKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegSaveKeyW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegSetKeySecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegSetKeySecurity h:0x{:x} arg2:0x{:x}",
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
        "RegSetKeyValueA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let reg_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            log_red!(
                emu,
                "advapi32!RegSetKeyValueA hkey:0x{:x} '{}' type:{}",
                hkey,
                name,
                reg_type
            );
            emu.regs_mut().rax = 0;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegSetKeyValueW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let reg_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(
                emu,
                "advapi32!RegSetKeyValueW hkey:0x{:x} '{}' type:{}",
                hkey,
                name,
                reg_type
            );
            emu.regs_mut().rax = 0;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegSetValueA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let reg_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_string(name_ptr);
            log_red!(
                emu,
                "advapi32!RegSetValueA hkey:0x{:x} '{}' type:{}",
                hkey,
                name,
                reg_type
            );
            emu.regs_mut().rax = 0;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegSetValueExA" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let value_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let data_size = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let name = emu.maps.read_string(name_ptr);
            let data_str = if value_type == 1 || value_type == 2 {
                emu.maps.read_string(data_ptr)
            } else {
                format!("binary[{}]", data_size)
            };
            log_red!(
                emu,
                "advapi32!RegSetValueExA hkey:0x{:x} '{}' type:{} data:'{}'",
                hkey,
                name,
                value_type,
                data_str
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "RegSetValueExW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let value_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let data_size = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let name = emu.maps.read_wide_string(name_ptr);
            let data_str = if value_type == 1 || value_type == 2 {
                emu.maps.read_wide_string(data_ptr)
            } else {
                format!("binary[{}]", data_size)
            };
            log_red!(
                emu,
                "advapi32!RegSetValueExW hkey:0x{:x} '{}' type:{} data:'{}'",
                hkey,
                name,
                value_type,
                data_str
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "RegSetValueW" => {
            let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let reg_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = emu.maps.read_wide_string(name_ptr);
            log_red!(
                emu,
                "advapi32!RegSetValueW hkey:0x{:x} '{}' type:{}",
                hkey,
                name,
                reg_type
            );
            emu.regs_mut().rax = 0;
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "RegUnLoadKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!RegUnLoadKeyA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegUnLoadKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!RegUnLoadKeyW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegisterEventSourceA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegisterEventSourceA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("advapi32://RegisterEventSourceA");
            emu.regs_mut().rax = hndl;
        }
        "RegisterEventSourceW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegisterEventSourceW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("advapi32://RegisterEventSourceW");
            emu.regs_mut().rax = hndl;
        }
        "RegisterIdleTask" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RegisterIdleTask arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterIdleTask");
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
        "RegisterServiceCtrlHandlerA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegisterServiceCtrlHandlerA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("advapi32://RegisterServiceCtrlHandlerA");
            emu.regs_mut().rax = hndl;
        }
        "RegisterServiceCtrlHandlerExA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegisterServiceCtrlHandlerExA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("advapi32://RegisterServiceCtrlHandlerExA");
            emu.regs_mut().rax = hndl;
        }
        "RegisterServiceCtrlHandlerExW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegisterServiceCtrlHandlerExW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("advapi32://RegisterServiceCtrlHandlerExW");
            emu.regs_mut().rax = hndl;
        }
        "RegisterServiceCtrlHandlerW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!RegisterServiceCtrlHandlerW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("advapi32://RegisterServiceCtrlHandlerW");
            emu.regs_mut().rax = hndl;
        }
        "RegisterTraceGuidsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegisterTraceGuidsA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegisterTraceGuidsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!RegisterTraceGuidsW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegisterWaitChainCOMCallback" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!RegisterWaitChainCOMCallback 0x{:x}", arg1);
            let handle = helper::handler_create("RegisterWaitChainCOMCallback");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RemoteRegEnumKeyWrapper" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RemoteRegEnumKeyWrapper h:0x{:x} arg2:0x{:x}",
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
        "RemoteRegEnumValueWrapper" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RemoteRegEnumValueWrapper arg1:0x{:x} arg2:0x{:x}",
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
        "RemoteRegQueryInfoKeyWrapper" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RemoteRegQueryInfoKeyWrapper h:0x{:x} arg2:0x{:x}",
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
        "RemoteRegQueryMultipleValues2Wrapper" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RemoteRegQueryMultipleValues2Wrapper arg1:0x{:x} arg2:0x{:x}",
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
        "RemoteRegQueryMultipleValuesWrapper" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RemoteRegQueryMultipleValuesWrapper arg1:0x{:x} arg2:0x{:x}",
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
        "RemoteRegQueryValueWrapper" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RemoteRegQueryValueWrapper arg1:0x{:x} arg2:0x{:x}",
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
        "RemoveTraceCallback" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RemoveTraceCallback h:0x{:x} arg2:0x{:x}",
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
        "RemoveUsersFromEncryptedFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!RemoveUsersFromEncryptedFile h:0x{:x} arg2:0x{:x}",
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
        "ReportEventA" => {
            let hevt_log = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let wtype = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let category = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let event_id = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let _user_sid = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let num_strings = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let _data_size = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
            let _strings_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);
            let _raw_data = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!ReportEventA log:0x{:x} type:{} cat:{} id:0x{:x} nstr:{}",
                hevt_log,
                wtype,
                category,
                event_id,
                num_strings
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReportEventW" => {
            let hevt_log = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let wtype = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let category = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let event_id = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let _user_sid = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let num_strings = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let _data_size = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
            let _strings_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);
            let _raw_data = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!ReportEventW log:0x{:x} type:{} cat:{} id:0x{:x} nstr:{}",
                hevt_log,
                wtype,
                category,
                event_id,
                num_strings
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RevertToSelf" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!RevertToSelf arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SafeBaseRegGetKeySecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SafeBaseRegGetKeySecurity h:0x{:x} arg2:0x{:x}",
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
        "SaferCloseLevel" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferCloseLevel arg1:0x{:x} arg2:0x{:x}",
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
        "SaferComputeTokenFromLevel" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferComputeTokenFromLevel h:0x{:x} arg2:0x{:x}",
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
        "SaferCreateLevel" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferCreateLevel arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SaferCreateLevel");
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
        "SaferGetLevelInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferGetLevelInformation arg1:0x{:x} arg2:0x{:x}",
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
        "SaferGetPolicyInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferGetPolicyInformation arg1:0x{:x} arg2:0x{:x}",
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
        "SaferIdentifyLevel" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferIdentifyLevel arg1:0x{:x} arg2:0x{:x}",
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
        "SaferRecordEventLogEntry" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!SaferRecordEventLogEntry 0x{:x}", arg1);
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
        "SaferSetLevelInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferSetLevelInformation arg1:0x{:x} arg2:0x{:x}",
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
        "SaferSetPolicyInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferSetPolicyInformation arg1:0x{:x} arg2:0x{:x}",
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
        "SaferiChangeRegistryScope" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferiChangeRegistryScope arg1:0x{:x} arg2:0x{:x}",
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
        "SaferiCompareTokenLevels" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferiCompareTokenLevels h:0x{:x} arg2:0x{:x}",
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
        "SaferiIsDllAllowed" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferiIsDllAllowed arg1:0x{:x} arg2:0x{:x}",
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
        "SaferiIsExecutableFileType" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferiIsExecutableFileType h:0x{:x} arg2:0x{:x}",
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
        "SaferiPopulateDefaultsInRegistry" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferiPopulateDefaultsInRegistry arg1:0x{:x} arg2:0x{:x}",
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
        "SaferiRecordEventLogEntry" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!SaferiRecordEventLogEntry 0x{:x}", arg1);
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
        "SaferiSearchMatchingHashRules" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SaferiSearchMatchingHashRules arg1:0x{:x} arg2:0x{:x}",
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
        "SetAclInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetAclInformation h:0x{:x} arg2:0x{:x}",
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
        "SetEncryptedFileMetadata" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetEncryptedFileMetadata h:0x{:x} arg2:0x{:x}",
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
        "SetEntriesInAccessListA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetEntriesInAccessListA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetEntriesInAccessListW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetEntriesInAccessListW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetEntriesInAclA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetEntriesInAclA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetEntriesInAclW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetEntriesInAclW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetEntriesInAuditListA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetEntriesInAuditListA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetEntriesInAuditListW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetEntriesInAuditListW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetFileSecurityA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetFileSecurityA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetFileSecurityW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetFileSecurityW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetInformationCodeAuthzLevelW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetInformationCodeAuthzLevelW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetInformationCodeAuthzPolicyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetInformationCodeAuthzPolicyW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetKernelObjectSecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetKernelObjectSecurity h:0x{:x} arg2:0x{:x}",
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
        "SetNamedSecurityInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetNamedSecurityInfoA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetNamedSecurityInfoExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetNamedSecurityInfoExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetNamedSecurityInfoExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetNamedSecurityInfoExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetNamedSecurityInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetNamedSecurityInfoW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetPrivateObjectSecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetPrivateObjectSecurity h:0x{:x} arg2:0x{:x}",
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
        "SetPrivateObjectSecurityEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetPrivateObjectSecurityEx h:0x{:x} arg2:0x{:x}",
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
        "SetSecurityAccessMask" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetSecurityAccessMask h:0x{:x} arg2:0x{:x}",
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
        "SetSecurityDescriptorControl" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetSecurityDescriptorControl h:0x{:x} arg2:0x{:x}",
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
        "SetSecurityDescriptorDacl" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetSecurityDescriptorDacl p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSecurityDescriptorGroup" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetSecurityDescriptorGroup h:0x{:x} arg2:0x{:x}",
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
        "SetSecurityDescriptorOwner" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetSecurityDescriptorOwner h:0x{:x} arg2:0x{:x}",
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
        "SetSecurityDescriptorRMControl" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetSecurityDescriptorRMControl h:0x{:x} arg2:0x{:x}",
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
        "SetSecurityDescriptorSacl" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetSecurityDescriptorSacl h:0x{:x} arg2:0x{:x}",
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
        "SetSecurityInfo" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetSecurityInfo p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSecurityInfoExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetSecurityInfoExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetSecurityInfoExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!SetSecurityInfoExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetServiceBits" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetServiceBits h:0x{:x} arg2:0x{:x}",
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
        "SetServiceObjectSecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetServiceObjectSecurity h:0x{:x} arg2:0x{:x}",
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
        "SetServiceStatus" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "advapi32!SetServiceStatus p1:0x{:x}", p1);
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
        "SetThreadToken" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetThreadToken h:0x{:x} arg2:0x{:x}",
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
        "SetTokenInformation" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetTokenInformation p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetTraceCallback" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetTraceCallback h:0x{:x} arg2:0x{:x}",
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
        "SetUserFileEncryptionKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetUserFileEncryptionKey h:0x{:x} arg2:0x{:x}",
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
        "SetUserFileEncryptionKeyEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SetUserFileEncryptionKeyEx h:0x{:x} arg2:0x{:x}",
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
        "StartServiceA" => {
            let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let num_args = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let args_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!StartServiceA svc:0x{:x} nargs:{}",
                hsvc,
                num_args
            );
            let handle = helper::handler_create("StartServiceA");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StartServiceW" => {
            let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let num_args = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let args_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "advapi32!StartServiceW svc:0x{:x} nargs:{}",
                hsvc,
                num_args
            );
            let handle = helper::handler_create("StartServiceW");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StartTraceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!StartTraceA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StartTraceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!StartTraceW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StopTraceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!StopTraceA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StopTraceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!StopTraceW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SystemFunction001" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction001 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction002" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction002 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction003" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction003 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction004" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction004 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction005" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction005 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction006" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction006 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction007" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction007 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction008" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction008 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction009" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction009 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction010" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction010 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction011" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction011 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction012" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction012 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction013" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction013 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction014" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction014 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction015" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction015 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction016" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction016 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction017" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction017 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction018" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction018 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction019" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction019 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction020" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction020 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction021" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction021 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction022" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction022 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction023" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction023 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction024" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction024 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction025" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction025 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction026" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction026 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction027" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction027 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction028" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction028 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction029" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction029 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction030" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction030 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction031" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction031 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction032" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction032 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction033" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction033 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction034" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction034 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction035" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction035 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction036" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction036 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction040" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction040 arg1:0x{:x} arg2:0x{:x}",
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
        "SystemFunction041" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!SystemFunction041 arg1:0x{:x} arg2:0x{:x}",
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
        "TraceEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!TraceEvent 0x{:x}", arg1);
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
        "TraceEventInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!TraceEventInstance 0x{:x}", arg1);
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
        "TraceMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!TraceMessage arg1:0x{:x} arg2:0x{:x}",
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
        "TraceMessageVa" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!TraceMessageVa arg1:0x{:x} arg2:0x{:x}",
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
        "TraceQueryInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!TraceQueryInformation arg1:0x{:x} arg2:0x{:x}",
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
        "TraceSetInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!TraceSetInformation arg1:0x{:x} arg2:0x{:x}",
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
        "TreeResetNamedSecurityInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!TreeResetNamedSecurityInfoA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "TreeResetNamedSecurityInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!TreeResetNamedSecurityInfoW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "TreeSetNamedSecurityInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!TreeSetNamedSecurityInfoA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "TreeSetNamedSecurityInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!TreeSetNamedSecurityInfoW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "TrusteeAccessToObjectA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!TrusteeAccessToObjectA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "TrusteeAccessToObjectW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!TrusteeAccessToObjectW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UninstallApplication" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!UninstallApplication arg1:0x{:x} arg2:0x{:x}",
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
        "UnlockServiceDatabase" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!UnlockServiceDatabase 0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UnregisterIdleTask" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!UnregisterIdleTask h:0x{:x} arg2:0x{:x}",
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
        "UnregisterTraceGuids" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!UnregisterTraceGuids h:0x{:x} arg2:0x{:x}",
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
        "UpdateTraceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!UpdateTraceA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UpdateTraceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "advapi32!UpdateTraceW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UsePinForEncryptedFilesA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!UsePinForEncryptedFilesA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UsePinForEncryptedFilesW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!UsePinForEncryptedFilesW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WaitServiceState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!WaitServiceState 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WmiCloseBlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!WmiCloseBlock 0x{:x}", arg1);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WmiDevInstToInstanceNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiDevInstToInstanceNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiDevInstToInstanceNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiDevInstToInstanceNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiEnumerateGuids" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!WmiEnumerateGuids arg1:0x{:x} arg2:0x{:x}",
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
        "WmiExecuteMethodA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiExecuteMethodA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiExecuteMethodW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiExecuteMethodW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiFileHandleToInstanceNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiFileHandleToInstanceNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiFileHandleToInstanceNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiFileHandleToInstanceNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiFreeBuffer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!WmiFreeBuffer arg1:0x{:x} arg2:0x{:x}",
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
        "WmiMofEnumerateResourcesA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiMofEnumerateResourcesA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiMofEnumerateResourcesW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiMofEnumerateResourcesW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiNotificationRegistrationA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiNotificationRegistrationA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiNotificationRegistrationW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiNotificationRegistrationW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiOpenBlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "advapi32!WmiOpenBlock 0x{:x}", arg1);
            let handle = helper::handler_create("WmiOpenBlock");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WmiQueryAllDataA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiQueryAllDataA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiQueryAllDataMultipleA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiQueryAllDataMultipleA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiQueryAllDataMultipleW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiQueryAllDataMultipleW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiQueryAllDataW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiQueryAllDataW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiQueryGuidInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!WmiQueryGuidInformation arg1:0x{:x} arg2:0x{:x}",
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
        "WmiQuerySingleInstanceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiQuerySingleInstanceA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiQuerySingleInstanceMultipleA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiQuerySingleInstanceMultipleA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiQuerySingleInstanceMultipleW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiQuerySingleInstanceMultipleW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiQuerySingleInstanceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiQuerySingleInstanceW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiReceiveNotificationsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiReceiveNotificationsA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiReceiveNotificationsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiReceiveNotificationsW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiSetSingleInstanceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiSetSingleInstanceA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiSetSingleInstanceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiSetSingleInstanceW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiSetSingleItemA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiSetSingleItemA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WmiSetSingleItemW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "advapi32!WmiSetSingleItemW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WriteEncryptedFileRaw" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "advapi32!WriteEncryptedFileRaw h:0x{:x} arg2:0x{:x}",
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
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn StartServiceCtrlDispatcherA(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    /*
    let service_name = emu.maps.read_dword(service_table_entry_ptr as u64)
        .unwrap_or(0);
    let service_name = emu.maps.read_dword((service_table_entry_ptr+4) as u64)
        .unwrap_or(0);*/

    log_red!(emu, "advapi321!StartServiceCtrlDispatcherA");

    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}

fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "advapi321!StartServiceCtrlDispatcherW");

    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}

///// CRYPTO API /////

fn CryptAcquireContextA(emu: &mut emu::Emu) {
    let out_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let container = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let provider = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let uri = "cryptctx://".to_string();
    let hndl = helper::handler_create(&uri) as u32;
    emu.maps.write_dword(out_handle, hndl);

    let mut sflags = String::new();
    if flags & CRYPT_VERIFYCONTEXT == CRYPT_VERIFYCONTEXT {
        sflags.push_str("CRYPT_VERIFYCONTEXT ");
    }
    if flags & CRYPT_NEWKEYSET == CRYPT_NEWKEYSET {
        sflags.push_str("CRYPT_NEWKEYSET ");
    }
    if flags & CRYPT_DELETEKEYSET == CRYPT_DELETEKEYSET {
        sflags.push_str("CRYPT_DELETEKEYSET ");
    }
    if flags & CRYPT_MACHINE_KEYSET == CRYPT_MACHINE_KEYSET {
        sflags.push_str("CRYPT_MACHINE_KEYSET ");
    }
    if flags & CRYPT_SILENT == CRYPT_SILENT {
        sflags.push_str("CRYPT_SILENT ");
    }
    if flags & CRYPT_DEFAULT_CONTAINER_OPTIONAL == CRYPT_DEFAULT_CONTAINER_OPTIONAL {
        sflags.push_str("CRYPT_DEFAULT_CONTAINER_OPTIONAL ");
    }

    log_red!(
        emu,
        "advapi321!CryptAcquireContextA =0x{:x} type: {} flags: `{}`",
        hndl,
        prov_type,
        &sflags
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptAcquireContextW(emu: &mut emu::Emu) {
    let out_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let container = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let provider = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let uri = "cryptctx://".to_string();
    let hndl = helper::handler_create(&uri) as u32;
    emu.maps.write_dword(out_handle, hndl);

    let mut sflags = String::new();
    if flags & CRYPT_VERIFYCONTEXT == CRYPT_VERIFYCONTEXT {
        sflags.push_str("CRYPT_VERIFYCONTEXT ");
    }
    if flags & CRYPT_NEWKEYSET == CRYPT_NEWKEYSET {
        sflags.push_str("CRYPT_NEWKEYSET ");
    }
    if flags & CRYPT_DELETEKEYSET == CRYPT_DELETEKEYSET {
        sflags.push_str("CRYPT_DELETEKEYSET ");
    }
    if flags & CRYPT_MACHINE_KEYSET == CRYPT_MACHINE_KEYSET {
        sflags.push_str("CRYPT_MACHINE_KEYSET ");
    }
    if flags & CRYPT_SILENT == CRYPT_SILENT {
        sflags.push_str("CRYPT_SILENT ");
    }
    if flags & CRYPT_DEFAULT_CONTAINER_OPTIONAL == CRYPT_DEFAULT_CONTAINER_OPTIONAL {
        sflags.push_str("CRYPT_DEFAULT_CONTAINER_OPTIONAL ");
    }

    log_red!(
        emu,
        "advapi321!CryptAcquireContextW =0x{:x} type: {} flags: `{}`",
        hndl,
        prov_type,
        &sflags
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn LookupPrivilegeValueW(emu: &mut emu::Emu) {
    let ptr_sysname = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let ptr_name = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let ptr_uid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    let sysname = emu.maps.read_wide_string(ptr_sysname);
    let name = emu.maps.read_wide_string(ptr_name);

    log_red!(
        emu,
        "advapi32!LookupPrivilegeValueW `{}` `{}`",
        sysname,
        name
    );

    if ptr_uid != 0 {
        let luid: u64 = match name.as_str() {
            "SeDebugPrivilege" => 20,
            "SeShutdownPrivilege" => 19,
            "SeBackupPrivilege" => 17,
            "SeRestorePrivilege" => 18,
            "SeTcbPrivilege" => 7,
            "SeAssignPrimaryTokenPrivilege" => 3,
            "SeIncreaseQuotaPrivilege" => 5,
            _ => 99,
        };
        emu.maps.write_qword(ptr_uid, luid);
    }

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptEncrypt(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let hhash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let bfinal = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let buff_len = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

    if data_ptr != 0 && data_len_ptr != 0 {
        let data_len = emu.maps.read_dword(data_len_ptr).unwrap_or(0) as usize;
        let len = data_len.min(0x100_0000);
        if len > 0 {
            let mut data = emu.maps.read_bytes(data_ptr, len).to_vec();
            crypto_state::encrypt_data(hkey, &mut data);
            emu.maps.write_bytes(data_ptr, &data);
        }
    }

    log_red!(emu, "advapi32!CryptEncrypt hKey=0x{:x}", hkey);

    for _ in 0..7 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}

fn CryptDecrypt(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let hhash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let bfinal = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

    if data_ptr != 0 && data_len_ptr != 0 {
        let data_len = emu.maps.read_dword(data_len_ptr).unwrap_or(0) as usize;
        let len = data_len.min(0x100_0000);
        if len > 0 {
            let mut data = emu.maps.read_bytes(data_ptr, len).to_vec();
            crypto_state::decrypt_data(hkey, &mut data);
            emu.maps.write_bytes(data_ptr, &data);
        }
    }

    log_red!(emu, "advapi32!CryptDecrypt hKey=0x{:x}", hkey);

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}

fn CryptReleaseContext(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(emu, "advapi32!CryptReleaseContext");

    helper::handler_close(hndl);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn CryptCreateHash(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let hkey = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let hash_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    let uri = format!("crypthash://{}", get_cryptoalgorithm_name(algid));
    let hndl = helper::handler_create(&uri);
    assert!(hndl < 0x00000001_00000000);

    crypto_state::create_hash(hndl, algid);
    emu.maps.write_dword(hash_ptr, hndl as u32);

    log_red!(
        emu,
        "advapi32!CryptCreateHash {} =0x{:x}",
        get_cryptoalgorithm_name(algid),
        hndl
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptGenKey(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let hkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

    let uri = format!("cryptkey://{}", get_cryptoalgorithm_name(algid));
    let hndl = helper::handler_create(&uri);
    assert!(hndl < 0x00000001_00000000);

    let key_bits = flags >> 16;
    crypto_state::create_key(hndl, algid, key_bits);

    if hkey_ptr != 0 {
        emu.maps.write_dword(hkey_ptr, hndl as u32);
    }

    log_red!(
        emu,
        "advapi32!CryptGenKey {} =0x{:x}",
        get_cryptoalgorithm_name(algid),
        hndl
    );

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptGetHashParam(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(
        emu,
        "advapi32!CryptGetHashParam hash:0x{:x} param:0x{:x}",
        hhash,
        param
    );

    if let Some(result) = crypto_state::get_hash_param(hhash, param) {
        if data_ptr != 0 {
            let buf_sz = if len_ptr != 0 {
                emu.maps.read_dword(len_ptr).unwrap_or(0) as usize
            } else {
                result.len()
            };
            let write_len = result.len().min(buf_sz);
            emu.maps.write_bytes(data_ptr, &result[..write_len]);
        }
        if len_ptr != 0 {
            emu.maps.write_dword(len_ptr, result.len() as u32);
        }
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

fn CryptGetKeyParam(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(
        emu,
        "advapi32!CryptGetKeyParam key:0x{:x} param:0x{:x}",
        hkey,
        param
    );

    if let Some(result) = crypto_state::get_key_param(hkey, param) {
        if data_ptr != 0 {
            let buf_sz = if len_ptr != 0 {
                emu.maps.read_dword(len_ptr).unwrap_or(0) as usize
            } else {
                result.len()
            };
            let write_len = result.len().min(buf_sz);
            emu.maps.write_bytes(data_ptr, &result[..write_len]);
        }
        if len_ptr != 0 {
            emu.maps.write_dword(len_ptr, result.len() as u32);
        }
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

fn CryptImportKey(emu: &mut emu::Emu) {
    let _hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _hpubkey = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let hkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

    let algid = if data_len >= 12 {
        emu.maps.read_dword(data_ptr + 8).unwrap_or(0x6801)
    } else {
        0x6801
    };

    log_red!(
        emu,
        "advapi32!CryptImportKey data:0x{:x} len:{} alg:0x{:x}",
        data_ptr,
        data_len,
        algid
    );

    if hkey_ptr != 0 {
        let key_handle = helper::handler_create("crypt_key");
        crypto_state::create_key(key_handle, algid, data_len * 8);
        emu.maps.write_dword(hkey_ptr, key_handle as u32);
    }

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptSignHashA(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let sig_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let desc = if desc_ptr != 0 {
        emu.maps.read_string(desc_ptr)
    } else {
        String::new()
    };

    if let Some(hash_val) = crypto_state::get_hash_param(hhash, 2) {
        if sig_ptr == 0 {
            if sig_len_ptr != 0 {
                emu.maps.write_dword(sig_len_ptr, hash_val.len() as u32);
            }
        } else {
            let write_len = if sig_len_ptr != 0 {
                let buf_sz = emu.maps.read_dword(sig_len_ptr).unwrap_or(0) as usize;
                hash_val.len().min(buf_sz)
            } else {
                hash_val.len()
            };
            emu.maps.write_bytes(sig_ptr, &hash_val[..write_len]);
            if sig_len_ptr != 0 {
                emu.maps.write_dword(sig_len_ptr, write_len as u32);
            }
        }
        log_red!(
            emu,
            "advapi32!CryptSignHashA hash:0x{:x} desc:'{}' siglen:{}",
            hhash,
            desc,
            hash_val.len()
        );
    } else {
        log_red!(
            emu,
            "advapi32!CryptSignHashA hash:0x{:x} desc:'{}'",
            hhash,
            desc
        );
    }

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptSignHashW(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let sig_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let desc = if desc_ptr != 0 {
        emu.maps.read_wide_string(desc_ptr)
    } else {
        String::new()
    };

    if let Some(hash_val) = crypto_state::get_hash_param(hhash, 2) {
        if sig_ptr == 0 {
            if sig_len_ptr != 0 {
                emu.maps.write_dword(sig_len_ptr, hash_val.len() as u32);
            }
        } else {
            let write_len = if sig_len_ptr != 0 {
                let buf_sz = emu.maps.read_dword(sig_len_ptr).unwrap_or(0) as usize;
                hash_val.len().min(buf_sz)
            } else {
                hash_val.len()
            };
            emu.maps.write_bytes(sig_ptr, &hash_val[..write_len]);
            if sig_len_ptr != 0 {
                emu.maps.write_dword(sig_len_ptr, write_len as u32);
            }
        }
        log_red!(
            emu,
            "advapi32!CryptSignHashW hash:0x{:x} desc:'{}' siglen:{}",
            hhash,
            desc,
            hash_val.len()
        );
    } else {
        log_red!(
            emu,
            "advapi32!CryptSignHashW hash:0x{:x} desc:'{}'",
            hhash,
            desc
        );
    }

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptHashData(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let len = data_len.min(0x100_0000);
    if data_ptr != 0 && len > 0 {
        let data = emu.maps.read_bytes(data_ptr, len);
        crypto_state::hash_data(hhash, data);
    }

    log_red!(
        emu,
        "advapi32!CryptHashData hHash=0x{:x} len={}",
        hhash,
        data_len
    );

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptDeriveKey(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _hbase_data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let hkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    let alg = get_cryptoalgorithm_name(algid);
    let mut alg_len = get_crypto_key_len(algid);

    let handle = helper::handler_create(alg);
    if alg_len == 0 {
        alg_len = ((flags >> 16) & 0xffff) as usize;
    }
    helper::handler_put_bytes(handle, &vec![0x41u8; alg_len]);
    crypto_state::create_key(handle, algid, alg_len as u32);

    log_red!(emu, "advapi32!CryptDeriveKey {} bits:{}", alg, alg_len);

    if hkey_ptr != 0 {
        emu.maps.write_dword(hkey_ptr, handle as u32);
    }

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let subkey = emu.maps.read_string(arg(emu, 1) as u64);
    let result_ptr = arg(emu, 4) as u64;
    log_red!(emu, "advapi32!RegOpenKeyExA {}", subkey);
    if result_ptr != 0 {
        let hndl = helper::handler_create(&format!("registry://{}", subkey));
        emu.maps.write_dword(result_ptr, hndl as u32);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 5);
}
pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    let value_ptr = arg(emu, 1) as u64;
    let type_ptr = arg(emu, 3) as u64;
    let data_ptr = arg(emu, 4) as u64;
    let data_size_ptr = arg(emu, 5) as u64;
    let value = if value_ptr != 0 {
        emu.maps.read_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!RegQueryValueExA `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    // The registry is not modelled: hand back a well-formed empty value.
    if data_ptr != 0 {
        emu.maps.write_string(data_ptr, "");
    }
    if data_size_ptr != 0 {
        emu.maps.write_dword(data_size_ptr, 1);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 6);
}
fn RegQueryValueExW(emu: &mut emu::Emu) {
    let value_ptr = arg(emu, 1) as u64;
    let type_ptr = arg(emu, 3) as u64;
    let data_ptr = arg(emu, 4) as u64;
    let data_size_ptr = arg(emu, 5) as u64;
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!RegQueryValueExW `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    // The registry is not modelled: hand back a well-formed empty value.
    if data_ptr != 0 {
        emu.maps.write_wide_string(data_ptr, "");
    }
    if data_size_ptr != 0 {
        emu.maps.write_dword(data_size_ptr, 2);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 6);
}
fn GetUserNameA(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let size_ptr = arg(emu, 1) as u64;
    let user = emu.cfg.user_name.clone();
    let required = user.chars().count() as u32 + 1;
    let provided = emu.maps.read_dword(size_ptr).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!GetUserNameA '{}' buf size: {}",
        user,
        provided
    );
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, required);
    }
    if buf == 0 || size_ptr == 0 || provided < required {
        emu.regs_mut().rax = 0;
    } else {
        emu.maps.write_string(buf, &user);
        emu.regs_mut().rax = 1;
    }
    stdcall_ret(emu, 2);
}
fn GetUserNameW(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let size_ptr = arg(emu, 1) as u64;
    let user = emu.cfg.user_name.clone();
    let required = user.chars().count() as u32 + 1;
    let provided = emu.maps.read_dword(size_ptr).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!GetUserNameW '{}' buf size: {}",
        user,
        provided
    );
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, required);
    }
    if buf == 0 || size_ptr == 0 || provided < required {
        emu.regs_mut().rax = 0;
    } else {
        emu.maps.write_wide_string(buf, &user);
        emu.regs_mut().rax = 1;
    }
    stdcall_ret(emu, 2);
}
fn CryptGenRandom(emu: &mut emu::Emu) {
    let len = arg(emu, 1) as u64;
    let buf = arg(emu, 2) as u64;
    log_red!(emu, "advapi32!CryptGenRandom len={} buf=0x{:x}", len, buf);
    let tick = emu.tick as u8;
    for i in 0..len {
        emu.maps.write_byte(buf + i, tick.wrapping_add(i as u8));
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let token = arg(emu, 0);
    let disable_all = arg(emu, 1);
    log_red!(
        emu,
        "advapi32!AdjustTokenPrivileges token=0x{:x} disableAll={}",
        token,
        disable_all
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
