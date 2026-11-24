use crate::api::windows::registry_store::HKEY_LOCAL_MACHINE;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::syscall::windows::syscall64;
use crate::tests::helpers;
use crate::windows::constants::errors::*;
use crate::windows::constants::security::*;

#[test]
fn registry_handles_share_values_within_one_guest_and_keep_pointer_width() {
    use crate::winapi::common::memory::read_pointer;
    use crate::winapi::registry_store::REG_DWORD;
    use helpers::call_winapi_export as call;
    const VALUE: u32 = 0x0123_4567;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let (mut other, other_data) = helpers::prepare_winapi(is64);
        let width = if is64 { 8 } else { 4 };
        let (name, handle, value, size, kind) = (
            data + 0x100,
            data + 0x180,
            data + 0x200,
            data + 0x220,
            data + 0x240,
        );
        let key = "Software\\MwEmuRegistryGuestTest";
        emu.maps.write_wide_string(data, key);
        other.maps.write_wide_string(other_data, key);
        emu.maps.write_qword(handle, u64::MAX);
        emu.maps.write_byte(handle + width, 0xcc);
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "RegCreateKeyExW",
                &[
                    HKEY_LOCAL_MACHINE,
                    data,
                    0,
                    0,
                    0,
                    KEY_ALL_ACCESS as u64,
                    0,
                    handle,
                    0
                ]
            ),
            ERROR_SUCCESS
        );
        assert_eq!(emu.maps.read_byte(handle + width), Some(0xcc));
        let held = read_pointer(&emu, handle).unwrap();
        assert_ne!(held, 0);
        emu.maps.write_string(name, "answer");
        emu.maps.write_dword(value, VALUE);
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "RegSetValueExA",
                &[held, name, 0, REG_DWORD as u64, value, 4]
            ),
            ERROR_SUCCESS
        );
        emu.maps.write_wide_string(name, "ANSWER");
        emu.maps.write_dword(value, 0);
        emu.maps.write_dword(size, 4);
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "RegGetValueW",
                &[held, 0, name, RRF_RT_REG_DWORD as u64, kind, value, size]
            ),
            ERROR_SUCCESS
        );
        assert_eq!(emu.maps.read_dword(kind), Some(REG_DWORD));
        assert_eq!(emu.maps.read_dword(value), Some(VALUE));
        emu.maps.write_byte(value + 4, 0xcc);
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "RegGetValueW",
                &[held, 0, name, RRF_ZEROONFAILURE as u64, kind, value, size]
            ),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!(emu.maps.read_dword(value), Some(0));
        assert_eq!(emu.maps.read_byte(value + 4), Some(0xcc));
        assert_eq!(
            call(
                &mut other,
                "advapi32.dll",
                "RegOpenKeyExW",
                &[
                    HKEY_LOCAL_MACHINE,
                    other_data,
                    0,
                    KEY_READ as u64,
                    other_data + 0x180
                ]
            ),
            ERROR_FILE_NOT_FOUND
        );
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "RegDeleteKeyW",
                &[HKEY_LOCAL_MACHINE, data]
            ),
            ERROR_SUCCESS
        );
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "RegCreateKeyW",
                &[HKEY_LOCAL_MACHINE, data, handle]
            ),
            ERROR_SUCCESS
        );
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "RegQueryValueExW",
                &[held, name, 0, kind, value, size]
            ),
            ERROR_KEY_DELETED
        );
        assert_eq!(
            call(&mut emu, "advapi32.dll", "RegCloseKey", &[held]),
            ERROR_SUCCESS
        );
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "RegQueryValueExW",
                &[held, name, 0, kind, value, size]
            ),
            ERROR_INVALID_HANDLE
        );
    }
}

#[test]
fn checked_arguments_preserve_outputs_before_late_registry_read_failure() {
    use crate::tests::helpers::{invoke_winapi_export, prepare_winapi};

    let (mut emu, data) = prepare_winapi(true);
    let stack = data + 0x10000;
    emu.maps
        .create_map(
            "short_registry_frame",
            stack,
            11 * size_of::<u64>() as u64,
            Permission::READ_WRITE,
        )
        .unwrap();
    emu.set_sp(stack);
    emu.regs_mut().rcx = HKEY_LOCAL_MACHINE;
    emu.regs_mut().rdx = data;
    emu.regs_mut().r8 = data + 4;
    emu.regs_mut().r9 = 0;
    for index in 4..11 {
        assert!(
            emu.maps
                .write_qword(stack + index * size_of::<u64>() as u64, data + 8)
        );
    }
    assert!(emu.maps.write_bytes(data, &[0xa5; 16]));
    emu.regs_mut().rax = 0x12345678;
    invoke_winapi_export(&mut emu, "advapi32.dll", "RegQueryInfoKeyA");
    assert!(emu.api_policy.stopped());
    assert_eq!(emu.regs().rax, 0x12345678);
    assert_eq!(&*emu.maps.read_bytes(data, 16), &[0xa5; 16]);
}

