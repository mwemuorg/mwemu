use crate::emu;
use crate::winapi::helper;

pub fn CreateJobObjectA(emu: &mut emu::Emu) {
    create_job_object(emu, "CreateJobObjectA");
}

fn create_job_object(emu: &mut emu::Emu, api: &str) {
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn CreateJobObjectW(emu: &mut emu::Emu) {
    create_job_object(emu, "CreateJobObjectW");
}
