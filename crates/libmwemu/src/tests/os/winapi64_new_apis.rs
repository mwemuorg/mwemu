use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi64;
use crate::*;

// ─── kernel32: GlobalAlloc / GlobalFree / GlobalLock / GlobalUnlock ───

#[test]
fn test_global_alloc_basic() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[0x0000, 0x200], // GMEM_FIXED, 0x200 bytes
    );
    assert_ne!(ptr, 0, "GlobalAlloc returned NULL");
    assert!(emu.maps.is_mapped(ptr), "GlobalAlloc pointer not mapped");

    emu.maps.write_dword(ptr, 0xDEADC0DE);
    assert_eq!(emu.maps.read_dword(ptr).unwrap(), 0xDEADC0DE);
}

#[test]
fn test_global_alloc_zeroinit() {
    helpers::setup();
    let mut emu = emu64();

    // GPTR = GMEM_FIXED | GMEM_ZEROINIT = 0x0040
    let ptr = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalAlloc, &[0x0040, 0x100]);
    assert_ne!(ptr, 0, "GlobalAlloc(GPTR) returned NULL");

    for i in 0..16u64 {
        assert_eq!(
            emu.maps.read_byte(ptr + i).unwrap(),
            0,
            "byte at +{} should be zero after GMEM_ZEROINIT",
            i
        );
    }
}

#[test]
fn test_global_free() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalAlloc, &[0, 0x100]);
    assert_ne!(ptr, 0);

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalFree, &[ptr]);
    assert_eq!(ret, 0, "GlobalFree should return NULL on success");
}

#[test]
fn test_global_lock_unlock() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalAlloc, &[0, 0x100]);
    assert_ne!(ptr, 0);

    let locked = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalLock, &[ptr]);
    assert_eq!(
        locked, ptr,
        "GlobalLock should return the same pointer for GMEM_FIXED"
    );

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalUnlock, &[ptr]);
    assert_eq!(ret, 1, "GlobalUnlock returns TRUE");
}

// ─── kernel32: HeapSize ───

#[test]
fn test_heap_size() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[0x1234, 0, 0x200]);
    assert_ne!(ptr, 0);

    let size = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapSize, &[0x1234, 0, ptr]);
    assert!(
        size >= 0x200,
        "HeapSize should be >= requested (got {})",
        size
    );
}

#[test]
fn test_heap_size_invalid_ptr() {
    helpers::setup();
    let mut emu = emu64();

    let size = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapSize, &[0x1234, 0, 0xDEAD]);
    assert_eq!(
        size, 0xFFFFFFFFFFFFFFFF,
        "HeapSize of invalid ptr should return -1"
    );
}

// ─── kernel32: SRW Locks ───

#[test]
fn test_srw_lock_lifecycle() {
    helpers::setup();
    let mut emu = emu64();

    let lock_addr = 0x100000u64;
    emu.maps
        .create_map("srwlock", lock_addr, 0x1000, Permission::READ_WRITE);

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::InitializeSRWLock,
        &[lock_addr],
    );
    assert_eq!(
        emu.maps.read_qword(lock_addr).unwrap(),
        0,
        "SRWLock should be zeroed after init"
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::AcquireSRWLockExclusive,
        &[lock_addr],
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::ReleaseSRWLockExclusive,
        &[lock_addr],
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::AcquireSRWLockShared,
        &[lock_addr],
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::ReleaseSRWLockShared,
        &[lock_addr],
    );
}

#[test]
fn test_try_acquire_srw_lock() {
    helpers::setup();
    let mut emu = emu64();

    let lock_addr = 0x100000u64;
    emu.maps
        .create_map("srwlock", lock_addr, 0x1000, Permission::READ_WRITE);

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::InitializeSRWLock,
        &[lock_addr],
    );

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::TryAcquireSRWLockExclusive,
        &[lock_addr],
    );
    assert_eq!(
        ret, 1,
        "TryAcquireSRWLockExclusive should succeed (return 1)"
    );

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::TryAcquireSRWLockShared,
        &[lock_addr],
    );
    assert_eq!(ret, 1, "TryAcquireSRWLockShared should succeed (return 1)");
}

// ─── kernel32: WaitForMultipleObjects ───

