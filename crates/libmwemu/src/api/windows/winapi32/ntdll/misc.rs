use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::common::heap::heap_allocation_size;
use crate::api::windows::libraries::advapi32;
use crate::emu;
use crate::exception::seh32::unwind;
use crate::syscall::windows::syscall64::query_system_information;
use crate::winapi::common::ntdll;
use crate::winapi::common::threads;
use crate::winapi::libraries::kernel32::file_time::nt_performance;
use crate::winapi::policy;
use crate::winapi::policy::api_try;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    if ntdll::etw(api, emu) {
        return true;
    }
    dispatch_api!(api, {
        "RtlUnwind" => RtlUnwind(emu),
        "RtlCreateAcl" => advapi32::RtlCreateAcl(emu),
        "RtlQueryInformationAcl" => advapi32::RtlQueryInformationAcl(emu),
        "RtlGetAce" => advapi32::RtlGetAce(emu),
        "RtlMapGenericMask" => advapi32::MapGenericMask(emu),
        "RtlAreAllAccessesGranted" => advapi32::AreAllAccessesGranted(emu),
        "RtlAreAnyAccessesGranted" => advapi32::AreAnyAccessesGranted(emu),
        "NtGetContextThread" => threads::nt_get_context_thread(emu),
        "RtlVectoredExceptionHandler" => RtlVectoredExceptionHandler(emu),
        "RtlExitUserThread" => threads::exit_thread(emu),
        "NtGetTickCount" => ntdll::native_tick(emu),
        "NtQueryPerformanceCounter" => nt_performance(emu,WinApi(Stdcall)),
        "RtlGetVersion" => ntdll::version_info(emu, api),
        "RtlExitUserProcess" => threads::exit_process(emu, api),
        "RtlSetUnhandledExceptionFilter" => ntdll::set_exception_filter(emu),
        "VerSetConditionMask" => VerSetConditionMask(emu),
        "NtQuerySystemInformation" => NtQuerySystemInformation(emu),
        "RtlDecompressBuffer" => ntdll::RtlDecompressBuffer(emu),
        "NtSetInformationThread" => policy::unsupported(emu, "ntdll", api, emu.pc()),
        "NtTerminateThread" => threads::nt_terminate_thread(emu),
        "RtlEncodePointer" => ntdll::pointer_round_trip(emu, api),
        "RtlDecodePointer" => ntdll::pointer_round_trip(emu, api),
        "RtlSizeHeap" => RtlSizeHeap(emu),
        "RtlCaptureContext" => threads::capture_context(emu),
        "RestoreLastError" | "RtlRestoreLastWin32Error" => ntdll::restore_last_error(emu),
        "NtQueueApcThread" => policy::unsupported(emu, "ntdll", api, emu.pc()),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn RtlVectoredExceptionHandler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg(emu, 0));
    let fptr = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "ntdll!RtlVectoredExceptionHandler  first={} callback=0x{:x}",
        first,
        fptr
    );

    emu.add_veh(first != 0, fptr);

    call.set_return(emu, 0x2c2878);
    call.cleanup_args(emu, 2);
}

pub fn VerSetConditionMask(emu: &mut emu::Emu) {
    ntdll::version_condition_mask(emu);
}

pub fn NtQuerySystemInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let class = api_try!(emu, call.arg_u32(emu, 0));
    let info = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg_u32(emu, 2));
    let ret_len = api_try!(emu, call.arg(emu, 3));
    query_system_information(emu, u64::from(class), info, len, ret_len);
    call.cleanup_args(emu, 4);
}

fn RtlSizeHeap(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let mem = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let size = heap_allocation_size(emu, mem);
    log_red!(emu, "ntdll!RtlSizeHeap mem=0x{:x} ={:?}", mem, size);
    call.set_return(emu, size.map(|s| s as u64).unwrap_or(0xffff_ffff));
    call.cleanup_args(emu, 3);
}

pub fn RtlUnwind(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let target = api_try!(emu, call.arg_u32(emu, 0));
    let target_ip = api_try!(emu, call.arg_u32(emu, 1));
    let record = api_try!(emu, call.arg_u32(emu, 2));
    let value = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "RtlUnwind target_frame={target:#x} target_ip={target_ip:#x} exception_record={record:#x} return_value={value:#x}"
    );
    // Windows i386 resumes at the caller's return address, including when TargetIp is nonzero.
    call.cleanup_args(emu, 4);
    call.set_return(emu, value as u64);
    unwind::start(emu, target, record);
}
