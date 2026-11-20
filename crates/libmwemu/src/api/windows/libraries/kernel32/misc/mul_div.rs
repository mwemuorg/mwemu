use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;

pub fn MulDiv(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let product = i64::from(api_try!(emu, call.arg_i32(emu, 0)))
        * i64::from(api_try!(emu, call.arg_i32(emu, 1)));
    let divisor = i64::from(api_try!(emu, call.arg_i32(emu, 2)));
    let value = if divisor == 0 {
        -1
    } else {
        let magnitude = (product.abs() + divisor.abs() / 2) / divisor.abs();
        i32::try_from(magnitude)
            .map(|v| {
                if (product < 0) != (divisor < 0) {
                    -v
                } else {
                    v
                }
            })
            .unwrap_or(-1)
    };
    log_red!(
        emu,
        "MulDiv product={} divisor={} = {}",
        product,
        divisor,
        value
    );
    call.set_return(emu, u64::from(value as u32));
    call.cleanup_args(emu, 3);
}