#[test]
fn test_wait_for_multiple_objects() {
    helpers::setup();
    let mut emu = emu64();

    let handles_addr = 0x100000u64;
    emu.maps
        .create_map("handles", handles_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(handles_addr, 0x10);
    emu.maps.write_qword(handles_addr + 8, 0x20);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::WaitForMultipleObjects,
        &[2, handles_addr, 0, 100],
    );
    assert_eq!(
        ret, 0,
        "WaitForMultipleObjects should return WAIT_OBJECT_0 (0)"
    );
}

// ─── kernel32: IsWow64Process ───

#[test]
fn test_is_wow64_process() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("wow64out", out_addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::IsWow64Process,
        &[0xFFFFFFFF, out_addr], // current process pseudo-handle
    );
    assert_eq!(ret, 1, "IsWow64Process should return TRUE");
    assert_eq!(
        emu.maps.read_qword(out_addr).unwrap(),
        0,
        "64-bit process should report WoW64 = FALSE"
    );
}

// ─── kernel32: DuplicateHandle ───

#[test]
fn test_duplicate_handle() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("dupout", out_addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::DuplicateHandle,
        &[0xFFFFFFFF, 0x42, 0xFFFFFFFF, out_addr],
    );
    assert_eq!(ret, 1, "DuplicateHandle should return TRUE");
    assert_eq!(
        emu.maps.read_qword(out_addr).unwrap(),
        0x42,
        "duplicated handle should match source"
    );
}

// ─── kernel32: GetTickCount64 ───

#[test]
fn test_get_tick_count64() {
    helpers::setup();
    let mut emu = emu64();

    emu.tick = 12345;
    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetTickCount64, &[]);
    assert_eq!(ret, 12345, "GetTickCount64 should return emu.tick");
}

// ─── kernel32: QueryPerformanceFrequency ───

#[test]
fn test_query_performance_frequency() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("qpf", out_addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::QueryPerformanceFrequency,
        &[out_addr],
    );
    assert_eq!(ret, 1, "QueryPerformanceFrequency should return TRUE");
    assert_eq!(
        emu.maps.read_qword(out_addr).unwrap(),
        10_000_000,
        "frequency should be 10MHz"
    );
}

// ─── kernel32: FreeLibrary ───

#[test]
fn test_free_library() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::FreeLibrary, &[0x7FF00000]);
    assert_eq!(ret, 1, "FreeLibrary should return TRUE");
}

// ─── kernel32: SetEnvironmentVariableA ───

#[test]
fn test_set_environment_variable_a() {
    helpers::setup();
    let mut emu = emu64();

    let name_addr = 0x100000u64;
    let value_addr = 0x100100u64;
    emu.maps
        .create_map("envvar", name_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_addr, "MY_VAR");
    emu.maps.write_string(value_addr, "MY_VALUE");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::SetEnvironmentVariableA,
        &[name_addr, value_addr],
    );
    assert_eq!(ret, 1, "SetEnvironmentVariableA should return TRUE");
}

// ─── kernel32: FormatMessageA ───

#[test]
fn test_format_message_a() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::FormatMessageA,
        &[0x1000, 0, 0, 0],
    );
    assert_eq!(ret, 0, "FormatMessageA stub should return 0");
}

// ─── kernel32: CreateDirectoryA ───

#[test]
fn test_create_directory_a() {
    helpers::setup();
    let mut emu = emu64();

    let path_addr = 0x100000u64;
    emu.maps
        .create_map("dirpath", path_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(path_addr, "C:\\test_dir");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::CreateDirectoryA,
        &[path_addr, 0],
    );
    assert_eq!(ret, 1, "CreateDirectoryA stub should return TRUE");
}

// ─── kernel32: DeleteFileW ───

#[test]
fn test_delete_file_w() {
    helpers::setup();
    let mut emu = emu64();

    let path_addr = 0x100000u64;
    emu.maps
        .create_map("filepath", path_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(path_addr, "C:\\test.txt");

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::DeleteFileW, &[path_addr]);
    assert_eq!(ret, 1, "DeleteFileW stub should return TRUE");
}

// ─── kernel32: OutputDebugStringA ───

#[test]
fn test_output_debug_string_a() {
    helpers::setup();
    let mut emu = emu64();

    let msg_addr = 0x100000u64;
    emu.maps
        .create_map("dbgmsg", msg_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(msg_addr, "Debug message");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::OutputDebugStringA,
        &[msg_addr],
    );
    assert_eq!(ret, 0);
}

