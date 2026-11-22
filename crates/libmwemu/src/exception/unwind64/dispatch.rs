use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::context::context64::Context64;
use crate::emu::Emu;
use crate::exception::{handlers, types::ExceptionRecord};
use crate::maps::mem64::Permission;
use crate::winapi::policy::{ApiStopReason, api_try, invalid_unwind};

fn read_exception_record(emu: &Emu, record_addr: u64) -> Option<ExceptionRecord> {
    if record_addr == 0
        || !emu.maps.can_access_range(
            record_addr,
            ExceptionRecord::SIZE64 as usize,
            Permission::READ_WRITE,
        )
    {
        return None;
    }
    ExceptionRecord::load(&emu.maps, record_addr, true)
}

pub(crate) fn dispatch_exception_api(emu: &mut Emu, library: &str) {
    const API: &str = "RtlDispatchException";
    let call = WinApi(Stdcall);
    let [record_addr, context] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "{library}!{API} exception_record={record_addr:#x} context={context:#x}"
    );
    let Some(record) = read_exception_record(emu, record_addr) else {
        return invalid_unwind(emu, library, API, ApiStopReason::InvalidMemory);
    };
    if context == 0
        || !emu.maps.can_access_range(
            context,
            Context64::GUEST_SIZE as usize,
            Permission::READ_WRITE,
        )
    {
        return invalid_unwind(emu, library, API, ApiStopReason::InvalidMemory);
    }

    if !handlers::dispatch_x64_api(emu, &record, record_addr, Some(context)) {
        invalid_unwind(emu, library, API, ApiStopReason::InvalidMemory);
    }
}

pub(crate) fn raise_exception_api(emu: &mut Emu, library: &str) {
    const API: &str = "RtlRaiseException";
    let call = WinApi(Stdcall);
    let record_addr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{library}!{API} exception_record={record_addr:#x}");
    let Some(mut record) = read_exception_record(emu, record_addr) else {
        return invalid_unwind(emu, library, API, ApiStopReason::InvalidMemory);
    };
    record.address = emu.pc();
    if !emu.maps.write_qword(
        record_addr + ExceptionRecord::ADDRESS64_OFFSET,
        record.address,
    ) {
        return invalid_unwind(emu, library, API, ApiStopReason::InvalidMemory);
    }

    if !handlers::dispatch_x64_api(emu, &record, record_addr, None) {
        invalid_unwind(emu, library, API, ApiStopReason::InvalidMemory);
    }
}
