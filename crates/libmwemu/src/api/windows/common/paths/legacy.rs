use super::{MAX_PATH, layout::*, read, write};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{api_try, unsupported};
use crate::windows::constants::errors::*;

fn ascii_fold(u: u16) -> u16 {
    u8::try_from(u).map_or(u, |v| u16::from(v.to_ascii_lowercase()))
}

fn query(emu: &mut Emu, op: &str, codec: Encoding) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let result = read(emu, addr, codec, MAX_PATH)
        .map(|p| {
            let pointer = |index: usize| addr + (index * codec.width()) as u64;
            match op {
                "PathGetDriveNumber" => drive(&p).map_or(u32::MAX.into(), u64::from),
                "PathFindExtension" => pointer(extension(&p)),
                "PathFindFileName" => pointer(filename(&p)),
                "PathFindNextComponent" if p.is_empty() => 0,
                "PathFindNextComponent" => {
                    pointer(p.iter().position(|u| *u == SLASH).map_or(p.len(), |i| {
                        i + 1 + usize::from(p.get(i + 1) == Some(&SLASH))
                    }))
                }
                "PathSkipRoot" => {
                    if drive(&p).is_some() && p.get(2) != Some(&SLASH)
                        || p.first() == Some(&SLASH) && !p.starts_with(&[SLASH, SLASH])
                    {
                        0
                    } else {
                        root(&p).map_or(0, pointer)
                    }
                }
                "PathIsFileSpec" => u64::from(!p.contains(&SLASH) && !p.contains(&(b':' as u16))),
                "PathIsRelative" => u64::from(p.first() != Some(&SLASH) && drive(&p).is_none()),
                "PathIsRoot" => u64::from(is_root(&p)),
                "PathIsUNC" => u64::from(p.starts_with(&[SLASH, SLASH])),
                "PathIsUNCServer" => {
                    u64::from(p.starts_with(&[SLASH, SLASH]) && !p[2..].contains(&SLASH))
                }
                "PathIsUNCServerShare" => u64::from(
                    p.starts_with(&[SLASH, SLASH])
                        && p[2..].iter().filter(|c| **c == SLASH).count() == 1
                        && p.last() != Some(&SLASH),
                ),
                "PathIsLFNFileSpec" => {
                    let mut parts = p.split(|c| *c == DOT);
                    u64::from(
                        p.contains(&SPACE)
                            || parts.next().is_some_and(|base| base.len() > 8)
                            || parts.next().is_some_and(|ext| ext.len() > 3)
                            || parts.next().is_some(),
                    )
                }
                "PathIsURL" => {
                    u64::from(p.iter().position(|c| *c == b':' as u16).is_some_and(|i| {
                        i > 1
                            && p[..i].iter().all(|c| {
                                u8::try_from(*c)
                                    .is_ok_and(|b| b.is_ascii_alphanumeric() || b"+-.".contains(&b))
                            })
                    }))
                }
                _ => 0,
            }
        })
        .unwrap_or_else(|_| match op {
            "PathGetDriveNumber" => u32::MAX.into(),
            "PathIsRelative" => 1,
            _ => 0,
        });
    call.cleanup_args(emu, 1);
    call.set_return(emu, result);
}

fn match_spec(s: &[u16], pat: &[u16]) -> bool {
    if pat == [b'*' as u16, DOT, b'*' as u16] {
        return true;
    }
    let (mut i, mut j, mut retry, mut star) = (0, 0, 0, None);
    while i < s.len() {
        if j < pat.len() && (pat[j] == b'?' as u16 || ascii_fold(s[i]) == ascii_fold(pat[j])) {
            i += 1;
            j += 1;
        } else if pat.get(j) == Some(&(b'*' as u16)) {
            star = Some(j + 1);
            retry = i;
            j += 1;
        } else if let Some(offset) = star {
            retry += 1;
            i = retry;
            j = offset;
        } else {
            return false;
        }
    }
    pat[j..].iter().all(|u| *u == b'*' as u16)
}

