use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "RealShellExecuteA" => RealShellExecuteA(emu),
        "SHGetFolderPathW" => SHGetFolderPathW(emu),
        "ShellExecuteA" => ShellExecuteA(emu),
        "ShellExecuteW" => ShellExecuteW(emu),
        "AppCompat_RunDLLW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!AppCompat_RunDLLW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "AssocCreateForClasses" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!AssocCreateForClasses arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("assoccreateforclasses");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "AssocElemCreateForKey" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!AssocElemCreateForKey arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("assocelemcreateforkey");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "AssocGetDetailsOfPropKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!AssocGetDetailsOfPropKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CDefFolderMenu_Create2" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!CDefFolderMenu_Create2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("cdeffoldermenu_create2");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "CIDLData_CreateFromIDArray" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!CIDLData_CreateFromIDArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("cidldata_createfromidarray");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "CStorageItem_GetValidatedStorageItemObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!CStorageItem_GetValidatedStorageItemObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallFileCopyHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CallFileCopyHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "CheckEscapesW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CheckEscapesW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "CommandLineToArgvW" => CommandLineToArgvW(emu),
        "Control_RunDLL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Control_RunDLL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "Control_RunDLLA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Control_RunDLLA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "Control_RunDLLAsUserW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Control_RunDLLAsUserW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "Control_RunDLLW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Control_RunDLLW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "CreateStorageItemFromPath_FullTrustCaller" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!CreateStorageItemFromPath_FullTrustCaller arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("createstorageitemfrompath_fulltrustcaller");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "CreateStorageItemFromPath_FullTrustCaller_ForPackage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!CreateStorageItemFromPath_FullTrustCaller_ForPackage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle =
                helper::handler_create("createstorageitemfrompath_fulltrustcaller_forpackage");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "CreateStorageItemFromPath_PartialTrustCaller" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!CreateStorageItemFromPath_PartialTrustCaller arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("createstorageitemfrompath_partialtrustcaller");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "CreateStorageItemFromShellItem_FullTrustCaller" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!CreateStorageItemFromShellItem_FullTrustCaller arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("createstorageitemfromshellitem_fulltrustcaller");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "CreateStorageItemFromShellItem_FullTrustCaller_ForPackage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!CreateStorageItemFromShellItem_FullTrustCaller_ForPackage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle =
                helper::handler_create("createstorageitemfromshellitem_fulltrustcaller_forpackage");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create(
                "createstorageitemfromshellitem_fulltrustcaller_forpackage_withprocesshandle",
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create(
                "createstorageitemfromshellitem_fulltrustcaller_useimplicitflagsandpackage",
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "DAD_AutoScroll" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DAD_AutoScroll arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DAD_DragEnterEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DAD_DragEnterEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DAD_DragEnterEx2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DAD_DragEnterEx2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DAD_DragLeave" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DAD_DragLeave arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DAD_DragMove" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DAD_DragMove arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DAD_SetDragImage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DAD_SetDragImage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DAD_ShowDragImage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DAD_ShowDragImage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DllCanUnloadNow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DllCanUnloadNow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DllGetActivationFactory" => {
            let class_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let factory_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "shell32!DllGetActivationFactory");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x80004001u64;
            for _ in 0..3u32 {}
        }
        "DllGetClassObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DllGetClassObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DllGetVersion" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DllGetVersion arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DllInstall" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DllInstall arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DllRegisterServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DllRegisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DllUnregisterServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DllUnregisterServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DoEnvironmentSubstA" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!DoEnvironmentSubstA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("doenvironmentsubsta");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "DoEnvironmentSubstW" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!DoEnvironmentSubstW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("doenvironmentsubstw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "DragAcceptFiles" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let accept = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DragAcceptFiles hwnd:0x{:x} accept:{}",
                hwnd,
                accept
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "DragFinish" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "shell32!DragFinish handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DragQueryFile" => {
            let hdrop = emu.regs().rcx;
            let index = emu.regs().rdx as u32;
            let buf_ptr = emu.regs().r8;
            let buf_size = emu.regs().r9 as u32;
            log_red!(
                emu,
                "shell32!DragQueryFile hdrop:0x{:x} index:{}",
                hdrop,
                index
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            if index == 0xFFFFFFFF {
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else if buf_ptr != 0 && buf_size > 10 {
                emu.maps.write_string(buf_ptr, "C:\\file.txt");
                emu.regs_mut().rax = 10;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 10;
            }
        }
        "DragQueryFileA" => {
            let hdrop = emu.regs().rcx;
            let index = emu.regs().rdx as u32;
            let buf_ptr = emu.regs().r8;
            let buf_size = emu.regs().r9 as u32;
            log_red!(
                emu,
                "shell32!DragQueryFileA hdrop:0x{:x} index:{}",
                hdrop,
                index
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            if index == 0xFFFFFFFF {
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else if buf_ptr != 0 && buf_size > 10 {
                emu.maps.write_string(buf_ptr, "C:\\file.txt");
                emu.regs_mut().rax = 10;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 10;
            }
        }
        "DragQueryFileAorW" => {
            let hdrop = emu.regs().rcx;
            let index = emu.regs().rdx as u32;
            let buf_ptr = emu.regs().r8;
            let buf_size = emu.regs().r9 as u32;
            log_red!(
                emu,
                "shell32!DragQueryFileAorW hdrop:0x{:x} index:{}",
                hdrop,
                index
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            if index == 0xFFFFFFFF {
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else if buf_ptr != 0 && buf_size > 10 {
                emu.maps.write_string(buf_ptr, "C:\\file.txt");
                emu.regs_mut().rax = 10;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 10;
            }
        }
        "DragQueryFileW" => {
            let hdrop = emu.regs().rcx;
            let index = emu.regs().rdx as u32;
            let buf_ptr = emu.regs().r8;
            let buf_size = emu.regs().r9 as u32;
            log_red!(
                emu,
                "shell32!DragQueryFileW hdrop:0x{:x} index:{}",
                hdrop,
                index
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            if index == 0xFFFFFFFF {
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else if buf_ptr != 0 && buf_size > 10 {
                emu.maps.write_string(buf_ptr, "C:\\file.txt");
                emu.regs_mut().rax = 10;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 10;
            }
        }
        "DragQueryPoint" => {
            let hdrop = emu.regs().rcx;
            let point_ptr = emu.regs().rdx;
            log_red!(emu, "shell32!DragQueryPoint hdrop:0x{:x}", hdrop);
            if point_ptr != 0 {
                emu.maps.write_dword(point_ptr, 0);
                emu.maps.write_dword(point_ptr + 4, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DriveType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "shell32!DriveType arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
            for _ in 0..1u32 {}
        }
        "DuplicateIcon" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!DuplicateIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("duplicateicon");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ExtractAssociatedIconA" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!ExtractAssociatedIconA '{}' arg2:0x{:x}",
                s,
                arg2
            );
            let handle = helper::handler_create("extractassociatedicona");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ExtractAssociatedIconExA" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!ExtractAssociatedIconExA '{}' arg2:0x{:x}",
                s,
                arg2
            );
            let handle = helper::handler_create("extractassociatediconexa");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ExtractAssociatedIconExW" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!ExtractAssociatedIconExW '{}' arg2:0x{:x}",
                s,
                arg2
            );
            let handle = helper::handler_create("extractassociatediconexw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ExtractAssociatedIconW" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!ExtractAssociatedIconW '{}' arg2:0x{:x}",
                s,
                arg2
            );
            let handle = helper::handler_create("extractassociatediconw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ExtractIconA" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!ExtractIconA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("extracticona");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ExtractIconEx" => {
            let file_ptr = emu.regs().rcx;
            let icon_index = emu.regs().rdx as i32;
            let large_ptr = emu.regs().r8;
            let small_ptr = emu.regs().r9;
            let file = if file_ptr != 0 {
                emu.maps.read_string(file_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "shell32!ExtractIconEx file:'{}' index:{}",
                file,
                icon_index
            );
            let handle = helper::handler_create("icon");
            if large_ptr != 0 {
                emu.maps.write_qword(large_ptr, handle);
            }
            if small_ptr != 0 {
                emu.maps.write_qword(small_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ExtractIconExA" => {
            let file_ptr = emu.regs().rcx;
            let icon_index = emu.regs().rdx as i32;
            let large_ptr = emu.regs().r8;
            let small_ptr = emu.regs().r9;
            let file = if file_ptr != 0 {
                emu.maps.read_string(file_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "shell32!ExtractIconExA file:'{}' index:{}",
                file,
                icon_index
            );
            let handle = helper::handler_create("icon");
            if large_ptr != 0 {
                emu.maps.write_qword(large_ptr, handle);
            }
            if small_ptr != 0 {
                emu.maps.write_qword(small_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ExtractIconExW" => {
            let file_ptr = emu.regs().rcx;
            let icon_index = emu.regs().rdx as i32;
            let large_ptr = emu.regs().r8;
            let small_ptr = emu.regs().r9;
            let file = if file_ptr != 0 {
                emu.maps.read_string(file_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "shell32!ExtractIconExW file:'{}' index:{}",
                file,
                icon_index
            );
            let handle = helper::handler_create("icon");
            if large_ptr != 0 {
                emu.maps.write_qword(large_ptr, handle);
            }
            if small_ptr != 0 {
                emu.maps.write_qword(small_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ExtractIconW" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!ExtractIconW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("extracticonw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "FindExecutableA" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!FindExecutableA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("findexecutablea");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "FindExecutableW" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!FindExecutableW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("findexecutablew");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "FreeIconList" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "shell32!FreeIconList handle:0x{:x}", handle);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentProcessExplicitAppUserModelID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(
                emu,
                "shell32!GetCurrentProcessExplicitAppUserModelID arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {}
        }
        "GetFileNameFromBrowse" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!GetFileNameFromBrowse h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemPersistedStorageItemList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!GetSystemPersistedStorageItemList h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ILAppendID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!ILAppendID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ILClone" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!ILClone arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("ilclone");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ILCloneFirst" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!ILCloneFirst arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ilclonefirst");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ILCombine" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!ILCombine arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("ilcombine");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ILCreateFromPath" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!ILCreateFromPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ilcreatefrompath");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ILCreateFromPathA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!ILCreateFromPathA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ilcreatefrompatha");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ILCreateFromPathW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!ILCreateFromPathW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ilcreatefrompathw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ILFindChild" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!ILFindChild arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {}
        }
        "ILFindLastID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!ILFindLastID arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {}
        }
        "ILFree" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "shell32!ILFree handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ILGetNext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!ILGetNext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {}
        }
        "ILGetSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "shell32!ILGetSize arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
            for _ in 0..1u32 {}
        }
        "ILIsEqual" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!ILIsEqual arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ILIsParent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ILIsParent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ILLoadFromStreamEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!ILLoadFromStreamEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ILRemoveLastID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ILRemoveLastID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ILSaveToStream" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ILSaveToStream arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "InitNetworkAddressControl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!InitNetworkAddressControl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "InternalExtractIconListA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!InternalExtractIconListA '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternalExtractIconListW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!InternalExtractIconListW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsDesktopExplorerProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!IsDesktopExplorerProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsLFNDrive" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!IsLFNDrive arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsLFNDriveA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!IsLFNDriveA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "IsLFNDriveW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!IsLFNDriveW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "IsNetDrive" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!IsNetDrive arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "IsProcessAnExplorer" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!IsProcessAnExplorer h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsUserAnAdmin" => {
            log_red!(emu, "shell32!IsUserAnAdmin => TRUE");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LaunchMSHelp_RunDLLW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!LaunchMSHelp_RunDLLW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OpenAs_RunDLL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!OpenAs_RunDLL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "OpenAs_RunDLLA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!OpenAs_RunDLLA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "OpenAs_RunDLLW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!OpenAs_RunDLLW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "OpenRegStream" => {
            let hkey = emu.regs().rcx;
            let subkey_ptr = emu.regs().rdx;
            let value_ptr = emu.regs().r8;
            let subkey = if subkey_ptr != 0 {
                emu.maps.read_string(subkey_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "shell32!OpenRegStream hkey:0x{:x} '{}'", hkey, subkey);
            let hndl = helper::handler_create("shell32://OpenRegStream");
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "Options_RunDLL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!Options_RunDLL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Options_RunDLLA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!Options_RunDLLA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Options_RunDLLW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!Options_RunDLLW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PathCleanupSpec" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PathCleanupSpec arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PathGetShortPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!PathGetShortPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PathIsExe" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!PathIsExe arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PathIsSlowA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PathIsSlowA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PathIsSlowW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PathIsSlowW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PathMakeUniqueName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PathMakeUniqueName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PathQualify" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PathQualify arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PathResolve" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PathResolve arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PathYetAnotherMakeUniqueName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PathYetAnotherMakeUniqueName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PickIconDlg" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!PickIconDlg arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PifMgr_CloseProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PifMgr_CloseProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PifMgr_GetProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PifMgr_GetProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PifMgr_OpenProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PifMgr_OpenProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PifMgr_SetProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PifMgr_SetProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PrepareDiscForBurnRunDllW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PrepareDiscForBurnRunDllW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PrintersGetCommand_RunDLL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PrintersGetCommand_RunDLL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "PrintersGetCommand_RunDLLA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!PrintersGetCommand_RunDLLA '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrintersGetCommand_RunDLLW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PrintersGetCommand_RunDLLW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ReadCabinetState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ReadCabinetState arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "RealDriveType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!RealDriveType arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "RealShellExecuteExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!RealShellExecuteExA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RealShellExecuteExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!RealShellExecuteExW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RealShellExecuteW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!RealShellExecuteW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegenerateUserEnvironment" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!RegenerateUserEnvironment arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "RestartDialog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!RestartDialog arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "RestartDialogEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!RestartDialogEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "RunAsNewUser_RunDLLW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!RunAsNewUser_RunDLLW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHAddDefaultPropertiesByExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHAddDefaultPropertiesByExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHAddFromPropSheetExtArray" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHAddFromPropSheetExtArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHAddToRecentDocs" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHAddToRecentDocs arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHAlloc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!SHAlloc arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {}
        }
        "SHAppBarMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHAppBarMessage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHAssocEnumHandlers" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHAssocEnumHandlers arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shassocenumhandlers");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHAssocEnumHandlersForProtocolByApplication" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHAssocEnumHandlersForProtocolByApplication arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shassocenumhandlersforprotocolbyapplication");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHBindToFolderIDListParent" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHBindToFolderIDListParent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbindtofolderidlistparent");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHBindToFolderIDListParentEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHBindToFolderIDListParentEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbindtofolderidlistparentex");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHBindToObject" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHBindToObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbindtoobject");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHBindToParent" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHBindToParent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbindtoparent");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHBrowseForFolder" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHBrowseForFolder arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbrowseforfolder");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHBrowseForFolderA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHBrowseForFolderA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbrowseforfoldera");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHBrowseForFolderW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHBrowseForFolderW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbrowseforfolderw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCLSIDFromString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHCLSIDFromString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHChangeNotification_Lock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHChangeNotification_Lock arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHChangeNotification_Unlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHChangeNotification_Unlock arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHChangeNotify" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHChangeNotify arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHChangeNotifyDeregister" => {
            let handle = emu.regs().rcx;
            log_red!(
                emu,
                "shell32!SHChangeNotifyDeregister handle:0x{:x}",
                handle
            );
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHChangeNotifyRegister" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHChangeNotifyRegister arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shchangenotifyregister");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHChangeNotifyRegisterThread" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHChangeNotifyRegisterThread h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHChangeNotifySuspendResume" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHChangeNotifySuspendResume arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHCloneSpecialIDList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHCloneSpecialIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHCoCreateInstance" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCoCreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcocreateinstance");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCoCreateInstanceWorker" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCoCreateInstanceWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcocreateinstanceworker");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateAssociationRegistration" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateAssociationRegistration arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateassociationregistration");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateCategoryEnum" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHCreateCategoryEnum arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHCreateDataObject" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateDataObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedataobject");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateDefaultContextMenu" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateDefaultContextMenu arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedefaultcontextmenu");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateDefaultExtractIcon" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateDefaultExtractIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedefaultextracticon");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateDefaultPropertiesOp" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateDefaultPropertiesOp arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedefaultpropertiesop");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateDirectory" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateDirectory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedirectory");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateDirectoryExA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateDirectoryExA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedirectoryexa");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateDirectoryExW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateDirectoryExW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedirectoryexw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateDrvExtIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHCreateDrvExtIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHCreateFileExtractIconW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!SHCreateFileExtractIconW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHCreateItemFromIDList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateItemFromIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateitemfromidlist");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateItemFromParsingName" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateItemFromParsingName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateitemfromparsingname");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateItemFromRelativeName" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateItemFromRelativeName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateitemfromrelativename");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateItemInKnownFolder" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateItemInKnownFolder arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateiteminknownfolder");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateItemWithParent" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateItemWithParent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateitemwithparent");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateLocalServerRunDll" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateLocalServerRunDll arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatelocalserverrundll");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateProcessAsUserW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateProcessAsUserW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHCreatePropSheetExtArray" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreatePropSheetExtArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatepropsheetextarray");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateQueryCancelAutoPlayMoniker" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateQueryCancelAutoPlayMoniker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatequerycancelautoplaymoniker");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateShellFolderView" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateShellFolderView arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellfolderview");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateShellFolderViewEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateShellFolderViewEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellfolderviewex");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateShellItem" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateShellItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitem");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateShellItemArray" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateShellItemArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitemarray");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateShellItemArrayFromDataObject" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateShellItemArrayFromDataObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitemarrayfromdataobject");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateShellItemArrayFromIDLists" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateShellItemArrayFromIDLists arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitemarrayfromidlists");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateShellItemArrayFromShellItem" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateShellItemArrayFromShellItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitemarrayfromshellitem");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHCreateStdEnumFmtEtc" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHCreateStdEnumFmtEtc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatestdenumfmtetc");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHDefExtractIconA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHDefExtractIconA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHDefExtractIconW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHDefExtractIconW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHDestroyPropSheetExtArray" => {
            let handle = emu.regs().rcx;
            log_red!(
                emu,
                "shell32!SHDestroyPropSheetExtArray handle:0x{:x}",
                handle
            );
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHDoDragDrop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHDoDragDrop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHELL32_AddToBackIconTable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_AddToBackIconTable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_AddToFrontIconTable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_AddToFrontIconTable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_AreAllItemsAvailable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_AreAllItemsAvailable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CCommonPlacesFolder_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CCommonPlacesFolder_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_CloseSession" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_CloseSession arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_DriveSupportedForDataBurn" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_DriveSupportedForDataBurn arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_Erase" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_Erase arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_GetCDInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_GetCDInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_GetLiveFSDiscInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_GetLiveFSDiscInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_GetStagingPathOrNormalPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_GetStagingPathOrNormalPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_GetTaskInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_GetTaskInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_IsBlankDisc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_IsBlankDisc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_IsBlankDisc2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_IsBlankDisc2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_IsLiveFS" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_IsLiveFS arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_OnDeviceChange" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_OnDeviceChange arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_OnEject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_OnEject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDBurn_OnMediaChange" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_OnMediaChange arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDefFolderMenu_Create2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDefFolderMenu_Create2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDefFolderMenu_Create2Ex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDefFolderMenu_Create2Ex arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDefFolderMenu_MergeMenu" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDefFolderMenu_MergeMenu arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDrivesContextMenu_Create" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDrivesContextMenu_Create arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDrivesDropTarget_Create" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDrivesDropTarget_Create arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CDrives_CreateSFVCB" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDrives_CreateSFVCB arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CFSDropTarget_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CFSDropTarget_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CFSFolderCallback_Create" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CFSFolderCallback_Create arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CFillPropertiesTask_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CFillPropertiesTask_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CLibraryDropTarget_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CLibraryDropTarget_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CLocationContextMenu_Create" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CLocationContextMenu_Create arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CLocationFolderUI_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CLocationFolderUI_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CMountPoint_DoAutorun" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_DoAutorun arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CMountPoint_DoAutorunPrompt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_DoAutorunPrompt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CMountPoint_IsAutoRunDriveAndEnabledByPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_IsAutoRunDriveAndEnabledByPolicy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CMountPoint_ProcessAutoRunFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_ProcessAutoRunFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CMountPoint_WantAutorunUI" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_WantAutorunUI arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CMountPoint_WantAutorunUIGetReady" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_WantAutorunUIGetReady arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CNetFolderUI_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CNetFolderUI_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CPL_CategoryIdArrayFromVariant" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CPL_CategoryIdArrayFromVariant arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CPL_IsLegacyCanonicalNameListedUnderKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CPL_IsLegacyCanonicalNameListedUnderKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CPL_ModifyWowDisplayName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CPL_ModifyWowDisplayName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CRecentDocsContextMenu_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CRecentDocsContextMenu_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CTransferConfirmation_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CTransferConfirmation_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CallFileCopyHooks" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CallFileCopyHooks h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CanDisplayWin8CopyDialog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CanDisplayWin8CopyDialog arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CloseAutoplayPrompt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CloseAutoplayPrompt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CommandLineFromMsiDescriptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CommandLineFromMsiDescriptor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CopySecondaryTiles" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CopySecondaryTiles arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CreateConfirmationInterrupt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateConfirmationInterrupt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CreateConflictInterrupt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateConflictInterrupt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CreateDefaultOperationDataProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateDefaultOperationDataProvider arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CreateFileFolderContextMenu" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateFileFolderContextMenu h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CreateLinkInfoW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!SHELL32_CreateLinkInfoW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CreateQosRecorder" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateQosRecorder arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_CreateSharePointView" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateSharePointView arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_Create_IEnumUICommand" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_Create_IEnumUICommand arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_DestroyLinkInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_DestroyLinkInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_EncryptDirectory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_EncryptDirectory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_EncryptedFileKeyInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_EncryptedFileKeyInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_EnumCommonTasks" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_EnumCommonTasks arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_FreeEncryptedFileKeyInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_FreeEncryptedFileKeyInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GenerateAppID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GenerateAppID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetAppIDRoot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetAppIDRoot arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetCommandProviderForFolderType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetCommandProviderForFolderType arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetDPIAdjustedLogicalSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetDPIAdjustedLogicalSize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetDiskCleanupPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetDiskCleanupPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetFileNameFromBrowse" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetFileNameFromBrowse h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetIconOverlayManager" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetIconOverlayManager arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetLinkInfoData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetLinkInfoData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetRatingBucket" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetRatingBucket arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetSqmableFileName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetSqmableFileName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetThumbnailAdornerFromFactory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetThumbnailAdornerFromFactory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_GetThumbnailAdornerFromFactory2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetThumbnailAdornerFromFactory2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_HandleUnrecognizedFileSystem" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_HandleUnrecognizedFileSystem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IconCacheCreate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCacheCreate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IconCacheDestroy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCacheDestroy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IconCacheHandleAssociationChanged" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCacheHandleAssociationChanged h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IconCacheRestore" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCacheRestore arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IconCache_AboutToExtractIcons" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCache_AboutToExtractIcons arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IconCache_DoneExtractingIcons" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCache_DoneExtractingIcons arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IconCache_ExpandEnvAndSearchPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCache_ExpandEnvAndSearchPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IconCache_RememberRecentlyExtractedIconsW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!SHELL32_IconCache_RememberRecentlyExtractedIconsW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IconOverlayManagerInit" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconOverlayManagerInit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IsGetKeyboardLayoutPresent" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IsGetKeyboardLayoutPresent h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IsSystemUpgradeInProgress" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IsSystemUpgradeInProgress arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_IsValidLinkInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IsValidLinkInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_LegacyEnumSpecialTasksByType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_LegacyEnumSpecialTasksByType arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_LegacyEnumTasks" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_LegacyEnumTasks arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_LookupBackIconIndex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_LookupBackIconIndex arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_LookupFrontIconIndex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_LookupFrontIconIndex arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_NormalizeRating" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_NormalizeRating arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_NotifyLinkTrackingServiceOfMove" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_NotifyLinkTrackingServiceOfMove arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_PifMgr_CloseProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PifMgr_CloseProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_PifMgr_GetProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PifMgr_GetProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_PifMgr_OpenProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PifMgr_OpenProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_PifMgr_SetProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PifMgr_SetProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_Printers_CreateBindInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_Printers_CreateBindInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_Printjob_GetPidl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_Printjob_GetPidl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_PurgeSystemIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PurgeSystemIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_RefreshOverlayImages" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_RefreshOverlayImages arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_ResolveLinkInfoW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!SHELL32_ResolveLinkInfoW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHAddSparseIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHAddSparseIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHCreateByValueOperationInterrupt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHCreateByValueOperationInterrupt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHCreateDefaultContextMenu" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHCreateDefaultContextMenu arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHCreateLocalServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHCreateLocalServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHCreateShellFolderView" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHCreateShellFolderView arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHDuplicateEncryptionInfoFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHDuplicateEncryptionInfoFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHEncryptFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHEncryptFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHFormatDriveAsync" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHFormatDriveAsync arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHGetThreadUndoManager" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHGetThreadUndoManager h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHGetUserNameW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHELL32_SHGetUserNameW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHIsVirtualDevice" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHIsVirtualDevice arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHLaunchPropSheet" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHLaunchPropSheet arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHLogILFromFSIL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHLogILFromFSIL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHOpenWithDialog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHOpenWithDialog arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHStartNetConnectionDialogW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!SHELL32_SHStartNetConnectionDialogW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SHUICommandFromGUID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHUICommandFromGUID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SendToMenu_InvokeTargetedCommand" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SendToMenu_InvokeTargetedCommand arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SendToMenu_VerifyTargetedCommand" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SendToMenu_VerifyTargetedCommand arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_ShowHideIconOnlyOnDesktop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_ShowHideIconOnlyOnDesktop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SimpleRatingToFilterCondition" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SimpleRatingToFilterCondition arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_StampIconForFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_StampIconForFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_SuspendUndo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SuspendUndo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_TryVirtualDiscImageDriveEject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_TryVirtualDiscImageDriveEject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHELL32_VerifySaferTrust" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_VerifySaferTrust arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHEmptyRecycleBinA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHEmptyRecycleBinA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHEmptyRecycleBinW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHEmptyRecycleBinW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHEnableServiceObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHEnableServiceObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHEnumerateUnreadMailAccountsW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!SHEnumerateUnreadMailAccountsW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHEvaluateSystemCommandTemplate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHEvaluateSystemCommandTemplate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHExecuteErrorMessageBox" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHExecuteErrorMessageBox arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHExtractIconsW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHExtractIconsW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHFileOperation" => {
            let file_op = emu.regs().rcx;
            let func = if file_op != 0 {
                emu.maps.read_dword(file_op + 4).unwrap_or(0)
            } else {
                0
            };
            let from_ptr = if file_op != 0 {
                emu.maps.read_dword(file_op + 8).unwrap_or(0) as u64
            } else {
                0
            };
            let from = if from_ptr != 0 {
                emu.maps.read_string(from_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "shell32!SHFileOperation op:{} from:'{}' ptr:0x{:x}",
                func,
                from,
                file_op
            );
            emu.regs_mut().rax = 0;
        }
        "SHFileOperationA" => {
            let file_op = emu.regs().rcx;
            let func = if file_op != 0 {
                emu.maps.read_dword(file_op + 8).unwrap_or(0)
            } else {
                0
            };
            let from_ptr = if file_op != 0 {
                emu.maps.read_qword(file_op + 16).unwrap_or(0)
            } else {
                0
            };
            let from = if from_ptr != 0 {
                emu.maps.read_string(from_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "shell32!SHFileOperationA op:{} from:'{}' ptr:0x{:x}",
                func,
                from,
                file_op
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SHFileOperationW" => {
            let file_op = emu.regs().rcx;
            let func = if file_op != 0 {
                emu.maps.read_dword(file_op + 8).unwrap_or(0)
            } else {
                0
            };
            let from_ptr = if file_op != 0 {
                emu.maps.read_qword(file_op + 16).unwrap_or(0)
            } else {
                0
            };
            let from = if from_ptr != 0 {
                emu.maps.read_wide_string(from_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "shell32!SHFileOperationW op:{} from:'{}' ptr:0x{:x}",
                func,
                from,
                file_op
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SHFindFiles" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHFindFiles h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHFind_InitMenuPopup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHFind_InitMenuPopup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shfind_initmenupopup");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHFlushSFCache" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHFlushSFCache arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHFormatDrive" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHFormatDrive arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!SHFree arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHFreeNameMappings" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "shell32!SHFreeNameMappings handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetAttributesFromDataObject" => {
            let pdtobj = emu.regs().rcx;
            let dw_attr_mask = emu.regs().rdx;
            let pdw_attr = emu.regs().r8;
            let pc_items = emu.regs().r9;
            log_red!(
                emu,
                "shell32!SHGetAttributesFromDataObject obj:0x{:x} mask:0x{:x}",
                pdtobj,
                dw_attr_mask
            );
            if pdw_attr != 0 {
                emu.maps.write_dword(pdw_attr, dw_attr_mask as u32);
            }
            if pc_items != 0 {
                emu.maps.write_dword(pc_items, 1);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetDataFromIDListA" => {
            let str_ptr = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "shell32!SHGetDataFromIDListA '{}'  0x{:x}", s, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetDataFromIDListW" => {
            let str_ptr = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "shell32!SHGetDataFromIDListW '{}'  0x{:x}", s, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetDesktopFolder" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetDesktopFolder arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetdesktopfolder");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetDiskFreeSpaceA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetDiskFreeSpaceA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetDiskFreeSpaceExA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetDiskFreeSpaceExA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetDiskFreeSpaceExW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetDiskFreeSpaceExW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetDriveMedia" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetDriveMedia arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetFileInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetFileInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetFileInfoA" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!SHGetFileInfoA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("shgetfileinfoa");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetFileInfoW" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!SHGetFileInfoW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("shgetfileinfow");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetFolderLocation" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetFolderLocation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetfolderlocation");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetFolderPathA" => SHGetFolderPathA(emu),
        "SHGetFolderPathAndSubDirA" => {
            let hwnd = emu.regs().rcx;
            let csidl = emu.regs().rdx as i32;
            let htoken = emu.regs().r8;
            let dw_flags = emu.regs().r9;
            let psz_sub_dir = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let psz_path = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let sub_dir = if psz_sub_dir != 0 {
                emu.maps.read_string(psz_sub_dir)
            } else {
                String::new()
            };
            let path = format!("C:\\Users\\user\\AppData\\{}", sub_dir);
            if psz_path != 0 {
                emu.maps.write_string(psz_path, &path);
            }
            log_red!(
                emu,
                "shell32!SHGetFolderPathAndSubDirA csidl:0x{:x} sub:'{}' -> '{}'",
                csidl,
                sub_dir,
                path
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SHGetFolderPathAndSubDirW" => {
            let hwnd = emu.regs().rcx;
            let csidl = emu.regs().rdx as i32;
            let htoken = emu.regs().r8;
            let dw_flags = emu.regs().r9;
            let psz_sub_dir = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let psz_path = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let sub_dir = if psz_sub_dir != 0 {
                emu.maps.read_wide_string(psz_sub_dir)
            } else {
                String::new()
            };
            let path = format!("C:\\Users\\user\\AppData\\{}", sub_dir);
            if psz_path != 0 {
                for (i, ch) in path.encode_utf16().enumerate() {
                    emu.maps.write_word(psz_path + (i as u64) * 2, ch);
                }
                emu.maps.write_word(psz_path + (path.len() as u64) * 2, 0);
            }
            log_red!(
                emu,
                "shell32!SHGetFolderPathAndSubDirW csidl:0x{:x} sub:'{}' -> '{}'",
                csidl,
                sub_dir,
                path
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SHGetFolderPathEx" => {
            let rfid = emu.regs().rcx;
            let dw_flags = emu.regs().rdx;
            let htoken = emu.regs().r8;
            let psz_path = emu.regs().r9;
            let cch_path = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;
            let path = "C:\\Users\\user";
            if psz_path != 0 {
                for (i, ch) in path.encode_utf16().enumerate() {
                    emu.maps.write_word(psz_path + (i as u64) * 2, ch);
                }
                emu.maps.write_word(psz_path + (path.len() as u64) * 2, 0);
            }
            log_red!(emu, "shell32!SHGetFolderPathEx -> '{}'", path);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetIDListFromObject" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetIDListFromObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetidlistfromobject");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetIconOverlayIndexA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetIconOverlayIndexA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetIconOverlayIndexW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetIconOverlayIndexW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetImageList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetImageList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetimagelist");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetInstanceExplorer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetInstanceExplorer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetItemFromDataObject" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetItemFromDataObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetitemfromdataobject");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetItemFromObject" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetItemFromObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetitemfromobject");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetKnownFolderIDList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetKnownFolderIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetknownfolderidlist");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetKnownFolderItem" => {
            let rfid = emu.regs().rcx;
            let flags = emu.regs().rdx;
            let htoken = emu.regs().r8;
            let riid = emu.regs().r9;
            let ppv = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let hndl = helper::handler_create("IShellItem://knownfolder");
            if ppv != 0 {
                emu.maps.write_qword(ppv, hndl);
            }
            log_red!(
                emu,
                "shell32!SHGetKnownFolderItem flags:0x{:x} =0x{:x}",
                flags,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetKnownFolderPath" => {
            let folder_id_ptr = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let token = emu.regs().r8;
            let path_out_ptr = emu.regs().r9;
            let path = "C:\\Users\\mwemu_user\\AppData\\Local";
            let alloc_addr = emu
                .maps
                .alloc(512)
                .expect("shell32!SHGetKnownFolderPath alloc");
            emu.maps.write_wide_string(alloc_addr, path);
            if path_out_ptr != 0 {
                emu.maps.write_qword(path_out_ptr, alloc_addr);
            }
            log_red!(
                emu,
                "shell32!SHGetKnownFolderPath flags:0x{:x} => '{}'",
                flags,
                path
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SHGetLocalizedName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetLocalizedName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetMalloc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!SHGetMalloc arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {}
        }
        "SHGetNameFromIDList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetNameFromIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetnamefromidlist");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetNewLinkInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetNewLinkInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetNewLinkInfoA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetNewLinkInfoA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetNewLinkInfoW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetNewLinkInfoW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHGetPathFromIDList" => {
            let pidl = emu.regs().rcx;
            let psz_path = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetPathFromIDList pidl:0x{:x} path:0x{:x}",
                pidl,
                psz_path
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            if psz_path != 0 {
                emu.maps.write_string(psz_path, "C:\\Users\\User\\Desktop");
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0;
            }
        }
        "SHGetPathFromIDListA" => {
            let pidl = emu.regs().rcx;
            let psz_path = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetPathFromIDListA pidl:0x{:x} path:0x{:x}",
                pidl,
                psz_path
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            if psz_path != 0 {
                emu.maps.write_string(psz_path, "C:\\Users\\User\\Desktop");
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0;
            }
        }
        "SHGetPathFromIDListEx" => {
            let pidl = emu.regs().rcx;
            let psz_path = emu.regs().rdx;
            let cch_path = emu.regs().r8;
            log_red!(
                emu,
                "shell32!SHGetPathFromIDListEx pidl:0x{:x} path:0x{:x} cch:{}",
                pidl,
                psz_path,
                cch_path
            );
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            if psz_path != 0 && cch_path > 0 {
                emu.maps
                    .write_wide_string(psz_path, "C:\\Users\\User\\Desktop");
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0;
            }
        }
        "SHGetPathFromIDListW" => {
            let pidl = emu.regs().rcx;
            let psz_path = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetPathFromIDListW pidl:0x{:x} path:0x{:x}",
                pidl,
                psz_path
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            if psz_path != 0 {
                emu.maps
                    .write_wide_string(psz_path, "C:\\Users\\User\\Desktop");
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0;
            }
        }
        "SHGetPropertyStoreForWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetPropertyStoreForWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetpropertystoreforwindow");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetPropertyStoreFromIDList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetPropertyStoreFromIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetpropertystorefromidlist");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetPropertyStoreFromParsingName" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetPropertyStoreFromParsingName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetpropertystorefromparsingname");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetRealIDL" => {
            let psf = emu.regs().rcx;
            let pidl_simple = emu.regs().rdx;
            let ppidl_real = emu.regs().r8;
            let hndl = helper::handler_create("pidl://real");
            if ppidl_real != 0 {
                emu.maps.write_qword(ppidl_real, hndl);
            }
            log_red!(emu, "shell32!SHGetRealIDL =0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetSetFolderCustomSettings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetSetFolderCustomSettings arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetSetSettings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetSetSettings arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetSettings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetSettings arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetSpecialFolderLocation" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetSpecialFolderLocation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetspecialfolderlocation");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetSpecialFolderPathA" => SHGetSpecialFolderPathA(emu),
        "SHGetSpecialFolderPathW" => SHGetSpecialFolderPathW(emu),
        "SHGetStockIconInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHGetStockIconInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetstockiconinfo");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHGetTemporaryPropertyForItem" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetTemporaryPropertyForItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHGetUnreadMailCountW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetUnreadMailCountW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHHandleUpdateImage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHHandleUpdateImage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHHelpShortcuts_RunDLL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHHelpShortcuts_RunDLL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHHelpShortcuts_RunDLLA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!SHHelpShortcuts_RunDLLA '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHHelpShortcuts_RunDLLW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!SHHelpShortcuts_RunDLLW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHILCreateFromPath" => {
            let psz_path = emu.regs().rcx;
            let ppidl = emu.regs().rdx;
            let rgf_in_out = emu.regs().r8;
            let path = emu.maps.read_wide_string(psz_path);
            let hndl = helper::handler_create(&format!("pidl://{}", path));
            if ppidl != 0 {
                emu.maps.write_qword(ppidl, hndl);
            }
            log_red!(emu, "shell32!SHILCreateFromPath '{}' =0x{:x}", path, hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHInvokePrinterCommandA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHInvokePrinterCommandA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHInvokePrinterCommandW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHInvokePrinterCommandW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHIsFileAvailableOffline" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHIsFileAvailableOffline arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHLimitInputEdit" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHLimitInputEdit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shlimitinputedit");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHLoadInProc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHLoadInProc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHLoadNonloadedIconOverlayIdentifiers" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHLoadNonloadedIconOverlayIdentifiers arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHMapPIDLToSystemImageListIndex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHMapPIDLToSystemImageListIndex arg1:0x{:x}",
                arg1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
            for _ in 0..1u32 {}
        }
        "SHMultiFileProperties" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHMultiFileProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shmultifileproperties");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHObjectProperties" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHObjectProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shobjectproperties");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHOpenFolderAndSelectItems" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHOpenFolderAndSelectItems arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shopenfolderandselectitems");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHOpenOrGetFolderView" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHOpenOrGetFolderView arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHOpenPropSheetW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHOpenPropSheetW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shopenpropsheetw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHOpenWithDialog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHOpenWithDialog arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHParseDisplayName" => {
            let psz_name = emu.regs().rcx;
            let pbc = emu.regs().rdx;
            let ppidl = emu.regs().r8;
            let sfgao_in = emu.regs().r9;
            let psfgao_out = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let name = emu.maps.read_wide_string(psz_name);
            let hndl = helper::handler_create(&format!("pidl://{}", name));
            if ppidl != 0 {
                emu.maps.write_qword(ppidl, hndl);
            }
            if psfgao_out != 0 {
                emu.maps.write_dword(psfgao_out, sfgao_in as u32);
            }
            log_red!(emu, "shell32!SHParseDisplayName '{}' =0x{:x}", name, hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHPathPrepareForWriteA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHPathPrepareForWriteA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHPathPrepareForWriteW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHPathPrepareForWriteW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHPropStgCreate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHPropStgCreate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHPropStgReadMultiple" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHPropStgReadMultiple arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHPropStgWriteMultiple" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHPropStgWriteMultiple arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHQueryRecycleBinA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHQueryRecycleBinA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHQueryRecycleBinW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHQueryRecycleBinW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHQueryUserNotificationState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHQueryUserNotificationState arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHRemoveLocalizedName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHRemoveLocalizedName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHReplaceFromPropSheetExtArray" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHReplaceFromPropSheetExtArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHResolveLibrary" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHResolveLibrary arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHRestricted" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHRestricted arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHSetDefaultProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHSetDefaultProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SHSetFolderPathA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!SHSetFolderPathA arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {}
        }
        "SHSetFolderPathW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!SHSetFolderPathW arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {}
        }
        "SHSetInstanceExplorer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHSetInstanceExplorer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHSetKnownFolderPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHSetKnownFolderPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHSetLocalizedName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHSetLocalizedName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHSetTemporaryPropertyForItem" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHSetTemporaryPropertyForItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHSetUnreadMailCountW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHSetUnreadMailCountW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHShellFolderView_Message" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHShellFolderView_Message arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHShowManageLibraryUI" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHShowManageLibraryUI arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHSimpleIDListFromPath" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHSimpleIDListFromPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shsimpleidlistfrompath");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHStartNetConnectionDialogW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHStartNetConnectionDialogW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shstartnetconnectiondialogw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHTestTokenMembership" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHTestTokenMembership arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHUpdateImageA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHUpdateImageA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHUpdateImageW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHUpdateImageW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SHUpdateRecycleBinIcon" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!SHUpdateRecycleBinIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shupdaterecyclebinicon");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "SHValidateUNC" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHValidateUNC arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SetCurrentProcessExplicitAppUserModelID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SetCurrentProcessExplicitAppUserModelID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "SheChangeDirA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SheChangeDirA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SheChangeDirExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SheChangeDirExW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SheGetDirA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SheGetDirA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SheSetCurDrive" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SheSetCurDrive arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellAboutA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ShellAboutA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ShellAboutW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ShellAboutW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ShellExec_RunDLL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ShellExec_RunDLL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ShellExec_RunDLLA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ShellExec_RunDLLA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ShellExec_RunDLLW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ShellExec_RunDLLW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ShellExecuteEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!ShellExecuteEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellExecuteExA" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!ShellExecuteExA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("shellexecuteexa");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ShellExecuteExW" => {
            let str_ptr = emu.regs().rcx;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.regs().rdx;
            log_red!(emu, "shell32!ShellExecuteExW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("shellexecuteexw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "ShellHookProc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ShellHookProc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "ShellMessageBoxA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!ShellMessageBoxA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellMessageBoxW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!ShellMessageBoxW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Shell_GetCachedImageIndex" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!Shell_GetCachedImageIndex arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shell_getcachedimageindex");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "Shell_GetCachedImageIndexA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!Shell_GetCachedImageIndexA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shell_getcachedimageindexa");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "Shell_GetCachedImageIndexW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "shell32!Shell_GetCachedImageIndexW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shell_getcachedimageindexw");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "Shell_GetImageLists" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "shell32!Shell_GetImageLists arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
            for _ in 0..1u32 {}
        }
        "Shell_MergeMenus" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!Shell_MergeMenus arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Shell_NotifyIcon" => {
            let dw_message = emu.regs().rcx as u32;
            let lp_data = emu.regs().rdx;
            let tip = if lp_data != 0 {
                emu.maps.read_string(lp_data + 88)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "shell32!Shell_NotifyIcon msg:{} tip:'{}' data:0x{:x}",
                dw_message,
                tip,
                lp_data
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Shell_NotifyIconA" => {
            let dw_message = emu.regs().rcx as u32;
            let lp_data = emu.regs().rdx;
            let tip = if lp_data != 0 {
                emu.maps.read_string(lp_data + 88)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "shell32!Shell_NotifyIconA msg:{} tip:'{}' data:0x{:x}",
                dw_message,
                tip,
                lp_data
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Shell_NotifyIconGetRect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Shell_NotifyIconGetRect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "Shell_NotifyIconW" => {
            let dw_message = emu.regs().rcx as u32;
            let lp_data = emu.regs().rdx;
            let tip = if lp_data != 0 {
                emu.maps.read_wide_string(lp_data + 88)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "shell32!Shell_NotifyIconW msg:{} tip:'{}' data:0x{:x}",
                dw_message,
                tip,
                lp_data
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SignalFileOpen" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SignalFileOpen arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "StateRepoNewMenuCache_EnsureCacheAsync" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!StateRepoNewMenuCache_EnsureCacheAsync arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StateRepoNewMenuCache_RebuildCacheAsync" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!StateRepoNewMenuCache_RebuildCacheAsync arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StgMakeUniqueName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!StgMakeUniqueName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "StrChrA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrChrA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrChrIA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrChrIA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrChrIW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrChrIW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrChrW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrChrW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrCmpNA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrCmpNA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrCmpNIA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrCmpNIA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrCmpNIW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrCmpNIW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrCmpNW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrCmpNW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrNCmpA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrNCmpA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrNCmpIA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrNCmpIA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrNCmpIW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrNCmpIW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrNCmpW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrNCmpW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrRChrA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRChrA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrRChrIA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRChrIA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrRChrIW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRChrIW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrRChrW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRChrW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrRStrA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRStrA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrRStrIA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRStrIA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrRStrIW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRStrIW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrRStrW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRStrW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrStrA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrStrA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrStrIA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrStrIA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrStrIW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrStrIW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StrStrW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrStrW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UsersLibrariesFolderUI_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!UsersLibrariesFolderUI_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {}
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WOWShellExecute" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!WOWShellExecute arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "WaitForExplorerRestartW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "shell32!WaitForExplorerRestartW '{}'  p1:0x{:x}",
                s,
                p1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Win32DeleteFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Win32DeleteFile arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {}
        }
        "WriteCabinetState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!WriteCabinetState arg1:0x{:x} arg2:0x{:x}",
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
                emu.regs_mut().rax = 42;
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

pub fn ShellExecuteA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let lp_operation = emu.regs().rdx;
    let lp_file = emu.regs().r8;
    let lp_parameters = emu.regs().r9;
    let lp_directory = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let n_show_cmd = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let operation = if lp_operation != 0 {
        emu.maps.read_string(lp_operation)
    } else {
        "open".to_string()
    };
    let file = emu.maps.read_string(lp_file);
    let params = if lp_parameters != 0 {
        emu.maps.read_string(lp_parameters)
    } else {
        "".to_string()
    };

    log_red!(
        emu,
        "shell32!ShellExecuteA op: {} file: {} params: {}",
        operation,
        file,
        params
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 42; // > 32 means success
}

pub fn ShellExecuteW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let lp_operation = emu.regs().rdx;
    let lp_file = emu.regs().r8;
    let lp_parameters = emu.regs().r9;
    let lp_directory = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let n_show_cmd = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let operation = if lp_operation != 0 {
        emu.maps.read_wide_string(lp_operation)
    } else {
        "open".to_string()
    };
    let file = emu.maps.read_wide_string(lp_file);
    let params = if lp_parameters != 0 {
        emu.maps.read_wide_string(lp_parameters)
    } else {
        "".to_string()
    };

    log_red!(
        emu,
        "shell32!ShellExecuteW op: {} file: {} params: {}",
        operation,
        file,
        params
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 42; // > 32 means success
}

pub fn RealShellExecuteA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let operation = emu.regs().rdx;
    let file_ptr = emu.regs().r8;
    let params_ptr = emu.regs().r9;
    let dir = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let bShowWindow = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let file = emu.maps.read_string(file_ptr);
    let params = emu.maps.read_string(params_ptr);

    log_red!(emu, "shell32!RealShellExecuteA {} {}", file, params);

    emu.set_last_error(0);
    emu.regs_mut().rax = 34;
}

pub fn SHGetFolderPathW(emu: &mut emu::Emu) {
    /*
    HRESULT SHGetFolderPathW(
        [in]  HWND   hwnd,
        [in]  int    csidl,
        [in]  HANDLE hToken,
        [in]  DWORD  dwFlags,
        [out] LPWSTR pszPath
    );
    */
    let hwnd = emu.regs().rcx;
    let csidl = emu.regs().rdx as i32;
    let h_token = emu.regs().r8;
    let dw_flags = emu.regs().r9 as u32;
    let psz_path = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(
        emu,
        "shell32!SHGetFolderPathW hwnd: 0x{:x} csidl: 0x{:x} token: 0x{:x} flags: 0x{:x} path: 0x{:x}",
        hwnd,
        csidl,
        h_token,
        dw_flags,
        psz_path
    );

    // Simple folder path mapping
    let folder_path = match csidl & 0xFFFF {
        0x0005 => "C:\\Users\\User\\Documents", // CSIDL_PERSONAL
        0x001a => "C:\\Users\\User\\AppData\\Roaming", // CSIDL_APPDATA
        0x001c => "C:\\Users\\User\\AppData\\Local", // CSIDL_LOCAL_APPDATA
        0x0020 => "C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\INetCache", // CSIDL_INTERNET_CACHE
        0x0021 => "C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\INetCookies", // CSIDL_COOKIES
        0x0022 => "C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\History", // CSIDL_HISTORY
        0x0023 => "C:\\ProgramData",           // CSIDL_COMMON_APPDATA
        0x0024 => "C:\\Windows",               // CSIDL_WINDOWS
        0x0025 => "C:\\Windows\\System32",     // CSIDL_SYSTEM
        0x0026 => "C:\\Program Files",         // CSIDL_PROGRAM_FILES
        0x0027 => "C:\\Users\\User\\Pictures", // CSIDL_MYPICTURES
        0x0028 => "C:\\Users\\User",           // CSIDL_PROFILE
        0x0029 => "C:\\Windows\\SysWOW64",     // CSIDL_SYSTEMX86
        0x002a => "C:\\Program Files (x86)",   // CSIDL_PROGRAM_FILESX86
        0x002b => "C:\\Program Files\\Common Files", // CSIDL_PROGRAM_FILES_COMMON
        0x002c => "C:\\Program Files (x86)\\Common Files", // CSIDL_PROGRAM_FILES_COMMONX86
        0x002d => "C:\\ProgramData\\Microsoft\\Windows\\Templates", // CSIDL_COMMON_TEMPLATES
        0x002e => "C:\\Users\\Public\\Documents", // CSIDL_COMMON_DOCUMENTS
        0x002f => "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Administrative Tools", // CSIDL_COMMON_ADMINTOOLS
        0x0030 => {
            "C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Administrative Tools"
        } // CSIDL_ADMINTOOLS
        0x0031 => "C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Network Shortcuts", // CSIDL_CONNECTIONS
        0x0035 => "C:\\Users\\Public\\Music", // CSIDL_COMMON_MUSIC
        0x0036 => "C:\\Users\\Public\\Pictures", // CSIDL_COMMON_PICTURES
        0x0037 => "C:\\Users\\Public\\Videos", // CSIDL_COMMON_VIDEO
        0x0038 => "C:\\Windows\\Resources",   // CSIDL_RESOURCES
        _ => {
            log::warn!(
                "SHGetFolderPathW: unsupported CSIDL 0x{:x} at {}",
                csidl,
                emu.pos
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x80070002; // E_FAIL / path not found
            return;
        }
    };

    if psz_path != 0 {
        emu.maps.write_wide_string(psz_path, folder_path);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

fn csidl_to_path(csidl: i32) -> Option<&'static str> {
    match csidl & 0xFFFF {
        0x0000 => Some("C:\\Users\\User\\Desktop"),
        0x0002 => {
            Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs")
        }
        0x0005 => Some("C:\\Users\\User\\Documents"),
        0x0006 => Some("C:\\Users\\User\\Favorites"),
        0x0007 => Some(
            "C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup",
        ),
        0x0008 => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Recent"),
        0x0009 => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\SendTo"),
        0x000b => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu"),
        0x000d => Some("C:\\Users\\User\\Music"),
        0x000e => Some("C:\\Users\\User\\Videos"),
        0x0010 => Some("C:\\Users\\User\\Desktop"),
        0x0013 => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Network Shortcuts"),
        0x0014 => Some("C:\\Windows\\Fonts"),
        0x0015 => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Templates"),
        0x0016 => Some("C:\\ProgramData\\Microsoft\\Windows\\Start Menu"),
        0x0017 => Some("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs"),
        0x0018 => Some("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup"),
        0x0019 => Some("C:\\Users\\Public\\Desktop"),
        0x001a => Some("C:\\Users\\User\\AppData\\Roaming"),
        0x001b => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Printer Shortcuts"),
        0x001c => Some("C:\\Users\\User\\AppData\\Local"),
        0x001d => Some(
            "C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup",
        ),
        0x001f => Some("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup"),
        0x0020 => Some("C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\INetCache"),
        0x0021 => Some("C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\INetCookies"),
        0x0022 => Some("C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\History"),
        0x0023 => Some("C:\\ProgramData"),
        0x0024 => Some("C:\\Windows"),
        0x0025 => Some("C:\\Windows\\System32"),
        0x0026 => Some("C:\\Program Files"),
        0x0027 => Some("C:\\Users\\User\\Pictures"),
        0x0028 => Some("C:\\Users\\User"),
        0x0029 => Some("C:\\Windows\\SysWOW64"),
        0x002a => Some("C:\\Program Files (x86)"),
        0x002b => Some("C:\\Program Files\\Common Files"),
        0x002c => Some("C:\\Program Files (x86)\\Common Files"),
        0x002d => Some("C:\\ProgramData\\Microsoft\\Windows\\Templates"),
        0x002e => Some("C:\\Users\\Public\\Documents"),
        0x0035 => Some("C:\\Users\\Public\\Music"),
        0x0036 => Some("C:\\Users\\Public\\Pictures"),
        0x0037 => Some("C:\\Users\\Public\\Videos"),
        0x0038 => Some("C:\\Windows\\Resources"),
        0x003b => Some(
            "C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\Temporary Internet Files\\Low",
        ),
        _ => None,
    }
}

pub fn SHGetFolderPathA(emu: &mut emu::Emu) {
    let csidl = emu.regs().rdx as i32;
    let psz_path = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(emu, "shell32!SHGetFolderPathA csidl: 0x{:x}", csidl);

    match csidl_to_path(csidl) {
        Some(path) => {
            if psz_path != 0 {
                emu.maps.write_string(psz_path, path);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 42; // S_OK
        }
        None => {
            log::warn!("SHGetFolderPathA: unsupported CSIDL 0x{:x}", csidl);
            emu.regs_mut().rax = 0x80070002;
        }
    }
}

pub fn SHGetSpecialFolderPathA(emu: &mut emu::Emu) {
    let psz_path = emu.regs().rdx;
    let csidl = emu.regs().r8 as i32;

    log_red!(emu, "shell32!SHGetSpecialFolderPathA csidl: 0x{:x}", csidl);

    match csidl_to_path(csidl) {
        Some(path) => {
            if psz_path != 0 {
                emu.maps.write_string(psz_path, path);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // TRUE
        }
        None => {
            emu.regs_mut().rax = 42; // FALSE
        }
    }
}

pub fn SHGetSpecialFolderPathW(emu: &mut emu::Emu) {
    let psz_path = emu.regs().rdx;
    let csidl = emu.regs().r8 as i32;

    log_red!(emu, "shell32!SHGetSpecialFolderPathW csidl: 0x{:x}", csidl);

    match csidl_to_path(csidl) {
        Some(path) => {
            if psz_path != 0 {
                emu.maps.write_wide_string(psz_path, path);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // TRUE
        }
        None => {
            emu.regs_mut().rax = 42; // FALSE
        }
    }
}

pub fn CommandLineToArgvW(emu: &mut emu::Emu) {
    let cmd_line_ptr = emu.regs().rcx;
    let argc_ptr = emu.regs().rdx;

    let cmd = if cmd_line_ptr != 0 {
        emu.maps.read_wide_string(cmd_line_ptr)
    } else {
        String::new()
    };

    log_red!(emu, "shell32!CommandLineToArgvW cmd: '{}'", cmd);

    // Allocate space for the argv array + string data
    let alloc_size = 1024u64;
    let addr = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            emu.set_last_error(0);
            emu.regs_mut().rax = 42;
            return;
        }
    };
    emu.maps.create_map(
        "cmdline_argv",
        addr,
        alloc_size,
        crate::maps::mem64::Permission::READ_WRITE,
    );

    // Write the command string as a single argv[0] wide string
    let str_offset = 64u64;
    emu.maps.write_wide_string(addr + str_offset, &cmd);
    // argv[0] pointer
    emu.maps.write_qword(addr, addr + str_offset);

    if argc_ptr != 0 {
        emu.maps.write_dword(argc_ptr, 1); // 1 argument
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}
