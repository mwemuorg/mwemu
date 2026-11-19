use super::EINVAL;
use super::read_into;
use super::set_errno;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn fread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let file = api_try!(emu, call.arg(emu, 3));
    let total = size.saturating_mul(count) as usize;
    let items = if total == 0 {
        Some(0)
    } else {
        read_into(emu, file, buf, total).map(|n| n as u64 / size)
    };
    log_red!(
        emu,
        "wincrt!fread buf:0x{:x} size:{} count:{} fp:0x{:x} = {:?}",
        buf,
        size,
        count,
        file,
        items
    );
    if items.is_none() {
        set_errno(emu, EINVAL);
    }
    call.set_return(emu, items.unwrap_or(0));
}