#[test]
fn user_token_sid_key_path_and_current_key_agree() {
    use crate::winapi::common::memory::read_pointer;
    use crate::windows::constants::process::NT_CURRENT_PROCESS;
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let width = if is64 { 8 } else { 4 };
        let (out, returned, token_user, text) = (data + 32, data + 64, data + 128, data + 256);
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "OpenProcessToken",
                &[NT_CURRENT_PROCESS, TOKEN_QUERY as u64, out]
            ),
            1
        );
        let token = read_pointer(&emu, out).unwrap();
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "GetTokenInformation",
                &[token, 1, 0, 0, returned]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        let required = emu.maps.read_dword(returned).unwrap() as u64;
        emu.maps.write_byte(token_user + required, 0xa5);
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "GetTokenInformation",
                &[token, 1, token_user, required, returned]
            ),
            1
        );
        assert_eq!(emu.maps.read_byte(token_user + required), Some(0xa5));
        let sid = read_pointer(&emu, token_user).unwrap();
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "ConvertSidToStringSidW",
                &[sid, text]
            ),
            1
        );
        let string = read_pointer(&emu, text).unwrap();
        let expected = format!(r"\REGISTRY\USER\{}", emu.maps.read_wide_string(string));
        assert_eq!(call(&mut emu, "kernel32.dll", "LocalFree", &[string]), 0);
        emu.maps.write_byte(data + 2 * width, 0xa5);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "RtlFormatCurrentUserKeyPath",
                &[data]
            ),
            STATUS_SUCCESS
        );
        let path = read_pointer(&emu, data + width).unwrap();
        assert_eq!(emu.maps.read_wide_string(path), expected);
        assert_eq!(
            emu.maps.read_word(data),
            Some((expected.encode_utf16().count() * 2) as u16)
        );
        assert_eq!(emu.maps.read_byte(data + 2 * width), Some(0xa5));
        call(&mut emu, "ntdll.dll", "RtlFreeUnicodeString", &[data]);
        assert_eq!(read_pointer(&emu, data + width), Some(0));
        for (library, api) in [
            ("ntdll.dll", "RtlOpenCurrentUser"),
            ("advapi32.dll", "RegOpenCurrentUser"),
        ] {
            assert_eq!(call(&mut emu, library, api, &[KEY_READ as u64, out]), 0);
            let key = read_pointer(&emu, out).unwrap();
            assert_eq!(
                emu.handle_management.registry_key(key),
                Some("HKEY_CURRENT_USER")
            );
            assert_eq!(
                call(&mut emu, "advapi32.dll", "RegCloseKey", &[key]),
                ERROR_SUCCESS
            );
        }
        assert_eq!(call(&mut emu, "kernel32.dll", "CloseHandle", &[token]), 1);
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "GetTokenInformation",
                &[token, 1, token_user, required, returned]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
    }
}

