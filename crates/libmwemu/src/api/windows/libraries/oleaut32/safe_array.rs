use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::{buffers, memory};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::{
    DISP_E_ARRAYISLOCKED, DISP_E_BADINDEX, HRESULT_E_INVALID_ARG, S_OK,
};

const FADF_HAVEVARTYPE: u16 = 0x80;
const ELEMENT_BYTES: u64 = 4;
const LOCK_COUNT: u64 = 8;
const TYPE_PREFIX: u64 = 4;
const BOUND_BYTES: u64 = 8;

fn element_size(vartype: u16) -> Option<u32> {
    const VT_I1: u16 = 16;
    const VT_UI1: u16 = 17;
    const VT_I2: u16 = 2;
    const VT_UI2: u16 = 18;
    const VT_BOOL: u16 = 11;
    const VT_I4: u16 = 3;
    const VT_R4: u16 = 4;
    const VT_ERROR: u16 = 10;
    const VT_UI4: u16 = 19;
    const VT_INT: u16 = 22;
    const VT_UINT: u16 = 23;
    const VT_R8: u16 = 5;
    const VT_CY: u16 = 6;
    const VT_DATE: u16 = 7;
    const VT_I8: u16 = 20;
    const VT_UI8: u16 = 21;
    const VT_DECIMAL: u16 = 14;
    match vartype {
        VT_I1 | VT_UI1 => Some(1),
        VT_I2 | VT_UI2 | VT_BOOL => Some(2),
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_ERROR | VT_R4 => Some(4),
        VT_I8 | VT_UI8 | VT_R8 | VT_DATE | VT_CY => Some(8),
        VT_DECIMAL => Some(16),
        _ => None,
    }
}

fn data_offset(emu: &Emu) -> u64 {
    if emu.cfg.arch.is_64bits() { 16 } else { 12 }
}

fn bounds_offset(emu: &Emu) -> u64 {
    data_offset(emu) + memory::pointer_size(emu)
}

fn allocate(emu: &mut Emu, vartype: u16, lower: i32, count: u32) -> Option<u64> {
    let width = element_size(vartype)?;
    let bounds = bounds_offset(emu);
    let header = bounds + BOUND_BYTES;
    let size = header + u64::from(count) * u64::from(width);
    let array = buffers::allocate(emu, size, 8, TYPE_PREFIX, BufferKind::SafeArray)?;
    let ok = emu.maps.fill_range(array, 0, size as usize)
        && emu
            .maps
            .write_dword(array - TYPE_PREFIX, u32::from(vartype))
        && emu.maps.write_word(array, 1)
        && emu.maps.write_word(array + 2, FADF_HAVEVARTYPE)
        && emu.maps.write_dword(array + ELEMENT_BYTES, width)
        && memory::write_pointer(emu, array + data_offset(emu), array + header)
        && emu.maps.write_dword(array + bounds, count)
        && emu.maps.write_dword(array + bounds + 4, lower as u32);
    if ok {
        Some(array)
    } else {
        buffers::free(emu, array, BufferKind::SafeArray);
        None
    }
}

fn create(emu: &mut Emu, api: &str, vector: bool, extended: bool) {
    let call = WinApi(Stdcall);
    let vartype = api_try!(emu, call.arg_u16(emu, 0));
    let bounds = if vector {
        let lower = api_try!(emu, call.arg_i32(emu, 1));
        let count = api_try!(emu, call.arg_u32(emu, 2));
        Some((lower, count))
    } else {
        let dimensions = api_try!(emu, call.arg_u32(emu, 1));
        let address = api_try!(emu, call.arg(emu, 2));
        (dimensions == 1)
            .then(|| memory::read_array::<8>(emu, address))
            .flatten()
            .map(|raw| {
                let [a, b, c, d, e, f, g, h] = raw;
                (
                    i32::from_le_bytes([e, f, g, h]),
                    u32::from_le_bytes([a, b, c, d]),
                )
            })
    };
    if extended {
        api_try!(emu, call.arg(emu, 3));
    }
    let array = bounds
        .and_then(|(lower, count)| allocate(emu, vartype, lower, count))
        .unwrap_or(0);
    log_red!(
        emu,
        "oleaut32!{api} vartype={vartype} bounds={bounds:?} array={array:#x}"
    );
    call.set_return(emu, array);
    call.cleanup_args(emu, 3 + usize::from(extended));
}

