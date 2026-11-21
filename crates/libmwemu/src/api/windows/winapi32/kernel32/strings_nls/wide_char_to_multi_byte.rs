use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WideCharToMultiByte(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let codepage = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let wstr_ptr = api_try!(emu, call.arg(emu, 2));
    let wstr_sz = api_try!(emu, call.arg_u32(emu, 3));
    let mbytestr_ptr = api_try!(emu, call.arg(emu, 4));
    let mbytestr_sz = api_try!(emu, call.arg_u32(emu, 5));
    let in_default_char = api_try!(emu, call.arg(emu, 6));
    let out_default_char = api_try!(emu, call.arg(emu, 7));

    //log::trace!("default_char_ptr 0x{:x}", in_default_char);
    //let default_char = emu.maps.read_byte(in_default_char)
    //    .unwrap_or(0);

    //emu.maps.write_byte(out_default_char, 0);

    let wchar_count = if wstr_sz as i32 == -1 {
        let s = emu.maps.read_wide_string(wstr_ptr);
        s.len() + 1
    } else {
        wstr_sz as usize
    };

    if mbytestr_ptr == 0 || mbytestr_sz == 0 {
        log_red!(
            emu,
            "kernel32!WideCharToMultiByte sz:{}->{} ={}",
            wstr_sz,
            mbytestr_sz,
            wchar_count
        );
        call.cleanup_args(emu, 8);
        call.set_return(emu, wchar_count as u64);
        return;
    }

    let write_count = wchar_count.min(mbytestr_sz as usize);
    for i in 0..write_count {
        let wc = emu.maps.read_word(wstr_ptr + (i as u64) * 2).unwrap_or(0);
        let byte = if wc <= 0xFF { wc as u8 } else { b'?' };
        emu.maps.write_byte(mbytestr_ptr + i as u64, byte);
    }

    log_red!(
        emu,
        "kernel32!WideCharToMultiByte sz:{}->{} ={}",
        wstr_sz,
        mbytestr_sz,
        write_count
    );

    call.cleanup_args(emu, 8);
    call.set_return(emu, write_count as u64);
}