// ─── ntdll: RtlInitUnicodeString ───

#[test]
fn test_rtl_init_unicode_string() {
    helpers::setup();
    let mut emu = emu64();

    let dest_addr = 0x100000u64;
    let source_addr = 0x100100u64;
    emu.maps
        .create_map("unistr", dest_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(source_addr, "Hello");

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitUnicodeString,
        &[dest_addr, source_addr],
    );

    // "Hello" = 5 UTF-16 code units = 10 bytes
    let length = emu.maps.read_word(dest_addr).unwrap();
    let max_length = emu.maps.read_word(dest_addr + 2).unwrap();
    let buffer = emu.maps.read_qword(dest_addr + 8).unwrap();

    assert_eq!(length, 10, "UNICODE_STRING.Length should be 10 for 'Hello'");
    assert_eq!(max_length, 12, "UNICODE_STRING.MaximumLength should be 12");
    assert_eq!(
        buffer, source_addr,
        "UNICODE_STRING.Buffer should point to source"
    );
}

#[test]
fn test_rtl_init_unicode_string_null() {
    helpers::setup();
    let mut emu = emu64();

    let dest_addr = 0x100000u64;
    emu.maps
        .create_map("unistr", dest_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(dest_addr, 0xFFFFFFFF_FFFFFFFF);
    emu.maps.write_qword(dest_addr + 8, 0xFFFFFFFF_FFFFFFFF);

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitUnicodeString,
        &[dest_addr, 0],
    );

    assert_eq!(emu.maps.read_qword(dest_addr).unwrap(), 0);
    assert_eq!(emu.maps.read_qword(dest_addr + 8).unwrap(), 0);
}

// ─── ntdll: RtlInitAnsiString ───

#[test]
fn test_rtl_init_ansi_string() {
    helpers::setup();
    let mut emu = emu64();

    let dest_addr = 0x100000u64;
    let source_addr = 0x100100u64;
    emu.maps
        .create_map("ansistr", dest_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(source_addr, "TestStr");

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitAnsiString,
        &[dest_addr, source_addr],
    );

    let length = emu.maps.read_word(dest_addr).unwrap();
    let max_length = emu.maps.read_word(dest_addr + 2).unwrap();
    let buffer = emu.maps.read_qword(dest_addr + 8).unwrap();

    assert_eq!(length, 7, "ANSI_STRING.Length should be 7 for 'TestStr'");
    assert_eq!(max_length, 8, "ANSI_STRING.MaximumLength should be 8");
    assert_eq!(
        buffer, source_addr,
        "ANSI_STRING.Buffer should point to source"
    );
}

// ─── ntdll: NtCreateSection + NtMapViewOfSection + NtUnmapViewOfSection ───

#[test]
fn test_nt_section_lifecycle() {
    helpers::setup();
    let mut emu = emu64();

    let handle_ptr = 0x100000u64;
    let base_ptr = 0x100010u64;
    let max_size_ptr = 0x100020u64;
    let view_size_ptr = 0x100030u64;
    emu.maps
        .create_map("section_ptrs", handle_ptr, 0x1000, Permission::READ_WRITE);

    // Write max size
    emu.maps.write_qword(max_size_ptr, 0x10000);

    // NtCreateSection(handle_ptr, SECTION_ALL_ACCESS, NULL, max_size_ptr, PAGE_READWRITE, SEC_COMMIT, NULL)
    let rsp = emu.regs().rsp;
    emu.maps.write_dword(rsp + 0x20, 0x04); // PAGE_READWRITE
    emu.maps.write_qword(rsp + 0x28, 0x08000000); // SEC_COMMIT
    emu.maps.write_qword(rsp + 0x30, 0); // no file handle

    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtCreateSection,
        &[handle_ptr, 0xF001F, 0, max_size_ptr],
    );
    assert_eq!(status, 0, "NtCreateSection should return STATUS_SUCCESS");

    let section_handle = emu.maps.read_qword(handle_ptr).unwrap();
    assert_ne!(section_handle, 0, "section handle should be non-zero");

    // NtMapViewOfSection(section_handle, process, base_ptr, 0, 0, NULL, view_size_ptr, ...)
    emu.maps.write_qword(view_size_ptr, 0x10000);
    emu.maps.write_qword(rsp + 0x20, 0); // commit size
    emu.maps.write_qword(rsp + 0x28, 0); // section offset
    emu.maps.write_qword(rsp + 0x30, view_size_ptr);

    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtMapViewOfSection,
        &[section_handle, 0xFFFFFFFF, base_ptr, 0],
    );
    assert_eq!(status, 0, "NtMapViewOfSection should return STATUS_SUCCESS");

    let mapped_base = emu.maps.read_qword(base_ptr).unwrap();
    assert_ne!(mapped_base, 0, "mapped base should be non-zero");
    assert!(
        emu.maps.is_mapped(mapped_base),
        "mapped region should be accessible"
    );

    emu.maps.write_dword(mapped_base, 0xCAFEBABE);
    assert_eq!(emu.maps.read_dword(mapped_base).unwrap(), 0xCAFEBABE);

    // NtUnmapViewOfSection
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtUnmapViewOfSection,
        &[0xFFFFFFFF, mapped_base],
    );
    assert_eq!(
        status, 0,
        "NtUnmapViewOfSection should return STATUS_SUCCESS"
    );
}

