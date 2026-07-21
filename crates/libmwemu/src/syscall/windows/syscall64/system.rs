use crate::emu::Emu;
use crate::windows::constants::*;

/// Synthetic kernel image base/size/full path used by `SystemModuleInformation`
/// and `SystemModuleInformationEx`. The values are stable across runs.
const FAKE_KERNEL_BASE: u64 = 0xFFFF_F800_0000_0000;
const FAKE_KERNEL_SIZE: u32 = 0x00A0_0000;
const FAKE_KERNEL_DIR: &[u8] = b"\\SystemRoot\\system32\\";
const FAKE_KERNEL_FULL_PATH: &[u8] = b"\\SystemRoot\\system32\\ntoskrnl.exe";

/// Synthetic `SYSTEM_PROCESS_INFORMATION` PID. Mirrors Sogen's behaviour where
/// the emulated process reports PID 1 and uses `ThreadContext.id` for thread
/// IDs (since `ThreadContext` does not own per-thread TEBs).
const SYNTHETIC_PROCESS_ID: u64 = 1;
const SYNTHETIC_PARENT_PROCESS_ID: u64 = 4;
const SYNTHETIC_HANDLE_COUNT: u32 = 3;
const SYNTHETIC_SESSION_ID: u32 = 1;

/// `SYSTEM_INFORMATION_CLASS` values commonly hit during ntdll bootstrap and
/// the practical Sogen-compatible classes. Numeric assignments are sourced from
/// PHNT `ntexapi.h` (x64 Windows). Local to this module because the numbers
/// reflect the implementation contract.
const SYSTEM_BASIC_INFORMATION: u64 = 0x00;
const SYSTEM_PROCESSOR_INFORMATION: u64 = 0x01;
const SYSTEM_PERFORMANCE_INFORMATION: u64 = 0x02;
const SYSTEM_TIME_OF_DAY_INFORMATION: u64 = 0x03;
const SYSTEM_PROCESS_INFORMATION: u64 = 0x05;
const SYSTEM_DEVICE_INFORMATION: u64 = 0x07;
const SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION: u64 = 0x08;
const SYSTEM_MODULE_INFORMATION: u64 = 0x0B;
const SYSTEM_FILE_CACHE_INFORMATION: u64 = 0x15;
const SYSTEM_EXCEPTION_INFORMATION: u64 = 0x21;
const SYSTEM_KERNEL_DEBUGGER_INFORMATION: u64 = 0x23;
const SYSTEM_FULL_MEMORY_INFORMATION: u64 = 0x19;
const SYSTEM_SUMMARY_MEMORY_INFORMATION: u64 = 0x1D;
const SYSTEM_CURRENT_TIME_ZONE_INFORMATION: u64 = 0x2C;
const SYSTEM_RANGE_START_INFORMATION: u64 = 0x32;
const SYSTEM_NUMA_PROCESSOR_MAP: u64 = 0x37;
const SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT: u64 = 0x3A;
const SYSTEM_NUMA_AVAILABLE_MEMORY: u64 = 0x3C;
const SYSTEM_EXTENDED_HANDLE_INFORMATION: u64 = 0x40;
const SYSTEM_LOGICAL_PROCESSOR_INFORMATION: u64 = 0x49;
const SYSTEM_MODULE_INFORMATION_EX: u64 = 0x4D;
const SYSTEM_MEMORY_LIST_INFORMATION: u64 = 0x50;
const SYSTEM_FILE_CACHE_INFORMATION_EX: u64 = 0x51;
const SYSTEM_BOOT_ENVIRONMENT_INFORMATION: u64 = 0x5A;
const SYSTEM_TIME_ZONE_INFORMATION: u64 = 0x5D;
const SYSTEM_DYNAMIC_TIME_ZONE_INFORMATION: u64 = 0x66;
const SYSTEM_CODE_INTEGRITY_INFORMATION: u64 = 0x67;
const SYSTEM_ERROR_PORT_TIMEOUTS: u64 = 0x73;
const SYSTEM_EMULATION_BASIC_INFORMATION: u64 = 0x3E;
const SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX: u64 = 0x95;
const SYSTEM_CODE_INTEGRITY_POLICY_INFORMATION: u64 = 0xA4;
const SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES: u64 = 0xB5;
const SYSTEM_MEMORY_USAGE_INFORMATION: u64 = 0xB6;
const SYSTEM_FLUSH_INFORMATION: u64 = 0xC0;
const SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION: u64 = 0xC5;
const SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES2: u64 = 0xE6;