#[test]
fn native_registry_reads_the_guest_store_through_handles_and_syscalls() {
    use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
    use crate::winapi::common::security::{process_user_sid, sid_string};
    use crate::winapi::registry_store::{HKEY_CURRENT_USER, REG_DWORD};
    use crate::windows::constants::syscalls::*;
    use helpers::call_winapi_export as call;

    fn string(emu: &mut Emu, desc: u64, text: &str) {
        let buf = desc + 0x20;
        let bytes = (text.encode_utf16().count() * 2) as u16;
        let width = pointer_size(emu);
        emu.maps.write_word(desc, bytes);
        emu.maps.write_word(desc + 2, bytes);
        write_pointer(emu, desc + width, buf);
        emu.maps.write_wide_string(buf, text);
    }
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let width = pointer_size(&emu);
        let (attr, key, value, out, required, info) = (
            data,
            data + 0x80,
            data + 0x200,
            data + 0x300,
            data + 0x310,
            data + 0x320,
        );
        let path = format!(r"\REGISTRY\USER\{}", sid_string(&process_user_sid()));
        string(&mut emu, key, &path);
        string(&mut emu, value, "ProductName");
        emu.maps.write_dword(info, 42);
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "RegSetValueExW",
                &[
                    HKEY_CURRENT_USER,
                    value + 0x20,
                    0,
                    REG_DWORD as u64,
                    info,
                    4
                ]
            ),
            ERROR_SUCCESS
        );
        emu.maps.write_dword(attr, (width * 6) as u32);
        write_pointer(&mut emu, attr + width * 2, key);
        emu.maps.write_byte(out + width, 0xa5);
        let args = [out, KEY_READ as u64, attr, 0];
        let status = if is64 {
            syscall(&mut emu, WIN64_NTOPENKEYEX, &args)
        } else {
            call(&mut emu, "ntdll.dll", "ZwOpenKeyEx", &args)
        };
        assert_eq!(status, STATUS_SUCCESS);
        assert_eq!(emu.maps.read_byte(out + width), Some(0xa5));
        let handle = read_pointer(&emu, out).unwrap();
        const KEY_HANDLE_TAGS_INFORMATION: u64 = 7;
        const KEY_SET_HANDLE_TAGS_INFORMATION: u64 = 5;
        emu.maps.write_dword(info, 0xabcd_1234);
        for (capacity, expected) in [(8, STATUS_INFO_LENGTH_MISMATCH), (4, STATUS_SUCCESS)] {
            let args = [handle, KEY_SET_HANDLE_TAGS_INFORMATION, info, capacity];
            let status = if is64 {
                syscall(&mut emu, WIN64_NTSETINFORMATIONKEY, &args)
            } else {
                call(&mut emu, "ntdll.dll", "NtSetInformationKey", &args)
            };
            assert_eq!(status, expected);
        }
        for (capacity, expected, tag) in [
            (2, STATUS_BUFFER_TOO_SMALL, u32::MAX),
            (4, STATUS_SUCCESS, 0x1234),
        ] {
            emu.maps.write_dword(info, u32::MAX);
            let args = [
                handle,
                KEY_HANDLE_TAGS_INFORMATION,
                info,
                capacity,
                required,
            ];
            let status = if is64 {
                syscall(&mut emu, WIN64_NTQUERYKEY, &args)
            } else {
                call(&mut emu, "ntdll.dll", "ZwQueryKey", &args)
            };
            assert_eq!(status, expected);
            assert_eq!(emu.maps.read_dword(required), Some(4));
            assert_eq!(emu.maps.read_dword(info), Some(tag));
        }
        string(&mut emu, value, "pRODUCTnAME");
        for (class, header, needed) in [
            (0, 12, 34),
            (1, 20, 52),
            (2, 12, 16),
            (3, 20, 52),
            (4, 8, 12),
        ] {
            for (capacity, expected) in [
                (0, STATUS_BUFFER_TOO_SMALL),
                (header, STATUS_BUFFER_OVERFLOW),
                (needed, STATUS_SUCCESS),
            ] {
                emu.maps.write_bytes(info, &[0xcc; 64]);
                let args = [handle, value, class, info, capacity, required];
                let status = if is64 {
                    syscall(&mut emu, WIN64_NTQUERYVALUEKEY, &args)
                } else {
                    call(&mut emu, "ntdll.dll", "NtQueryValueKey", &args)
                };
                assert_eq!(status, expected, "class {class}, size {capacity}");
                assert_eq!(emu.maps.read_dword(required), Some(needed as u32));
                assert_eq!(emu.maps.read_byte(info + capacity), Some(0xcc));
                if capacity == needed && class != 0 {
                    assert_eq!(emu.maps.read_dword(info + needed - 4), Some(42));
                }
                if capacity == needed && class <= 1 {
                    assert_eq!(emu.maps.read_word(info + header), Some('P' as u16));
                }
            }
        }
        write_pointer(&mut emu, attr + width, handle);
        string(&mut emu, key, "Software");
        let request = [out, KEY_QUERY_VALUE as u64, attr];
        let status = if is64 {
            syscall(&mut emu, WIN64_NTOPENKEY, &request)
        } else {
            call(&mut emu, "ntdll.dll", "NtOpenKey", &request)
        };
        assert_eq!(status, STATUS_SUCCESS);
        let child = read_pointer(&emu, out).unwrap();
        assert_eq!(
            emu.handle_management.registry_key(child),
            Some(r"HKEY_CURRENT_USER\Software")
        );
        assert_eq!(
            call(&mut emu, "advapi32.dll", "RegCloseKey", &[child]),
            ERROR_SUCCESS
        );
        string(&mut emu, key, "");
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtOpenKey", &[out, 0, attr]),
            STATUS_SUCCESS
        );
        let restricted = read_pointer(&emu, out).unwrap();
        emu.maps.write_dword(required, 0xa5a5a5a5);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "NtQueryValueKey",
                &[restricted, value, 2, info, 16, required]
            ),
            STATUS_ACCESS_DENIED
        );
        assert_eq!(emu.maps.read_dword(required), Some(0xa5a5a5a5));
        assert_eq!(
            call(&mut emu, "advapi32.dll", "RegCloseKey", &[restricted]),
            ERROR_SUCCESS
        );
        string(&mut emu, value, "NoSuchValue");
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "NtQueryValueKey",
                &[handle, value, 2, info, 16, required]
            ),
            STATUS_OBJECT_NAME_NOT_FOUND
        );
        assert_eq!(emu.maps.read_dword(required), Some(0xa5a5a5a5));
        for missing in ["DoesNotExist", r"DoesNotExist\Child"] {
            string(&mut emu, key, missing);
            write_pointer(&mut emu, out, u64::MAX);
            assert_eq!(
                call(
                    &mut emu,
                    "ntdll.dll",
                    "NtOpenKey",
                    &[out, KEY_READ as u64, attr]
                ),
                STATUS_OBJECT_NAME_NOT_FOUND
            );
            assert_eq!(read_pointer(&emu, out), Some(0));
            assert_eq!(emu.maps.read_byte(out + width), Some(0xa5));
        }
        assert_eq!(
            call(&mut emu, "ntdll.dll", "ZwClose", &[handle]),
            STATUS_SUCCESS
        );
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "ZwQueryValueKey",
                &[handle, value, 2, info, 16, required]
            ),
            STATUS_INVALID_HANDLE
        );
    }
}

