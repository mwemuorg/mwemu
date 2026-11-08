use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};

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
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheCheckManifest arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheCloseHandle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheCloseHandle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheCreateAndCommitFile" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheCreateAndCommitFile arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheDeleteGroup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheDeleteGroup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheDeleteIEGroup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheDeleteIEGroup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheDuplicateHandle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheDuplicateHandle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheFinalize" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFinalize arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheFreeDownloadList" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFreeDownloadList arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheFreeGroupList" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFreeGroupList arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheFreeIESpace" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFreeIESpace arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheFreeSpace" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheFreeSpace arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheGetDownloadList" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetDownloadList arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheGetFallbackUrl" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetFallbackUrl arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheGetGroupList" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetGroupList arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheGetIEGroupList" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetIEGroupList arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheGetInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheGetManifestUrl" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheGetManifestUrl arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppCacheLookup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!AppCacheLookup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CommitUrlCacheEntryA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CommitUrlCacheEntryA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 10);
        }
        "CommitUrlCacheEntryBinaryBlob" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CommitUrlCacheEntryBinaryBlob arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CommitUrlCacheEntryW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CommitUrlCacheEntryW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 10);
        }
        "CreateMD5SSOHash" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CreateMD5SSOHash arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createmd5ssohash");
            stdcall_ret(emu, 1);
        }
        "CreateUrlCacheContainerA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CreateUrlCacheContainerA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CreateUrlCacheContainerW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CreateUrlCacheContainerW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CreateUrlCacheEntryA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CreateUrlCacheEntryA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createurlcacheentrya");
            stdcall_ret(emu, 6);
        }
        "CreateUrlCacheEntryExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CreateUrlCacheEntryExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CreateUrlCacheEntryW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CreateUrlCacheEntryW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createurlcacheentryw");
            stdcall_ret(emu, 6);
        }
        "CreateUrlCacheGroup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!CreateUrlCacheGroup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createurlcachegroup");
            stdcall_ret(emu, 2);
        }
        "DeleteIE3Cache" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!DeleteIE3Cache handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DeleteUrlCacheContainerA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!DeleteUrlCacheContainerA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DeleteUrlCacheContainerW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!DeleteUrlCacheContainerW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DeleteUrlCacheEntry" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!DeleteUrlCacheEntry handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DeleteUrlCacheEntryA" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!DeleteUrlCacheEntryA handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DeleteUrlCacheEntryW" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!DeleteUrlCacheEntryW handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DeleteUrlCacheGroup" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!DeleteUrlCacheGroup handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DeleteWpadCacheForNetworks" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "wininet!DeleteWpadCacheForNetworks handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DetectAutoProxyUrl" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!DetectAutoProxyUrl arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "DispatchAPICall" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!DispatchAPICall arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DllCanUnloadNow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!DllCanUnloadNow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DllGetClassObject" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!DllGetClassObject arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DllInstall" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!DllInstall arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DllRegisterServer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!DllRegisterServer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("dllregisterserver");
            stdcall_ret(emu, 2);
        }
        "DllUnregisterServer" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!DllUnregisterServer handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FindCloseUrlCache" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!FindCloseUrlCache handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "FindFirstUrlCacheContainerA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheContainerA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("findfirsturlcachecontainera");
            stdcall_ret(emu, 2);
        }
        "FindFirstUrlCacheContainerW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheContainerW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("findfirsturlcachecontainerw");
            stdcall_ret(emu, 2);
        }
        "FindFirstUrlCacheEntryA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("findfirsturlcacheentrya");
            stdcall_ret(emu, 2);
        }
        "FindFirstUrlCacheEntryExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("findfirsturlcacheentryexa");
            stdcall_ret(emu, 2);
        }
        "FindFirstUrlCacheEntryExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("findfirsturlcacheentryexw");
            stdcall_ret(emu, 2);
        }
        "FindFirstUrlCacheEntryW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheEntryW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("findfirsturlcacheentryw");
            stdcall_ret(emu, 2);
        }
        "FindFirstUrlCacheGroup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindFirstUrlCacheGroup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("findfirsturlcachegroup");
            stdcall_ret(emu, 2);
        }
        "FindNextUrlCacheContainerA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindNextUrlCacheContainerA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "FindNextUrlCacheContainerW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindNextUrlCacheContainerW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "FindNextUrlCacheEntryA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindNextUrlCacheEntryA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "FindNextUrlCacheEntryExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindNextUrlCacheEntryExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "FindNextUrlCacheEntryExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindNextUrlCacheEntryExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "FindNextUrlCacheEntryW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindNextUrlCacheEntryW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "FindNextUrlCacheGroup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FindNextUrlCacheGroup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "ForceNexusLookup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ForceNexusLookup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ForceNexusLookupExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ForceNexusLookupExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FreeUrlCacheSpaceA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FreeUrlCacheSpaceA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "FreeUrlCacheSpaceW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FreeUrlCacheSpaceW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "FtpCommandA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpCommandA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "FtpCommandW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpCommandW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "FtpCreateDirectoryA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpCreateDirectoryA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FtpCreateDirectoryW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpCreateDirectoryW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FtpDeleteFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpDeleteFileA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FtpDeleteFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpDeleteFileW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FtpFindFirstFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpFindFirstFileA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
            stdcall_ret(emu, 4);
        }
        "FtpFindFirstFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpFindFirstFileW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
            stdcall_ret(emu, 4);
        }
        "FtpGetCurrentDirectoryA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpGetCurrentDirectoryA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "FtpGetCurrentDirectoryW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpGetCurrentDirectoryW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "FtpGetFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpGetFileA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "FtpGetFileEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpGetFileEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FtpGetFileSize" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpGetFileSize arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "FtpGetFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpGetFileW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "FtpOpenFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpOpenFileA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("ftpopenfilea");
            stdcall_ret(emu, 4);
        }
        "FtpOpenFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpOpenFileW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("ftpopenfilew");
            stdcall_ret(emu, 4);
        }
        "FtpPutFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpPutFileA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "FtpPutFileEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpPutFileEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FtpPutFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!FtpPutFileW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "FtpRemoveDirectoryA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpRemoveDirectoryA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FtpRemoveDirectoryW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpRemoveDirectoryW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FtpRenameFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpRenameFileA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "FtpRenameFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpRenameFileW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "FtpSetCurrentDirectoryA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpSetCurrentDirectoryA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FtpSetCurrentDirectoryW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!FtpSetCurrentDirectoryW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetProxyDllInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetProxyDllInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetUrlCacheConfigInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheConfigInfoA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetUrlCacheConfigInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheConfigInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetUrlCacheEntryBinaryBlob" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheEntryBinaryBlob arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetUrlCacheEntryInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheEntryInfoA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetUrlCacheEntryInfoExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheEntryInfoExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetUrlCacheEntryInfoExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheEntryInfoExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetUrlCacheEntryInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheEntryInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetUrlCacheGroupAttributeA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheGroupAttributeA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetUrlCacheGroupAttributeW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheGroupAttributeW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetUrlCacheHeaderData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GetUrlCacheHeaderData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GopherCreateLocatorA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherCreateLocatorA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("gophercreatelocatora");
            stdcall_ret(emu, 2);
        }
        "GopherCreateLocatorW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherCreateLocatorW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("gophercreatelocatorw");
            stdcall_ret(emu, 2);
        }
        "GopherFindFirstFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherFindFirstFileA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
            stdcall_ret(emu, 6);
        }
        "GopherFindFirstFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherFindFirstFileW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
            stdcall_ret(emu, 6);
        }
        "GopherGetAttributeA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherGetAttributeA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GopherGetAttributeW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherGetAttributeW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GopherGetLocatorTypeA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherGetLocatorTypeA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GopherGetLocatorTypeW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherGetLocatorTypeW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GopherOpenFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherOpenFileA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("gopheropenfilea");
            stdcall_ret(emu, 6);
        }
        "GopherOpenFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!GopherOpenFileW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("gopheropenfilew");
            stdcall_ret(emu, 6);
        }
        "HttpAddRequestHeadersA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpAddRequestHeadersA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "HttpAddRequestHeadersW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpAddRequestHeadersW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "HttpCheckDavCompliance" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpCheckDavCompliance arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HttpCloseDependencyHandle" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "wininet!HttpCloseDependencyHandle handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HttpDuplicateDependencyHandle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpDuplicateDependencyHandle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("httpduplicatedependencyhandle");
            stdcall_ret(emu, 2);
        }
        "HttpEndRequestA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpEndRequestA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "HttpEndRequestW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpEndRequestW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "HttpGetServerCredentials" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpGetServerCredentials arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HttpGetTunnelSocket" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpGetTunnelSocket arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HttpIndicatePageLoadComplete" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpIndicatePageLoadComplete arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("httpindicatepageloadcomplete");
            stdcall_ret(emu, 2);
        }
        "HttpIsHostHstsEnabled" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpIsHostHstsEnabled arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("httpishosthstsenabled");
            stdcall_ret(emu, 2);
        }
        "HttpOpenDependencyHandle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpOpenDependencyHandle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("httpopendependencyhandle");
            stdcall_ret(emu, 2);
        }
        "HttpPushClose" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!HttpPushClose handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HttpPushEnable" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpPushEnable arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("httppushenable");
            stdcall_ret(emu, 2);
        }
        "HttpPushWait" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!HttpPushWait arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HttpSendRequestExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpSendRequestExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "HttpSendRequestExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpSendRequestExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "HttpWebSocketClose" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!HttpWebSocketClose handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HttpWebSocketCompleteUpgrade" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpWebSocketCompleteUpgrade arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HttpWebSocketQueryCloseStatus" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "wininet!HttpWebSocketQueryCloseStatus handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "HttpWebSocketReceive" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpWebSocketReceive arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HttpWebSocketSend" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpWebSocketSend arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HttpWebSocketShutdown" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpWebSocketShutdown arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IncrementUrlCacheHeaderData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!IncrementUrlCacheHeaderData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetAlgIdToStringA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetAlgIdToStringA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetAlgIdToStringW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetAlgIdToStringW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetAttemptConnect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetAttemptConnect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("internetattemptconnect");
            stdcall_ret(emu, 1);
        }
        "InternetAutodial" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetAutodial arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetAutodialCallback" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetAutodialCallback arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetAutodialHangup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetAutodialHangup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetCanonicalizeUrlA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetCanonicalizeUrlA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetCanonicalizeUrlW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetCanonicalizeUrlW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetCheckConnectionA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetCheckConnectionA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetCheckConnectionW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetCheckConnectionW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetClearAllPerSiteCookieDecisions" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetClearAllPerSiteCookieDecisions arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetCombineUrlA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetCombineUrlA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetCombineUrlW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetCombineUrlW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetConfirmZoneCrossing" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetConfirmZoneCrossing arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "InternetConfirmZoneCrossingA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetConfirmZoneCrossingA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "InternetConfirmZoneCrossingW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetConfirmZoneCrossingW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "InternetConvertUrlFromWireToWideChar" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetConvertUrlFromWireToWideChar arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetCreateUrlA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetCreateUrlA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetCreateUrlW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetCreateUrlW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetDial" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!InternetDial arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "InternetDialA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!InternetDialA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "InternetDialW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "wininet!InternetDialW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "InternetEnumPerSiteCookieDecisionA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetEnumPerSiteCookieDecisionA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "InternetEnumPerSiteCookieDecisionW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetEnumPerSiteCookieDecisionW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "InternetFindNextFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetFindNextFileA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetFindNextFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetFindNextFileW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetFortezzaCommand" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetFortezzaCommand arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetFreeCookies" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!InternetFreeCookies handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetFreeProxyInfoList" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "wininet!InternetFreeProxyInfoList handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetGetCertByURL" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetCertByURL arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetGetCertByURLA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetCertByURLA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetGetConnectedState" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetConnectedState arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetGetConnectedStateEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetConnectedStateEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetGetConnectedStateExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetConnectedStateExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetGetConnectedStateExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetConnectedStateExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetGetCookieA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetCookieA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "InternetGetCookieEx2" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetCookieEx2 arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "InternetGetCookieExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetCookieExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "InternetGetCookieExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetCookieExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "InternetGetCookieW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetCookieW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "InternetGetLastResponseInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetLastResponseInfoA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetGetLastResponseInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetLastResponseInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetGetPerSiteCookieDecisionA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetPerSiteCookieDecisionA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "InternetGetPerSiteCookieDecisionW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetPerSiteCookieDecisionW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "InternetGetProxyForUrl" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetProxyForUrl arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetGetSecurityInfoByURL" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetSecurityInfoByURL arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetGetSecurityInfoByURLA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetSecurityInfoByURLA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetGetSecurityInfoByURLW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGetSecurityInfoByURLW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetGoOnline" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGoOnline arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetGoOnlineA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGoOnlineA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetGoOnlineW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetGoOnlineW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetHangUp" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetHangUp arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetInitializeAutoProxyDll" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetInitializeAutoProxyDll arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("internetinitializeautoproxydll");
            stdcall_ret(emu, 3);
        }
        "InternetLockRequestFile" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetLockRequestFile arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetOpenUrlA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetOpenUrlA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("internetopenurla");
            stdcall_ret(emu, 4);
        }
        "InternetOpenUrlW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetOpenUrlW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("internetopenurlw");
            stdcall_ret(emu, 4);
        }
        "InternetQueryDataAvailable" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetQueryDataAvailable arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetQueryFortezzaStatus" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetQueryFortezzaStatus arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetSecurityProtocolToStringA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSecurityProtocolToStringA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetSecurityProtocolToStringW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSecurityProtocolToStringW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetSetCookieA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetCookieA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetSetCookieEx2" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetCookieEx2 arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "InternetSetCookieExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetCookieExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetSetCookieExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetCookieExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetSetCookieW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetCookieW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetSetDialState" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetDialState arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetSetDialStateA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetDialStateA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetSetDialStateW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetDialStateW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetSetFilePointer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetFilePointer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
        }
        "InternetSetOptionExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetOptionExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetSetOptionExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetOptionExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetSetPerSiteCookieDecisionA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetPerSiteCookieDecisionA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetSetPerSiteCookieDecisionW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetPerSiteCookieDecisionW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetSetSecureLegacyServersAppCompat" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetSetSecureLegacyServersAppCompat arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetShowSecurityInfoByURL" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetShowSecurityInfoByURL arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "InternetShowSecurityInfoByURLA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetShowSecurityInfoByURLA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "InternetShowSecurityInfoByURLW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetShowSecurityInfoByURLW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "InternetTimeFromSystemTime" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetTimeFromSystemTime arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetTimeFromSystemTimeA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetTimeFromSystemTimeA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetTimeFromSystemTimeW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetTimeFromSystemTimeW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetTimeToSystemTime" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetTimeToSystemTime arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternetTimeToSystemTimeA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetTimeToSystemTimeA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InternetTimeToSystemTimeW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetTimeToSystemTimeW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "_GetFileExtensionFromUrl" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!_GetFileExtensionFromUrl arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "InternetUnlockRequestFile" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "wininet!InternetUnlockRequestFile handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "InternetWriteFile" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetWriteFile arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetWriteFileExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetWriteFileExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetWriteFileExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetWriteFileExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "IsHostInProxyBypassList" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!IsHostInProxyBypassList arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsUrlCacheEntryExpiredA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!IsUrlCacheEntryExpiredA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsUrlCacheEntryExpiredW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!IsUrlCacheEntryExpiredW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "LoadUrlCacheContent" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!LoadUrlCacheContent arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("loadurlcachecontent");
            stdcall_ret(emu, 1);
        }
        "ParseX509EncodedCertificateForListBoxEntry" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ParseX509EncodedCertificateForListBoxEntry arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PrivacyGetZonePreferenceW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!PrivacyGetZonePreferenceW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PrivacySetZonePreferenceW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!PrivacySetZonePreferenceW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReadUrlCacheEntryStream" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ReadUrlCacheEntryStream arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReadUrlCacheEntryStreamEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ReadUrlCacheEntryStreamEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RegisterUrlCacheNotification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!RegisterUrlCacheNotification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerurlcachenotification");
            stdcall_ret(emu, 6);
        }
        "ResumeSuspendedDownload" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ResumeSuspendedDownload arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RetrieveUrlCacheEntryFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!RetrieveUrlCacheEntryFileA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RetrieveUrlCacheEntryFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!RetrieveUrlCacheEntryFileW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RetrieveUrlCacheEntryStreamA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!RetrieveUrlCacheEntryStreamA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RetrieveUrlCacheEntryStreamW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!RetrieveUrlCacheEntryStreamW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RunOnceUrlCache" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!RunOnceUrlCache arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetUrlCacheConfigInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheConfigInfoA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetUrlCacheConfigInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheConfigInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetUrlCacheEntryGroup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryGroup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetUrlCacheEntryGroupA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryGroupA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetUrlCacheEntryGroupW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryGroupW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetUrlCacheEntryInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryInfoA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetUrlCacheEntryInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheEntryInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetUrlCacheGroupAttributeA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheGroupAttributeA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetUrlCacheGroupAttributeW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheGroupAttributeW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetUrlCacheHeaderData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!SetUrlCacheHeaderData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShowCertificate" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ShowCertificate arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShowClientAuthCerts" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ShowClientAuthCerts arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShowSecurityInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ShowSecurityInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "ShowX509EncodedCertificate" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!ShowX509EncodedCertificate arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnlockUrlCacheEntryFile" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!UnlockUrlCacheEntryFile handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnlockUrlCacheEntryFileA" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "wininet!UnlockUrlCacheEntryFileA handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnlockUrlCacheEntryFileW" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "wininet!UnlockUrlCacheEntryFileW handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnlockUrlCacheEntryStream" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "wininet!UnlockUrlCacheEntryStream handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UpdateUrlCacheContentPath" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UpdateUrlCacheContentPath arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheCheckEntriesExist" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheCheckEntriesExist arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheCloseEntryHandle" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheCloseEntryHandle handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "UrlCacheContainerSetEntryMaximumAge" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheContainerSetEntryMaximumAge arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "UrlCacheCreateContainer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheCreateContainer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("urlcachecreatecontainer");
            stdcall_ret(emu, 2);
        }
        "UrlCacheFindFirstEntry" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheFindFirstEntry arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheFindNextEntry" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheFindNextEntry arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheFreeEntryInfo" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!UrlCacheFreeEntryInfo handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheFreeGlobalSpace" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!UrlCacheFreeGlobalSpace handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheGetContentPaths" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheGetContentPaths arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheGetEntryInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheGetEntryInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheGetGlobalCacheSize" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheGetGlobalCacheSize arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheGetGlobalLimit" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheGetGlobalLimit arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheReadEntryStream" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheReadEntryStream arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "UrlCacheReloadSettings" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheReloadSettings arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheRetrieveEntryFile" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheRetrieveEntryFile arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheRetrieveEntryStream" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheRetrieveEntryStream arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "UrlCacheServer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheServer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheSetGlobalLimit" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheSetGlobalLimit arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UrlCacheUpdateEntryExtraData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!UrlCacheUpdateEntryExtraData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "UrlZonesDetach" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "wininet!UrlZonesDetach handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HttpQueryInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!HttpQueryInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetQueryOptionA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetQueryOptionA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetQueryOptionW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "wininet!InternetQueryOptionW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InternetSetStatusCallback"
        | "InternetSetStatusCallbackA"
        | "InternetSetStatusCallbackW" => {
            let handle = arg(emu, 0) as u64;
            let callback = arg(emu, 1) as u64;
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
            stdcall_ret(emu, 2);
        }

        _ => {
            return fallback::unimplemented(addr, api, emu);
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
