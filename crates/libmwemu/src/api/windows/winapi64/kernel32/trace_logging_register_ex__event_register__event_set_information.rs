
use crate::emu;
use crate::winapi::helper;
pub fn TraceLoggingRegisterEx_EventRegister_EventSetInformation(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!TraceLoggingRegisterEx_EventRegister_EventSetInformation param1:0x{:x} param2:0x{:x} param3:0x{:x}",
        param1,
        param2,
        param3
    );
    let handle = helper::handler_create("TraceLoggingRegisterExEventRegisterEventSetInformation");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