fn compare(emu: &mut Emu, op: &str, codec: Encoding) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg(emu, 0));
    let second = api_try!(emu, call.arg(emu, 1));
    let extended = op == "PathMatchSpecEx";
    let flags = if extended {
        api_try!(emu, call.arg_u32(emu, 2))
    } else {
        0
    };
    let result = (|| {
        let a = read(emu, first, codec, MAX_PATH)?;
        let b = read(emu, second, codec, MAX_PATH)?;
        let yes = match op {
            "PathIsSameRoot" => root(&a).zip(root(&b)).is_some_and(|(i, j)| {
                a[..i]
                    .iter()
                    .copied()
                    .map(ascii_fold)
                    .eq(b[..j].iter().copied().map(ascii_fold))
            }),
            "PathIsPrefix" => {
                !a.is_empty()
                    && a.len() <= b.len()
                    && a.iter()
                        .zip(&b)
                        .all(|(&x, &y)| ascii_fold(x) == ascii_fold(y))
                    && (a.len() == b.len()
                        || a.last() == Some(&SLASH)
                        || b.get(a.len()) == Some(&SLASH))
            }
            _ => {
                if flags > 1 {
                    return Err(E_NOTIMPL);
                }
                if extended && flags == 0 {
                    match_spec(&a, &b)
                } else {
                    b.split(|c| *c == b';' as u16).any(|pattern| {
                        match_spec(
                            &a,
                            &pattern[pattern
                                .iter()
                                .position(|v| *v != SPACE)
                                .unwrap_or(pattern.len())..],
                        )
                    })
                }
            }
        };
        Ok(if extended {
            if yes { S_OK } else { S_FALSE }
        } else {
            u64::from(yes)
        })
    })();
    call.cleanup_args(emu, 2 + usize::from(extended));
    call.set_return(emu, result.unwrap_or_else(|e| if extended { e } else { 0 }));
}

fn build(emu: &mut Emu, op: &str, codec: Encoding) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let a = api_try!(emu, call.arg(emu, 1));
    let b = if op == "PathCombine" {
        api_try!(emu, call.arg(emu, 2))
    } else {
        0
    };
    let result = (|| {
        let text = match op {
            "PathCombine" => {
                if a == 0 && b == 0 {
                    return Err(HRESULT_E_INVALID_ARG);
                }
                let a = if a == 0 {
                    Vec::new()
                } else {
                    read(emu, a, codec, MAX_PATH)?
                };
                let b = if b == 0 {
                    Vec::new()
                } else {
                    read(emu, b, codec, MAX_PATH)?
                };
                combine(&a, &b, false)
            }
            "PathAppend" => combine(
                &read(emu, out, codec, MAX_PATH)?,
                &read(emu, a, codec, MAX_PATH)?,
                true,
            ),
            _ => canonicalize(&read(emu, a, codec, MAX_PATH)?),
        };
        write(emu, out, codec, MAX_PATH, &text)?;
        Ok(if op == "PathCombine" { out } else { 1 })
    })();
    call.cleanup_args(emu, if op == "PathCombine" { 3 } else { 2 });
    call.set_return(emu, result.unwrap_or(0));
}

