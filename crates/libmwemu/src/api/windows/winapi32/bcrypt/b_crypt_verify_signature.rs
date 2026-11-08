use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants;

pub(super) fn BCryptVerifySignature(emu: &mut emu::Emu) {
    let h_key = arg(emu, 0) as u64;
    let _padding_info = arg(emu, 1) as u64;
    let pb_hash = arg(emu, 2) as u64;
    let cb_hash = arg(emu, 3);
    let pb_signature = arg(emu, 4) as u64;
    let cb_signature = arg(emu, 5);
    let _flags = arg(emu, 6);

    log_red!(
        emu,
        "bcrypt!BCryptVerifySignature key:0x{:x} hash:0x{:x}({}) sig:0x{:x}({}) => valid",
        h_key,
        pb_hash,
        cb_hash,
        pb_signature,
        cb_signature
    );
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}