// ─── ntdll: NtQueryInformationProcess ───

#[test]
fn test_nt_query_information_process() {
    helpers::setup();
    let mut emu = emu64();

    let buf_addr = 0x100000u64;
    let ret_len_addr = 0x100100u64;
    emu.maps
        .create_map("qip", buf_addr, 0x1000, Permission::READ_WRITE);

    // NtQueryInformationProcess(handle, class, buffer, length, return_length_ptr)
    // return_length_ptr is the 5th arg (at rsp+0x20)
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtQueryInformationProcess,
        &[0xFFFFFFFF, 0, buf_addr, 48, ret_len_addr],
    );
    assert_eq!(
        status, 0,
        "NtQueryInformationProcess should return STATUS_SUCCESS"
    );

    let ret_len = emu.maps.read_dword(ret_len_addr).unwrap();
    assert_eq!(
        ret_len, 48,
        "returned length should be 48 for ProcessBasicInformation"
    );
}

// ─── ntdll: NtQuerySystemInformation ───

#[test]
fn test_nt_query_system_information() {
    helpers::setup();
    let mut emu = emu64();

    let buf_addr = 0x100000u64;
    let ret_len_addr = 0x100100u64;
    emu.maps
        .create_map("qsi", buf_addr, 0x1000, Permission::READ_WRITE);

    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtQuerySystemInformation,
        &[0, buf_addr, 64, ret_len_addr],
    );
    assert_eq!(
        status, 0,
        "NtQuerySystemInformation should return STATUS_SUCCESS"
    );

    let ret_len = emu.maps.read_dword(ret_len_addr).unwrap();
    assert_eq!(ret_len, 64);
}

// ─── ntdll: RtlDecompressBuffer ───

#[test]
fn test_rtl_decompress_buffer() {
    helpers::setup();
    let mut emu = emu64();

    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlDecompressBuffer,
        &[0x0002, 0, 0, 0], // COMPRESSION_FORMAT_LZNT1
    );
    assert_ne!(
        status, 0,
        "RtlDecompressBuffer stub should return STATUS_NOT_IMPLEMENTED"
    );
}

// ─── ntdll: NtDelayExecution ───

#[test]
fn test_nt_delay_execution() {
    helpers::setup();
    let mut emu = emu64();

    let interval_addr = 0x100000u64;
    emu.maps
        .create_map("delay", interval_addr, 0x1000, Permission::READ_WRITE);

    let start_tick = emu.tick;

    // -500_000_0 = -5_000_000 (100ns units) = 500ms relative
    let interval: i64 = -5_000_000;
    emu.maps.write_qword(interval_addr, interval as u64);

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtDelayExecution,
        &[0, interval_addr],
    );

    assert!(
        emu.tick > start_tick,
        "NtDelayExecution should advance tick (start: {}, now: {})",
        start_tick,
        emu.tick
    );
}

// ─── ntdll: NtFreeVirtualMemory ───

