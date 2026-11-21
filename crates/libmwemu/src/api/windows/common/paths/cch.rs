use super::{layout::*, read, write};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::{writable_pointer, write_pointer};
use crate::winapi::environment_store::Encoding::Wide;
use crate::winapi::policy::api_try;
use crate::windows::constants::{errors::*, files::*};

fn pointer_outputs(emu: &mut Emu, outputs: &[(u64, u64)]) -> Result<(), u64> {
    if outputs
        .iter()
        .any(|(p, _)| *p != 0 && !writable_pointer(emu, *p))
    {
        return Err(E_POINTER);
    }
    for &(p, v) in outputs {
        if p != 0 {
            write_pointer(emu, p, v);
        }
    }
    Ok(())
}

fn backslash(emu: &mut Emu, add: bool, extended: bool) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let cap = api_try!(emu, call.arg(emu, 1)) as usize;
    let (end, remaining) = if extended {
        (
            api_try!(emu, call.arg(emu, 2)),
            api_try!(emu, call.arg(emu, 3)),
        )
    } else {
        (0, 0)
    };
    let result = (|| {
        pointer_outputs(emu, &[(end, 0), (remaining, 0)])?;
        let mut p = read(emu, address, Wide, cap)?;
        let mut end_index = p.len();
        let changed = if add {
            if !p.is_empty() && p.last() != Some(&SLASH) {
                p.push(SLASH);
                end_index = p.len();
                true
            } else {
                false
            }
        } else if p.last() == Some(&SLASH) {
            end_index -= 1;
            if is_root(&p) {
                false
            } else {
                p.pop();
                true
            }
        } else {
            false
        };
        if changed {
            write(emu, address, Wide, cap, &p)?;
        }
        pointer_outputs(
            emu,
            &[
                (end, address + 2 * end_index as u64),
                (remaining, (cap - end_index) as u64),
            ],
        )?;
        Ok(if changed { S_OK } else { S_FALSE })
    })();
    call.cleanup_args(emu, if extended { 4 } else { 2 });
    call.set_return(emu, result.unwrap_or_else(|hr| hr));
}

fn find(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let find_ext = api == "PathCchFindExtension";
    let cap = if find_ext {
        api_try!(emu, call.arg(emu, 1)) as usize
    } else {
        PATHCCH_MAX_CCH
    };
    let output = api_try!(emu, call.arg(emu, if find_ext { 2 } else { 1 }));
    let result = (|| {
        if output == 0 && api != "PathIsUNCEx" {
            return Err(HRESULT_E_INVALID_ARG);
        }
        pointer_outputs(emu, &[(output, 0)])?;
        if find_ext && (cap == 0 || cap > PATHCCH_MAX_CCH) {
            return Err(HRESULT_E_INVALID_ARG);
        }
        let p = read(emu, address, Wide, cap).map_err(|_| HRESULT_E_INVALID_ARG)?;
        let index = match api {
            "PathCchFindExtension" => Some(extension(&p)),
            "PathCchSkipRoot" => root(&p),
            _ => unc(&p),
        }
        .ok_or(HRESULT_E_INVALID_ARG)?;
        pointer_outputs(emu, &[(output, address + 2 * index as u64)])?;
        Ok(S_OK)
    })();
    let value = if api == "PathIsUNCEx" {
        u64::from(result.is_ok())
    } else {
        result.unwrap_or_else(|hr| hr)
    };
    call.cleanup_args(emu, if find_ext { 3 } else { 2 });
    call.set_return(emu, value);
}