/// Fixed payload sizes for the classes we model directly. The values are the
/// authoritative native x64 sizes from PHNT `ntexapi.h`. Callers must accept at
/// least these sizes.
const SYSTEM_BASIC_INFO_SIZE: u32 = 0x40;
const SYSTEM_PROCESSOR_INFO_SIZE: u32 = 0x0C;
const SYSTEM_TIME_OF_DAY_INFO_SIZE: u32 = 0x30;
const SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE: u32 = 0x30;
const SYSTEM_DEVICE_INFO_SIZE: u32 = 0x18;
const SYSTEM_FILE_CACHE_INFO_SIZE: u32 = 0x3C;
const SYSTEM_EXCEPTION_INFO_SIZE: u32 = 0x10;
const SYSTEM_MODULE_INFO_REQUIRED: u32 = 0x130;
const SYSTEM_MODULE_INFO_EX_REQUIRED: u32 = 0x140;
const SYSTEM_PROCESS_INFO_PREFIX_SIZE: u32 = 0x100;
const SYSTEM_THREAD_INFO_SIZE: u32 = 0x50;
const SYSTEM_CODE_INTEGRITY_INFO_SIZE: u32 = 0x08;
const SYSTEM_KERNEL_DEBUGGER_INFO_SIZE: u32 = 0x02;
const SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE: u32 = 0x03;
const SYSTEM_EXTENDED_HANDLE_HEADER_SIZE: u32 = 0x10;
const SYSTEM_MEMORY_LIST_INFO_SIZE: u32 = 0xB0;
const SYSTEM_ERROR_PORT_TIMEOUTS_SIZE: u32 = 0x08;
const SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE: u32 = 0x04;
const SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE: u32 = 0x20;

/// Win32 `KWAIT_REASON` values used to populate `SYSTEM_THREAD_INFORMATION.WaitReason`.
const KWAIT_REASON_DELAY_EXECUTION: u32 = 4;
const KWAIT_REASON_SUSPENDED: u32 = 5;
const KWAIT_REASON_WR_EXECUTIVE: u32 = 7;

/// Win32 `KTHREAD_STATE` values used to populate `SYSTEM_THREAD_INFORMATION.ThreadState`.
const KTHREAD_STATE_READY: u32 = 1;
const KTHREAD_STATE_RUNNING: u32 = 2;
const KTHREAD_STATE_WAITING: u32 = 5;

/// `SYSTEM_CODEINTEGRITY_INFORMATION.CodeIntegrityOptions` reported to callers.
/// Matches a normal retail configuration (driver signature enforcement on).
const CODE_INTEGRITY_OPTION_ENABLED: u32 = 0x1;

/// Default thread scheduling values used for `SYSTEM_PROCESS_INFORMATION`.
const THREAD_BASE_PRIORITY: u32 = 8;

/// Write the required 4-byte length to a non-null `ReturnLength` pointer.
/// Returns `true` when the write succeeded. Returns `true` for a null pointer
/// (nothing to do). Returns `false` only when the pointer is non-null and the
/// write failed — the dispatcher must propagate that as an error status.
fn write_return_length(emu: &mut Emu, ret_len_ptr: u64, n: u32) -> bool {
    if ret_len_ptr == 0 {
        return true;
    }
    // Validate the full 4-byte destination range, then write.
    if !emu.maps.validate_write_range(ret_len_ptr, 4) {
        return false;
    }
    emu.maps.write_bytes(ret_len_ptr, &n.to_le_bytes())
}

