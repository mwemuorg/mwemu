use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;

fn probe(emu: &mut Emu, permission: Permission, code: bool) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let length = if code {
        1
    } else {
        api_try!(emu, call.arg(emu, 1))
    };
    let ok = usize::try_from(length)
        .is_ok_and(|size| emu.maps.can_access_range(address, size, permission));
    log_red!(
        emu,
        "pointer probe address=0x{:x} size={} permission={:?}",
        address,
        length,
        permission
    );
    call.set_return(emu, u64::from(!ok));
    call.cleanup_args(emu, if code { 1 } else { 2 });
}
pub fn IsBadCodePtr(emu: &mut Emu) {
    probe(emu, Permission::READ, true);
}
pub fn IsBadReadPtr(emu: &mut Emu) {
    probe(emu, Permission::READ, false);
}
pub fn IsBadWritePtr(emu: &mut Emu) {
    probe(emu, Permission::WRITE, false);
}
pub use IsBadReadPtr as IsBadHugeReadPtr;
pub use IsBadWritePtr as IsBadHugeWritePtr;
