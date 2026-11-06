use crate::api::windows::common::fallback;
use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::emu;
use crate::windows::constants;
//use crate::endpoint;
use crate::winapi::helper;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "InternetOpenA" => InternetOpenA(emu),
        "InternetOpenW" => InternetOpenW(emu),
        "InternetConnectA" => InternetConnectA(emu),
        "InternetConnectW" => InternetConnectW(emu),
        "HttpOpenRequestA" => HttpOpenRequestA(emu),
        "HttpOpenRequestW" => HttpOpenRequestW(emu),
        "InternetSetOptionA" => InternetSetOptionA(emu),
        "InternetSetOptionW" => InternetSetOptionW(emu),
        "HttpSendRequestA" => HttpSendRequestA(emu),
        "HttpSendRequestW" => HttpSendRequestW(emu),
        "InternetReadFile" => InternetReadFile(emu),
        "InternetReadFileExA" => InternetReadFileExA(emu),
        "InternetReadFileExW" => InternetReadFileExW(emu),
        "InternetErrorDlg" => InternetErrorDlg(emu),
        "AppCacheCheckManifest" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheCheckManifest arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheCloseHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheCloseHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheCreateAndCommitFile" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheCreateAndCommitFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheDeleteGroup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheDeleteGroup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheDeleteIEGroup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheDeleteIEGroup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheDuplicateHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheDuplicateHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheFinalize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheFinalize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheFreeDownloadList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheFreeDownloadList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheFreeGroupList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheFreeGroupList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheFreeIESpace" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheFreeIESpace arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheFreeSpace" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheFreeSpace arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetDownloadList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheGetDownloadList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetFallbackUrl" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheGetFallbackUrl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetGroupList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheGetGroupList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetIEGroupList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheGetIEGroupList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheGetInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetManifestUrl" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheGetManifestUrl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppCacheLookup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!AppCacheLookup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CommitUrlCacheEntryA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            let arg9 = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);
            log_red!(
                emu,
                "wininet!CommitUrlCacheEntryA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CommitUrlCacheEntryBinaryBlob" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!CommitUrlCacheEntryBinaryBlob arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CommitUrlCacheEntryW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            let arg9 = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);
            log_red!(
                emu,
                "wininet!CommitUrlCacheEntryW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateMD5SSOHash" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("CreateMD5SSOHash");
            log_red!(emu, "wininet!CreateMD5SSOHash => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateUrlCacheContainerA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!CreateUrlCacheContainerA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateUrlCacheContainerW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!CreateUrlCacheContainerW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateUrlCacheEntryA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "wininet!CreateUrlCacheEntryA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let handle = helper::handler_create("CreateUrlCacheEntryA");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateUrlCacheEntryExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!CreateUrlCacheEntryExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateUrlCacheEntryW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "wininet!CreateUrlCacheEntryW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let handle = helper::handler_create("CreateUrlCacheEntryW");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateUrlCacheGroup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!CreateUrlCacheGroup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CreateUrlCacheGroup");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x10001;
        }
        "DeleteIE3Cache" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "wininet!DeleteIE3Cache handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "DeleteUrlCacheContainerA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!DeleteUrlCacheContainerA '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteUrlCacheContainerW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!DeleteUrlCacheContainerW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteUrlCacheEntry" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "wininet!DeleteUrlCacheEntry handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "DeleteUrlCacheEntryA" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "wininet!DeleteUrlCacheEntryA handle:0x{:x}", handle);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteUrlCacheEntryW" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "wininet!DeleteUrlCacheEntryW handle:0x{:x}", handle);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteUrlCacheGroup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!DeleteUrlCacheGroup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteWpadCacheForNetworks" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(
                emu,
                "wininet!DeleteWpadCacheForNetworks handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
        }
        "DetectAutoProxyUrl" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!DetectAutoProxyUrl arg1:0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "DispatchAPICall" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!DispatchAPICall arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllCanUnloadNow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!DllCanUnloadNow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "DllGetClassObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!DllGetClassObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllInstall" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!DllInstall arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllRegisterServer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!DllRegisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("DllRegisterServer");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DllUnregisterServer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!DllUnregisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindCloseUrlCache" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "wininet!FindCloseUrlCache handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.regs_mut().rax = 1;
        }
        "FindFirstUrlCacheContainerA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheContainerA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcachecontainera");
            emu.regs_mut().rax = handle;
        }
        "FindFirstUrlCacheContainerW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheContainerW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcachecontainerw");
            emu.regs_mut().rax = handle;
        }
        "FindFirstUrlCacheEntryA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcacheentrya");
            emu.regs_mut().rax = handle;
        }
        "FindFirstUrlCacheEntryExA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryExA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcacheentryexa");
            emu.regs_mut().rax = handle;
        }
        "FindFirstUrlCacheEntryExW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryExW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcacheentryexw");
            emu.regs_mut().rax = handle;
        }
        "FindFirstUrlCacheEntryW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcacheentryw");
            emu.regs_mut().rax = handle;
        }
        "FindFirstUrlCacheGroup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheGroup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcachegroup");
            emu.regs_mut().rax = handle;
        }
        "FindNextUrlCacheContainerA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!FindNextUrlCacheContainerA arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "FindNextUrlCacheContainerW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!FindNextUrlCacheContainerW arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "FindNextUrlCacheEntryA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!FindNextUrlCacheEntryA arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "FindNextUrlCacheEntryExA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!FindNextUrlCacheEntryExA arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "FindNextUrlCacheEntryExW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!FindNextUrlCacheEntryExW arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "FindNextUrlCacheEntryW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!FindNextUrlCacheEntryW arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "FindNextUrlCacheGroup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!FindNextUrlCacheGroup arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "ForceNexusLookup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!ForceNexusLookup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ForceNexusLookupExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "wininet!ForceNexusLookupExW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FreeUrlCacheSpaceA" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "wininet!FreeUrlCacheSpaceA '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FreeUrlCacheSpaceW" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "wininet!FreeUrlCacheSpaceW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpCommandA" => {
            let ftp_handle = emu.regs().rcx;
            let expect_response = emu.regs().rdx as u32;
            let flags = emu.regs().r8 as u32;
            let cmd_ptr = emu.regs().r9;
            let cmd = if cmd_ptr != 0 {
                emu.maps.read_string(cmd_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpCommandA handle:0x{:x} cmd:'{}'",
                ftp_handle,
                cmd
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpCommandW" => {
            let ftp_handle = emu.regs().rcx;
            let expect_response = emu.regs().rdx as u32;
            let flags = emu.regs().r8 as u32;
            let cmd_ptr = emu.regs().r9;
            let cmd = if cmd_ptr != 0 {
                emu.maps.read_wide_string(cmd_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpCommandW handle:0x{:x} cmd:'{}'",
                ftp_handle,
                cmd
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpCreateDirectoryA" => {
            let ftp_handle = emu.regs().rcx;
            let dir_ptr = emu.regs().rdx;
            let dir = if dir_ptr != 0 {
                emu.maps.read_string(dir_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpCreateDirectoryA handle:0x{:x} dir:'{}'",
                ftp_handle,
                dir
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpCreateDirectoryW" => {
            let ftp_handle = emu.regs().rcx;
            let dir_ptr = emu.regs().rdx;
            let dir = if dir_ptr != 0 {
                emu.maps.read_wide_string(dir_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpCreateDirectoryW handle:0x{:x} dir:'{}'",
                ftp_handle,
                dir
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpDeleteFileA" => {
            let ftp_handle = emu.regs().rcx;
            let file_ptr = emu.regs().rdx;
            let file = if file_ptr != 0 {
                emu.maps.read_string(file_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpDeleteFileA handle:0x{:x} file:'{}'",
                ftp_handle,
                file
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpDeleteFileW" => {
            let ftp_handle = emu.regs().rcx;
            let file_ptr = emu.regs().rdx;
            let file = if file_ptr != 0 {
                emu.maps.read_wide_string(file_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpDeleteFileW handle:0x{:x} file:'{}'",
                ftp_handle,
                file
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpFindFirstFileA" => {
            let ftp_handle = emu.regs().rcx;
            let search_ptr = emu.regs().rdx;
            let find_data_ptr = emu.regs().r8;
            let flags = emu.regs().r9 as u32;
            let search = if search_ptr != 0 {
                emu.maps.read_string(search_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpFindFirstFileA handle:0x{:x} search:'{}'",
                ftp_handle,
                search
            );
            emu.set_last_error(18);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
        }
        "FtpFindFirstFileW" => {
            let ftp_handle = emu.regs().rcx;
            let search_ptr = emu.regs().rdx;
            let find_data_ptr = emu.regs().r8;
            let flags = emu.regs().r9 as u32;
            let search = if search_ptr != 0 {
                emu.maps.read_wide_string(search_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpFindFirstFileW handle:0x{:x} search:'{}'",
                ftp_handle,
                search
            );
            emu.set_last_error(18);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
        }
        "FtpGetCurrentDirectoryA" => {
            let handle = emu.regs().rcx;
            let dir_ptr = emu.regs().rdx;
            let dir_len_ptr = emu.regs().r8;
            log_red!(emu, "wininet!FtpGetCurrentDirectoryA handle:0x{:x}", handle);
            if dir_ptr != 0 {
                emu.maps.write_string(dir_ptr, "/");
            }
            if dir_len_ptr != 0 {
                emu.maps.write_dword(dir_len_ptr, 1);
            }
            emu.regs_mut().rax = 1;
        }
        "FtpGetCurrentDirectoryW" => {
            let handle = emu.regs().rcx;
            let dir_ptr = emu.regs().rdx;
            let dir_len_ptr = emu.regs().r8;
            log_red!(emu, "wininet!FtpGetCurrentDirectoryW handle:0x{:x}", handle);
            if dir_ptr != 0 {
                emu.maps.write_string(dir_ptr, "/");
            }
            if dir_len_ptr != 0 {
                emu.maps.write_dword(dir_len_ptr, 1);
            }
            emu.regs_mut().rax = 1;
        }
        "FtpGetFileA" => {
            let ftp_handle = emu.regs().rcx;
            let remote_ptr = emu.regs().rdx;
            let local_ptr = emu.regs().r8;
            let remote = if remote_ptr != 0 {
                emu.maps.read_string(remote_ptr)
            } else {
                String::new()
            };
            let local = if local_ptr != 0 {
                emu.maps.read_string(local_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpGetFileA handle:0x{:x} remote:'{}' local:'{}'",
                ftp_handle,
                remote,
                local
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpGetFileEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FtpGetFileEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpGetFileSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FtpGetFileSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "FtpGetFileW" => {
            let ftp_handle = emu.regs().rcx;
            let remote_ptr = emu.regs().rdx;
            let local_ptr = emu.regs().r8;
            let remote = if remote_ptr != 0 {
                emu.maps.read_wide_string(remote_ptr)
            } else {
                String::new()
            };
            let local = if local_ptr != 0 {
                emu.maps.read_wide_string(local_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpGetFileW handle:0x{:x} remote:'{}' local:'{}'",
                ftp_handle,
                remote,
                local
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpOpenFileA" => {
            let ftp_handle = emu.regs().rcx;
            let filename_ptr = emu.regs().rdx;
            let access = emu.regs().r8 as u32;
            let flags = emu.regs().r9 as u32;
            let filename = if filename_ptr != 0 {
                emu.maps.read_string(filename_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpOpenFileA handle:0x{:x} file:'{}' access:0x{:x}",
                ftp_handle,
                filename,
                access
            );
            let uri = format!("ftp://{}", filename);
            emu.regs_mut().rax = helper::handler_create(&uri);
        }
        "FtpOpenFileW" => {
            let ftp_handle = emu.regs().rcx;
            let filename_ptr = emu.regs().rdx;
            let access = emu.regs().r8 as u32;
            let flags = emu.regs().r9 as u32;
            let filename = if filename_ptr != 0 {
                emu.maps.read_wide_string(filename_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpOpenFileW handle:0x{:x} file:'{}' access:0x{:x}",
                ftp_handle,
                filename,
                access
            );
            let uri = format!("ftp://{}", filename);
            emu.regs_mut().rax = helper::handler_create(&uri);
        }
        "FtpPutFileA" => {
            let ftp_handle = emu.regs().rcx;
            let local_ptr = emu.regs().rdx;
            let remote_ptr = emu.regs().r8;
            let local = if local_ptr != 0 {
                emu.maps.read_string(local_ptr)
            } else {
                String::new()
            };
            let remote = if remote_ptr != 0 {
                emu.maps.read_string(remote_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpPutFileA handle:0x{:x} local:'{}' remote:'{}'",
                ftp_handle,
                local,
                remote
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpPutFileEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!FtpPutFileEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpPutFileW" => {
            let ftp_handle = emu.regs().rcx;
            let local_ptr = emu.regs().rdx;
            let remote_ptr = emu.regs().r8;
            let local = if local_ptr != 0 {
                emu.maps.read_wide_string(local_ptr)
            } else {
                String::new()
            };
            let remote = if remote_ptr != 0 {
                emu.maps.read_wide_string(remote_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpPutFileW handle:0x{:x} local:'{}' remote:'{}'",
                ftp_handle,
                local,
                remote
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpRemoveDirectoryA" => {
            let ftp_handle = emu.regs().rcx;
            let dir_ptr = emu.regs().rdx;
            let dir = if dir_ptr != 0 {
                emu.maps.read_string(dir_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpRemoveDirectoryA handle:0x{:x} dir:'{}'",
                ftp_handle,
                dir
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpRemoveDirectoryW" => {
            let ftp_handle = emu.regs().rcx;
            let dir_ptr = emu.regs().rdx;
            let dir = if dir_ptr != 0 {
                emu.maps.read_wide_string(dir_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpRemoveDirectoryW handle:0x{:x} dir:'{}'",
                ftp_handle,
                dir
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpRenameFileA" => {
            let ftp_handle = emu.regs().rcx;
            let old_ptr = emu.regs().rdx;
            let new_ptr = emu.regs().r8;
            let old_name = if old_ptr != 0 {
                emu.maps.read_string(old_ptr)
            } else {
                String::new()
            };
            let new_name = if new_ptr != 0 {
                emu.maps.read_string(new_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpRenameFileA handle:0x{:x} old:'{}' new:'{}'",
                ftp_handle,
                old_name,
                new_name
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpRenameFileW" => {
            let ftp_handle = emu.regs().rcx;
            let old_ptr = emu.regs().rdx;
            let new_ptr = emu.regs().r8;
            let old_name = if old_ptr != 0 {
                emu.maps.read_wide_string(old_ptr)
            } else {
                String::new()
            };
            let new_name = if new_ptr != 0 {
                emu.maps.read_wide_string(new_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpRenameFileW handle:0x{:x} old:'{}' new:'{}'",
                ftp_handle,
                old_name,
                new_name
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpSetCurrentDirectoryA" => {
            let ftp_handle = emu.regs().rcx;
            let dir_ptr = emu.regs().rdx;
            let dir = if dir_ptr != 0 {
                emu.maps.read_string(dir_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpSetCurrentDirectoryA handle:0x{:x} dir:'{}'",
                ftp_handle,
                dir
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FtpSetCurrentDirectoryW" => {
            let ftp_handle = emu.regs().rcx;
            let dir_ptr = emu.regs().rdx;
            let dir = if dir_ptr != 0 {
                emu.maps.read_wide_string(dir_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!FtpSetCurrentDirectoryW handle:0x{:x} dir:'{}'",
                ftp_handle,
                dir
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProxyDllInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!GetProxyDllInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUrlCacheConfigInfoA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!GetUrlCacheConfigInfoA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUrlCacheConfigInfoW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!GetUrlCacheConfigInfoW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUrlCacheEntryBinaryBlob" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!GetUrlCacheEntryBinaryBlob h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUrlCacheEntryInfoA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GetUrlCacheEntryInfoA arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GetUrlCacheEntryInfoExA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GetUrlCacheEntryInfoExA arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GetUrlCacheEntryInfoExW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GetUrlCacheEntryInfoExW arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GetUrlCacheEntryInfoW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GetUrlCacheEntryInfoW arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GetUrlCacheGroupAttributeA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GetUrlCacheGroupAttributeA arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GetUrlCacheGroupAttributeW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GetUrlCacheGroupAttributeW arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GetUrlCacheHeaderData" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!GetUrlCacheHeaderData h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GopherCreateLocatorA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GopherCreateLocatorA arg1:0x{:x}", arg1);
            let handle = helper::handler_create("GopherCreateLocatorA");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GopherCreateLocatorW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GopherCreateLocatorW arg1:0x{:x}", arg1);
            let handle = helper::handler_create("GopherCreateLocatorW");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GopherFindFirstFileA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "wininet!GopherFindFirstFileA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
        }
        "GopherFindFirstFileW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "wininet!GopherFindFirstFileW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
        }
        "GopherGetAttributeA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GopherGetAttributeA arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GopherGetAttributeW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GopherGetAttributeW arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GopherGetLocatorTypeA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GopherGetLocatorTypeA arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GopherGetLocatorTypeW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!GopherGetLocatorTypeW arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "GopherOpenFileA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "wininet!GopherOpenFileA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let handle = helper::handler_create("GopherOpenFileA");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
        }
        "GopherOpenFileW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "wininet!GopherOpenFileW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let handle = helper::handler_create("GopherOpenFileW");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
        }
        "HttpAddRequestHeadersA" => {
            let handle = emu.regs().rcx;
            let headers_ptr = emu.regs().rdx;
            let headers_len = emu.regs().r8 as i32;
            let modifiers = emu.regs().r9 as u32;
            let hdrs = if headers_ptr != 0 {
                emu.maps.read_string(headers_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!HttpAddRequestHeadersA handle:0x{:x} hdrs:'{}' mod:0x{:x}",
                handle,
                hdrs,
                modifiers
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpAddRequestHeadersW" => {
            let handle = emu.regs().rcx;
            let headers_ptr = emu.regs().rdx;
            let headers_len = emu.regs().r8 as i32;
            let modifiers = emu.regs().r9 as u32;
            let hdrs = if headers_ptr != 0 {
                emu.maps.read_wide_string(headers_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!HttpAddRequestHeadersW handle:0x{:x} hdrs:'{}' mod:0x{:x}",
                handle,
                hdrs,
                modifiers
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpCheckDavCompliance" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpCheckDavCompliance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "HttpCloseDependencyHandle" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "wininet!HttpCloseDependencyHandle 0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "HttpDuplicateDependencyHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpDuplicateDependencyHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("HttpDuplicateDependencyHandle");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpEndRequestA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "wininet!HttpEndRequestA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            if arg1 != 0 && !helper::handler_exist(arg1) {
                emu.set_last_error(12019);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpEndRequestW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "wininet!HttpEndRequestW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            if arg1 != 0 && !helper::handler_exist(arg1) {
                emu.set_last_error(12019);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpGetServerCredentials" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpGetServerCredentials arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpGetTunnelSocket" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpGetTunnelSocket h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpIndicatePageLoadComplete" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpIndicatePageLoadComplete arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("HttpIndicatePageLoadComplete");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpIsHostHstsEnabled" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpIsHostHstsEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("HttpIsHostHstsEnabled");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpOpenDependencyHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpOpenDependencyHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("HttpOpenDependencyHandle");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpPushClose" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "wininet!HttpPushClose 0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "HttpPushEnable" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpPushEnable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("HttpPushEnable");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpPushWait" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpPushWait arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpQueryInfoA" => HttpQueryInfoA(emu),
        "HttpQueryInfoW" => HttpQueryInfoW(emu),
        "HttpSendRequestExA" => {
            let handle = emu.regs().rcx;
            let buffers_in = emu.regs().rdx;
            let buffers_out = emu.regs().r8;
            let flags = emu.regs().r9;
            log_red!(
                emu,
                "wininet!HttpSendRequestExA handle:0x{:x} buf_in:0x{:x} flags:0x{:x}",
                handle,
                buffers_in,
                flags
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpSendRequestExW" => {
            let handle = emu.regs().rcx;
            let buffers_in = emu.regs().rdx;
            let buffers_out = emu.regs().r8;
            let flags = emu.regs().r9;
            log_red!(
                emu,
                "wininet!HttpSendRequestExW handle:0x{:x} buf_in:0x{:x} flags:0x{:x}",
                handle,
                buffers_in,
                flags
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpWebSocketClose" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "wininet!HttpWebSocketClose 0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "HttpWebSocketCompleteUpgrade" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpWebSocketCompleteUpgrade h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpWebSocketQueryCloseStatus" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "wininet!HttpWebSocketQueryCloseStatus 0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "HttpWebSocketReceive" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpWebSocketReceive h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpWebSocketSend" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpWebSocketSend h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HttpWebSocketShutdown" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!HttpWebSocketShutdown h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IncrementUrlCacheHeaderData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!IncrementUrlCacheHeaderData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetAlgIdToStringA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!InternetAlgIdToStringA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetAlgIdToStringW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!InternetAlgIdToStringW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetAttemptConnect" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "wininet!InternetAttemptConnect arg1:0x{:x}", arg1);
            let handle = helper::handler_create("InternetAttemptConnect");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "InternetAutodial" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetAutodial arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetAutodialCallback" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetAutodialCallback arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetAutodialHangup" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "wininet!InternetAutodialHangup handle:0x{:x}", handle);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetCanonicalizeUrlA" => {
            let url_ptr = emu.regs().rcx;
            let out_ptr = emu.regs().rdx;
            let out_len_ptr = emu.regs().r8;
            let flags = emu.regs().r9 as u32;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetCanonicalizeUrlA url:'{}' flags:0x{:x}",
                url,
                flags
            );
            if out_ptr != 0 {
                emu.maps.write_string(out_ptr, &url);
            }
            if out_len_ptr != 0 {
                emu.maps.write_dword(out_len_ptr, url.len() as u32);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetCanonicalizeUrlW" => {
            let url_ptr = emu.regs().rcx;
            let out_ptr = emu.regs().rdx;
            let out_len_ptr = emu.regs().r8;
            let flags = emu.regs().r9 as u32;
            let url = if url_ptr != 0 {
                emu.maps.read_wide_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetCanonicalizeUrlW url:'{}' flags:0x{:x}",
                url,
                flags
            );
            if out_ptr != 0 {
                emu.maps.write_wide_string(out_ptr, &url);
            }
            if out_len_ptr != 0 {
                emu.maps.write_dword(out_len_ptr, url.len() as u32);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetCheckConnectionA" => {
            let url_ptr = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetCheckConnectionA url:'{}' flags:0x{:x}",
                url,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetCheckConnectionW" => {
            let url_ptr = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let url = if url_ptr != 0 {
                emu.maps.read_wide_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetCheckConnectionW url:'{}' flags:0x{:x}",
                url,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetClearAllPerSiteCookieDecisions" => {
            log_red!(emu, "wininet!InternetClearAllPerSiteCookieDecisions");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetCloseHandle" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "wininet!InternetCloseHandle handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetCombineUrlA" => {
            let base_ptr = emu.regs().rcx;
            let relative_ptr = emu.regs().rdx;
            let combined_ptr = emu.regs().r8;
            let combined_len_ptr = emu.regs().r9;
            let base = if base_ptr != 0 {
                emu.maps.read_string(base_ptr)
            } else {
                String::new()
            };
            let relative = if relative_ptr != 0 {
                emu.maps.read_string(relative_ptr)
            } else {
                String::new()
            };
            let combined = if relative.starts_with("http://") || relative.starts_with("https://") {
                relative.clone()
            } else {
                format!(
                    "{}/{}",
                    base.trim_end_matches('/'),
                    relative.trim_start_matches('/')
                )
            };
            log_red!(
                emu,
                "wininet!InternetCombineUrlA base:'{}' rel:'{}' => '{}'",
                base,
                relative,
                combined
            );
            if combined_ptr != 0 {
                emu.maps.write_string(combined_ptr, &combined);
            }
            if combined_len_ptr != 0 {
                emu.maps
                    .write_dword(combined_len_ptr, combined.len() as u32);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetCombineUrlW" => {
            let base_ptr = emu.regs().rcx;
            let relative_ptr = emu.regs().rdx;
            let combined_ptr = emu.regs().r8;
            let combined_len_ptr = emu.regs().r9;
            let base = if base_ptr != 0 {
                emu.maps.read_wide_string(base_ptr)
            } else {
                String::new()
            };
            let relative = if relative_ptr != 0 {
                emu.maps.read_wide_string(relative_ptr)
            } else {
                String::new()
            };
            let combined = if relative.starts_with("http://") || relative.starts_with("https://") {
                relative.clone()
            } else {
                format!(
                    "{}/{}",
                    base.trim_end_matches('/'),
                    relative.trim_start_matches('/')
                )
            };
            log_red!(
                emu,
                "wininet!InternetCombineUrlW base:'{}' rel:'{}' => '{}'",
                base,
                relative,
                combined
            );
            if combined_ptr != 0 {
                emu.maps.write_wide_string(combined_ptr, &combined);
            }
            if combined_len_ptr != 0 {
                emu.maps
                    .write_dword(combined_len_ptr, combined.len() as u32);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetConfirmZoneCrossing" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetConfirmZoneCrossing arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "InternetConfirmZoneCrossingA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetConfirmZoneCrossingA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "InternetConfirmZoneCrossingW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetConfirmZoneCrossingW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "InternetConvertUrlFromWireToWideChar" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetConvertUrlFromWireToWideChar arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetCrackUrlA" => InternetCrackUrlA(emu),
        "InternetCrackUrlW" => InternetCrackUrlW(emu),
        "InternetCreateUrlA" => {
            let url_ptr = emu.regs().rcx;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetCreateUrlA url:'{}'", url);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetCreateUrlW" => {
            let url_ptr = emu.regs().rcx;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetCreateUrlW url:'{}'", url);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetDial" => {
            let hwnd = emu.regs().rcx;
            let connect_ptr = emu.regs().rdx;
            let conn_id_ptr = emu.regs().r8;
            let flags = emu.regs().r9 as u32;
            let connect = if connect_ptr != 0 {
                emu.maps.read_string(connect_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetDial connect:'{}' flags:0x{:x}",
                connect,
                flags
            );
            if conn_id_ptr != 0 {
                emu.maps.write_dword(conn_id_ptr, 1);
            }
            emu.regs_mut().rax = 0;
        }
        "InternetDialA" => {
            let hwnd = emu.regs().rcx;
            let connect_ptr = emu.regs().rdx;
            let conn_id_ptr = emu.regs().r8;
            let flags = emu.regs().r9 as u32;
            let connect = if connect_ptr != 0 {
                emu.maps.read_string(connect_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetDialA connect:'{}' flags:0x{:x}",
                connect,
                flags
            );
            if conn_id_ptr != 0 {
                emu.maps.write_dword(conn_id_ptr, 1);
            }
            emu.regs_mut().rax = 0;
        }
        "InternetDialW" => {
            let hwnd = emu.regs().rcx;
            let connect_ptr = emu.regs().rdx;
            let conn_id_ptr = emu.regs().r8;
            let flags = emu.regs().r9 as u32;
            let connect = if connect_ptr != 0 {
                emu.maps.read_wide_string(connect_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetDialW connect:'{}' flags:0x{:x}",
                connect,
                flags
            );
            if conn_id_ptr != 0 {
                emu.maps.write_dword(conn_id_ptr, 1);
            }
            emu.regs_mut().rax = 0;
        }
        "InternetEnumPerSiteCookieDecisionA" => {
            let buf_ptr = emu.regs().rcx;
            let buf_size = emu.regs().rdx;
            let index_ptr = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetEnumPerSiteCookieDecisionA buf:0x{:x} idx:0x{:x}",
                buf_ptr,
                index_ptr
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(259);
            emu.regs_mut().rax = 0;
        }
        "InternetEnumPerSiteCookieDecisionW" => {
            let buf_ptr = emu.regs().rcx;
            let buf_size = emu.regs().rdx;
            let index_ptr = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetEnumPerSiteCookieDecisionW buf:0x{:x} idx:0x{:x}",
                buf_ptr,
                index_ptr
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(259);
            emu.regs_mut().rax = 0;
        }
        "InternetFindNextFileA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!InternetFindNextFileA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetFindNextFileW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!InternetFindNextFileW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetFortezzaCommand" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "wininet!InternetFortezzaCommand arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetFreeCookies" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "wininet!InternetFreeCookies handle:0x{:x}", handle);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetFreeProxyInfoList" => {
            let handle = emu.regs().rcx;
            log_red!(
                emu,
                "wininet!InternetFreeProxyInfoList handle:0x{:x}",
                handle
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetGetCertByURL" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetGetCertByURL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetGetCertByURLA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!InternetGetCertByURLA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetGetConnectedState" => {
            let flags_ptr = emu.regs().rcx;
            log_red!(emu, "wininet!InternetGetConnectedState");
            if flags_ptr != 0 {
                emu.maps.write_dword(flags_ptr, 0x20);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetConnectedStateEx" => {
            let flags_ptr = emu.regs().rcx;
            log_red!(emu, "wininet!InternetGetConnectedStateEx");
            if flags_ptr != 0 {
                emu.maps.write_dword(flags_ptr, 0x20);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetConnectedStateExA" => {
            let flags_ptr = emu.regs().rcx;
            log_red!(emu, "wininet!InternetGetConnectedStateExA");
            if flags_ptr != 0 {
                emu.maps.write_dword(flags_ptr, 0x20);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetConnectedStateExW" => {
            let flags_ptr = emu.regs().rcx;
            log_red!(emu, "wininet!InternetGetConnectedStateExW");
            if flags_ptr != 0 {
                emu.maps.write_dword(flags_ptr, 0x20);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetCookieA" => {
            let url_ptr = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let data_ptr = emu.regs().r8;
            let size_ptr = emu.regs().r9;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetGetCookieA url:'{}'", url);
            if size_ptr != 0 {
                emu.maps.write_dword(size_ptr, 0);
            }
            emu.set_last_error(259);
            emu.regs_mut().rax = 0;
        }
        "InternetGetCookieEx2" => {
            let url_ptr = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let data_ptr = emu.regs().r8;
            let size_ptr = emu.regs().r9;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetGetCookieEx2 url:'{}'", url);
            if size_ptr != 0 {
                emu.maps.write_dword(size_ptr, 0);
            }
            emu.set_last_error(259);
            emu.regs_mut().rax = 0;
        }
        "InternetGetCookieExA" => {
            let url_ptr = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let data_ptr = emu.regs().r8;
            let size_ptr = emu.regs().r9;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetGetCookieExA url:'{}'", url);
            if size_ptr != 0 {
                emu.maps.write_dword(size_ptr, 0);
            }
            emu.set_last_error(259);
            emu.regs_mut().rax = 0;
        }
        "InternetGetCookieExW" => {
            let url_ptr = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let data_ptr = emu.regs().r8;
            let size_ptr = emu.regs().r9;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetGetCookieExW url:'{}'", url);
            if size_ptr != 0 {
                emu.maps.write_dword(size_ptr, 0);
            }
            emu.set_last_error(259);
            emu.regs_mut().rax = 0;
        }
        "InternetGetCookieW" => {
            let url_ptr = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let data_ptr = emu.regs().r8;
            let size_ptr = emu.regs().r9;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetGetCookieW url:'{}'", url);
            if size_ptr != 0 {
                emu.maps.write_dword(size_ptr, 0);
            }
            emu.set_last_error(259);
            emu.regs_mut().rax = 0;
        }
        "InternetGetLastResponseInfoA" => {
            let error_ptr = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            let buf_len_ptr = emu.regs().r8;
            log_red!(emu, "wininet!InternetGetLastResponseInfoA");
            if error_ptr != 0 {
                emu.maps.write_dword(error_ptr, 0);
            }
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetLastResponseInfoW" => {
            let error_ptr = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            let buf_len_ptr = emu.regs().r8;
            log_red!(emu, "wininet!InternetGetLastResponseInfoW");
            if error_ptr != 0 {
                emu.maps.write_dword(error_ptr, 0);
            }
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetPerSiteCookieDecisionA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetGetPerSiteCookieDecisionA arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "InternetGetPerSiteCookieDecisionW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetGetPerSiteCookieDecisionW arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "InternetGetProxyForUrl" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "wininet!InternetGetProxyForUrl arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetGetSecurityInfoByURL" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetGetSecurityInfoByURL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetGetSecurityInfoByURLA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!InternetGetSecurityInfoByURLA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetGetSecurityInfoByURLW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!InternetGetSecurityInfoByURLW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetGoOnline" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetGoOnline arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetGoOnlineA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetGoOnlineA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetGoOnlineW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetGoOnlineW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetHangUp" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetHangUp arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetInitializeAutoProxyDll" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetInitializeAutoProxyDll arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let handle = helper::handler_create("InternetInitializeAutoProxyDll");
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetLockRequestFile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetLockRequestFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            if arg1 != 0 && !helper::handler_exist(arg1) {
                emu.set_last_error(12019);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetOpenUrlA" => {
            let inet_handle = emu.regs().rcx;
            let url_ptr = emu.regs().rdx;
            let headers_ptr = emu.regs().r8;
            let headers_len = emu.regs().r9 as u32;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetOpenUrlA inet:0x{:x} url:'{}'",
                inet_handle,
                url
            );
            let uri = format!("InternetOpenUrlA://{}", url);
            let handle = helper::handler_create(&uri);
            emu.regs_mut().rax = handle;
        }
        "InternetOpenUrlW" => {
            let inet_handle = emu.regs().rcx;
            let url_ptr = emu.regs().rdx;
            let headers_ptr = emu.regs().r8;
            let headers_len = emu.regs().r9 as u32;
            let url = if url_ptr != 0 {
                emu.maps.read_wide_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetOpenUrlW inet:0x{:x} url:'{}'",
                inet_handle,
                url
            );
            let uri = format!("InternetOpenUrlW://{}", url);
            let handle = helper::handler_create(&uri);
            emu.regs_mut().rax = handle;
        }
        "InternetQueryDataAvailable" => {
            let handle = emu.regs().rcx;
            let bytes_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetQueryDataAvailable handle:0x{:x}",
                handle
            );
            if bytes_ptr != 0 {
                emu.maps.write_dword(bytes_ptr, 0);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetQueryFortezzaStatus" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetQueryFortezzaStatus arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetQueryOptionA" => InternetQueryOptionA(emu),
        "InternetQueryOptionW" => InternetQueryOptionW(emu),
        "InternetSecurityProtocolToStringA" => {
            let protocol = emu.regs().rcx as u32;
            let buf_ptr = emu.regs().rdx;
            let buf_len_ptr = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetSecurityProtocolToStringA protocol:{}",
                protocol
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            if buf_ptr != 0 {
                emu.maps.write_string(buf_ptr, "TLS 1.2");
            }
            emu.regs_mut().rax = 1;
        }
        "InternetSecurityProtocolToStringW" => {
            let protocol = emu.regs().rcx as u32;
            let buf_ptr = emu.regs().rdx;
            let buf_len_ptr = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetSecurityProtocolToStringW protocol:{}",
                protocol
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            if buf_ptr != 0 {
                emu.maps.write_string(buf_ptr, "TLS 1.2");
            }
            emu.regs_mut().rax = 1;
        }
        "InternetSetCookieA" => {
            let url_ptr = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let data_ptr = emu.regs().r8;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            let data = if data_ptr != 0 {
                emu.maps.read_string(data_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetSetCookieA url:'{}' name:'{}' data:'{}'",
                url,
                name,
                data
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetCookieEx2" => {
            let url_ptr = emu.regs().rcx;
            let cookie_ptr = emu.regs().rdx;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            let cookie = if cookie_ptr != 0 {
                emu.maps.read_string(cookie_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetSetCookieEx2 url:'{}' cookie:'{}'",
                url,
                cookie
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // COOKIE_STATE_ACCEPT
        }
        "InternetSetCookieExA" => {
            let url_ptr = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let data_ptr = emu.regs().r8;
            let flags = emu.regs().r9;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetSetCookieExA url:'{}' name:'{}' flags:0x{:x}",
                url,
                name,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetCookieExW" => {
            let url_ptr = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let data_ptr = emu.regs().r8;
            let flags = emu.regs().r9;
            let url = if url_ptr != 0 {
                emu.maps.read_wide_string(url_ptr)
            } else {
                String::new()
            };
            let name = if name_ptr != 0 {
                emu.maps.read_wide_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetSetCookieExW url:'{}' name:'{}' flags:0x{:x}",
                url,
                name,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetCookieW" => {
            let url_ptr = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let data_ptr = emu.regs().r8;
            let url = if url_ptr != 0 {
                emu.maps.read_wide_string(url_ptr)
            } else {
                String::new()
            };
            let name = if name_ptr != 0 {
                emu.maps.read_wide_string(name_ptr)
            } else {
                String::new()
            };
            let data = if data_ptr != 0 {
                emu.maps.read_wide_string(data_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "wininet!InternetSetCookieW url:'{}' name:'{}' data:'{}'",
                url,
                name,
                data
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetDialState" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetSetDialState arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetDialStateA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetSetDialStateA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetDialStateW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!InternetSetDialStateW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetFilePointer" => {
            let handle = emu.regs().rcx;
            let distance = emu.regs().rdx as i32;
            let method = emu.regs().r8 as u32;
            log_red!(
                emu,
                "wininet!InternetSetFilePointer handle:0x{:x} dist:{} method:{}",
                handle,
                distance,
                method
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "InternetSetOptionExA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "wininet!InternetSetOptionExA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetOptionExW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "wininet!InternetSetOptionExW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetPerSiteCookieDecisionA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetSetPerSiteCookieDecisionA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetPerSiteCookieDecisionW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetSetPerSiteCookieDecisionW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetSecureLegacyServersAppCompat" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetSetSecureLegacyServersAppCompat arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetSetStatusCallback"
        | "InternetSetStatusCallbackA"
        | "InternetSetStatusCallbackW" => {
            let handle = emu.regs().rcx;
            let callback = emu.regs().rdx;
            let valid = helper::handler_exist(handle);
            log_red!(
                emu,
                "wininet!InternetSetStatusCallback handle: 0x{:x} callback: 0x{:x} valid: {}",
                handle,
                callback,
                valid
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "InternetShowSecurityInfoByURL" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetShowSecurityInfoByURL arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "InternetShowSecurityInfoByURLA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetShowSecurityInfoByURLA arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "InternetShowSecurityInfoByURLW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetShowSecurityInfoByURLW arg1:0x{:x}",
                arg1
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "InternetTimeFromSystemTime" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetTimeFromSystemTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetTimeFromSystemTimeA" => {
            let time_ptr = emu.regs().rcx;
            let format = emu.regs().rdx as u32;
            let buf_ptr = emu.regs().r8;
            log_red!(emu, "wininet!InternetTimeFromSystemTimeA format:{}", format);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            if buf_ptr != 0 {
                emu.maps
                    .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetTimeFromSystemTimeW" => {
            let time_ptr = emu.regs().rcx;
            let format = emu.regs().rdx as u32;
            let buf_ptr = emu.regs().r8;
            log_red!(emu, "wininet!InternetTimeFromSystemTimeW format:{}", format);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            if buf_ptr != 0 {
                emu.maps
                    .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetTimeToSystemTime" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!InternetTimeToSystemTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetTimeToSystemTimeA" => {
            let time_ptr = emu.regs().rcx;
            let format = emu.regs().rdx as u32;
            let buf_ptr = emu.regs().r8;
            log_red!(emu, "wininet!InternetTimeToSystemTimeA format:{}", format);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            if buf_ptr != 0 {
                emu.maps
                    .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetTimeToSystemTimeW" => {
            let time_ptr = emu.regs().rcx;
            let format = emu.regs().rdx as u32;
            let buf_ptr = emu.regs().r8;
            log_red!(emu, "wininet!InternetTimeToSystemTimeW format:{}", format);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            if buf_ptr != 0 {
                emu.maps
                    .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetUnlockRequestFile" => {
            let handle = emu.regs().rcx;
            log_red!(
                emu,
                "wininet!InternetUnlockRequestFile handle:0x{:x}",
                handle
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetWriteFile" => {
            let handle = emu.regs().rcx;
            let buffer = emu.regs().rdx;
            let bytes_to_write = emu.regs().r8 as u32;
            let bytes_written_ptr = emu.regs().r9;
            log_red!(
                emu,
                "wininet!InternetWriteFile handle:0x{:x} buf:0x{:x} sz:{}",
                handle,
                buffer,
                bytes_to_write
            );
            if bytes_written_ptr != 0 {
                emu.maps.write_dword(bytes_written_ptr, bytes_to_write);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetWriteFileExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!InternetWriteFileExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternetWriteFileExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!InternetWriteFileExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsHostInProxyBypassList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!IsHostInProxyBypassList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsUrlCacheEntryExpiredA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!IsUrlCacheEntryExpiredA '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsUrlCacheEntryExpiredW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!IsUrlCacheEntryExpiredW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadUrlCacheContent" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "wininet!LoadUrlCacheContent handle:0x{:x}", handle);
            let handle = helper::handler_create("LoadUrlCacheContent");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ParseX509EncodedCertificateForListBoxEntry" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!ParseX509EncodedCertificateForListBoxEntry arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrivacyGetZonePreferenceW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!PrivacyGetZonePreferenceW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrivacySetZonePreferenceW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!PrivacySetZonePreferenceW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadUrlCacheEntryStream" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!ReadUrlCacheEntryStream h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadUrlCacheEntryStreamEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!ReadUrlCacheEntryStreamEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterUrlCacheNotification" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "wininet!RegisterUrlCacheNotification arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let handle = helper::handler_create("RegisterUrlCacheNotification");
            let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ResumeSuspendedDownload" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!ResumeSuspendedDownload arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RetrieveUrlCacheEntryFileA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!RetrieveUrlCacheEntryFileA '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RetrieveUrlCacheEntryFileW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!RetrieveUrlCacheEntryFileW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RetrieveUrlCacheEntryStreamA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!RetrieveUrlCacheEntryStreamA '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RetrieveUrlCacheEntryStreamW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!RetrieveUrlCacheEntryStreamW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RunOnceUrlCache" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!RunOnceUrlCache arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheConfigInfoA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!SetUrlCacheConfigInfoA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheConfigInfoW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!SetUrlCacheConfigInfoW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheEntryGroup" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryGroup h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheEntryGroupA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryGroupA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheEntryGroupW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryGroupW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheEntryInfoA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryInfoA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheEntryInfoW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryInfoW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheGroupAttributeA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!SetUrlCacheGroupAttributeA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheGroupAttributeW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!SetUrlCacheGroupAttributeW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheHeaderData" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!SetUrlCacheHeaderData h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowCertificate" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!ShowCertificate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowClientAuthCerts" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!ShowClientAuthCerts arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowSecurityInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!ShowSecurityInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ShowX509EncodedCertificate" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!ShowX509EncodedCertificate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnlockUrlCacheEntryFile" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UnlockUrlCacheEntryFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnlockUrlCacheEntryFileA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UnlockUrlCacheEntryFileA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnlockUrlCacheEntryFileW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UnlockUrlCacheEntryFileW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnlockUrlCacheEntryStream" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UnlockUrlCacheEntryStream arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateUrlCacheContentPath" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UpdateUrlCacheContentPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheCheckEntriesExist" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheCheckEntriesExist arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "UrlCacheCloseEntryHandle" => {
            let handle = emu.regs().rcx;
            log_red!(
                emu,
                "wininet!UrlCacheCloseEntryHandle handle:0x{:x}",
                handle
            );
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheContainerSetEntryMaximumAge" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!UrlCacheContainerSetEntryMaximumAge arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheCreateContainer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheCreateContainer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("UrlCacheCreateContainer");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheFindFirstEntry" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheFindFirstEntry arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheFindNextEntry" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheFindNextEntry arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheFreeEntryInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheFreeEntryInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheFreeGlobalSpace" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheFreeGlobalSpace arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheGetContentPaths" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheGetContentPaths arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheGetEntryInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheGetEntryInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheGetGlobalCacheSize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheGetGlobalCacheSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheGetGlobalLimit" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheGetGlobalLimit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheReadEntryStream" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!UrlCacheReadEntryStream arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "UrlCacheReloadSettings" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheReloadSettings arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheRetrieveEntryFile" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheRetrieveEntryFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheRetrieveEntryStream" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!UrlCacheRetrieveEntryStream arg1:0x{:x}", arg1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "UrlCacheServer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheSetGlobalLimit" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!UrlCacheSetGlobalLimit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlCacheUpdateEntryExtraData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "wininet!UrlCacheUpdateEntryExtraData arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UrlZonesDetach" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "wininet!UrlZonesDetach arg1:0x{:x}", arg1);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
        }
        "_GetFileExtensionFromUrl" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "wininet!_GetFileExtensionFromUrl h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

lazy_static! {
    static ref COUNT_RECEIVE: Mutex<u32> = Mutex::new(0);
}

pub fn InternetOpenA(emu: &mut emu::Emu) {
    let uagent_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let proxy_ptr = emu.regs().r8;
    let proxybypass_ptr = emu.regs().r9;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let mut uagent = "".to_string();
    let mut proxy = "".to_string();
    let mut proxy_bypass = "".to_string();

    if uagent_ptr != 0 {
        uagent = emu.maps.read_string(uagent_ptr);
    }
    if proxy_ptr != 0 {
        proxy = emu.maps.read_string(proxy_ptr);
    }
    if proxybypass_ptr != 0 {
        proxy_bypass = emu.maps.read_string(proxybypass_ptr);
    }

    log_red!(
        emu,
        "wininet!InternetOpenA uagent: {} proxy: {} {}",
        uagent,
        proxy,
        proxy_bypass
    );

    /*
    if emu.cfg.endpoint {
        // endpoint mode
        if uagent_ptr != 0 && uagent != "" {
            endpoint::http_set_headers("User-Agent", &uagent);
        }
    }*/

    let uri = format!("InternetOpenA://{}", uagent);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub fn InternetOpenW(emu: &mut emu::Emu) {
    let uagent_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let proxy_ptr = emu.regs().r8;
    let proxybypass_ptr = emu.regs().r9;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let mut uagent = "".to_string();
    let mut proxy = "".to_string();
    let mut proxy_bypass = "".to_string();

    if uagent_ptr != 0 {
        uagent = emu.maps.read_wide_string(uagent_ptr);
    }
    if proxy_ptr != 0 {
        proxy = emu.maps.read_wide_string(proxy_ptr);
    }
    if proxybypass_ptr != 0 {
        proxy_bypass = emu.maps.read_wide_string(proxybypass_ptr);
    }

    log_red!(
        emu,
        "wininet!InternetOpenW uagent: {} proxy: {} {}",
        uagent,
        proxy,
        proxy_bypass
    );

    /*
    if emu.cfg.endpoint {
        // endpoint mode
        if uagent_ptr != 0 && uagent != "" {
            endpoint::http_set_headers("User-Agent", &uagent.replace("\x00", ""));
        }
    }*/

    emu.regs_mut().rax = helper::handler_create("InternetOpenW://"); // internet handle
}

pub fn InternetConnectA(emu: &mut emu::Emu) {
    let internet_hndl = emu.regs().rcx;
    let server_ptr = emu.regs().rdx;
    let port = emu.regs().r8;
    let login_ptr = emu.regs().r9;
    let passw_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let service = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let ctx = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    let mut server = "".to_string();
    let mut login = "".to_string();
    let mut passw = "".to_string();

    if server_ptr != 0 {
        server = emu.maps.read_string(server_ptr);
    }
    if login_ptr != 0 {
        login = emu.maps.read_string(login_ptr);
    }
    if passw_ptr != 0 {
        passw = emu.maps.read_string(passw_ptr);
    }

    log_red!(
        emu,
        "wininet!InternetConnectA host: {} port: {} login: {} passw: {}",
        server,
        port,
        login,
        passw
    );

    if !helper::handler_exist(internet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_serverport(&server, port as u16);
    }*/

    let uri = format!("InternetConnectA://{}:{}", server, port);
    emu.regs_mut().rax = helper::handler_create(&uri); // connect handle
}

pub fn InternetConnectW(emu: &mut emu::Emu) {
    let internet_hndl = emu.regs().rcx;
    let server_ptr = emu.regs().rdx;
    let port = emu.regs().r8;
    let login_ptr = emu.regs().r9;
    let passw_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let service = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let ctx = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    let mut server = "".to_string();
    let mut login = "".to_string();
    let mut passw = "".to_string();

    if server_ptr != 0 {
        server = emu.maps.read_wide_string(server_ptr);
    }
    if login_ptr != 0 {
        login = emu.maps.read_wide_string(login_ptr);
    }
    if passw_ptr != 0 {
        passw = emu.maps.read_wide_string(passw_ptr);
    }

    log_red!(
        emu,
        "wininet!InternetConnectW host: {} port: {} login: {} passw: {}",
        server,
        port,
        login,
        passw
    );

    if !helper::handler_exist(internet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_serverport(&server.replace("\x00", ""), port as u16);
    }*/

    let uri = format!("InternetConnectW://{}:{}", server, port);
    emu.regs_mut().rax = helper::handler_create(&uri); // connect handle
}

fn HttpOpenRequestA(emu: &mut emu::Emu) {
    let conn_hndl = emu.regs().rcx;
    let method_ptr = emu.regs().rdx;
    let path_ptr = emu.regs().r8;
    let version_ptr = emu.regs().r9;
    let referrer_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let access_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let ctx = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    let mut method = "".to_string();
    let mut path = "".to_string();
    let mut version = "".to_string();
    let mut referrer = "".to_string();
    let mut access = "".to_string();

    if method_ptr != 0 {
        method = emu.maps.read_string(method_ptr);
    }
    if path_ptr != 0 {
        path = emu.maps.read_string(path_ptr);
    }
    if version_ptr != 0 {
        version = emu.maps.read_string(version_ptr);
    }
    if referrer_ptr != 0 {
        referrer = emu.maps.read_string(referrer_ptr);
    }
    if access_ptr != 0 {
        access = emu.maps.read_string(access_ptr);
    }

    log_red!(
        emu,
        "wininet!HttpOpenRequestA method: {} path: {} ver: {} ref: {} access: {}",
        method,
        path,
        version,
        referrer,
        access
    );

    if !helper::handler_exist(conn_hndl) {
        log::trace!("\tinvalid handle.");
    }

    if flags & constants::INTERNET_FLAG_SECURE != 0 {
        log::trace!("\tssl communication.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_path(&path);
        if flags & constants::INTERNET_FLAG_SECURE == 1 {
            endpoint::http_set_ssl();
        }

        if method_ptr != 0 {
            if method == "" {
                endpoint::http_set_method("get");
            } else {
                endpoint::http_set_method(&method);
            }
        } else {
            endpoint::http_set_method("get");
        }
    }*/

    let uri = format!("HttpOpenRequestA://{}", path);
    emu.regs_mut().rax = helper::handler_create(&uri); // request handle
}

fn HttpOpenRequestW(emu: &mut emu::Emu) {
    let conn_hndl = emu.regs().rcx;
    let method_ptr = emu.regs().rdx;
    let path_ptr = emu.regs().r8;
    let version_ptr = emu.regs().r9;
    let referrer_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let access_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let ctx = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    let mut method = "".to_string();
    let mut path = "".to_string();
    let mut version = "".to_string();
    let mut referrer = "".to_string();
    let mut access = "".to_string();
    if method_ptr != 0 {
        method = emu.maps.read_wide_string(method_ptr);
    }
    if path_ptr != 0 {
        path = emu.maps.read_wide_string(path_ptr);
    }
    if version_ptr != 0 {
        version = emu.maps.read_wide_string(version_ptr);
    }
    if referrer_ptr != 0 {
        referrer = emu.maps.read_wide_string(referrer_ptr);
    }
    if access_ptr != 0 {
        access = emu.maps.read_wide_string(access_ptr);
    }

    log_red!(
        emu,
        "wininet!HttpOpenRequestW method: {} path: {} ver: {} ref: {} access: {}",
        method,
        path,
        version,
        referrer,
        access
    );

    if !helper::handler_exist(conn_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_path(&path);
        if flags & constants::INTERNET_FLAG_SECURE == 1 {
            endpoint::http_set_ssl();
        }

        if method_ptr != 0 {
            if method.len() < 3 {
                endpoint::http_set_method("get");
            } else {
                endpoint::http_set_method(&method.replace("\x00", ""));
            }
        } else {
            endpoint::http_set_method("get");
        }
    }*/

    let uri = format!("HttpOpenRequestW://{}", path);
    emu.regs_mut().rax = helper::handler_create(&uri); // request handle
}

fn InternetSetOptionA(emu: &mut emu::Emu) {
    let inet_hndl = emu.regs().rcx;
    let option = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let len = emu.regs().r9;

    let mut buffer_content = "".to_string();
    if buffer != 0 {
        buffer_content = emu.maps.read_string_of_bytes(buffer, len as usize);
    }
    let sbuff = emu.maps.read_string(buffer);

    log_red!(
        emu,
        "wininet!InternetSetOptionA option: 0x{:x} buff: {{{}}} {}",
        option,
        buffer_content,
        sbuff
    );

    if !helper::handler_exist(inet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}

fn InternetSetOptionW(emu: &mut emu::Emu) {
    let inet_hndl = emu.regs().rcx;
    let option = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let len = emu.regs().r9;

    let mut buffer_content = "".to_string();
    if buffer != 0 {
        buffer_content = emu.maps.read_string_of_bytes(buffer, len as usize);
    }
    let sbuff = emu.maps.read_wide_string(buffer);

    log_red!(
        emu,
        "wininet!InternetSetOptionW option: 0x{:x} buff: {{{}}} {}",
        option,
        buffer_content,
        sbuff
    );

    if !helper::handler_exist(inet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}

fn HttpSendRequestA(emu: &mut emu::Emu) {
    let req_hndl = emu.regs().rcx;
    let hdrs_ptr = emu.regs().rdx;
    let hdrs_len = emu.regs().r8;
    let opt_ptr = emu.regs().r9;
    let opt_len = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let hdrs = emu.maps.read_string(hdrs_ptr);
    let opt = emu.maps.read_string(opt_ptr);

    log_red!(emu, "wininet!HttpSendRequestA hdrs: {} opt: {}", hdrs, opt);

    if !helper::handler_exist(req_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_headers_str(&hdrs);
        endpoint::http_send_request();
    }*/

    emu.regs_mut().rax = 1; // true
}

fn HttpSendRequestW(emu: &mut emu::Emu) {
    let req_hndl = emu.regs().rcx;
    let hdrs_ptr = emu.regs().rdx;
    let hdrs_len = emu.regs().r8;
    let opt_ptr = emu.regs().r9;
    let opt_len = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let hdrs = emu.maps.read_wide_string(hdrs_ptr);
    let opt = emu.maps.read_wide_string(opt_ptr);

    log_red!(emu, "wininet!HttpSendRequestW hdrs: {} opt: {}", hdrs, opt);

    if !helper::handler_exist(req_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_headers_str(&hdrs.replace("\x00", ""));
        endpoint::http_send_request();
    }*/

    emu.regs_mut().rax = 1; // true
}

fn InternetErrorDlg(emu: &mut emu::Emu) {
    let err = emu.regs().rcx;

    log_red!(emu, "wininet!InternetErrorDlg err: {}", err);

    emu.regs_mut().rax = 0;
}

fn InternetReadFile(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    let bytes_to_read = emu.regs().r8;
    let bytes_read_ptr = emu.regs().r9;

    log_red!(
        emu,
        "wininet!InternetReadFile sz: {} buff: 0x{:x}",
        bytes_to_read,
        buff_ptr
    );

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }

    if emu.cfg.endpoint {
        /*
        let buff = endpoint::http_read_data();
        emu.maps.write_buffer(buff_ptr, &buff);
        emu.maps.write_dword(bytes_read_ptr, buff.len() as u32);
        */
    } else {
        let mut count = COUNT_RECEIVE.lock().unwrap();
        *count += 1;

        if *count < 3 {
            emu.maps.write_spaced_bytes(buff_ptr, "90 90 90 90");
            emu.maps.write_dword(bytes_read_ptr, bytes_to_read as u32);
        } else {
            emu.maps.write_dword(bytes_read_ptr, 0);
        }
    }

    emu.regs_mut().rax = 1; // true
}

fn InternetReadFileExA(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    let flags = emu.regs().r8;
    let ctx = emu.regs().r9;

    log_red!(emu, "wininet!InternetReadFileExA buff: 0x{:x}", buff_ptr);

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}

fn InternetReadFileExW(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let _ctx = emu.regs().r9;

    log_red!(emu, "wininet!InternetReadFileExW buff: 0x{:x}", buff_ptr);

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}

fn HttpQueryInfoA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let info_level = emu.regs().rdx as u32;
    let buffer = emu.regs().r8;
    let buffer_len_ptr = emu.regs().r9;
    let level_raw = info_level & 0x0000FFFF;

    log_red!(
        emu,
        "wininet!HttpQueryInfoA handle: 0x{:x} level: 0x{:x} (raw: {})",
        handle,
        info_level,
        level_raw
    );

    let response: Option<&str> = match level_raw {
        1 => Some("text/html"),            // HTTP_QUERY_CONTENT_TYPE
        5 => Some("1024"),                 // HTTP_QUERY_CONTENT_LENGTH
        19 => Some("200"),                 // HTTP_QUERY_STATUS_CODE
        21 => Some("HTTP/1.1 200 OK"),     // HTTP_QUERY_RAW_HEADERS
        22 => Some("HTTP/1.1 200 OK\r\n"), // HTTP_QUERY_RAW_HEADERS_CRLF
        _ => None,
    };

    match response {
        Some(val) => {
            if buffer != 0 {
                emu.maps.write_string(buffer, val);
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, val.len() as u32);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        None => {
            log_red!(emu, "  unsupported info level {}", level_raw);
            emu.regs_mut().rax = 0; // FALSE
        }
    }
}

fn HttpQueryInfoW(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let info_level = emu.regs().rdx as u32;
    let buffer = emu.regs().r8;
    let buffer_len_ptr = emu.regs().r9;
    let level_raw = info_level & 0x0000FFFF;

    log_red!(
        emu,
        "wininet!HttpQueryInfoW handle: 0x{:x} level: 0x{:x} (raw: {})",
        handle,
        info_level,
        level_raw
    );

    let response: Option<&str> = match level_raw {
        1 => Some("text/html"),
        5 => Some("1024"),
        19 => Some("200"),
        21 => Some("HTTP/1.1 200 OK"),
        22 => Some("HTTP/1.1 200 OK\r\n"),
        _ => None,
    };

    match response {
        Some(val) => {
            if buffer != 0 {
                emu.maps.write_wide_string(buffer, val);
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, (val.len() * 2) as u32);
            }
            emu.regs_mut().rax = 1;
        }
        None => {
            emu.regs_mut().rax = 0;
        }
    }
}

fn InternetQueryOptionA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let option = emu.regs().rdx as u32;
    let buffer = emu.regs().r8;
    let buffer_len_ptr = emu.regs().r9;

    log_red!(
        emu,
        "wininet!InternetQueryOptionA handle: 0x{:x} option: {}",
        handle,
        option
    );

    match option {
        34 => {
            // INTERNET_OPTION_URL
            let url = "http://emulated.local/";
            if buffer != 0 {
                emu.maps.write_string(buffer, url);
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, url.len() as u32);
            }
            emu.regs_mut().rax = 1;
        }
        37 => {
            // INTERNET_OPTION_HANDLE_TYPE
            if buffer != 0 {
                emu.maps.write_dword(buffer, 1); // INTERNET_HANDLE_TYPE_INTERNET
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, 4);
            }
            emu.regs_mut().rax = 1;
        }
        _ => {
            log_red!(emu, "  unsupported option {}", option);
            emu.regs_mut().rax = 0;
        }
    }
}

fn InternetQueryOptionW(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let option = emu.regs().rdx as u32;
    let buffer = emu.regs().r8;
    let buffer_len_ptr = emu.regs().r9;

    log_red!(
        emu,
        "wininet!InternetQueryOptionW handle: 0x{:x} option: {}",
        handle,
        option
    );

    match option {
        34 => {
            let url = "http://emulated.local/";
            if buffer != 0 {
                emu.maps.write_wide_string(buffer, url);
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, (url.len() * 2) as u32);
            }
            emu.regs_mut().rax = 1;
        }
        37 => {
            if buffer != 0 {
                emu.maps.write_dword(buffer, 1);
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, 4);
            }
            emu.regs_mut().rax = 1;
        }
        _ => {
            emu.regs_mut().rax = 0;
        }
    }
}

fn InternetCrackUrlA(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let url_len = emu.regs().rdx as u32;
    let _flags = emu.regs().r8;
    let components_ptr = emu.regs().r9;

    let url = if url_ptr != 0 {
        if url_len == 0 {
            emu.maps.read_string(url_ptr)
        } else {
            emu.maps.read_string_of_bytes(url_ptr, url_len as usize)
        }
    } else {
        String::new()
    };

    log_red!(emu, "wininet!InternetCrackUrlA url: '{}'", url);

    if components_ptr == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    // Parse simple URL: scheme://host:port/path
    let (scheme, rest) = if let Some(pos) = url.find("://") {
        (&url[..pos], &url[pos + 3..])
    } else {
        ("http", url.as_str())
    };

    let (host_port, path) = if let Some(pos) = rest.find('/') {
        (&rest[..pos], &rest[pos..])
    } else {
        (rest, "/")
    };

    let (host, port) = if let Some(pos) = host_port.find(':') {
        (
            &host_port[..pos],
            host_port[pos + 1..].parse::<u16>().unwrap_or(80),
        )
    } else {
        (
            host_port,
            match scheme {
                "https" => 443u16,
                "ftp" => 21,
                _ => 80,
            },
        )
    };

    let scheme_id: u32 = match scheme {
        "https" => 2,
        "ftp" => 3,
        _ => 1, // INTERNET_SCHEME_HTTP
    };

    // URL_COMPONENTSA struct layout (64-bit):
    // 0x08: lpszScheme + 0x10: dwSchemeLength
    // 0x18: nScheme
    // 0x20: lpszHostName + 0x28: dwHostNameLength
    // 0x30: nPort
    // 0x58: lpszUrlPath + 0x60: dwUrlPathLength
    let scheme_buf = emu.maps.read_qword(components_ptr + 0x08).unwrap_or(0);
    let host_buf = emu.maps.read_qword(components_ptr + 0x20).unwrap_or(0);
    let path_buf = emu.maps.read_qword(components_ptr + 0x58).unwrap_or(0);

    if scheme_buf != 0 {
        emu.maps.write_string(scheme_buf, scheme);
        emu.maps
            .write_dword(components_ptr + 0x10, scheme.len() as u32);
    }
    emu.maps.write_dword(components_ptr + 0x18, scheme_id);

    if host_buf != 0 {
        emu.maps.write_string(host_buf, host);
        emu.maps
            .write_dword(components_ptr + 0x28, host.len() as u32);
    }
    emu.maps.write_word(components_ptr + 0x30, port);

    if path_buf != 0 {
        emu.maps.write_string(path_buf, path);
        emu.maps
            .write_dword(components_ptr + 0x60, path.len() as u32);
    }

    emu.regs_mut().rax = 1; // TRUE
}

fn InternetCrackUrlW(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let _url_len = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let components_ptr = emu.regs().r9;

    let url = if url_ptr != 0 {
        emu.maps.read_wide_string(url_ptr)
    } else {
        String::new()
    };

    log_red!(emu, "wininet!InternetCrackUrlW url: '{}'", url);

    if components_ptr == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let (scheme, rest) = if let Some(pos) = url.find("://") {
        (&url[..pos], &url[pos + 3..])
    } else {
        ("http", url.as_str())
    };

    let (host_port, path) = if let Some(pos) = rest.find('/') {
        (&rest[..pos], &rest[pos..])
    } else {
        (rest, "/")
    };

    let (host, port) = if let Some(pos) = host_port.find(':') {
        (
            &host_port[..pos],
            host_port[pos + 1..].parse::<u16>().unwrap_or(80),
        )
    } else {
        (
            host_port,
            match scheme {
                "https" => 443u16,
                "ftp" => 21,
                _ => 80,
            },
        )
    };

    let scheme_id: u32 = match scheme {
        "https" => 2,
        "ftp" => 3,
        _ => 1,
    };

    let scheme_buf = emu.maps.read_qword(components_ptr + 0x08).unwrap_or(0);
    let host_buf = emu.maps.read_qword(components_ptr + 0x20).unwrap_or(0);
    let path_buf = emu.maps.read_qword(components_ptr + 0x58).unwrap_or(0);

    if scheme_buf != 0 {
        emu.maps.write_wide_string(scheme_buf, scheme);
        emu.maps
            .write_dword(components_ptr + 0x10, scheme.len() as u32);
    }
    emu.maps.write_dword(components_ptr + 0x18, scheme_id);

    if host_buf != 0 {
        emu.maps.write_wide_string(host_buf, host);
        emu.maps
            .write_dword(components_ptr + 0x28, host.len() as u32);
    }
    emu.maps.write_word(components_ptr + 0x30, port);

    if path_buf != 0 {
        emu.maps.write_wide_string(path_buf, path);
        emu.maps
            .write_dword(components_ptr + 0x60, path.len() as u32);
    }

    emu.regs_mut().rax = 1;
}