#[test]
fn test_nt_free_virtual_memory() {
    helpers::setup();
    let mut emu = emu64();

    // First allocate some virtual memory
    let base = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAlloc,
        &[0, 0x1000, 0x3000, 0x40],
    );
    assert_ne!(base, 0);
    assert!(emu.maps.is_mapped(base));

    let base_ptr_addr = 0x100000u64;
    emu.maps
        .create_map("freevm", base_ptr_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(base_ptr_addr, base);

    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtFreeVirtualMemory,
        &[0xFFFFFFFF, base_ptr_addr, 0, 0x8000], // MEM_RELEASE
    );
    assert_eq!(
        status, 0,
        "NtFreeVirtualMemory should return STATUS_SUCCESS"
    );
}

// ─── advapi32: CryptAcquireContextA + CryptReleaseContext ───

#[test]
fn test_crypt_acquire_release_context() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("crypt", out_addr, 0x1000, Permission::READ_WRITE);

    // CryptAcquireContextA(phProv, NULL, NULL, PROV_RSA_FULL=1, CRYPT_VERIFYCONTEXT=0xF0000000)
    emu.maps.write_qword(emu.regs().rsp + 0x20, 0xF0000000);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptAcquireContextA,
        &[out_addr, 0, 0, 1],
    );
    assert_eq!(ret, 1, "CryptAcquireContextA should return TRUE");

    let handle = emu.maps.read_qword(out_addr).unwrap();
    assert_ne!(handle, 0, "crypto handle should be non-zero");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptReleaseContext,
        &[handle, 0],
    );
    assert_eq!(ret, 1, "CryptReleaseContext should return TRUE");
}

// ─── advapi32: CryptGenRandom ───

#[test]
fn test_crypt_gen_random() {
    helpers::setup();
    let mut emu = emu64();

    let buf_addr = 0x100000u64;
    emu.maps
        .create_map("rng", buf_addr, 0x1000, Permission::READ_WRITE);

    for i in 0..16u64 {
        emu.maps.write_byte(buf_addr + i, 0);
    }

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptGenRandom,
        &[0x1234, 16, buf_addr],
    );
    assert_eq!(ret, 1, "CryptGenRandom should return TRUE");

    let mut all_zero = true;
    for i in 0..16u64 {
        if emu.maps.read_byte(buf_addr + i).unwrap() != 0 {
            all_zero = false;
            break;
        }
    }
    assert!(
        !all_zero,
        "CryptGenRandom should fill buffer with non-zero data"
    );
}

// ─── advapi32: CryptCreateHash + CryptHashData ───

#[test]
fn test_crypt_create_hash() {
    helpers::setup();
    let mut emu = emu64();

    let hash_out = 0x100000u64;
    emu.maps
        .create_map("hash", hash_out, 0x1000, Permission::READ_WRITE);

    // CryptCreateHash(hProv, ALG_MD5=0x8003, hKey=0, flags=0, phHash)
    // phHash is the 5th argument (at rsp+0x20)
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptCreateHash,
        &[0x1234, 0x8003, 0, 0, hash_out],
    );
    assert_eq!(ret, 1, "CryptCreateHash should return TRUE");

    let hash_handle = emu.maps.read_qword(hash_out).unwrap();
    assert_ne!(hash_handle, 0, "hash handle should be non-zero");
}

// ─── advapi32: CryptGenKey ───

#[test]
fn test_crypt_gen_key() {
    helpers::setup();
    let mut emu = emu64();

    let key_out = 0x100000u64;
    emu.maps
        .create_map("genkey", key_out, 0x1000, Permission::READ_WRITE);

    // CryptGenKey(hProv, ALG_ID, flags, phKey)
    // phKey is r9 (4th arg)
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptGenKey,
        &[0x1234, 0x6801, 0, key_out],
    );
    assert_eq!(ret, 1, "CryptGenKey should return TRUE");

    let key_handle = emu.maps.read_qword(key_out).unwrap();
    assert_ne!(key_handle, 0, "key handle should be non-zero");
}

// ─── advapi32: CryptEncrypt / CryptDecrypt ───