pub fn SafeArrayCreate(emu: &mut Emu) {
    create(emu, "SafeArrayCreate", false, false);
}
pub fn SafeArrayCreateEx(emu: &mut Emu) {
    create(emu, "SafeArrayCreateEx", false, true);
}
pub fn SafeArrayCreateVector(emu: &mut Emu) {
    create(emu, "SafeArrayCreateVector", true, false);
}
pub fn SafeArrayCreateVectorEx(emu: &mut Emu) {
    create(emu, "SafeArrayCreateVectorEx", true, true);
}

struct ScalarArray {
    data: u64,
    width: u32,
    lower: i32,
    count: u32,
    vartype: u16,
    locks: u32,
}

impl ScalarArray {
    fn read(emu: &Emu, pointer: u64) -> Option<Self> {
        let allocation = buffers::size(emu, pointer, BufferKind::SafeArray);
        if !(TYPE_PREFIX..=u64::MAX - (bounds_offset(emu) + BOUND_BYTES)).contains(&pointer) {
            return None;
        }
        let bounds = bounds_offset(emu);
        if emu.maps.read_word(pointer)? != 1 || emu.maps.read_word(pointer + 2)? != FADF_HAVEVARTYPE
        {
            return None;
        }
        let vartype = emu.maps.read_word(pointer - TYPE_PREFIX)?;
        let width = element_size(vartype)?;
        let count = emu.maps.read_dword(pointer + bounds)?;
        let data = memory::read_pointer(emu, pointer + data_offset(emu))?;
        let header = bounds + BOUND_BYTES;
        if width != emu.maps.read_dword(pointer + ELEMENT_BYTES)?
            || allocation.is_some_and(|size| {
                data != pointer + header || header + u64::from(width) * u64::from(count) > size
            })
            || data
                .checked_add(u64::from(width) * u64::from(count))
                .is_none()
        {
            return None;
        }
        Some(Self {
            data,
            width,
            lower: emu.maps.read_dword(pointer + bounds + 4)? as i32,
            count,
            vartype,
            locks: emu.maps.read_dword(pointer + LOCK_COUNT)?,
        })
    }
}

pub fn SafeArrayGetDim(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let array = api_try!(emu, call.arg(emu, 0));
    let dimensions = emu.maps.read_word(array).unwrap_or(0);
    log_red!(
        emu,
        "oleaut32!SafeArrayGetDim array={array:#x} dimensions={dimensions}"
    );
    call.set_return(emu, u64::from(dimensions));
    call.cleanup_args(emu, 1);
}

pub fn SafeArrayGetElemsize(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let array = api_try!(emu, call.arg(emu, 0));
    let width = array
        .checked_add(ELEMENT_BYTES)
        .and_then(|p| emu.maps.read_dword(p))
        .unwrap_or(0);
    log_red!(
        emu,
        "oleaut32!SafeArrayGetElemsize array={array:#x} size={width}"
    );
    call.set_return(emu, u64::from(width));
    call.cleanup_args(emu, 1);
}

fn bound(emu: &mut Emu, api: &str, upper: bool) {
    let call = WinApi(Stdcall);
    let [array, dimension, output] = api_try!(emu, call.args(emu));
    let result = if let Some(model) = ScalarArray::read(emu, array) {
        if dimension as u32 != 1 {
            DISP_E_BADINDEX
        } else {
            let n = if upper {
                (model.lower as u32)
                    .wrapping_add(model.count)
                    .wrapping_sub(1)
            } else {
                model.lower as u32
            };
            if output != 0 && emu.maps.write_dword(output, n) {
                S_OK
            } else {
                HRESULT_E_INVALID_ARG
            }
        }
    } else {
        HRESULT_E_INVALID_ARG
    };
    log_red!(
        emu,
        "oleaut32!{api} array={array:#x} dimension={dimension} output={output:#x}"
    );
    call.set_return(emu, result);
    call.cleanup_args(emu, 3);
}

pub fn SafeArrayGetLBound(emu: &mut Emu) {
    bound(emu, "SafeArrayGetLBound", false);
}
pub fn SafeArrayGetUBound(emu: &mut Emu) {
    bound(emu, "SafeArrayGetUBound", true);
}

enum ElementAccess {
    Read,
    Write,
    Pointer,
}