fn edit(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let cap = api_try!(emu, call.arg(emu, 1)) as usize;
    let extra = matches!(api, "PathCchAddExtension" | "PathCchRenameExtension");
    let suffix = if extra {
        api_try!(emu, call.arg(emu, 2))
    } else {
        0
    };
    let result = (|| {
        if cap == 0 || cap > PATHCCH_MAX_CCH {
            return Err(HRESULT_E_INVALID_ARG);
        }
        let mut p = read(emu, address, Wide, cap)?;
        if extra {
            let mut ext = read(emu, suffix, Wide, PATHCCH_MAX_CCH)?;
            if ext.first() == Some(&DOT) {
                ext.remove(0);
            }
            if ext.iter().any(|c| [DOT, SPACE, SLASH].contains(c)) {
                return Err(HRESULT_E_INVALID_ARG);
            }
            let at = extension(&p);
            if api == "PathCchAddExtension" && at < p.len() {
                return Ok(S_FALSE);
            }
            p.truncate(at);
            if !ext.is_empty() {
                p.push(DOT);
                p.extend(ext);
            }
        } else if api == "PathCchStripPrefix" {
            let Some(value) = strip_prefix(&p) else {
                return Ok(S_FALSE);
            };
            p = value;
        } else {
            let len = match api {
                "PathCchRemoveExtension" => extension(&p),
                "PathCchRemoveFileSpec" => remove_spec(&p),
                _ => {
                    let Some(mut n) = root(&p) else {
                        write(emu, address, Wide, cap, &[])?;
                        return Err(HRESULT_E_INVALID_ARG);
                    };
                    if unc(&p).is_some() && n > 2 && p.get(n - 1) == Some(&SLASH) {
                        n -= 1;
                    }
                    n
                }
            };
            if len == p.len() {
                return Ok(S_FALSE);
            }
            p.truncate(len);
        }
        write(emu, address, Wide, cap, &p)?;
        Ok(S_OK)
    })();
    call.cleanup_args(emu, if extra { 3 } else { 2 });
    call.set_return(emu, result.unwrap_or_else(|e| e));
}

fn build(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let output = api_try!(emu, call.arg(emu, 0));
    let cap = api_try!(emu, call.arg(emu, 1)) as usize;
    let first = api_try!(emu, call.arg(emu, 2));
    let combined = api.starts_with("PathCchCombine");
    let append = api.starts_with("PathCchAppend");
    let second = if combined {
        api_try!(emu, call.arg(emu, 3))
    } else {
        0
    };
    let extended = api.ends_with("Ex");
    let flags = if extended {
        api_try!(emu, call.arg_u32(emu, 3 + usize::from(combined)))
    } else {
        0
    };
    let result = (|| {
        if output == 0 || cap == 0 || cap > PATHCCH_MAX_CCH {
            return Err(HRESULT_E_INVALID_ARG);
        }
        if flags != 0 {
            return Err(E_NOTIMPL);
        }
        if first == 0 && second == 0 && !append {
            return Err(HRESULT_E_INVALID_ARG);
        }
        let a = if first != 0 {
            read(emu, first, Wide, PATHCCH_MAX_CCH)?
        } else {
            Vec::new()
        };
        let text = if append {
            combine(&read(emu, output, Wide, cap)?, &a, true)
        } else if combined {
            let b = if second != 0 {
                read(emu, second, Wide, PATHCCH_MAX_CCH)?
            } else {
                Vec::new()
            };
            combine(&a, &b, false)
        } else {
            canonicalize(&a)
        };
        if let Err(hr) = write(emu, output, Wide, cap, &text) {
            if !append && emu.maps.validate_write_range(output, 2) {
                emu.maps.write_word(output, 0);
            }
            return Err(hr);
        }
        Ok(S_OK)
    })();
    call.cleanup_args(emu, 3 + usize::from(combined) + usize::from(extended));
    call.set_return(emu, result.unwrap_or_else(|e| e));
}

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    if !api.starts_with("PathCch") && api != "PathIsUNCEx" {
        return false;
    }
    log_red!(emu, "kernelbase!{api}");
    match api {
        "PathCchAddBackslash"
        | "PathCchAddBackslashEx"
        | "PathCchRemoveBackslash"
        | "PathCchRemoveBackslashEx" => {
            backslash(emu, api.starts_with("PathCchAdd"), api.ends_with("Ex"))
        }
        "PathCchSkipRoot" | "PathCchFindExtension" | "PathIsUNCEx" => find(emu, api),
        "PathCchAddExtension"
        | "PathCchRenameExtension"
        | "PathCchRemoveExtension"
        | "PathCchRemoveFileSpec"
        | "PathCchStripPrefix"
        | "PathCchStripToRoot" => edit(emu, api),
        "PathCchAppend"
        | "PathCchAppendEx"
        | "PathCchCanonicalize"
        | "PathCchCanonicalizeEx"
        | "PathCchCombine"
        | "PathCchCombineEx" => build(emu, api),
        "PathCchIsRoot" => test_root(emu),
        _ => return false,
    }
    true
}

fn test_root(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let result = read(emu, ptr, Wide, PATHCCH_MAX_CCH).is_ok_and(|p| is_root(&p));
    call.cleanup_args(emu, 1);
    call.set_return(emu, u64::from(result));
}