#[test]
fn test_crypt_encrypt_decrypt() {
    helpers::setup();
    let mut emu = emu64();

    let data_addr = 0x100000u64;
    let len_addr = 0x100100u64;
    emu.maps
        .create_map("crypted", data_addr, 0x1000, Permission::READ_WRITE);

    emu.maps.write_qword(emu.regs().rsp + 0x20, data_addr);
    emu.maps.write_qword(emu.regs().rsp + 0x28, len_addr);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptEncrypt,
        &[0x1234, 0, 1, 0],
    );
    assert_eq!(ret, 1, "CryptEncrypt should return TRUE");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptDecrypt,
        &[0x1234, 0, 1, 0],
    );
    assert_eq!(ret, 1, "CryptDecrypt should return TRUE");
}

// ─── advapi32: AdjustTokenPrivileges ───

#[test]
fn test_adjust_token_privileges() {
    helpers::setup();
    let mut emu = emu64();

    let data_addr = 0x100000u64;
    emu.maps
        .create_map("tokpriv", data_addr, 0x1000, Permission::READ_WRITE);

    emu.maps.write_qword(emu.regs().rsp + 0x20, 0);
    emu.maps.write_qword(emu.regs().rsp + 0x28, 0);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::AdjustTokenPrivileges,
        &[0x1234, 0, data_addr, 0],
    );
    assert_eq!(ret, 1, "AdjustTokenPrivileges should return TRUE");
}

// ─── advapi32: RegOpenKeyExW ───

#[test]
fn test_reg_open_key_ex_w() {
    helpers::setup();
    let mut emu = emu64();

    let subkey_addr = 0x100000u64;
    let result_addr = 0x100100u64;
    emu.maps
        .create_map("regdata", subkey_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(subkey_addr, "Software\\Test");

    // RegOpenKeyExW(hKey=HKLM=0x80000002, lpSubKey, ulOptions=0, samDesired, phkResult)
    emu.maps.write_qword(emu.regs().rsp + 0x20, result_addr);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::RegOpenKeyExW,
        &[0x80000002, subkey_addr, 0, 0xF003F],
    );
    assert_eq!(ret, 0, "RegOpenKeyExW should return ERROR_SUCCESS (0)");
}

// ─── bcrypt: BCryptOpenAlgorithmProvider + BCryptGenRandom + BCryptClose ───

#[test]
fn test_bcrypt_lifecycle() {
    helpers::setup();
    let mut emu = emu64();

    let handle_addr = 0x100000u64;
    let algo_addr = 0x100100u64;
    let buf_addr = 0x100200u64;
    emu.maps
        .create_map("bcrypt_data", handle_addr, 0x1000, Permission::READ_WRITE);

    emu.maps.write_wide_string(algo_addr, "AES");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::bcrypt::BCryptOpenAlgorithmProvider,
        &[handle_addr, algo_addr, 0, 0],
    );
    assert_eq!(
        ret, 0,
        "BCryptOpenAlgorithmProvider should return STATUS_SUCCESS"
    );

    let handle = emu.maps.read_qword(handle_addr).unwrap();
    assert_ne!(handle, 0, "bcrypt handle should be non-zero");

    // BCryptGenRandom
    for i in 0..16u64 {
        emu.maps.write_byte(buf_addr + i, 0);
    }
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::bcrypt::BCryptGenRandom,
        &[handle, buf_addr, 16, 0],
    );
    assert_eq!(ret, 0, "BCryptGenRandom should return STATUS_SUCCESS");

    // BCryptCloseAlgorithmProvider
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::bcrypt::BCryptCloseAlgorithmProvider,
        &[handle, 0],
    );
    assert_eq!(
        ret, 0,
        "BCryptCloseAlgorithmProvider should return STATUS_SUCCESS"
    );
}

// ─── kernel32: GlobalAlloc + HeapSize integration ───

#[test]
fn test_global_alloc_heap_size_integration() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalAlloc, &[0, 0x300]);
    assert_ne!(ptr, 0);

    let size = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapSize, &[0x1234, 0, ptr]);
    assert!(
        size >= 0x300,
        "HeapSize of GlobalAlloc'd memory should be >= requested (got {})",
        size
    );
}

// ─── kernel32: two GlobalAlloc must not overlap ───

#[test]
fn test_global_alloc_no_overlap() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalAlloc, &[0, 0x200]);
    let p2 = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalAlloc, &[0, 0x200]);
    assert_ne!(p1, 0);
    assert_ne!(p2, 0);
    assert_ne!(
        p1, p2,
        "two GlobalAlloc calls must return different pointers"
    );
}