#[test]
fn kernel_security_buffers_use_sid_length_and_guest_descriptor_layout() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::winapi::common::memory::read_pointer;
    use crate::windows::constants::errors::ERROR_INSUFFICIENT_BUFFER;
    for is64 in [false, true] {
        let (mut emu, sd) = prepare_winapi(is64);
        let (sid, authority, copy, out, defaulted) =
            (sd + 64, sd + 128, sd + 160, sd + 224, sd + 240);
        emu.maps.write_bytes(authority, &[0, 0, 0, 0, 0, 5]);
        let dll = "kernelbase.dll";
        assert_eq!(
            call(&mut emu, dll, "InitializeSid", &[sid, authority, 2]),
            1
        );
        assert_eq!(call(&mut emu, dll, "GetLengthSid", &[sid]), 16);
        assert_eq!(
            call(&mut emu, dll, "GetSidSubAuthority", &[sid, 1]),
            sid + 12
        );
        emu.maps.write_dword(copy, 0xfeed);
        assert_eq!(call(&mut emu, dll, "CopySid", &[15, copy, sid]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        assert_eq!(emu.maps.read_dword(copy), Some(0xfeed));
        assert_eq!(call(&mut emu, dll, "CopySid", &[16, copy, sid]), 1);
        assert_eq!(call(&mut emu, dll, "EqualSid", &[sid, copy]), 1);
        assert_eq!(
            call(&mut emu, dll, "InitializeSecurityDescriptor", &[sd, 1]),
            1
        );
        assert_eq!(
            call(&mut emu, dll, "SetSecurityDescriptorOwner", &[sd, sid, 1]),
            1
        );
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "GetSecurityDescriptorOwner",
                &[sd, out, defaulted]
            ),
            1
        );
        let pointer = if is64 {
            emu.maps.read_qword(out)
        } else {
            emu.maps.read_dword(out).map(u64::from)
        };
        assert_eq!(pointer, Some(sid));
        assert_eq!(emu.maps.read_dword(defaulted), Some(1));
        assert_eq!(
            call(&mut emu, dll, "GetSecurityDescriptorLength", &[sd]),
            if is64 { 56 } else { 36 }
        );
        let acl = sd + 256;
        assert_eq!(
            call(&mut emu, "advapi32.dll", "InitializeAcl", &[acl, 64, 2]),
            1
        );
        assert_eq!(
            call(&mut emu, dll, "AddAccessAllowedAce", &[acl, 2, 0x1234, sid]),
            1
        );
        assert_eq!(emu.maps.read_word(acl + 4), Some(1));
        assert_eq!(emu.maps.read_word(acl + 10), Some(24));
        assert_eq!(
            call(&mut emu, "advapi32.dll", "FindFirstFreeAce", &[acl, out]),
            1
        );
        assert_eq!(read_pointer(&emu, out), Some(acl + 32));
        assert_eq!(emu.maps.read_dword(acl + 12), Some(0x1234));
    }
}

#[test]
fn remote_registry_delete_keeps_the_parent_open_until_close() {
    use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
    use crate::winapi::registry_store::dword;
    use helpers::call_winapi_export as call;

    let dll = "advapi32.dll";
    let parent = r"HKEY_CURRENT_USER\Remote";
    let child = r"HKEY_CURRENT_USER\Remote\Leaf";
    for is64 in [false, true] {
        let (mut emu, descriptor) = helpers::prepare_winapi(is64);
        let width = pointer_size(&emu);
        let text = descriptor + 32;
        let key_ptr = text + 64;
        assert_eq!(emu.registry.create_key(child), Some(true));
        let hkey = emu.handle_management.open_registry_key(parent.into()) as u64;
        assert!(emu.registry.set_value(parent, "Leaf", dword(7)));
        emu.maps.write_wide_string(text, "Leaf");
        let bytes = "Leaf".encode_utf16().count() as u16 * 2;
        assert!(emu.maps.write_word(descriptor, bytes));
        assert!(emu.maps.write_word(descriptor + 2, bytes + 2));
        assert!(write_pointer(&mut emu, descriptor + width, text));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "BaseRegOpenKey",
                &[hkey, descriptor, 0, KEY_READ.into(), key_ptr]
            ),
            ERROR_SUCCESS
        );
        let child_key = read_pointer(&emu, key_ptr).unwrap();
        assert!(write_pointer(&mut emu, key_ptr, hkey));

        emu.set_last_error(ERROR_MORE_DATA);
        assert_eq!(
            call(&mut emu, dll, "BaseRegFlushKey", &[hkey]),
            ERROR_SUCCESS
        );
        assert_eq!(
            call(&mut emu, dll, "BaseRegDeleteValue", &[hkey, descriptor]),
            ERROR_SUCCESS
        );
        assert!(emu.registry.get_value(parent, "Leaf").is_none());
        let both_views = (KEY_WOW64_32KEY | KEY_WOW64_64KEY) as u64;
        for (handle, name, access, expected) in [
            (hkey, 0, 0, ERROR_INVALID_PARAMETER),
            (hkey, descriptor, both_views, ERROR_INVALID_PARAMETER),
            (
                hkey,
                descriptor,
                KEY_WOW64_32KEY as u64,
                ERROR_NOT_SUPPORTED,
            ),
            (0, descriptor, 0, ERROR_INVALID_HANDLE),
        ] {
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    "BaseRegDeleteKeyEx",
                    &[handle, name, access, 0]
                ),
                expected
            );
            assert!(emu.registry.key_exists(child));
        }
        assert!(emu.maps.write_word(descriptor, bytes + 2));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "BaseRegDeleteKeyEx",
                &[hkey, descriptor, 0, 17]
            ),
            ERROR_SUCCESS
        );
        assert!(!emu.registry.key_exists(child));
        assert_eq!(
            call(&mut emu, dll, "BaseRegFlushKey", &[child_key]),
            ERROR_KEY_DELETED
        );
        assert_eq!(
            call(&mut emu, dll, "BaseRegFlushKey", &[hkey]),
            ERROR_SUCCESS
        );
        assert_eq!(
            call(
                &mut emu,
                dll,
                "BaseRegDeleteKeyEx",
                &[hkey, descriptor, 0, 0]
            ),
            ERROR_FILE_NOT_FOUND
        );
        assert_eq!(
            call(&mut emu, dll, "BaseRegCloseKey", &[key_ptr]),
            ERROR_SUCCESS
        );
        assert_eq!(read_pointer(&emu, key_ptr), Some(0));
        assert_eq!(
            call(&mut emu, dll, "BaseRegFlushKey", &[hkey]),
            ERROR_INVALID_HANDLE
        );
        assert_eq!(
            call(
                &mut emu,
                dll,
                "BaseRegOpenKey",
                &[hkey, descriptor, 0, KEY_READ.into(), key_ptr]
            ),
            ERROR_INVALID_HANDLE
        );
        assert_eq!(read_pointer(&emu, key_ptr), Some(0));
        assert_eq!(emu.get_last_error(), ERROR_MORE_DATA);
    }
}

