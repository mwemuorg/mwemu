use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;

mod request;
mod session;
mod teardown;
mod url;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "InternetOpenA" => session::internet_open_a(emu),
        "InternetOpenW" => session::internet_open_w(emu),
        "InternetConnectA" => session::internet_connect_a(emu),
        "InternetConnectW" => session::internet_connect_w(emu),
        "HttpOpenRequestA" => request::http_open_request_a(emu),
        "HttpOpenRequestW" => request::http_open_request_w(emu),
        "InternetSetOptionA" => request::internet_set_option_a(emu),
        "InternetSetOptionW" => request::internet_set_option_w(emu),
        "HttpSendRequestA" => request::http_send_request_a(emu),
        "HttpSendRequestW" => request::http_send_request_w(emu),
        "InternetReadFile" => teardown::internet_read_file(emu),
        "InternetErrorDlg" => teardown::internet_error_dlg(emu),
        "InternetReadFileExA" => teardown::internet_read_file_ex_a(emu),
        "InternetReadFileExW" => teardown::internet_read_file_ex_w(emu),
        "HttpQueryInfoA" => request::http_query_info_a(emu),
        "InternetCloseHandle" => teardown::internet_close_handle(emu),
        "InternetCrackUrlA" => url::internet_crack_url_a(emu),
        "InternetCrackUrlW" => url::internet_crack_url_w(emu),
        "AppCacheCheckManifest" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheCheckManifest arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheCloseHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheCloseHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheCreateAndCommitFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheCreateAndCommitFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheDeleteGroup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheDeleteGroup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheDeleteIEGroup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheDeleteIEGroup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheDuplicateHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheDuplicateHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheFinalize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFinalize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheFreeDownloadList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFreeDownloadList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheFreeGroupList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFreeGroupList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheFreeIESpace" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFreeIESpace arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheFreeSpace" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFreeSpace arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetDownloadList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetDownloadList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetFallbackUrl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetFallbackUrl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetGroupList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetGroupList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetIEGroupList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetIEGroupList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheGetManifestUrl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetManifestUrl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppCacheLookup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!AppCacheLookup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CommitUrlCacheEntryA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!CommitUrlCacheEntryA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CommitUrlCacheEntryBinaryBlob" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!CommitUrlCacheEntryBinaryBlob arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "CommitUrlCacheEntryW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!CommitUrlCacheEntryW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateMD5SSOHash" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateMD5SSOHash");
            log_red!(emu, "wininet!CreateMD5SSOHash => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateUrlCacheContainerA" => {
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
                "wininet!CreateUrlCacheContainerA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateUrlCacheContainerW" => {
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
                "wininet!CreateUrlCacheContainerW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateUrlCacheEntryA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!CreateUrlCacheEntryA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateUrlCacheEntryExW" => {
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
                "wininet!CreateUrlCacheEntryExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateUrlCacheEntryW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!CreateUrlCacheEntryW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateUrlCacheGroup" => {
            let flags = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "wininet!CreateUrlCacheGroup flags:0x{:x}", flags);
            emu.regs_mut().rax = 0x10001;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteIE3Cache" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "wininet!DeleteIE3Cache handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteUrlCacheContainerA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DeleteUrlCacheContainerW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DeleteUrlCacheEntry" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "wininet!DeleteUrlCacheEntry handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteUrlCacheEntryA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!DeleteUrlCacheEntryA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteUrlCacheEntryW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!DeleteUrlCacheEntryW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteUrlCacheGroup" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!DeleteUrlCacheGroup handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteWpadCacheForNetworks" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "wininet!DeleteWpadCacheForNetworks handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DetectAutoProxyUrl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!DetectAutoProxyUrl arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DispatchAPICall" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!DispatchAPICall arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DllCanUnloadNow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!DllCanUnloadNow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DllGetClassObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!DllGetClassObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DllInstall" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!DllInstall arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
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
                "wininet!DllRegisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DllUnregisterServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!DllUnregisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FindCloseUrlCache" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FindCloseUrlCache handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FindFirstUrlCacheContainerA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheContainerA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcachecontainera");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FindFirstUrlCacheContainerW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheContainerW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcachecontainerw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FindFirstUrlCacheEntryA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcacheentrya");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FindFirstUrlCacheEntryExA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryExA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcacheentryexa");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FindFirstUrlCacheEntryExW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryExW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcacheentryexw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FindFirstUrlCacheEntryW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcacheentryw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FindFirstUrlCacheGroup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheGroup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("findfirsturlcachegroup");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FindNextUrlCacheContainerA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FindNextUrlCacheContainerA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FindNextUrlCacheContainerW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FindNextUrlCacheContainerW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FindNextUrlCacheEntryA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FindNextUrlCacheEntryA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FindNextUrlCacheEntryExA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FindNextUrlCacheEntryExA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FindNextUrlCacheEntryExW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FindNextUrlCacheEntryExW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FindNextUrlCacheEntryW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FindNextUrlCacheEntryW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FindNextUrlCacheGroup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FindNextUrlCacheGroup arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ForceNexusLookup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!ForceNexusLookup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ForceNexusLookupExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "wininet!ForceNexusLookupExW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FreeUrlCacheSpaceA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "wininet!FreeUrlCacheSpaceA '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FreeUrlCacheSpaceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "wininet!FreeUrlCacheSpaceW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FtpCommandA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpCommandA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpCommandW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpCommandW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpCreateDirectoryA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpCreateDirectoryA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpCreateDirectoryW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpCreateDirectoryW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpDeleteFileA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpDeleteFileA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpDeleteFileW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpDeleteFileW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpFindFirstFileA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpFindFirstFileA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpFindFirstFileW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpFindFirstFileW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpGetCurrentDirectoryA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dir_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dir_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpGetCurrentDirectoryA handle:0x{:x}", handle);
            if dir_ptr != 0 {
                emu.maps.write_string(dir_ptr, "/");
            }
            if dir_len_ptr != 0 {
                emu.maps.write_dword(dir_len_ptr, 1);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpGetCurrentDirectoryW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dir_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dir_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpGetCurrentDirectoryW handle:0x{:x}", handle);
            if dir_ptr != 0 {
                emu.maps.write_string(dir_ptr, "/");
            }
            if dir_len_ptr != 0 {
                emu.maps.write_dword(dir_len_ptr, 1);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpGetFileA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpGetFileA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpGetFileEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!FtpGetFileEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FtpGetFileSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "wininet!FtpGetFileSize arg1:0x{:x}", arg1);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpGetFileW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpGetFileW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpOpenFileA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpOpenFileA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpOpenFileW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpOpenFileW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpPutFileA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpPutFileA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpPutFileEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!FtpPutFileEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FtpPutFileW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpPutFileW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpRemoveDirectoryA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpRemoveDirectoryA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpRemoveDirectoryW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpRemoveDirectoryW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpRenameFileA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpRenameFileA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpRenameFileW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpRenameFileW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpSetCurrentDirectoryA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpSetCurrentDirectoryA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FtpSetCurrentDirectoryW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!FtpSetCurrentDirectoryW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetProxyDllInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!GetProxyDllInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetUrlCacheConfigInfoA" => {
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
                "wininet!GetUrlCacheConfigInfoA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetUrlCacheConfigInfoW" => {
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
                "wininet!GetUrlCacheConfigInfoW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetUrlCacheEntryBinaryBlob" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheEntryBinaryBlob h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetUrlCacheEntryInfoA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GetUrlCacheEntryInfoA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetUrlCacheEntryInfoExA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GetUrlCacheEntryInfoExA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetUrlCacheEntryInfoExW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GetUrlCacheEntryInfoExW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetUrlCacheEntryInfoW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GetUrlCacheEntryInfoW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetUrlCacheGroupAttributeA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GetUrlCacheGroupAttributeA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetUrlCacheGroupAttributeW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GetUrlCacheGroupAttributeW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetUrlCacheHeaderData" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheHeaderData h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GopherCreateLocatorA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherCreateLocatorA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GopherCreateLocatorW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherCreateLocatorW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GopherFindFirstFileA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherFindFirstFileA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GopherFindFirstFileW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherFindFirstFileW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GopherGetAttributeA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherGetAttributeA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GopherGetAttributeW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherGetAttributeW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GopherGetLocatorTypeA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherGetLocatorTypeA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GopherGetLocatorTypeW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherGetLocatorTypeW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GopherOpenFileA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherOpenFileA arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GopherOpenFileW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!GopherOpenFileW arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "HttpAddRequestHeadersA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hdrs_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let _hdrs_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _modifiers = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let hdrs = read_ansi_string_if_present(emu, hdrs_ptr);
            log_red!(emu, "wininet!HttpAddRequestHeadersA hdrs:'{}'", hdrs);
            pop_stack32(emu, 4);
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "HttpAddRequestHeadersW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hdrs_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let _hdrs_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let _modifiers = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let hdrs = read_wide_string_if_present(emu, hdrs_ptr);
            log_red!(emu, "wininet!HttpAddRequestHeadersW hdrs:'{}'", hdrs);
            pop_stack32(emu, 4);
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "HttpCheckDavCompliance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpCheckDavCompliance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpCloseDependencyHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "wininet!HttpCloseDependencyHandle 0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "HttpDuplicateDependencyHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpDuplicateDependencyHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpEndRequestA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!HttpEndRequestA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "HttpEndRequestW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!HttpEndRequestW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "HttpGetServerCredentials" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpGetServerCredentials arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpGetTunnelSocket" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpGetTunnelSocket h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpIndicatePageLoadComplete" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpIndicatePageLoadComplete arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpIsHostHstsEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpIsHostHstsEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpOpenDependencyHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpOpenDependencyHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpPushClose" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "wininet!HttpPushClose 0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "HttpPushEnable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpPushEnable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpPushWait" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "wininet!HttpPushWait 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "HttpQueryInfoW" => request::http_query_info_w(emu),
        "HttpSendRequestExA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _buffers_in = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let _buffers_out = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let _ctx = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            log_red!(emu, "wininet!HttpSendRequestExA handle:0x{:x}", handle);
            pop_stack32(emu, 5);
            emu.regs_mut().rax = 1;
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
        }
        "HttpSendRequestExW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _buffers_in = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let _buffers_out = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let _ctx = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            log_red!(emu, "wininet!HttpSendRequestExW handle:0x{:x}", handle);
            pop_stack32(emu, 5);
            emu.regs_mut().rax = 1;
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
        }
        "HttpWebSocketClose" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "wininet!HttpWebSocketClose 0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "HttpWebSocketCompleteUpgrade" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpWebSocketCompleteUpgrade h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpWebSocketQueryCloseStatus" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "wininet!HttpWebSocketQueryCloseStatus 0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "HttpWebSocketReceive" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpWebSocketReceive h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpWebSocketSend" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpWebSocketSend h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HttpWebSocketShutdown" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!HttpWebSocketShutdown h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IncrementUrlCacheHeaderData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!IncrementUrlCacheHeaderData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetAlgIdToStringA" => {
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
                "wininet!InternetAlgIdToStringA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetAlgIdToStringW" => {
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
                "wininet!InternetAlgIdToStringW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetAttemptConnect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "wininet!InternetAttemptConnect arg1:0x{:x}", arg1);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetAutodial" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetAutodial handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetAutodialCallback" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetAutodialCallback arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetAutodialHangup" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetAutodialHangup handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetCanonicalizeUrlA" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetCanonicalizeUrlA url:'{}'", url);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetCanonicalizeUrlW" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetCanonicalizeUrlW url:'{}'", url);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetCheckConnectionA" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let url = read_ansi_string_if_present(emu, url_ptr);
            log_red!(
                emu,
                "wininet!InternetCheckConnectionA url:'{}' => offline",
                url
            );
            pop_stack32(emu, 3);
            emu.set_last_error(12029); // ERROR_INTERNET_CANNOT_CONNECT
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetCheckConnectionW" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let url = read_wide_string_if_present(emu, url_ptr);
            log_red!(
                emu,
                "wininet!InternetCheckConnectionW url:'{}' => offline",
                url
            );
            pop_stack32(emu, 3);
            emu.set_last_error(12029); // ERROR_INTERNET_CANNOT_CONNECT
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetClearAllPerSiteCookieDecisions" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetClearAllPerSiteCookieDecisions handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetCombineUrlA" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetCombineUrlA url:'{}'", url);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetCombineUrlW" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetCombineUrlW url:'{}'", url);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetConfirmZoneCrossing" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "wininet!InternetConfirmZoneCrossing arg1:0x{:x}", arg1);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetConfirmZoneCrossingA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(
                emu,
                "wininet!InternetConfirmZoneCrossingA arg1:0x{:x}",
                arg1
            );
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetConfirmZoneCrossingW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(
                emu,
                "wininet!InternetConfirmZoneCrossingW arg1:0x{:x}",
                arg1
            );
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetConvertUrlFromWireToWideChar" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetConvertUrlFromWireToWideChar arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetCreateUrlA" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetCreateUrlA url:'{}'", url);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetCreateUrlW" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let url = if url_ptr != 0 {
                emu.maps.read_string(url_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "wininet!InternetCreateUrlW url:'{}'", url);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetDial" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!InternetDial arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("internetdial");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetDialA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!InternetDialA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("internetdiala");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetDialW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "wininet!InternetDialW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("internetdialw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetEnumPerSiteCookieDecisionA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetEnumPerSiteCookieDecisionA arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetEnumPerSiteCookieDecisionW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetEnumPerSiteCookieDecisionW arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetFindNextFileA" => {
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
                "wininet!InternetFindNextFileA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetFindNextFileW" => {
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
                "wininet!InternetFindNextFileW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetFortezzaCommand" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetFortezzaCommand handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetFreeCookies" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetFreeCookies handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetFreeProxyInfoList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetFreeProxyInfoList handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetCertByURL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetGetCertByURL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetCertByURLA" => {
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
                "wininet!InternetGetCertByURLA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetConnectedState" => {
            let flags_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetGetConnectedState");
            if flags_ptr != 0 {
                emu.maps.write_dword(flags_ptr, 0x20);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetConnectedStateEx" => {
            let flags_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetGetConnectedStateEx");
            if flags_ptr != 0 {
                emu.maps.write_dword(flags_ptr, 0x20);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetConnectedStateExA" => {
            let flags_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetGetConnectedStateExA");
            if flags_ptr != 0 {
                emu.maps.write_dword(flags_ptr, 0x20);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetConnectedStateExW" => {
            let flags_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetGetConnectedStateExW");
            if flags_ptr != 0 {
                emu.maps.write_dword(flags_ptr, 0x20);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetCookieA" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let size_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetCookieEx2" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let size_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetCookieExA" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let size_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetCookieExW" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let size_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetCookieW" => {
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let size_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetLastResponseInfoA" => {
            let error_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetGetLastResponseInfoA");
            if error_ptr != 0 {
                emu.maps.write_dword(error_ptr, 0);
            }
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 0);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetLastResponseInfoW" => {
            let error_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetGetLastResponseInfoW");
            if error_ptr != 0 {
                emu.maps.write_dword(error_ptr, 0);
            }
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 0);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetPerSiteCookieDecisionA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetGetPerSiteCookieDecisionA arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetPerSiteCookieDecisionW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetGetPerSiteCookieDecisionW arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetProxyForUrl" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetGetProxyForUrl handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGetSecurityInfoByURL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetGetSecurityInfoByURL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetSecurityInfoByURLA" => {
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
                "wininet!InternetGetSecurityInfoByURLA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGetSecurityInfoByURLW" => {
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
                "wininet!InternetGetSecurityInfoByURLW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGoOnline" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetGoOnline arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetGoOnlineA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetGoOnlineA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetGoOnlineW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetGoOnlineW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetHangUp" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetHangUp arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetInitializeAutoProxyDll" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetInitializeAutoProxyDll handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetLockRequestFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetLockRequestFile handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetOpenUrlA" => {
            let inet_hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hdrs_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _hdrs_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let _ctx = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let url = read_ansi_string_if_present(emu, url_ptr);
            let hdrs = read_ansi_string_if_present(emu, hdrs_ptr);
            log_red!(
                emu,
                "wininet!InternetOpenUrlA url:'{}' hdrs:'{}'",
                url,
                hdrs
            );
            pop_stack32(emu, 6);
            let uri = format!("InternetOpenUrlA://{}", url);
            emu.regs_mut().rax = helper::handler_create(&uri);
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetOpenUrlW" => {
            let inet_hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let url_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hdrs_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let _hdrs_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let _flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let _ctx = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            let url = read_wide_string_if_present(emu, url_ptr);
            let hdrs = read_wide_string_if_present(emu, hdrs_ptr);
            log_red!(
                emu,
                "wininet!InternetOpenUrlW url:'{}' hdrs:'{}'",
                url,
                hdrs
            );
            pop_stack32(emu, 6);
            let uri = format!("InternetOpenUrlW://{}", url);
            emu.regs_mut().rax = helper::handler_create(&uri);
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetQueryDataAvailable" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let bytes_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetQueryDataAvailable handle:0x{:x}",
                handle
            );
            if bytes_ptr != 0 {
                emu.maps.write_dword(bytes_ptr, 0);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetQueryFortezzaStatus" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetQueryFortezzaStatus arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetQueryOptionA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let option = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetQueryOptionA handle:0x{:x} option:{}",
                handle,
                option
            );
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 4);
            }
            if buf_ptr != 0 {
                emu.maps.write_dword(buf_ptr, 0);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetQueryOptionW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let option = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetQueryOptionW handle:0x{:x} option:{}",
                handle,
                option
            );
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 4);
            }
            if buf_ptr != 0 {
                emu.maps.write_dword(buf_ptr, 0);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSecurityProtocolToStringA" => {
            let protocol = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetSecurityProtocolToStringA protocol:{}",
                protocol
            );
            if buf_ptr != 0 {
                emu.maps.write_string(buf_ptr, "TLS 1.2");
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSecurityProtocolToStringW" => {
            let protocol = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetSecurityProtocolToStringW protocol:{}",
                protocol
            );
            if buf_ptr != 0 {
                emu.maps.write_string(buf_ptr, "TLS 1.2");
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetCookieA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetSetCookieA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetCookieEx2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetSetCookieEx2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetSetCookieExA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetSetCookieExA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetCookieExW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetSetCookieExW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetCookieW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetSetCookieW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetDialState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetSetDialState arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetSetDialStateA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetSetDialStateA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetDialStateW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetSetDialStateW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetFilePointer" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let distance = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
            let method = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "wininet!InternetSetFilePointer handle:0x{:x} dist:{} method:{}",
                handle,
                distance,
                method
            );
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetOptionExA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetSetOptionExA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetOptionExW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetSetOptionExW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetPerSiteCookieDecisionA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetSetPerSiteCookieDecisionA handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetPerSiteCookieDecisionW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetSetPerSiteCookieDecisionW handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetSetSecureLegacyServersAppCompat" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetSetSecureLegacyServersAppCompat arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetSetStatusCallback"
        | "InternetSetStatusCallbackA"
        | "InternetSetStatusCallbackW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let callback = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let valid = helper::handler_exist(handle);
            log_red!(
                emu,
                "wininet!InternetSetStatusCallback handle: 0x{:x} callback: 0x{:x} valid: {}",
                handle,
                callback,
                valid
            );
            emu.regs_mut().rax = 0;
        }
        "InternetShowSecurityInfoByURL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetShowSecurityInfoByURL arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetShowSecurityInfoByURLA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetShowSecurityInfoByURLA arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetShowSecurityInfoByURLW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetShowSecurityInfoByURLW arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetTimeFromSystemTime" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetTimeFromSystemTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetTimeFromSystemTimeA" => {
            let time_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let format = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetTimeFromSystemTimeA format:{}", format);
            if buf_ptr != 0 {
                emu.maps
                    .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetTimeFromSystemTimeW" => {
            let time_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let format = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetTimeFromSystemTimeW format:{}", format);
            if buf_ptr != 0 {
                emu.maps
                    .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetTimeToSystemTime" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetTimeToSystemTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetTimeToSystemTimeA" => {
            let time_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let format = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetTimeToSystemTimeA format:{}", format);
            if buf_ptr != 0 {
                emu.maps
                    .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetTimeToSystemTimeW" => {
            let time_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let format = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetTimeToSystemTimeW format:{}", format);
            if buf_ptr != 0 {
                emu.maps
                    .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
            }
            emu.regs_mut().rax = 1;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetUnlockRequestFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!InternetUnlockRequestFile handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetWriteFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!InternetWriteFile handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InternetWriteFileExA" => {
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
                "wininet!InternetWriteFileExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternetWriteFileExW" => {
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
                "wininet!InternetWriteFileExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IsHostInProxyBypassList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!IsHostInProxyBypassList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IsUrlCacheEntryExpiredA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IsUrlCacheEntryExpiredW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadUrlCacheContent" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!LoadUrlCacheContent handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ParseX509EncodedCertificateForListBoxEntry" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!ParseX509EncodedCertificateForListBoxEntry arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PrivacyGetZonePreferenceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PrivacySetZonePreferenceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadUrlCacheEntryStream" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!ReadUrlCacheEntryStream h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadUrlCacheEntryStreamEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!ReadUrlCacheEntryStreamEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegisterUrlCacheNotification" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!RegisterUrlCacheNotification handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ResumeSuspendedDownload" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!ResumeSuspendedDownload handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RetrieveUrlCacheEntryFileA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RetrieveUrlCacheEntryFileW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RetrieveUrlCacheEntryStreamA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RetrieveUrlCacheEntryStreamW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RunOnceUrlCache" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!RunOnceUrlCache arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheConfigInfoA" => {
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
                "wininet!SetUrlCacheConfigInfoA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheConfigInfoW" => {
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
                "wininet!SetUrlCacheConfigInfoW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheEntryGroup" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryGroup h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheEntryGroupA" => {
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
                "wininet!SetUrlCacheEntryGroupA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheEntryGroupW" => {
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
                "wininet!SetUrlCacheEntryGroupW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetUrlCacheEntryInfoA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!SetUrlCacheEntryInfoA handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetUrlCacheEntryInfoW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!SetUrlCacheEntryInfoW handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetUrlCacheGroupAttributeA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheGroupAttributeA handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetUrlCacheGroupAttributeW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheGroupAttributeW handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetUrlCacheHeaderData" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheHeaderData h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ShowCertificate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!ShowCertificate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ShowClientAuthCerts" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!ShowClientAuthCerts arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ShowSecurityInfo" => {
            log_red!(emu, "wininet!ShowSecurityInfo");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
        }
        "ShowX509EncodedCertificate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!ShowX509EncodedCertificate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UnlockUrlCacheEntryFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "wininet!UnlockUrlCacheEntryFile 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UnlockUrlCacheEntryFileA" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UnlockUrlCacheEntryFileA handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UnlockUrlCacheEntryFileW" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UnlockUrlCacheEntryFileW handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UnlockUrlCacheEntryStream" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "wininet!UnlockUrlCacheEntryStream 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UpdateUrlCacheContentPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UpdateUrlCacheContentPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheCheckEntriesExist" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheCheckEntriesExist arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheCloseEntryHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheCloseEntryHandle handle:0x{:x}",
                handle
            );
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UrlCacheContainerSetEntryMaximumAge" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheContainerSetEntryMaximumAge handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UrlCacheCreateContainer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheCreateContainer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheFindFirstEntry" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheFindFirstEntry arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheFindNextEntry" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheFindNextEntry arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheFreeEntryInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheFreeEntryInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheFreeGlobalSpace" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!UrlCacheFreeGlobalSpace handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UrlCacheGetContentPaths" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheGetContentPaths arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheGetEntryInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheGetEntryInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheGetGlobalCacheSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheGetGlobalCacheSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheGetGlobalLimit" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheGetGlobalLimit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheReadEntryStream" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!UrlCacheReadEntryStream arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UrlCacheReloadSettings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheReloadSettings arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheRetrieveEntryFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheRetrieveEntryFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheRetrieveEntryStream" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!UrlCacheRetrieveEntryStream arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UrlCacheServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheSetGlobalLimit" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheSetGlobalLimit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UrlCacheUpdateEntryExtraData" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheUpdateEntryExtraData handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UrlZonesDetach" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "wininet!UrlZonesDetach arg1:0x{:x}", arg1);
            emu.set_last_error(2);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "_GetFileExtensionFromUrl" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "wininet!_GetFileExtensionFromUrl h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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

pub(super) fn pop_stack32(emu: &mut emu::Emu, count: usize) {
    for _ in 0..count {
        emu.stack_pop32(false);
    }
}

pub(super) fn read_stack_dword(emu: &mut emu::Emu, offset: u64, _what: &str) -> u32 {
    emu.maps
        .read_dword(emu.regs().get_esp() + offset)
        .unwrap_or(0)
}

pub(super) fn read_ansi_string_if_present(emu: &mut emu::Emu, ptr: u64) -> String {
    if ptr == 0 {
        String::new()
    } else {
        emu.maps.read_string(ptr)
    }
}

pub(super) fn read_wide_string_if_present(emu: &mut emu::Emu, ptr: u64) -> String {
    if ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(ptr)
    }
}