/// Fill `[addr, addr + n)` with zero bytes. Returns `true` on success and
/// `false` if the range cannot be fully written. Callers must propagate the
/// failure.
fn bulk_write_zero(emu: &mut Emu, addr: u64, n: u32) -> bool {
    if n == 0 {
        return true;
    }
    if !emu.maps.validate_write_range(addr, u64::from(n)) {
        return false;
    }
    // `write_bytes` already handles single-map fast path and boundary
    // crossings, and reports whether the bulk write succeeded.
    let amount = match usize::try_from(n) {
        Ok(amount) => amount,
        Err(_) => return false,
    };
    let zeros = vec![0u8; amount];
    emu.maps.write_bytes(addr, &zeros)
}

/// Reject short buffers and report the required size in `ReturnLength`.
/// Returns `true` when the caller's buffer is too small and the dispatcher
/// must return immediately. `ret_len_ptr` may be null; in that case the
/// required size is simply not reported.
fn short_buffer(emu: &mut Emu, ret_len_ptr: u64, required: u32, len: u32) -> bool {
    if len < required {
        emu.regs_mut().rax = if write_return_length(emu, ret_len_ptr, required) {
            STATUS_INFO_LENGTH_MISMATCH
        } else {
            STATUS_ACCESS_VIOLATION
        };
        return true;
    }
    false
}

/// Validate the native output range for a modeled class after handling a short
/// buffer. Unsupported and unknown classes intentionally bypass this helper.
fn validate_modeled_output_buffer(
    emu: &mut Emu,
    info: u64,
    len: u32,
    ret_len_ptr: u64,
    required: u32,
) -> bool {
    if short_buffer(emu, ret_len_ptr, required, len) {
        return false;
    }
    validate_output_buffer(emu, info, required)
}

/// Validate the caller-provided output pointer and length. On failure writes
/// the appropriate NTSTATUS into `rax` and returns `false`; the dispatcher
/// must then early-return.
fn validate_output_buffer(emu: &mut Emu, info: u64, len: u32) -> bool {
    if info == 0 && len > 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return false;
    }
    if len > 0 && !emu.maps.validate_write_range(info, u64::from(len)) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return false;
    }
    true
}

/// Fill a 0x40-byte x64 `SYSTEM_BASIC_INFORMATION`. Field offsets follow the
/// native 64-bit layout: the `ULONG_PTR` members are 8-byte aligned, so
/// `MinimumUserModeAddress` is at +0x20 and `MaximumUserModeAddress` at +0x28
/// (not +0x1c/+0x24 as in the 32-bit struct). ntdll reads
/// `MaximumUserModeAddress` to size loader bitmaps, so it must be the real
/// top-of-user-VA value.
fn fill_system_basic_information(emu: &mut Emu, info: u64) -> bool {
    if !bulk_write_zero(emu, info, SYSTEM_BASIC_INFO_SIZE) {
        return false;
    }
    emu.maps.write_dword(info + 0x08, 0x1000) // PageSize
        && emu.maps.write_dword(info + 0x0C, 0x0010_0000) // NumberOfPhysicalPages (~4GB)
        && emu.maps.write_dword(info + 0x18, 0x0001_0000) // AllocationGranularity (64KB)
        && emu
            .maps
            .write_qword(info + 0x20, 0x0000_0000_0001_0000) // MinimumUserModeAddress
        && emu
            .maps
            .write_qword(info + 0x28, 0x0000_7fff_fffe_ffff) // MaximumUserModeAddress
        && emu.maps.write_qword(info + 0x30, 1) // ActiveProcessorsAffinityMask
        && emu.maps.write_byte(info + 0x38, 1) // NumberOfProcessors
}

/// Fill a 0x0C-byte x64 `SYSTEM_PROCESSOR_INFORMATION`:
///   +0x00  ProcessorArchitecture  USHORT
///   +0x02  ProcessorLevel         USHORT
///   +0x04  ProcessorRevision      USHORT
///   +0x06  MaximumProcessors      USHORT
///   +0x08  ProcessorFeatureBits   ULONG
/// Only the declared 12 bytes are written; nothing beyond the native
/// structure is touched.
fn fill_system_processor_information(emu: &mut Emu, info: u64) -> bool {
    if !bulk_write_zero(emu, info, SYSTEM_PROCESSOR_INFO_SIZE) {
        return false;
    }
    // ProcessorArchitecture = PROCESSOR_ARCHITECTURE_AMD64; all other
    // fields remain zero except MaximumProcessors.
    emu.maps.write_word(info, 0x0009) && emu.maps.write_word(info + 0x06, 1)
}

