use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "AssocCreateForClasses" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!AssocCreateForClasses arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("assoccreateforclasses");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "AssocElemCreateForKey" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!AssocElemCreateForKey arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("assocelemcreateforkey");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "CDefFolderMenu_Create2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CDefFolderMenu_Create2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("cdeffoldermenu_create2");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CIDLData_CreateFromIDArray" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CIDLData_CreateFromIDArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("cidldata_createfromidarray");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "CallFileCopyHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CallFileCopyHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CallFileCopyHook");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CommandLineToArgvW" => {
            let cmd_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let argc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cmd = emu.maps.read_wide_string(cmd_ptr);
            if argc_ptr != 0 {
                emu.maps.write_dword(argc_ptr, 1);
            }
            let argv_alloc = emu
                .maps
                .alloc(256)
                .expect("shell32!CommandLineToArgvW alloc");
            emu.maps.write_dword(argv_alloc, (argv_alloc + 4) as u32);
            emu.maps.write_wide_string(argv_alloc + 4, &cmd);
            log_red!(
                emu,
                "shell32!CommandLineToArgvW cmd:'{}' => 0x{:x}",
                cmd,
                argv_alloc
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = argv_alloc;
        }
        "Control_RunDLL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Control_RunDLL arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateStorageItemFromPath_FullTrustCaller" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CreateStorageItemFromPath_FullTrustCaller arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("createstorageitemfrompath_fulltrustcaller");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateStorageItemFromPath_FullTrustCaller_ForPackage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CreateStorageItemFromPath_FullTrustCaller_ForPackage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle =
                helper::handler_create("createstorageitemfrompath_fulltrustcaller_forpackage");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateStorageItemFromPath_PartialTrustCaller" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CreateStorageItemFromPath_PartialTrustCaller arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("createstorageitemfrompath_partialtrustcaller");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateStorageItemFromShellItem_FullTrustCaller" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CreateStorageItemFromShellItem_FullTrustCaller arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("createstorageitemfromshellitem_fulltrustcaller");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateStorageItemFromShellItem_FullTrustCaller_ForPackage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CreateStorageItemFromShellItem_FullTrustCaller_ForPackage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle =
                helper::handler_create("createstorageitemfromshellitem_fulltrustcaller_forpackage");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create(
                "createstorageitemfromshellitem_fulltrustcaller_forpackage_withprocesshandle",
            );
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create(
                "createstorageitemfromshellitem_fulltrustcaller_useimplicitflagsandpackage",
            );
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("DAD_DragEnterEx");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("DAD_DragEnterEx2");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "DllGetActivationFactory" => {
            let class_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let factory_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "shell32!DllGetActivationFactory");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x80004001u64;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("DllRegisterServer");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "DoEnvironmentSubstA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!DoEnvironmentSubstA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("doenvironmentsubsta");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "DoEnvironmentSubstW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!DoEnvironmentSubstW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("doenvironmentsubstw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "DragFinish" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "shell32!DragFinish handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DragQueryFile" => {
            let hdrop = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DragQueryFile hdrop:0x{:x} index:{}",
                hdrop,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            if index == 0xFFFFFFFF {
                emu.regs_mut().rax = 1;
            } else if buf_ptr != 0 && buf_size > 10 {
                emu.maps.write_string(buf_ptr, "C:\\file.txt");
                emu.regs_mut().rax = 10;
            } else {
                emu.regs_mut().rax = 10;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "DragQueryFileA" => {
            let hdrop = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DragQueryFileA hdrop:0x{:x} index:{}",
                hdrop,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            if index == 0xFFFFFFFF {
                emu.regs_mut().rax = 1;
            } else if buf_ptr != 0 && buf_size > 10 {
                emu.maps.write_string(buf_ptr, "C:\\file.txt");
                emu.regs_mut().rax = 10;
            } else {
                emu.regs_mut().rax = 10;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "DragQueryFileAorW" => {
            let hdrop = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DragQueryFileAorW hdrop:0x{:x} index:{}",
                hdrop,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            if index == 0xFFFFFFFF {
                emu.regs_mut().rax = 1;
            } else if buf_ptr != 0 && buf_size > 10 {
                emu.maps.write_string(buf_ptr, "C:\\file.txt");
                emu.regs_mut().rax = 10;
            } else {
                emu.regs_mut().rax = 10;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "DragQueryFileW" => {
            let hdrop = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DragQueryFileW hdrop:0x{:x} index:{}",
                hdrop,
                index
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            if index == 0xFFFFFFFF {
                emu.regs_mut().rax = 1;
            } else if buf_ptr != 0 && buf_size > 10 {
                emu.maps.write_string(buf_ptr, "C:\\file.txt");
                emu.regs_mut().rax = 10;
            } else {
                emu.regs_mut().rax = 10;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "DragQueryPoint" => {
            let hdrop = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let point_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "shell32!DragQueryPoint hdrop:0x{:x}", hdrop);
            if point_ptr != 0 {
                emu.maps.write_dword(point_ptr, 0);
                emu.maps.write_dword(point_ptr + 4, 0);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "DriveType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "shell32!DriveType arg1:0x{:x}", arg1);
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DuplicateIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!DuplicateIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("duplicateicon");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ExtractAssociatedIconA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ExtractAssociatedIconA '{}' arg2:0x{:x}",
                s,
                arg2
            );
            let handle = helper::handler_create("extractassociatedicona");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ExtractAssociatedIconExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ExtractAssociatedIconExA '{}' arg2:0x{:x}",
                s,
                arg2
            );
            let handle = helper::handler_create("extractassociatediconexa");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ExtractAssociatedIconExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ExtractAssociatedIconExW '{}' arg2:0x{:x}",
                s,
                arg2
            );
            let handle = helper::handler_create("extractassociatediconexw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ExtractAssociatedIconW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ExtractAssociatedIconW '{}' arg2:0x{:x}",
                s,
                arg2
            );
            let handle = helper::handler_create("extractassociatediconw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ExtractIconA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!ExtractIconA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("extracticona");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ExtractIconEx" => {
            let file_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let icon_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
            let large_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let small_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
                emu.maps.write_dword(large_ptr, handle as u32);
            }
            if small_ptr != 0 {
                emu.maps.write_dword(small_ptr, handle as u32);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "ExtractIconExA" => {
            let file_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let icon_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
            let large_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let small_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
                emu.maps.write_dword(large_ptr, handle as u32);
            }
            if small_ptr != 0 {
                emu.maps.write_dword(small_ptr, handle as u32);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "ExtractIconExW" => {
            let file_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let icon_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
            let large_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let small_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
                emu.maps.write_dword(large_ptr, handle as u32);
            }
            if small_ptr != 0 {
                emu.maps.write_dword(small_ptr, handle as u32);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "ExtractIconW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!ExtractIconW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("extracticonw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FindExecutableA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!FindExecutableA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("findexecutablea");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FindExecutableW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!FindExecutableW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("findexecutablew");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "FreeIconList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "shell32!FreeIconList handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
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
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
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
        "GetSystemPersistedStorageItemList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!GetSystemPersistedStorageItemList h:0x{:x} arg2:0x{:x}",
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
        "ILAppendID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!ILAppendID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ILAppendID");
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
        "ILClone" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!ILClone arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("ilclone");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ILCloneFirst" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ILCloneFirst arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ilclonefirst");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ILCombine" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!ILCombine arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("ilcombine");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ILCreateFromPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ILCreateFromPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ilcreatefrompath");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ILCreateFromPathA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ILCreateFromPathA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ilcreatefrompatha");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ILCreateFromPathW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ILCreateFromPathW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ilcreatefrompathw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ILFindChild" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!ILFindChild arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ILFindLastID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!ILFindLastID arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ILFree" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "shell32!ILFree handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ILGetNext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!ILGetNext arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ILGetSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "shell32!ILGetSize arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ILIsEqual" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!ILIsEqual arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("ILLoadFromStreamEx");
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
        "ILRemoveLastID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ILRemoveLastID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("InitNetworkAddressControl");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "InternalExtractIconListA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternalExtractIconListW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
        "IsLFNDrive" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!IsLFNDrive arg1:0x{:x} arg2:0x{:x}",
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "IsUserAnAdmin" => {
            log_red!(emu, "shell32!IsUserAnAdmin => TRUE");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LaunchMSHelp_RunDLLW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!LaunchMSHelp_RunDLLW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("OpenAs_RunDLL");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("OpenAs_RunDLLA");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("OpenAs_RunDLLW");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "OpenRegStream" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "shell32!OpenRegStream arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            let nparams = emu.banzai.get_params("OpenRegStream");
            if nparams > 0 {
                for _ in 0..nparams {
                    emu.stack_pop32(false);
                }
            }
            let hndl = helper::handler_create("shell32://OpenRegStream");
            assert!(hndl < 0x00000001_00000000);
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
        "Options_RunDLLA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!Options_RunDLLA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "Options_RunDLLW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!Options_RunDLLW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "PathIsExe" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!PathIsExe arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "PifMgr_CloseProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!PifMgr_CloseProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("PifMgr_OpenProperties");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "PrintersGetCommand_RunDLLA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "RealShellExecuteA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!RealShellExecuteA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RealShellExecuteExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!RealShellExecuteExA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RealShellExecuteExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!RealShellExecuteExW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RealShellExecuteW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!RealShellExecuteW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "RunAsNewUser_RunDLLW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!RunAsNewUser_RunDLLW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHAddDefaultPropertiesByExt");
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
        "SHAddFromPropSheetExtArray" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHAddFromPropSheetExtArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHAddFromPropSheetExtArray");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHAddToRecentDocs");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHAlloc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!SHAlloc arg1:0x{:x}", arg1);
            let handle = helper::handler_create("SHAlloc");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHAssocEnumHandlers" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHAssocEnumHandlers arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shassocenumhandlers");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHAssocEnumHandlersForProtocolByApplication" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHAssocEnumHandlersForProtocolByApplication arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shassocenumhandlersforprotocolbyapplication");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHBindToFolderIDListParent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHBindToFolderIDListParent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbindtofolderidlistparent");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHBindToFolderIDListParentEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHBindToFolderIDListParentEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbindtofolderidlistparentex");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHBindToObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHBindToObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbindtoobject");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHBindToParent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHBindToParent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbindtoparent");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHBrowseForFolder" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHBrowseForFolder arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbrowseforfolder");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHBrowseForFolderA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHBrowseForFolderA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbrowseforfoldera");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHBrowseForFolderW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHBrowseForFolderW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shbrowseforfolderw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "SHChangeNotification_Lock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHChangeNotification_Lock arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHChangeNotification_Lock");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHChangeNotifyDeregister" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHChangeNotifyDeregister handle:0x{:x}",
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
        "SHChangeNotifyRegister" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHChangeNotifyRegister arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shchangenotifyregister");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHChangeNotifyRegisterThread");
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
        "SHChangeNotifySuspendResume" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHChangeNotifySuspendResume arg1:0x{:x} arg2:0x{:x}",
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
        "SHCloneSpecialIDList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHCloneSpecialIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHCloneSpecialIDList");
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
        "SHCoCreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCoCreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcocreateinstance");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCoCreateInstanceWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCoCreateInstanceWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcocreateinstanceworker");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateAssociationRegistration" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateAssociationRegistration arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateassociationregistration");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHCreateCategoryEnum");
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
        "SHCreateDataObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateDataObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedataobject");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateDefaultContextMenu" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateDefaultContextMenu arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedefaultcontextmenu");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateDefaultExtractIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateDefaultExtractIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedefaultextracticon");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateDefaultPropertiesOp" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateDefaultPropertiesOp arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedefaultpropertiesop");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateDirectory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateDirectory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedirectory");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateDirectoryExA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateDirectoryExA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedirectoryexa");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateDirectoryExW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateDirectoryExW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatedirectoryexw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHCreateDrvExtIcon");
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
        "SHCreateFileExtractIconW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHCreateItemFromIDList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateItemFromIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateitemfromidlist");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateItemFromParsingName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateItemFromParsingName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateitemfromparsingname");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateItemFromRelativeName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateItemFromRelativeName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateitemfromrelativename");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateItemInKnownFolder" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateItemInKnownFolder arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateiteminknownfolder");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateItemWithParent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateItemWithParent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateitemwithparent");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateLocalServerRunDll" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateLocalServerRunDll arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatelocalserverrundll");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHCreateProcessAsUserW");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreatePropSheetExtArray" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreatePropSheetExtArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatepropsheetextarray");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateQueryCancelAutoPlayMoniker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateQueryCancelAutoPlayMoniker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatequerycancelautoplaymoniker");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateShellFolderView" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateShellFolderView arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellfolderview");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateShellFolderViewEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateShellFolderViewEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellfolderviewex");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateShellItem" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateShellItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitem");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateShellItemArray" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateShellItemArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitemarray");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateShellItemArrayFromDataObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateShellItemArrayFromDataObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitemarrayfromdataobject");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateShellItemArrayFromIDLists" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateShellItemArrayFromIDLists arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitemarrayfromidlists");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateShellItemArrayFromShellItem" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateShellItemArrayFromShellItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreateshellitemarrayfromshellitem");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHCreateStdEnumFmtEtc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHCreateStdEnumFmtEtc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shcreatestdenumfmtetc");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHDefExtractIconA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHDefExtractIconA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHDefExtractIconW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHDefExtractIconW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHDestroyPropSheetExtArray" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHDestroyPropSheetExtArray handle:0x{:x}",
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
        "SHDoDragDrop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHDoDragDrop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHELL32_AddToBackIconTable");
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
        "SHELL32_AddToFrontIconTable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_AddToFrontIconTable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_AddToFrontIconTable");
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
        "SHELL32_AreAllItemsAvailable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_AreAllItemsAvailable arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CCommonPlacesFolder_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CCommonPlacesFolder_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CCommonPlacesFolder_CreateInstance");
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
        "SHELL32_CDBurn_CloseSession" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_CloseSession arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_DriveSupportedForDataBurn" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_DriveSupportedForDataBurn arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_Erase" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_Erase arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_GetCDInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_GetCDInfo arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_GetLiveFSDiscInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_GetLiveFSDiscInfo arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_GetStagingPathOrNormalPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_GetStagingPathOrNormalPath arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_GetTaskInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_GetTaskInfo arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_IsBlankDisc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_IsBlankDisc arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_IsBlankDisc2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_IsBlankDisc2 arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_IsLiveFS" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_IsLiveFS arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_OnDeviceChange" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_OnDeviceChange arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_OnEject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_OnEject arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDBurn_OnMediaChange" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDBurn_OnMediaChange arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDefFolderMenu_Create2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDefFolderMenu_Create2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CDefFolderMenu_Create2");
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
        "SHELL32_CDefFolderMenu_Create2Ex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDefFolderMenu_Create2Ex arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CDefFolderMenu_Create2Ex");
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
        "SHELL32_CDefFolderMenu_MergeMenu" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDefFolderMenu_MergeMenu arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CDrivesContextMenu_Create" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDrivesContextMenu_Create arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CDrivesContextMenu_Create");
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
        "SHELL32_CDrivesDropTarget_Create" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDrivesDropTarget_Create arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CDrivesDropTarget_Create");
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
        "SHELL32_CDrives_CreateSFVCB" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CDrives_CreateSFVCB arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CDrives_CreateSFVCB");
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
        "SHELL32_CFSDropTarget_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CFSDropTarget_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CFSDropTarget_CreateInstance");
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
        "SHELL32_CFSFolderCallback_Create" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CFSFolderCallback_Create arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CFSFolderCallback_Create");
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
        "SHELL32_CFillPropertiesTask_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CFillPropertiesTask_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CFillPropertiesTask_CreateInstance");
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
        "SHELL32_CLibraryDropTarget_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CLibraryDropTarget_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CLibraryDropTarget_CreateInstance");
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
        "SHELL32_CLocationContextMenu_Create" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CLocationContextMenu_Create arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CLocationContextMenu_Create");
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
        "SHELL32_CLocationFolderUI_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CLocationFolderUI_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CLocationFolderUI_CreateInstance");
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
        "SHELL32_CMountPoint_DoAutorun" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_DoAutorun arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CMountPoint_DoAutorunPrompt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_DoAutorunPrompt arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CMountPoint_IsAutoRunDriveAndEnabledByPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_IsAutoRunDriveAndEnabledByPolicy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle =
                helper::handler_create("SHELL32_CMountPoint_IsAutoRunDriveAndEnabledByPolicy");
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
        "SHELL32_CMountPoint_ProcessAutoRunFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_ProcessAutoRunFile h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CMountPoint_WantAutorunUI" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_WantAutorunUI arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CMountPoint_WantAutorunUIGetReady" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CMountPoint_WantAutorunUIGetReady arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CNetFolderUI_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CNetFolderUI_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CNetFolderUI_CreateInstance");
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
        "SHELL32_CPL_CategoryIdArrayFromVariant" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CPL_CategoryIdArrayFromVariant arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CPL_IsLegacyCanonicalNameListedUnderKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CPL_IsLegacyCanonicalNameListedUnderKey h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CPL_ModifyWowDisplayName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CPL_ModifyWowDisplayName arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CRecentDocsContextMenu_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CRecentDocsContextMenu_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CRecentDocsContextMenu_CreateInstance");
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
        "SHELL32_CTransferConfirmation_CreateInstance" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CTransferConfirmation_CreateInstance arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CTransferConfirmation_CreateInstance");
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
        "SHELL32_CallFileCopyHooks" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CallFileCopyHooks h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CallFileCopyHooks");
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
        "SHELL32_CanDisplayWin8CopyDialog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CanDisplayWin8CopyDialog arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CanDisplayWin8CopyDialog");
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
        "SHELL32_CloseAutoplayPrompt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CloseAutoplayPrompt arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CommandLineFromMsiDescriptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CommandLineFromMsiDescriptor arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_CopySecondaryTiles" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CopySecondaryTiles arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CopySecondaryTiles");
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
        "SHELL32_CreateConfirmationInterrupt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateConfirmationInterrupt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CreateConfirmationInterrupt");
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
        "SHELL32_CreateConflictInterrupt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateConflictInterrupt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CreateConflictInterrupt");
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
        "SHELL32_CreateDefaultOperationDataProvider" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateDefaultOperationDataProvider arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CreateDefaultOperationDataProvider");
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
        "SHELL32_CreateFileFolderContextMenu" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateFileFolderContextMenu h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CreateFileFolderContextMenu");
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
        "SHELL32_CreateLinkInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHELL32_CreateQosRecorder");
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
        "SHELL32_CreateSharePointView" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_CreateSharePointView arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_CreateSharePointView");
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
        "SHELL32_Create_IEnumUICommand" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_Create_IEnumUICommand arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_Create_IEnumUICommand");
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
        "SHELL32_DestroyLinkInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_DestroyLinkInfo arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_EncryptDirectory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_EncryptDirectory arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_EncryptedFileKeyInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_EncryptedFileKeyInfo h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_EnumCommonTasks" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_EnumCommonTasks arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_FreeEncryptedFileKeyInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_FreeEncryptedFileKeyInfo h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GenerateAppID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GenerateAppID arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetAppIDRoot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetAppIDRoot arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetCommandProviderForFolderType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetCommandProviderForFolderType arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetDPIAdjustedLogicalSize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetDPIAdjustedLogicalSize arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetDiskCleanupPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetDiskCleanupPath arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetFileNameFromBrowse" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetFileNameFromBrowse h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetIconOverlayManager" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetIconOverlayManager arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetLinkInfoData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetLinkInfoData arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetRatingBucket" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetRatingBucket arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetSqmableFileName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetSqmableFileName h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetThumbnailAdornerFromFactory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetThumbnailAdornerFromFactory arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_GetThumbnailAdornerFromFactory2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_GetThumbnailAdornerFromFactory2 arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_HandleUnrecognizedFileSystem" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_HandleUnrecognizedFileSystem h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_IconCacheCreate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCacheCreate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_IconCacheCreate");
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
        "SHELL32_IconCacheDestroy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCacheDestroy arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_IconCacheHandleAssociationChanged" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCacheHandleAssociationChanged h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_IconCacheRestore" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCacheRestore arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_IconCache_AboutToExtractIcons" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCache_AboutToExtractIcons arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_IconCache_DoneExtractingIcons" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCache_DoneExtractingIcons arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_IconCache_ExpandEnvAndSearchPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IconCache_ExpandEnvAndSearchPath arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_IconCache_RememberRecentlyExtractedIconsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHELL32_IconOverlayManagerInit");
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
        "SHELL32_IsGetKeyboardLayoutPresent" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IsGetKeyboardLayoutPresent h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_IsSystemUpgradeInProgress" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IsSystemUpgradeInProgress arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_IsValidLinkInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_IsValidLinkInfo arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_LegacyEnumSpecialTasksByType" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_LegacyEnumSpecialTasksByType arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_LegacyEnumTasks" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_LegacyEnumTasks arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_LookupBackIconIndex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_LookupBackIconIndex arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_LookupFrontIconIndex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_LookupFrontIconIndex arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_NormalizeRating" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_NormalizeRating arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_NotifyLinkTrackingServiceOfMove" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_NotifyLinkTrackingServiceOfMove arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_PifMgr_CloseProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PifMgr_CloseProperties arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_PifMgr_GetProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PifMgr_GetProperties arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_PifMgr_OpenProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PifMgr_OpenProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_PifMgr_OpenProperties");
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
        "SHELL32_PifMgr_SetProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PifMgr_SetProperties arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_Printers_CreateBindInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_Printers_CreateBindInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_Printers_CreateBindInfo");
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
        "SHELL32_Printjob_GetPidl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_Printjob_GetPidl arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_PurgeSystemIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_PurgeSystemIcon arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_RefreshOverlayImages" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_RefreshOverlayImages arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_ResolveLinkInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHELL32_SHAddSparseIcon");
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
        "SHELL32_SHCreateByValueOperationInterrupt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHCreateByValueOperationInterrupt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_SHCreateByValueOperationInterrupt");
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
        "SHELL32_SHCreateDefaultContextMenu" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHCreateDefaultContextMenu arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_SHCreateDefaultContextMenu");
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
        "SHELL32_SHCreateLocalServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHCreateLocalServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_SHCreateLocalServer");
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
        "SHELL32_SHCreateShellFolderView" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHCreateShellFolderView arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_SHCreateShellFolderView");
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
        "SHELL32_SHDuplicateEncryptionInfoFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHDuplicateEncryptionInfoFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("SHELL32_SHDuplicateEncryptionInfoFile");
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
        "SHELL32_SHEncryptFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHEncryptFile h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_SHFormatDriveAsync" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHFormatDriveAsync arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_SHGetThreadUndoManager" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHGetThreadUndoManager h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_SHGetUserNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHELL32_SHGetUserNameW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
        "SHELL32_SHLaunchPropSheet" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHLaunchPropSheet arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_SHLaunchPropSheet");
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
        "SHELL32_SHLogILFromFSIL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHLogILFromFSIL arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_SHOpenWithDialog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SHOpenWithDialog arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHELL32_SHOpenWithDialog");
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
        "SHELL32_SHStartNetConnectionDialogW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
        "SHELL32_SendToMenu_InvokeTargetedCommand" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SendToMenu_InvokeTargetedCommand arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_SendToMenu_VerifyTargetedCommand" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SendToMenu_VerifyTargetedCommand arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_ShowHideIconOnlyOnDesktop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_ShowHideIconOnlyOnDesktop arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_SimpleRatingToFilterCondition" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SimpleRatingToFilterCondition arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_StampIconForFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_StampIconForFile h:0x{:x} arg2:0x{:x}",
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
        "SHELL32_SuspendUndo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_SuspendUndo arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_TryVirtualDiscImageDriveEject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_TryVirtualDiscImageDriveEject arg1:0x{:x} arg2:0x{:x}",
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
        "SHELL32_VerifySaferTrust" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHELL32_VerifySaferTrust arg1:0x{:x} arg2:0x{:x}",
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
        "SHEmptyRecycleBinA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHEmptyRecycleBinA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHEnableServiceObject");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHEnumerateUnreadMailAccountsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
        "SHExecuteErrorMessageBox" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHExecuteErrorMessageBox arg1:0x{:x} arg2:0x{:x}",
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
        "SHExtractIconsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHExtractIconsW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHFileOperation" => {
            let file_op = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
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
            emu.stack_pop32(false);
            emu.regs_mut().rax = 0;
        }
        "SHFileOperationA" => {
            let file_op = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
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
                "shell32!SHFileOperationA op:{} from:'{}' ptr:0x{:x}",
                func,
                from,
                file_op
            );
            emu.stack_pop32(false);
            emu.regs_mut().rax = 0;
        }
        "SHFileOperationW" => {
            let file_op = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
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
            emu.stack_pop32(false);
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
        "SHFind_InitMenuPopup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHFind_InitMenuPopup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shfind_initmenupopup");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!SHFree arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHFreeNameMappings" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "shell32!SHFreeNameMappings handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetAttributesFromDataObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetAttributesFromDataObject h:0x{:x} arg2:0x{:x}",
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
        "SHGetDataFromIDListA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHGetDataFromIDListA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHGetDataFromIDListW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHGetDataFromIDListW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHGetDesktopFolder" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetDesktopFolder arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetdesktopfolder");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "SHGetFileInfoA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!SHGetFileInfoA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("shgetfileinfoa");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetFileInfoW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!SHGetFileInfoW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("shgetfileinfow");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetFolderLocation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetFolderLocation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetfolderlocation");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetFolderPathA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let csidl = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let path_buf = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let path = match csidl {
                0x001a => "C:\\Users\\mwemu_user\\AppData\\Roaming",
                0x001c => "C:\\Users\\mwemu_user\\AppData\\Local",
                0x0024 => "C:\\Windows",
                0x0025 => "C:\\Windows\\System32",
                0x0027 => "C:\\Program Files",
                0x002b => "C:\\Program Files (x86)",
                0x0005 => "C:\\Users\\mwemu_user\\Documents",
                0x0028 => "C:\\Users\\mwemu_user\\Desktop",
                _ => "C:\\Users\\mwemu_user",
            };
            if path_buf != 0 {
                emu.maps.write_string(path_buf, path);
            }
            log_red!(
                emu,
                "shell32!SHGetFolderPathA csidl:0x{:x} => '{}'",
                csidl,
                path
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "SHGetFolderPathAndSubDirA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let csidl = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let path_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if path_ptr != 0 {
                emu.maps.write_string(path_ptr, "C:\\Users\\mwemu");
            }
            log_red!(
                emu,
                "shell32!SHGetFolderPathAndSubDirA csidl:0x{:x} => C:\\Users\\mwemu",
                csidl
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "SHGetFolderPathAndSubDirW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let csidl = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let path_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if path_ptr != 0 {
                emu.maps.write_wide_string(path_ptr, "C:\\Users\\mwemu");
            }
            log_red!(
                emu,
                "shell32!SHGetFolderPathAndSubDirW csidl:0x{:x} => C:\\Users\\mwemu",
                csidl
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "SHGetFolderPathEx" => {
            let rfid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let h_token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let psz_path = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let cch_path = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            if psz_path != 0 && cch_path > 0 {
                emu.maps
                    .write_wide_string(psz_path, "C:\\Users\\mwemu_user");
            }
            log_red!(
                emu,
                "shell32!SHGetFolderPathEx rfid:0x{:x} flags:0x{:x} => C:\\Users\\mwemu_user",
                rfid,
                dw_flags
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0; // S_OK
        }
        "SHGetFolderPathW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let csidl = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let path_buf = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let path = match csidl {
                0x001a => "C:\\Users\\mwemu_user\\AppData\\Roaming",
                0x001c => "C:\\Users\\mwemu_user\\AppData\\Local",
                0x0024 => "C:\\Windows",
                0x0025 => "C:\\Windows\\System32",
                0x0027 => "C:\\Program Files",
                0x002b => "C:\\Program Files (x86)",
                0x0005 => "C:\\Users\\mwemu_user\\Documents",
                0x0028 => "C:\\Users\\mwemu_user\\Desktop",
                _ => "C:\\Users\\mwemu_user",
            };
            if path_buf != 0 {
                emu.maps.write_wide_string(path_buf, path);
            }
            log_red!(
                emu,
                "shell32!SHGetFolderPathW csidl:0x{:x} => '{}'",
                csidl,
                path
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "SHGetIDListFromObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetIDListFromObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetidlistfromobject");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetImageList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetImageList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetimagelist");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetItemFromDataObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetItemFromDataObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetitemfromdataobject");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetItemFromObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetItemFromObject arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetitemfromobject");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetKnownFolderIDList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetKnownFolderIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetknownfolderidlist");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetKnownFolderItem" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetKnownFolderItem arg1:0x{:x} arg2:0x{:x}",
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
        "SHGetKnownFolderPath" => {
            let folder_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let path_out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let path = "C:\\Users\\mwemu_user\\AppData\\Local";
            let alloc_addr = emu
                .maps
                .alloc(512)
                .expect("shell32!SHGetKnownFolderPath alloc");
            emu.maps.write_wide_string(alloc_addr, path);
            if path_out_ptr != 0 {
                emu.maps.write_dword(path_out_ptr, alloc_addr as u32);
            }
            log_red!(
                emu,
                "shell32!SHGetKnownFolderPath flags:0x{:x} => '{}'",
                flags,
                path
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
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
        "SHGetMalloc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!SHGetMalloc arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetNameFromIDList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetNameFromIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetnamefromidlist");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHGetNewLinkInfo");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHGetNewLinkInfoA");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHGetNewLinkInfoW");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetPathFromIDList" => {
            let pidl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let psz_path = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetPathFromIDList pidl:0x{:x} path:0x{:x}",
                pidl,
                psz_path
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            if psz_path != 0 {
                emu.maps.write_string(psz_path, "C:\\Users\\User\\Desktop");
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0;
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "SHGetPathFromIDListA" => {
            let pidl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let psz_path = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetPathFromIDListA pidl:0x{:x} path:0x{:x}",
                pidl,
                psz_path
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            if psz_path != 0 {
                emu.maps.write_string(psz_path, "C:\\Users\\User\\Desktop");
                emu.set_last_error(0);
                emu.regs_mut().rax = 1;
            } else {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0;
            }
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "SHGetPathFromIDListEx" => {
            let pidl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let psz_path = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let cch_path = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetPathFromIDListEx pidl:0x{:x} path:0x{:x} cch:{}",
                pidl,
                psz_path,
                cch_path
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
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
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "SHGetPathFromIDListW" => {
            let pidl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let psz_path = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetPathFromIDListW pidl:0x{:x} path:0x{:x}",
                pidl,
                psz_path
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "SHGetPropertyStoreForWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetPropertyStoreForWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetpropertystoreforwindow");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetPropertyStoreFromIDList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetPropertyStoreFromIDList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetpropertystorefromidlist");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetPropertyStoreFromParsingName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetPropertyStoreFromParsingName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetpropertystorefromparsingname");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetRealIDL" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetRealIDL arg1:0x{:x} arg2:0x{:x}",
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
        "SHGetSetFolderCustomSettings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetSetFolderCustomSettings arg1:0x{:x} arg2:0x{:x}",
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
        "SHGetSetSettings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetSetSettings arg1:0x{:x} arg2:0x{:x}",
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
        "SHGetSettings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHGetSettings arg1:0x{:x} arg2:0x{:x}",
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
        "SHGetSpecialFolderLocation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetSpecialFolderLocation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetspecialfolderlocation");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHGetSpecialFolderPathA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let path_buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let csidl = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let create = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let path = match csidl {
                0x001a => "C:\\Users\\mwemu_user\\AppData\\Roaming",
                0x001c => "C:\\Users\\mwemu_user\\AppData\\Local",
                0x0024 => "C:\\Windows",
                0x0025 => "C:\\Windows\\System32",
                _ => "C:\\Users\\mwemu_user",
            };
            if path_buf != 0 {
                emu.maps.write_string(path_buf, path);
            }
            log_red!(
                emu,
                "shell32!SHGetSpecialFolderPathA csidl:0x{:x} => '{}'",
                csidl,
                path
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHGetSpecialFolderPathW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let path_buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let csidl = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let create = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let path = match csidl {
                0x001a => "C:\\Users\\mwemu_user\\AppData\\Roaming",
                0x001c => "C:\\Users\\mwemu_user\\AppData\\Local",
                0x0024 => "C:\\Windows",
                0x0025 => "C:\\Windows\\System32",
                _ => "C:\\Users\\mwemu_user",
            };
            if path_buf != 0 {
                emu.maps.write_wide_string(path_buf, path);
            }
            log_red!(
                emu,
                "shell32!SHGetSpecialFolderPathW csidl:0x{:x} => '{}'",
                csidl,
                path
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHGetStockIconInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetStockIconInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shgetstockiconinfo");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "SHGetUnreadMailCountW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHGetUnreadMailCountW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "SHHelpShortcuts_RunDLLA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHHelpShortcuts_RunDLLW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHILCreateFromPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHILCreateFromPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SHILCreateFromPath");
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
        "SHInvokePrinterCommandA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHInvokePrinterCommandA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHLimitInputEdit" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHLimitInputEdit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shlimitinputedit");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHLoadInProc");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHLoadNonloadedIconOverlayIdentifiers");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHMapPIDLToSystemImageListIndex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHMapPIDLToSystemImageListIndex arg1:0x{:x}",
                arg1
            );
            let handle = helper::handler_create("SHMapPIDLToSystemImageListIndex");
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SHMultiFileProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHMultiFileProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shmultifileproperties");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHObjectProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHObjectProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shobjectproperties");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHOpenFolderAndSelectItems" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHOpenFolderAndSelectItems arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shopenfolderandselectitems");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHOpenOrGetFolderView");
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
        "SHOpenPropSheetW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHOpenPropSheetW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shopenpropsheetw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHOpenWithDialog");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHParseDisplayName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHParseDisplayName arg1:0x{:x} arg2:0x{:x}",
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
        "SHPathPrepareForWriteA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHPathPrepareForWriteA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SHPathPrepareForWriteW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SHPathPrepareForWriteW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SHPropStgCreate");
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
        "SHPropStgReadMultiple" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHPropStgReadMultiple arg1:0x{:x} arg2:0x{:x}",
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
        "SHPropStgWriteMultiple" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHPropStgWriteMultiple arg1:0x{:x} arg2:0x{:x}",
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
        "SHQueryRecycleBinA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHQueryRecycleBinA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
        "SHRemoveLocalizedName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHRemoveLocalizedName arg1:0x{:x} arg2:0x{:x}",
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
        "SHReplaceFromPropSheetExtArray" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHReplaceFromPropSheetExtArray arg1:0x{:x} arg2:0x{:x}",
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
        "SHResolveLibrary" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHResolveLibrary arg1:0x{:x} arg2:0x{:x}",
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
        "SHRestricted" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHRestricted arg1:0x{:x} arg2:0x{:x}",
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
        "SHSetDefaultProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!SHSetDefaultProperties arg1:0x{:x} arg2:0x{:x}",
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
        "SHSetFolderPathA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!SHSetFolderPathA arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SHSetFolderPathW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "shell32!SHSetFolderPathW arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHSimpleIDListFromPath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHSimpleIDListFromPath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shsimpleidlistfrompath");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHStartNetConnectionDialogW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHStartNetConnectionDialogW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shstartnetconnectiondialogw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SHUpdateRecycleBinIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!SHUpdateRecycleBinIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shupdaterecyclebinicon");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }

            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SheChangeDirA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SheChangeDirA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SheChangeDirExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SheChangeDirExW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SheGetDirA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!SheGetDirA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
        "ShellAboutA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!ShellAboutA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("ShellExecuteEx");
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
        "ShellExecuteExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!ShellExecuteExA '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("shellexecuteexa");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ShellExecuteExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if str_ptr != 0 {
                emu.maps.read_wide_string(str_ptr)
            } else {
                String::new()
            };
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "shell32!ShellExecuteExW '{}' arg2:0x{:x}", s, arg2);
            let handle = helper::handler_create("shellexecuteexw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "ShellMessageBoxA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!ShellMessageBoxA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ShellMessageBoxW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!ShellMessageBoxW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "Shell_GetCachedImageIndex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Shell_GetCachedImageIndex arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shell_getcachedimageindex");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "Shell_GetCachedImageIndexA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Shell_GetCachedImageIndexA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shell_getcachedimageindexa");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "Shell_GetCachedImageIndexW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!Shell_GetCachedImageIndexW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("shell_getcachedimageindexw");
            emu.regs_mut().rax = handle;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "Shell_GetImageLists" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "shell32!Shell_GetImageLists arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
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
        "Shell_NotifyIcon" => {
            let dw_message = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let lp_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let tip = if lp_data != 0 {
                emu.maps.read_string(lp_data + 64)
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "Shell_NotifyIconA" => {
            let dw_message = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let lp_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let tip = if lp_data != 0 {
                emu.maps.read_string(lp_data + 64)
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "Shell_NotifyIconW" => {
            let dw_message = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let lp_data = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let tip = if lp_data != 0 {
                emu.maps.read_wide_string(lp_data + 64)
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("SignalFileOpen");
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, handle as u32);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("StateRepoNewMenuCache_EnsureCacheAsync");
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
        "StateRepoNewMenuCache_RebuildCacheAsync" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "shell32!StateRepoNewMenuCache_RebuildCacheAsync arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("StateRepoNewMenuCache_RebuildCacheAsync");
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
        "StgMakeUniqueName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!StgMakeUniqueName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "StrChrA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrChrA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrChrIA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrChrIA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrChrIW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrChrIW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrChrW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrChrW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrCmpNA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrCmpNA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrCmpNIA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrCmpNIA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrCmpNIW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrCmpNIW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrCmpNW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrCmpNW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrNCmpA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrNCmpA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrNCmpIA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrNCmpIA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrNCmpIW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrNCmpIW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrNCmpW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrNCmpW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrRChrA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRChrA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrRChrIA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRChrIA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrRChrIW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRChrIW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrRChrW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRChrW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrRStrA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRStrA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrRStrIA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRStrIA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrRStrIW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRStrIW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrRStrW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrRStrW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrStrA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrStrA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrStrIA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrStrIA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrStrIW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrStrIW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "StrStrW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "shell32!StrStrW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let handle = helper::handler_create("UsersLibrariesFolderUI_CreateInstance");
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
        "WOWShellExecute" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "shell32!WOWShellExecute arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForExplorerRestartW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
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
            let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
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
            let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if out_ptr != 0 {
                emu.maps.write_dword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

pub fn ShellExecuteA(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let lp_operation = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let lp_file = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let lp_parameters = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let lp_directory = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let n_show_cmd = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

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

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 42; // > 32 means success
}

fn ShellExecuteW(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let lp_operation = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let lp_file = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let lp_parameters = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let lp_directory = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let n_show_cmd = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

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

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 42; // > 32 means success
}