fn edit(emu: &mut Emu, op: &str, codec: Encoding) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let has_ext = matches!(op, "PathAddExtension" | "PathRenameExtension");
    let ext = if has_ext {
        api_try!(emu, call.arg(emu, 1))
    } else {
        0
    };
    let result: Result<u64, u64> = (|| {
        let mut p = read(emu, addr, codec, MAX_PATH)?;
        let mut ret = 1;
        match op {
            "PathAddBackslash" => {
                if !p.is_empty() && p.last() != Some(&SLASH) {
                    p.push(SLASH);
                }
                ret = addr + (p.len() * codec.width()) as u64;
            }
            "PathRemoveBackslash" => {
                let last = p.len().saturating_sub(1);
                if p.last() == Some(&SLASH) && !is_root(&p) {
                    p.pop();
                }
                ret = addr + (last * codec.width()) as u64;
            }
            "PathRemoveExtension" => p.truncate(extension(&p)),
            "PathAddExtension" | "PathRenameExtension" => {
                let i = extension(&p);
                if op == "PathAddExtension" && i != p.len() {
                    return Ok(0);
                }
                let ext = if ext == 0 && op == "PathAddExtension" {
                    ".exe".encode_utf16().collect()
                } else {
                    read(emu, ext, codec, MAX_PATH)?
                };
                p.truncate(i);
                p.extend(ext);
            }
            "PathRemoveFileSpec" => {
                let i = remove_spec(&p);
                ret = u64::from(i != p.len());
                p.truncate(i);
            }
            "PathStripToRoot" => {
                let i = root(&p).unwrap_or(0);
                ret = u64::from(i != 0);
                p.truncate(i);
            }
            "PathStripPath" => {
                let i = filename(&p);
                p.drain(..i);
            }
            "PathRemoveBlanks" => {
                let start = p.iter().position(|c| *c != SPACE).unwrap_or(p.len());
                p.drain(..start);
                p.truncate(p.iter().rposition(|c| *c != SPACE).map_or(0, |i| i + 1));
            }
            "PathQuoteSpaces" => {
                if !p.contains(&SPACE) {
                    return Ok(0);
                }
                p.insert(0, b'"' as u16);
                p.push(b'"' as u16);
            }
            "PathUnquoteSpaces"
                if p.len() >= 2 && p.first() == Some(&(b'"' as u16)) && p.last() == p.first() =>
            {
                p.remove(0);
                p.pop();
            }
            _ => {}
        }
        write(emu, addr, codec, MAX_PATH, &p)?;
        Ok(ret)
    })();
    call.cleanup_args(emu, 1 + usize::from(has_ext));
    if !matches!(
        op,
        "PathRemoveExtension" | "PathStripPath" | "PathRemoveBlanks" | "PathUnquoteSpaces"
    ) {
        call.set_return(emu, result.unwrap_or(0));
    }
}

pub(super) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    let Some((op, wide)) = api
        .strip_suffix('A')
        .map(|s| (s, false))
        .or_else(|| api.strip_suffix('W').map(|s| (s, true)))
    else {
        if library == "kernelbase"
            && matches!(
                api,
                "PathAllocCanonicalize" | "PathAllocCombine" | "PathCreateFromUrlAlloc"
            )
        {
            unsupported(emu, library, api, emu.pc());
            return true;
        }
        return false;
    };
    let handler: fn(&mut Emu, &str, Encoding) = match op {
        "PathGetDriveNumber"
        | "PathFindExtension"
        | "PathFindFileName"
        | "PathFindNextComponent"
        | "PathSkipRoot"
        | "PathIsFileSpec"
        | "PathIsLFNFileSpec"
        | "PathIsRelative"
        | "PathIsRoot"
        | "PathIsUNC"
        | "PathIsUNCServer"
        | "PathIsUNCServerShare"
        | "PathIsURL" => query,
        "PathIsPrefix" | "PathIsSameRoot" | "PathMatchSpec" | "PathMatchSpecEx" => compare,
        "PathAppend" | "PathCanonicalize" | "PathCombine" => build,
        "PathAddBackslash"
        | "PathRemoveBackslash"
        | "PathRemoveFileSpec"
        | "PathAddExtension"
        | "PathRemoveExtension"
        | "PathRenameExtension"
        | "PathRemoveBlanks"
        | "PathQuoteSpaces"
        | "PathUnquoteSpaces"
        | "PathStripPath"
        | "PathStripToRoot" => edit,
        "PathCommonPrefix"
        | "PathParseIconLocation"
        | "PathRelativePathTo"
        | "PathSearchAndQualify"
        | "PathCreateFromUrl"
        | "PathGetArgs"
        | "PathGetCharType"
        | "PathUnExpandEnvStrings" => {
            unsupported(emu, library, api, emu.pc());
            return true;
        }
        "PathIsValidChar" if library == "kernelbase" => {
            unsupported(emu, library, api, emu.pc());
            return true;
        }
        _ => return false,
    };
    log_red!(emu, "{library}!{api}");
    handler(emu, op, Encoding::from_wide(wide));
    true
}