#[test]
fn subkey_value_operations_agree_with_ordinary_queries() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::winapi::common::memory::read_pointer;
    use crate::winapi::registry_store::{REG_DWORD, REG_SZ};
    for is64 in [false, true] {
        let (mut emu, base) = prepare_winapi(is64);
        let (name, value, data, key, size) =
            (base + 96, base + 128, base + 192, base + 256, base + 272);
        let dll = "advapi32.dll";
        emu.maps.write_wide_string(base, "Software\\GuestValueTest");
        emu.maps.write_wide_string(name, "Value");
        emu.maps.write_dword(data, 12345);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RegSetKeyValueW",
                &[
                    HKEY_LOCAL_MACHINE,
                    base,
                    name,
                    u64::from(REG_DWORD),
                    data,
                    4
                ]
            ),
            ERROR_SUCCESS
        );
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RegOpenKeyW",
                &[HKEY_LOCAL_MACHINE, base, key]
            ),
            ERROR_SUCCESS
        );
        let opened = read_pointer(&emu, key).unwrap();
        emu.maps.write_dword(size, 4);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RegQueryValueExW",
                &[opened, name, 0, 0, value, size]
            ),
            ERROR_SUCCESS
        );
        assert_eq!(emu.maps.read_dword(value), Some(12345));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RegDeleteKeyValueW",
                &[HKEY_LOCAL_MACHINE, base, name]
            ),
            ERROR_SUCCESS
        );
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RegQueryValueExW",
                &[opened, name, 0, 0, value, size]
            ),
            ERROR_FILE_NOT_FOUND
        );
        emu.maps.write_wide_string(data, "Default");
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RegSetValueW",
                &[opened, 0, u64::from(REG_SZ), data, 1]
            ),
            ERROR_SUCCESS
        );
        emu.maps.write_dword(size, 32);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RegQueryValueExW",
                &[opened, 0, 0, 0, value, size]
            ),
            ERROR_SUCCESS
        );
        assert_eq!(emu.maps.read_wide_string(value), "Default");
        emu.maps.write_dword(size, 32);
        assert_eq!(
            call(&mut emu, dll, "RegQueryValueW", &[opened, 0, value, size]),
            ERROR_SUCCESS
        );
        assert_eq!(emu.maps.read_wide_string(value), "Default");
        assert_eq!(
            call(
                &mut emu,
                dll,
                "RegConnectRegistryW",
                &[0, HKEY_LOCAL_MACHINE, key]
            ),
            ERROR_SUCCESS
        );
        let local_machine = read_pointer(&emu, key).unwrap();
        assert_eq!(
            call(&mut emu, dll, "RegOpenKeyW", &[local_machine, base, key]),
            ERROR_SUCCESS
        );
        assert_eq!(
            call(&mut emu, dll, "RegCloseKey", &[local_machine]),
            ERROR_SUCCESS
        );
    }
}