/// Compute the per-thread state for `SystemProcessInformation`. Precedence:
/// suspended > blocked_on_cs > sleeping > runnable. The current runnable
/// thread is reported as `Running`; all other runnable threads are `Ready`.
fn thread_state_reason(emu: &Emu, thread_idx: usize) -> (u32, u32) {
    let thread = &emu.threads[thread_idx];
    if thread.suspended {
        return (KTHREAD_STATE_WAITING, KWAIT_REASON_SUSPENDED);
    }
    if thread.blocked_on_cs.is_some() {
        return (KTHREAD_STATE_WAITING, KWAIT_REASON_WR_EXECUTIVE);
    }
    if thread.wake_tick > emu.tick {
        return (KTHREAD_STATE_WAITING, KWAIT_REASON_DELAY_EXECUTION);
    }
    if thread_idx == emu.current_thread_id {
        (KTHREAD_STATE_RUNNING, 0)
    } else {
        (KTHREAD_STATE_READY, 0)
    }
}

/// Write one x64 `SYSTEM_THREAD_INFORMATION` (0x50 bytes) at `addr`.
fn write_system_thread_information(
    emu: &mut Emu,
    addr: u64,
    tid: u64,
    state: u32,
    wait_reason: u32,
) -> bool {
    if !bulk_write_zero(emu, addr, SYSTEM_THREAD_INFO_SIZE) {
        return false;
    }
    // ClientId at +0x28, Priority/BasePriority at +0x38/+0x3C,
    // ContextSwitches at +0x40, ThreadState at +0x44, WaitReason at +0x48.
    emu.maps.write_qword(addr + 0x28, SYNTHETIC_PROCESS_ID)
        && emu.maps.write_qword(addr + 0x30, tid)
        && emu.maps.write_dword(addr + 0x38, THREAD_BASE_PRIORITY)
        && emu.maps.write_dword(addr + 0x3C, THREAD_BASE_PRIORITY)
        && emu.maps.write_dword(addr + 0x40, 0)
        && emu.maps.write_dword(addr + 0x44, state)
        && emu.maps.write_dword(addr + 0x48, wait_reason)
}

/// Write a single x64 `RTL_PROCESS_MODULE_INFORMATION` (0x128 bytes) at
/// `addr` for the synthetic `ntoskrnl.exe`. Offsets follow the PHNT x64
/// definition:
///   +0x00  Section         HANDLE
///   +0x08  MappedBase      PVOID
///   +0x10  ImageBase       PVOID
///   +0x18  ImageSize       ULONG
///   +0x1C  Flags           ULONG
///   +0x20  LoadCount       USHORT
///   +0x22  OffsetToFileName USHORT
///   +0x24  FullPathName[256]
fn write_rtl_process_module_information(emu: &mut Emu, addr: u64) -> bool {
    const SIZE: u32 = 0x128;
    if !bulk_write_zero(emu, addr, SIZE) {
        return false;
    }
    let Ok(filename_offset) = u16::try_from(FAKE_KERNEL_DIR.len()) else {
        return false;
    };

    // Section and Flags remain zero. FullPathName is NUL-terminated by the
    // structure-wide zero fill.
    emu.maps.write_qword(addr + 0x08, FAKE_KERNEL_BASE)
        && emu.maps.write_qword(addr + 0x10, FAKE_KERNEL_BASE)
        && emu.maps.write_dword(addr + 0x18, FAKE_KERNEL_SIZE)
        && emu.maps.write_word(addr + 0x20, 1)
        && emu.maps.write_word(addr + 0x22, filename_offset)
        && emu.maps.write_bytes(addr + 0x24, FAKE_KERNEL_FULL_PATH)
}

/// Write a 0x20-byte x64 `SYSTEM_CODEINTEGRITYPOLICY_INFORMATION`:
///   +0x00  Options        ULONG
///   +0x04  HVCIOptions    ULONG
///   +0x08  Version        ULONGLONG
///   +0x10  PolicyGuid     GUID (16 bytes)
/// Only the declared 32 bytes are written.
fn fill_system_code_integrity_policy_information(emu: &mut Emu, info: u64) -> bool {
    if !bulk_write_zero(emu, info, SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE) {
        return false;
    }
    // The structure-wide zero fill models Options, HVCIOptions, Version, and
    // PolicyGuid as zero.
    true
}

