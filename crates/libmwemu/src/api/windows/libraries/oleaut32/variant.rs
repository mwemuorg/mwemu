use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::{bstr, buffers, memory};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::{
    E_NOTIMPL, E_OUTOFMEMORY, E_POINTER, HRESULT_E_INVALID_ARG, S_OK,
};

const VT_BSTR: u16 = 8;
const VT_BYREF: u16 = 0x4000;
const VALUE: u64 = 8;

fn size(emu: &Emu) -> usize {
    if emu.cfg.arch.is_64bits() { 24 } else { 16 }
}

fn plain(ty: u16) -> bool {
    matches!(ty,0..=7|10|11|14|16..=23) || ty & VT_BYREF != 0
}

fn clear(emu: &mut Emu, value: u64, ty: u16) -> Result<(), u64> {
    if !emu.maps.validate_write_range(value, 2) {
        return Err(E_POINTER);
    }
    if ty == VT_BSTR {
        let string = memory::read_pointer(emu, value.checked_add(VALUE).ok_or(E_POINTER)?)
            .ok_or(E_POINTER)?;
        if string != 0 && !buffers::free(emu, string, BufferKind::Bstr) {
            return Err(HRESULT_E_INVALID_ARG);
        }
    }
    emu.maps.write_word(value, 0);
    Ok(())
}
pub fn VariantInit(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let pointer = api_try!(emu, call.arg(emu, 0));
    let length = size(emu);
    log_red!(emu, "oleaut32!VariantInit variant={pointer:#x}");
    emu.maps.fill_range(pointer, 0, length);
    call.cleanup_args(emu, 1);
}
fn clear_api(emu: &mut Emu, dll: &str, name: &str) {
    let call = WinApi(Stdcall);
    let value = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{dll}!{name} variant={value:#x}");
    let status = match emu.maps.read_word(value) {
        Some(ty) if ty == VT_BSTR || plain(ty) => clear(emu, value, ty).err().unwrap_or(S_OK),
        Some(_) => {
            policy::unsupported(emu, dll, name, emu.pc());
            return;
        }
        None => HRESULT_E_INVALID_ARG,
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 1);
}
pub fn VariantClear(emu: &mut Emu) {
    clear_api(emu, "oleaut32", "VariantClear");
}
pub fn PropVariantClear(emu: &mut Emu) {
    clear_api(emu, "ole32", "PropVariantClear");
}

fn copy(emu: &mut Emu, dll: &str, name: &str, words: usize, indirect: bool, change: bool) {
    let call = WinApi(Stdcall);
    let [destination, source] = api_try!(emu, call.args(emu));
    let target = if change {
        Some(api_try!(emu, call.arg_u16(emu, words - 1)))
    } else {
        None
    };
    log_red!(
        emu,
        "{dll}!{name} destination={destination:#x} source={source:#x} target={target:?}"
    );
    let status = (|| {
        let width = size(emu);
        let src_ty = emu.maps.read_word(source).ok_or(E_POINTER)?;
        let old_ty = emu.maps.read_word(destination).ok_or(E_POINTER)?;
        if target.is_some_and(|ty| ty != src_ty)
            || (indirect && src_ty & VT_BYREF != 0)
            || !(plain(src_ty) || src_ty == VT_BSTR)
            || !(plain(old_ty) || old_ty == VT_BSTR)
        {
            return Err(E_NOTIMPL);
        }
        if destination == source {
            return Ok(());
        }
        if !emu.maps.validate_write_range(destination, width as u64) {
            return Err(E_POINTER);
        }
        let bytes = memory::read_guest_bytes(emu, source, width).ok_or(E_POINTER)?;
        let mut owned = 0;
        if src_ty == VT_BSTR {
            let address = memory::read_pointer(emu, source.checked_add(VALUE).ok_or(E_POINTER)?)
                .ok_or(E_POINTER)?;
            if address != 0 {
                let length = address
                    .checked_sub(4)
                    .and_then(|p| emu.maps.read_dword(p))
                    .ok_or(E_POINTER)?;
                owned = bstr::allocate(emu, address, u64::from(length)).ok_or(E_OUTOFMEMORY)?;
            }
        }
        if let Err(err) = clear(emu, destination, old_ty) {
            if owned != 0 {
                buffers::free(emu, owned, BufferKind::Bstr);
            }
            return Err(err);
        }
        emu.maps.write_bytes(destination, &bytes);
        if src_ty == VT_BSTR {
            memory::write_pointer(emu, destination + VALUE, owned);
        }
        Ok(())
    })();
    let status = match status {
        Ok(()) => S_OK,
        Err(E_NOTIMPL) => {
            policy::unsupported(emu, dll, name, emu.pc());
            return;
        }
        Err(error) => error,
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, words);
}
pub fn VariantCopy(emu: &mut Emu) {
    copy(emu, "oleaut32", "VariantCopy", 2, false, false);
}
pub fn PropVariantCopy(emu: &mut Emu) {
    copy(emu, "ole32", "PropVariantCopy", 2, false, false);
}
pub fn VariantCopyInd(emu: &mut Emu) {
    copy(emu, "oleaut32", "VariantCopyInd", 2, true, false);
}
pub fn VariantChangeType(emu: &mut Emu) {
    copy(emu, "oleaut32", "VariantChangeType", 4, true, true);
}
pub fn VariantChangeTypeEx(emu: &mut Emu) {
    copy(emu, "oleaut32", "VariantChangeTypeEx", 5, true, true);
}