#[test]
fn desktop_token_queries_report_session_and_security_flags_on_both_windows_abis() {
    use crate::windows::structures::{Peb, Peb64};
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let peb = emu.maps.map("peb", 4096, Permission::READ_WRITE);
        let session_offset = if is64 {
            Peb64::SESSION_ID_OFFSET
        } else {
            Peb::SESSION_ID_OFFSET
        };
        emu.maps.write_dword(peb + session_offset, 3);
        let returned = data + 16;
        let token = CURRENT_PROCESS_TOKEN as i64 as u64;
        for (class, expected) in [
            (TOKEN_SESSION_ID, 3),
            (TOKEN_ELEVATION, 0),
            (TOKEN_UI_ACCESS, 0),
            (TOKEN_IS_APP_CONTAINER, 0),
            (TOKEN_PRIVATE_NAME_SPACE, 0),
        ] {
            emu.maps.write_qword(data, u64::MAX);
            assert_eq!(
                call(
                    &mut emu,
                    "ntdll.dll",
                    "NtQueryInformationToken",
                    &[token, class as u64, data, 3, returned]
                ),
                if class == TOKEN_ELEVATION {
                    STATUS_INFO_LENGTH_MISMATCH
                } else {
                    STATUS_BUFFER_TOO_SMALL
                }
            );
            assert_eq!(emu.maps.read_qword(data), Some(u64::MAX));
            assert_eq!(
                call(
                    &mut emu,
                    "advapi32.dll",
                    "GetTokenInformation",
                    &[token, class as u64, data, 4, returned]
                ),
                1
            );
            assert_eq!(emu.maps.read_dword(data), Some(expected));
            assert_eq!(emu.maps.read_dword(data + 4), Some(u32::MAX));
            assert_eq!(emu.maps.read_dword(returned), Some(4));
        }
        assert_eq!(
            call(
                &mut emu,
                "advapi32.dll",
                "GetTokenInformation",
                &[token, TOKEN_ELEVATION as u64, data, 8, returned]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_BAD_LENGTH);
        emu.maps.write_dword(returned, u32::MAX);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "NtQueryInformationToken",
                &[token, TOKEN_ELEVATION as u64, data + 1, 4, returned]
            ),
            STATUS_DATATYPE_MISALIGNMENT
        );
        assert_eq!(emu.maps.read_dword(returned), Some(u32::MAX));
    }
}

#[test]
fn native_membership_check_distinguishes_the_call_status_from_the_access_decision() {
    use crate::winapi::common::memory::write_pointer;
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, sd) = helpers::prepare_winapi(is64);
        let width = if is64 { 8 } else { 4 };
        let acl = sd + 64;
        let ace = acl + 8;
        let sid = ace + 8;
        let mapping = sd + 160;
        let privileges = sd + 192;
        let length = sd + 240;
        let mask = sd + 248;
        let status = sd + 252;
        emu.maps.write_byte(sd, SECURITY_DESCRIPTOR_REVISION);
        emu.maps.write_word(sd + 2, SE_DACL_PRESENT);
        write_pointer(&mut emu, sd + width, sid);
        write_pointer(&mut emu, sd + 2 * width, sid);
        write_pointer(&mut emu, sd + 4 * width, acl);
        emu.maps.write_byte(acl, ACL_REVISION as u8);
        emu.maps.write_word(acl + 2, 32);
        emu.maps.write_word(acl + 4, 1);
        emu.maps.write_byte(ace, ACCESS_ALLOWED_ACE_TYPE);
        emu.maps.write_word(ace + 2, 24);
        emu.maps.write_dword(ace + 4, 1);
        emu.maps.write_bytes(sid, &[1, 2, 0, 0, 0, 0, 0, 5]);
        emu.maps.write_dword(sid + 8, SECURITY_BUILTIN_DOMAIN_RID);
        let args = [
            sd,
            CURRENT_PROCESS_TOKEN as i64 as u64,
            1,
            mapping,
            privileges,
            length,
            mask,
            status,
        ];
        for (rid, expected_access, expected_result) in [
            (DOMAIN_ALIAS_RID_USERS, 1, STATUS_SUCCESS),
            (DOMAIN_ALIAS_RID_DEFAULT_ACCOUNT, 0, STATUS_ACCESS_DENIED),
        ] {
            emu.maps.write_dword(sid + 12, rid);
            emu.maps.write_dword(length, 8);
            emu.maps.write_qword(mask, u64::MAX);
            assert_eq!(
                call(&mut emu, "ntdll.dll", "NtAccessCheck", &args),
                STATUS_BUFFER_TOO_SMALL
            );
            assert_eq!(emu.maps.read_dword(length), Some(20));
            assert_eq!(emu.maps.read_qword(mask), Some(u64::MAX));
            emu.maps.write_qword(privileges, u64::MAX);
            emu.maps.write_dword(privileges + 8, u32::MAX);
            assert_eq!(
                call(&mut emu, "ntdll.dll", "ZwAccessCheck", &args),
                STATUS_SUCCESS
            );
            assert_eq!(emu.maps.read_qword(privileges), Some(0));
            assert_eq!(emu.maps.read_dword(privileges + 8), Some(u32::MAX));
            assert_eq!(emu.maps.read_dword(mask), Some(expected_access));
            assert_eq!(emu.maps.read_dword(status), Some(expected_result as u32));
        }
    }
}

fn syscall(emu: &mut Emu, nr: u64, args: &[u64]) -> u64 {
    emu.syscall_number_map.insert(nr, nr);
    let sp = emu.sp();
    for (i, &arg) in args.iter().enumerate() {
        match i {
            0 => emu.regs_mut().rcx = arg,
            1 => emu.regs_mut().rdx = arg,
            2 => emu.regs_mut().r8 = arg,
            3 => emu.regs_mut().r9 = arg,
            _ => {
                assert!(emu.maps.write_qword(sp + (i as u64 + 1) * 8, arg));
            }
        }
    }
    emu.regs_mut().rax = nr;
    syscall64::gateway(emu);
    assert_eq!(emu.sp(), sp);
    emu.regs().rax
}