/// `NtQuerySystemInformation` — x64: RCX `Class`, RDX `Buffer`, R8 `Length`, R9 `ReturnLength`.
///
/// Dispatcher policy:
/// - `correctly modeled` branches validate the native required size, write only
///   the native response, return the native response size through `ReturnLength`,
///   and propagate any write failure as an error status.
/// - `recognized but unsupported` branches return `STATUS_NOT_SUPPORTED`.
/// - Unknown classes return `STATUS_INVALID_INFO_CLASS`.
/// - A failed non-null `ReturnLength` write takes precedence over either class
///   status and returns `STATUS_ACCESS_VIOLATION`.
pub fn nt_query_system_information(emu: &mut Emu) {
    let class = emu.regs().rcx;
    let info = emu.regs().rdx;
    let len = match u32::try_from(emu.regs().r8) {
        Ok(len) => len,
        Err(_) => {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
    };
    let ret_len_ptr = emu.regs().r9;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQuerySystemInformation class: 0x{:x}, buf: 0x{:x}, len: 0x{:x}, ret_len: 0x{:x}",
        WIN64_NTQUERYSYSTEMINFORMATION,
        class,
        info,
        len,
        ret_len_ptr
    );

    match class {
        SYSTEM_BASIC_INFORMATION | SYSTEM_EMULATION_BASIC_INFORMATION => {
            if !validate_modeled_output_buffer(emu, info, len, ret_len_ptr, SYSTEM_BASIC_INFO_SIZE)
            {
                return;
            }
            if !fill_system_basic_information(emu, info)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_BASIC_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESSOR_INFORMATION => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_PROCESSOR_INFO_SIZE,
            ) {
                return;
            }
            if !fill_system_processor_information(emu, info)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_PROCESSOR_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        // `SystemPerformanceInformation` is OS-version-dependent and not
        // modeled by this emulator. Unsupported classes do not write the output
        // buffer and report zero through a valid `ReturnLength` pointer.
        SYSTEM_PERFORMANCE_INFORMATION => {
            if !write_return_length(emu, ret_len_ptr, 0) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
        }

        SYSTEM_TIME_OF_DAY_INFORMATION => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_TIME_OF_DAY_INFO_SIZE,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_TIME_OF_DAY_INFO_SIZE) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            // CurrentTime at +0x08, TimeZoneId at +0x18.
            if !emu.maps.write_qword(info + 0x08, 1)
                || !emu.maps.write_dword(info + 0x18, 0x2)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_TIME_OF_DAY_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESS_INFORMATION => {
            let thread_count = match u32::try_from(emu.threads.len()) {
                Ok(count) => count,
                Err(_) => {
                    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                    return;
                }
            };
            let thread_bytes = match thread_count.checked_mul(SYSTEM_THREAD_INFO_SIZE) {
                Some(bytes) => bytes,
                None => {
                    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                    return;
                }
            };
            let total = match SYSTEM_PROCESS_INFO_PREFIX_SIZE.checked_add(thread_bytes) {
                Some(total) => total,
                None => {
                    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                    return;
                }
            };
            if !validate_modeled_output_buffer(emu, info, len, ret_len_ptr, total) {
                return;
            }
            if !bulk_write_zero(emu, info, total) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            // Native x64 PHNT layout:
            // BasePriority +0x48, UniqueProcessId +0x50,
            // InheritedFromUniqueProcessId +0x58, HandleCount +0x60,
            // SessionId +0x64, and the first thread at +0x100.
            if !emu.maps.write_dword(info, 0)
                || !emu.maps.write_dword(info + 0x04, thread_count)
                || !emu.maps.write_dword(info + 0x048, THREAD_BASE_PRIORITY)
                || !emu.maps.write_qword(info + 0x050, SYNTHETIC_PROCESS_ID)
                || !emu
                    .maps
                    .write_qword(info + 0x058, SYNTHETIC_PARENT_PROCESS_ID)
                || !emu.maps.write_dword(info + 0x060, SYNTHETIC_HANDLE_COUNT)
                || !emu.maps.write_dword(info + 0x064, SYNTHETIC_SESSION_ID)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }

            let entries: Vec<(u64, u32, u32)> = emu
                .threads
                .iter()
                .enumerate()
                .map(|(i, thread)| {
                    let (state, reason) = thread_state_reason(emu, i);
                    (thread.id, state, reason)
                })
                .collect();
            for (i, (tid, state, reason)) in entries.into_iter().enumerate() {
                let index = match u64::try_from(i) {
                    Ok(index) => index,
                    Err(_) => {
                        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                        return;
                    }
                };
                let thread_offset = match index
                    .checked_mul(u64::from(SYSTEM_THREAD_INFO_SIZE))
                    .and_then(|offset| {
                        u64::from(SYSTEM_PROCESS_INFO_PREFIX_SIZE).checked_add(offset)
                    }) {
                    Some(offset) => offset,
                    None => {
                        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                        return;
                    }
                };
                let thread_addr = match info.checked_add(thread_offset) {
                    Some(addr) => addr,
                    None => {
                        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                        return;
                    }
                };
                if !write_system_thread_information(emu, thread_addr, tid, state, reason) {
                    emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                    return;
                }
            }
            if !write_return_length(emu, ret_len_ptr, total) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            // IdleTime +0x00, KernelTime +0x08; user/dpc/interrupt remain 0.
            let tick = emu.pos;
            if !emu.maps.write_qword(info, tick)
                || !emu.maps.write_qword(info + 0x08, tick)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_DEVICE_INFORMATION => {
            if !validate_modeled_output_buffer(emu, info, len, ret_len_ptr, SYSTEM_DEVICE_INFO_SIZE)
            {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_DEVICE_INFO_SIZE) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            // NumberOfDisks at +0x00.
            if !emu.maps.write_dword(info, 1)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_DEVICE_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_EXCEPTION_INFORMATION => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_EXCEPTION_INFO_SIZE,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_EXCEPTION_INFO_SIZE)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_EXCEPTION_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_FILE_CACHE_INFORMATION | SYSTEM_FILE_CACHE_INFORMATION_EX => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_FILE_CACHE_INFO_SIZE,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_FILE_CACHE_INFO_SIZE)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_FILE_CACHE_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_MEMORY_LIST_INFORMATION => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_MEMORY_LIST_INFO_SIZE,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_MEMORY_LIST_INFO_SIZE)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_MEMORY_LIST_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_MODULE_INFORMATION => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_MODULE_INFO_REQUIRED,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_MODULE_INFO_REQUIRED) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            // NumberOfModules at +0x00; the module array starts at +0x08.
            if !emu.maps.write_dword(info, 1)
                || !write_rtl_process_module_information(emu, info + 0x08)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_MODULE_INFO_REQUIRED)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_MODULE_INFORMATION_EX => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_MODULE_INFO_EX_REQUIRED,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_MODULE_INFO_EX_REQUIRED) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            // Native x64 `RTL_PROCESS_MODULE_INFORMATION_EX` (0x140 bytes):
            //   +0x000  NextOffset     ULONG
            //   +0x004  padding
            //   +0x008  BaseInfo       RTL_PROCESS_MODULE_INFORMATION (0x128)
            //   +0x130  ImageChecksum  ULONG
            //   +0x134  TimeDateStamp  ULONG
            //   +0x138  DefaultBase    PVOID
            // We write only the 0x140-byte native structure.
            if !write_rtl_process_module_information(emu, info + 0x08)
                || !emu.maps.write_dword(info + 0x130, 0) // ImageChecksum
                || !emu.maps.write_dword(info + 0x134, 0) // TimeDateStamp
                || !emu
                    .maps
                    .write_qword(info + 0x138, FAKE_KERNEL_BASE) // DefaultBase
                || !write_return_length(emu, ret_len_ptr, SYSTEM_MODULE_INFO_EX_REQUIRED)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_EXTENDED_HANDLE_INFORMATION => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_EXTENDED_HANDLE_HEADER_SIZE,
            ) {
                return;
            }
            // 16-byte header: NumberOfHandles (8) + Reserved (8) = 0 handles.
            if !bulk_write_zero(emu, info, SYSTEM_EXTENDED_HANDLE_HEADER_SIZE)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_EXTENDED_HANDLE_HEADER_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_ERROR_PORT_TIMEOUTS => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_ERROR_PORT_TIMEOUTS_SIZE,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_ERROR_PORT_TIMEOUTS_SIZE)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_ERROR_PORT_TIMEOUTS_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            if !emu.maps.write_dword(info, 64)
                || !write_return_length(
                    emu,
                    ret_len_ptr,
                    SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE,
                )
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_KERNEL_DEBUGGER_INFORMATION => {
            // SYSTEM_KERNEL_DEBUGGER_INFORMATION: { DebuggerEnabled: BOOLEAN, DebuggerNotPresent: BOOLEAN }
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_KERNEL_DEBUGGER_INFO_SIZE,
            ) {
                return;
            }
            if !emu.maps.write_byte(info, 0) // DebuggerEnabled = FALSE
                || !emu.maps.write_byte(info + 1, 1) // DebuggerNotPresent = TRUE
                || !write_return_length(emu, ret_len_ptr, SYSTEM_KERNEL_DEBUGGER_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_CODE_INTEGRITY_INFORMATION => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_CODE_INTEGRITY_INFO_SIZE,
            ) {
                return;
            }
            // Length at +0x00, CodeIntegrityOptions at +0x04.
            if !emu.maps.write_dword(info, SYSTEM_CODE_INTEGRITY_INFO_SIZE)
                || !emu
                    .maps
                    .write_dword(info + 0x04, CODE_INTEGRITY_OPTION_ENABLED)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_CODE_INTEGRITY_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_CODE_INTEGRITY_POLICY_INFORMATION => {
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE,
            ) {
                return;
            }
            if !fill_system_code_integrity_policy_information(emu, info)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX => {
            // 3-byte response: { DebuggerAllowed, DebuggerEnabled, DebuggerPresent }.
            if !validate_modeled_output_buffer(
                emu,
                info,
                len,
                ret_len_ptr,
                SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE,
            ) {
                return;
            }
            if !bulk_write_zero(emu, info, SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE)
                || !write_return_length(emu, ret_len_ptr, SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE)
            {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_FULL_MEMORY_INFORMATION
        | SYSTEM_SUMMARY_MEMORY_INFORMATION
        | SYSTEM_MEMORY_USAGE_INFORMATION
        | SYSTEM_TIME_ZONE_INFORMATION
        | SYSTEM_CURRENT_TIME_ZONE_INFORMATION
        | SYSTEM_DYNAMIC_TIME_ZONE_INFORMATION
        | SYSTEM_RANGE_START_INFORMATION
        | SYSTEM_NUMA_PROCESSOR_MAP
        | SYSTEM_NUMA_AVAILABLE_MEMORY
        | SYSTEM_LOGICAL_PROCESSOR_INFORMATION
        | SYSTEM_BOOT_ENVIRONMENT_INFORMATION
        | SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION
        | SYSTEM_FLUSH_INFORMATION
        | SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES
        | SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES2 => {
            log_orange!(
                emu,
                "NtQuerySystemInformation: class 0x{:x} valid but unmodeled, returning STATUS_NOT_SUPPORTED",
                class
            );
            if !write_return_length(emu, ret_len_ptr, 0) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
        }

        _ => {
            log_orange!(
                emu,
                "NtQuerySystemInformation: unhandled class 0x{:x}, returning STATUS_INVALID_INFO_CLASS",
                class
            );
            if !write_return_length(emu, ret_len_ptr, 0) {
                emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
                return;
            }
            emu.regs_mut().rax = STATUS_INVALID_INFO_CLASS;
        }
    }
}