fn element(emu: &mut Emu, api: &str, access: ElementAccess) {
    let call = WinApi(Stdcall);
    let [array, indices, value] = api_try!(emu, call.args(emu));
    let result = match (ScalarArray::read(emu, array), emu.maps.read_dword(indices)) {
        (Some(model), Some(index)) if value != 0 => {
            let offset = i64::from(index as i32) - i64::from(model.lower);
            if offset < 0 || offset >= i64::from(model.count) {
                DISP_E_BADINDEX
            } else {
                let at = model.data + (offset as u64) * u64::from(model.width);
                let copied = match access {
                    ElementAccess::Pointer => memory::write_pointer(emu, value, at),
                    ElementAccess::Read => emu.maps.copy_range(value, at, model.width as usize),
                    ElementAccess::Write => emu.maps.copy_range(at, value, model.width as usize),
                };
                if copied { S_OK } else { HRESULT_E_INVALID_ARG }
            }
        }
        _ => HRESULT_E_INVALID_ARG,
    };
    log_red!(
        emu,
        "oleaut32!{api} array={array:#x} indices={indices:#x} value={value:#x}"
    );
    call.set_return(emu, result);
    call.cleanup_args(emu, 3);
}

pub fn SafeArrayGetElement(emu: &mut Emu) {
    element(emu, "SafeArrayGetElement", ElementAccess::Read);
}
pub fn SafeArrayPutElement(emu: &mut Emu) {
    element(emu, "SafeArrayPutElement", ElementAccess::Write);
}
pub fn SafeArrayPtrOfIndex(emu: &mut Emu) {
    element(emu, "SafeArrayPtrOfIndex", ElementAccess::Pointer);
}

pub fn SafeArrayGetVartype(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [array, output] = api_try!(emu, call.args(emu));
    let ok = output != 0
        && ScalarArray::read(emu, array)
            .is_some_and(|model| emu.maps.write_word(output, model.vartype));
    log_red!(
        emu,
        "oleaut32!SafeArrayGetVartype array={array:#x} output={output:#x}"
    );
    call.set_return(emu, if ok { S_OK } else { HRESULT_E_INVALID_ARG });
    call.cleanup_args(emu, 2);
}

fn lock(emu: &mut Emu, api: &str, acquire: bool, return_data: bool) {
    let call = WinApi(Stdcall);
    let array = api_try!(emu, call.arg(emu, 0));
    let output = if return_data {
        Some(api_try!(emu, call.arg(emu, 1)))
    } else {
        None
    };
    let ok = ScalarArray::read(emu, array).is_some_and(|model| {
        let count = if acquire {
            model.locks.checked_add(1)
        } else {
            model.locks.checked_sub(1)
        };
        let Some(count) = count else { return false };
        if !emu.maps.validate_write_range(array + LOCK_COUNT, 4) {
            return false;
        }
        if output.is_some_and(|p| p == 0 || !memory::write_pointer(emu, p, model.data)) {
            return false;
        }
        emu.maps.write_dword(array + LOCK_COUNT, count)
    });
    log_red!(emu, "oleaut32!{api} array={array:#x}");
    call.set_return(emu, if ok { S_OK } else { HRESULT_E_INVALID_ARG });
    call.cleanup_args(emu, 1 + usize::from(return_data));
}
pub fn SafeArrayAccessData(emu: &mut Emu) {
    lock(emu, "SafeArrayAccessData", true, true);
}
pub fn SafeArrayUnaccessData(emu: &mut Emu) {
    lock(emu, "SafeArrayUnaccessData", false, false);
}
pub fn SafeArrayLock(emu: &mut Emu) {
    lock(emu, "SafeArrayLock", true, false);
}
pub fn SafeArrayUnlock(emu: &mut Emu) {
    lock(emu, "SafeArrayUnlock", false, false);
}

pub fn SafeArrayDestroy(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let array = api_try!(emu, call.arg(emu, 0));
    let status = if array == 0 {
        S_OK
    } else if let Some(model) = ScalarArray::read(emu, array) {
        if model.locks != 0 {
            DISP_E_ARRAYISLOCKED
        } else if buffers::free(emu, array, BufferKind::SafeArray) {
            S_OK
        } else {
            HRESULT_E_INVALID_ARG
        }
    } else {
        HRESULT_E_INVALID_ARG
    };
    log_red!(emu, "oleaut32!SafeArrayDestroy array={array:#x}");
    call.set_return(emu, status);
    call.cleanup_args(emu, 1);
}