#[test]
fn typed_feature_notice_reports_the_guest_stamp_and_required_capacity() {
    use crate::windows::constants::syscalls::WIN64_NTQUERYWNFSTATEDATA;
    use crate::windows::constants::system::{
        FEATURE_CONFIGURATION_CHANGE_STAMP, WNF_CMFC_FEATURE_CONFIGURATION_CHANGED,
    };
    let (mut emu, data) = helpers::prepare_winapi(true);
    let (type_id, stamp, output, size) = (data + 8, data + 32, data + 40, data + 56);
    emu.maps
        .write_qword(data, WNF_CMFC_FEATURE_CONFIGURATION_CHANGED);
    emu.maps.write_bytes(
        type_id,
        &[
            0x63, 0x64, 0xf9, 0x48, 0xd6, 0x52, 0x85, 0x4e, 0x9c, 0x02, 0xf2, 0xc1, 0x4e, 0x17,
            0x26, 0x4e,
        ],
    );
    let args = [data, type_id, 0, stamp, output, size];
    emu.maps.write_qword(output, u64::MAX);
    emu.maps.write_dword(size, 4);
    assert_eq!(
        syscall(&mut emu, WIN64_NTQUERYWNFSTATEDATA, &args),
        STATUS_BUFFER_TOO_SMALL
    );
    assert_eq!(emu.maps.read_dword(size), Some(8));
    assert_eq!(emu.maps.read_dword(stamp), Some(1));
    assert_eq!(emu.maps.read_qword(output), Some(u64::MAX));
    assert_eq!(
        syscall(&mut emu, WIN64_NTQUERYWNFSTATEDATA, &args),
        STATUS_SUCCESS
    );
    assert_eq!(
        emu.maps.read_qword(output),
        Some(FEATURE_CONFIGURATION_CHANGE_STAMP)
    );
    emu.maps.write_dword(stamp, u32::MAX);
    emu.maps.write_dword(size, 16);
    emu.maps.write_byte(type_id, 0);
    assert_eq!(
        syscall(&mut emu, WIN64_NTQUERYWNFSTATEDATA, &args),
        STATUS_INVALID_PARAMETER
    );
    assert_eq!(emu.maps.read_dword(stamp), Some(u32::MAX));
    assert_eq!(emu.maps.read_dword(size), Some(16));
}

#[test]
fn feature_notice_subscriptions_are_stable_until_the_guest_unsubscribes() {
    use crate::windows::constants::syscalls::{
        WIN64_NTSUBSCRIBEWNFSTATECHANGE, WIN64_NTUNSUBSCRIBEWNFSTATECHANGE,
    };
    use crate::windows::constants::system::{
        WIN11_ETW_WNF_EVENT_MASK, WNF_CMFC_FEATURE_CONFIGURATION_CHANGED,
    };
    let (mut emu, state) = helpers::prepare_winapi(true);
    let out = state + 8;
    emu.maps
        .write_qword(state, WNF_CMFC_FEATURE_CONFIGURATION_CHANGED);
    let subscribe = [state, 1, WIN11_ETW_WNF_EVENT_MASK as u64, out];
    let unsubscribe = [state];

    let mut invalid_output = subscribe;
    invalid_output[3] = u64::MAX;
    assert_eq!(
        syscall(&mut emu, WIN64_NTSUBSCRIBEWNFSTATECHANGE, &invalid_output),
        STATUS_ACCESS_VIOLATION
    );
    assert_eq!(
        syscall(&mut emu, WIN64_NTUNSUBSCRIBEWNFSTATECHANGE, &unsubscribe),
        STATUS_OBJECT_NAME_NOT_FOUND
    );
    assert_eq!(
        syscall(&mut emu, WIN64_NTSUBSCRIBEWNFSTATECHANGE, &subscribe),
        STATUS_SUCCESS
    );
    let first = emu.maps.read_qword(out).unwrap();
    assert_ne!(first, 0);
    emu.maps.write_qword(out, 0);
    assert_eq!(
        syscall(&mut emu, WIN64_NTSUBSCRIBEWNFSTATECHANGE, &subscribe),
        STATUS_SUCCESS
    );
    assert_eq!(emu.maps.read_qword(out), Some(first));
    assert_eq!(
        syscall(&mut emu, WIN64_NTUNSUBSCRIBEWNFSTATECHANGE, &unsubscribe),
        STATUS_SUCCESS
    );
    assert_eq!(
        syscall(&mut emu, WIN64_NTUNSUBSCRIBEWNFSTATECHANGE, &unsubscribe),
        STATUS_OBJECT_NAME_NOT_FOUND
    );
    assert_eq!(
        syscall(&mut emu, WIN64_NTSUBSCRIBEWNFSTATECHANGE, &subscribe),
        STATUS_SUCCESS
    );
    assert_ne!(emu.maps.read_qword(out), Some(first));
}