/// `NtManageHotPatch` — RCX `HotPatchInfo` (pointer to struct).
/// Stub: hot-patching is not supported in the emulator.
pub fn nt_manage_hot_patch(emu: &mut Emu) {
    let info = emu.regs().rcx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtManageHotPatch info: 0x{:x}",
        WIN64_NTMANAGEHOTPATCH,
        info
    );

    emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
}

/// `NtQueryDebugFilterState(ComponentId, Level)` — returns FALSE (0) to indicate
/// that debug output is suppressed for this component/level (no debugger attached).
pub fn nt_query_debug_filter_state(emu: &mut Emu) {
    let component = emu.regs().rcx;
    let level = emu.regs().rdx;
    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryDebugFilterState component: 0x{:x}, level: 0x{:x}",
        WIN64_NTQUERYDEBUGFILTERSTATE,
        component,
        level
    );
    emu.regs_mut().rax = 0; // FALSE — debug output suppressed
}

/// `NtTraceEvent` — stub; ETW tracing is not emulated.
pub fn nt_trace_event(emu: &mut Emu) {
    log_orange!(
        emu,
        "syscall 0x{:x}: NtTraceEvent (stub)",
        WIN64_NTTRACEEVENT
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtQueryInformationTransactionManager` — syscall 0x15a.
/// x64: RCX=`TransactionManagerHandle`, RDX=`InformationClass`,
///      R8=`Buffer`, R9=`BufferLength`, `[rsp+0x28]`=`ReturnLength` (PULONG).
///
/// Kernel Transaction Manager (KTM) query. Called by ntdll during loader init
/// to probe transaction support. We handle the two most common classes and
/// return STATUS_INVALID_INFO_CLASS for everything else.
///
/// Information classes:
///   0 = TransactionManagerBasicInformation  — GUID(16) + VirtualClock(8) = 24 bytes
///   1 = TransactionManagerLogInformation    — GUID(16) = 16 bytes
pub fn nt_query_information_transaction_manager(emu: &mut Emu) {
    let _handle = emu.regs().rcx;
    let info_class = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let buffer_len = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let return_length_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryInformationTransactionManager class: {} buf: 0x{:x} len: {}",
        WIN64_NTQUERYINFORMATIONTRANSACTIONMANAGER,
        info_class,
        buffer,
        buffer_len
    );

    let needed: u32 = match info_class {
        0 => 24, // GUID(16) + LARGE_INTEGER(8)
        1 => 16, // GUID(16)
        _ => {
            emu.regs_mut().rax = STATUS_INVALID_INFO_CLASS;
            return;
        }
    };

    if !write_return_length(emu, return_length_ptr, needed) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    if buffer == 0 || buffer_len < u64::from(needed) {
        emu.regs_mut().rax = STATUS_BUFFER_TOO_SMALL;
        return;
    }

    // Zero-fill the output — no real KTM state to return.
    if !bulk_write_zero(emu, buffer, needed) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtQueryIoCompletion` — syscall 0x15e.
/// RCX=IoCompletionHandle, RDX=IoCompletionInformationClass,
/// R8=IoCompletionInformation (out), R9=IoCompletionInformationLength,
/// [rsp+0x28]=ReturnLength (out PULONG).
///
/// IoCompletionBasicInformation (class 0) returns a single ULONG Depth.
/// Since we do not track real I/O completion ports, return STATUS_INVALID_HANDLE
/// so callers fall back gracefully rather than receiving STATUS_NOT_IMPLEMENTED.
pub fn nt_query_io_completion(emu: &mut Emu) {
    let handle = emu.regs().rcx;
    let info_class = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let buffer_len = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let return_length_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryIoCompletion handle: 0x{:x}, class: {}, buf: 0x{:x}, len: {}",
        WIN64_NTQUERYIOCOMPLETION,
        handle,
        info_class,
        buffer,
        buffer_len,
    );

    // IoCompletionBasicInformation (0): single ULONG Depth.
    // Accept any class and return zeroed output — callers interpret 0 as "no queued items".
    const NEEDED: u64 = 4; // sizeof(ULONG)
    let _ = write_return_length(emu, return_length_ptr, NEEDED as u32);

    if buffer == 0 || buffer_len < NEEDED {
        emu.regs_mut().rax = STATUS_BUFFER_TOO_SMALL;
        return;
    }

    if !emu.maps.is_mapped(buffer) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    // Depth = 0: completion port exists but has no queued items.
    let _ = emu.maps.write_dword(buffer, 0);
    emu.regs_mut().rax = STATUS_SUCCESS;
}