#[test]
fn local_registry_handles_leave_provider_tags_free_across_snapshot_and_reuse() {
    use crate::emu::object_handle::HandleManagement;
    use crate::windows::constants::security::KEY_QUERY_VALUE;

    let mut handles = HandleManagement::new();
    let other = handles.insert_ipc_handle() as u64;
    let first = handles.open_registry_key("first".into()) as u64;
    let second = handles.open_registry_key_access("second".into(), KEY_QUERY_VALUE) as u64;
    assert_ne!(first, second);
    *handles.registry_tags_mut(second).unwrap() = 0x100;
    let mut restored = HandleManagement::from_memory_snapshot(handles.memory_snapshot());
    restored.restore_windows_snapshot(handles.windows_snapshot());
    for keys in [&mut handles, &mut restored] {
        assert_eq!(keys.native_type_name(other), Some("File"));
        assert_eq!(keys.registry_key(other), None);
        for (handle, path) in [(first, "first"), (second, "second")] {
            assert_ne!(handle, 0);
            assert_eq!(handle & 3, 0);
            assert_eq!(keys.registry_key(handle), Some(path));
            assert_eq!(keys.registry_key(handle | 1), None);
        }
        assert!(keys.close_registry_key(first));
        assert_eq!(keys.registry_key(first), None);
        let next = keys.open_registry_key("next".into()) as u64;
        assert_eq!(next, first);
        assert_eq!(keys.registry_key(next), Some("next"));
        assert_eq!(keys.registry_tags(next), Some(0));
        assert_eq!(keys.registry_tags(second), Some(0x100));
        assert_eq!(keys.registry_key(second), Some("second"));
        let file = keys.insert_ipc_handle() as u64;
        assert!(file < second);
        assert_ne!(file, other);
        assert_eq!(keys.registry_key(file), None);
        assert_eq!(keys.native_type_name(file), Some("File"));
    }
}

#[test]
fn registry_name_notifications_signal_once_on_change_or_close() {
    use crate::winapi::registry_store::{REG_NOTIFY_CHANGE_NAME, REG_NOTIFY_THREAD_AGNOSTIC};
    use crate::windows::constants::syscalls::{
        WIN64_NTCLOSE, WIN64_NTCREATEEVENT, WIN64_NTNOTIFYCHANGEKEY, WIN64_NTRESETEVENT,
        WIN64_NTWAITFORSINGLEOBJECT,
    };

    let (mut emu, data) = helpers::prepare_winapi(true);
    let (name, output, timeout, iosb) = (data, data + 0x100, data + 0x110, data + 0x120);
    let access = SYNCHRONIZE | EVENT_MODIFY_STATE;
    assert_eq!(
        syscall(
            &mut emu,
            WIN64_NTCREATEEVENT,
            &[output, access as u64, 0, 0, 1]
        ),
        STATUS_SUCCESS
    );
    let event = emu.maps.read_qword(output).unwrap();
    let root = "HKEY_CURRENT_USER\\Software";
    let denied = emu
        .handle_management
        .open_registry_key_access(root.to_owned(), KEY_QUERY_VALUE) as u64;
    let watched = emu
        .handle_management
        .open_registry_key_access(root.to_owned(), KEY_NOTIFY) as u64;
    let notify = |emu: &mut Emu, key: u64, subtree: bool| {
        syscall(
            emu,
            WIN64_NTNOTIFYCHANGEKEY,
            &[
                key,
                event,
                0,
                0,
                iosb,
                (REG_NOTIFY_THREAD_AGNOSTIC | REG_NOTIFY_CHANGE_NAME) as u64,
                subtree as u64,
                0,
                0,
                1,
            ],
        )
    };
    let wait = |emu: &mut Emu| syscall(emu, WIN64_NTWAITFORSINGLEOBJECT, &[event, 0, timeout]);
    let create = |emu: &mut Emu, subkey: &str| {
        emu.maps.write_wide_string(name, subkey);
        assert_eq!(
            helpers::call_winapi_export(
                emu,
                "advapi32.dll",
                "RegCreateKeyW",
                &[watched, name, output]
            ),
            ERROR_SUCCESS
        );
    };
    emu.maps.write_qword(iosb, u64::MAX);
    emu.maps.write_qword(iosb + 8, u64::MAX);

    assert_eq!(notify(&mut emu, denied, true), STATUS_ACCESS_DENIED);
    assert_eq!(wait(&mut emu), STATUS_SUCCESS);
    assert_eq!(notify(&mut emu, watched, false), STATUS_PENDING);
    assert_eq!(wait(&mut emu), STATUS_TIMEOUT);
    create(&mut emu, "Microsoft\\UnwatchedGrandchild");
    assert_eq!(wait(&mut emu), STATUS_TIMEOUT);
    create(&mut emu, "NewParent\\NewChild");
    assert_eq!(wait(&mut emu), STATUS_SUCCESS);

    assert_eq!(notify(&mut emu, watched, true), STATUS_PENDING);
    create(&mut emu, "NewParent\\NewChild");
    assert_eq!(wait(&mut emu), STATUS_TIMEOUT);
    create(&mut emu, "NewParent\\OtherChild");
    assert_eq!(wait(&mut emu), STATUS_SUCCESS);
    assert_eq!(
        syscall(&mut emu, WIN64_NTRESETEVENT, &[event, 0]),
        STATUS_SUCCESS
    );
    create(&mut emu, "AfterDelivery");
    assert_eq!(wait(&mut emu), STATUS_TIMEOUT);

    assert_eq!(notify(&mut emu, watched, true), STATUS_PENDING);
    assert_eq!(syscall(&mut emu, WIN64_NTCLOSE, &[watched]), STATUS_SUCCESS);
    assert_eq!(wait(&mut emu), STATUS_SUCCESS);
    assert_eq!(emu.maps.read_qword(iosb), Some(u64::MAX));
    assert_eq!(emu.maps.read_qword(iosb + 8), Some(u64::MAX));
}
