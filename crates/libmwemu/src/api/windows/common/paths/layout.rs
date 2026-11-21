pub(super) const SLASH: u16 = b'\\' as u16;
pub(super) const DOT: u16 = b'.' as u16;
pub(super) const SPACE: u16 = b' ' as u16;
const COLON: u16 = b':' as u16;

pub(super) fn ascii_prefix(s: &[u16], ascii: &[u8]) -> bool {
    s.len() >= ascii.len()
        && s.iter()
            .zip(ascii)
            .all(|(&a, &b)| u8::try_from(a).is_ok_and(|a| a.eq_ignore_ascii_case(&b)))
}

pub(super) fn drive(p: &[u16]) -> Option<u8> {
    let letter = u8::try_from(*p.first()?).ok()?;
    (letter.is_ascii_alphabetic() && p.get(1) == Some(&COLON))
        .then(|| letter.to_ascii_uppercase() - b'A')
}

pub(super) fn unc(p: &[u16]) -> Option<usize> {
    if ascii_prefix(p, br"\\?\UNC\") {
        Some(8)
    } else if p.starts_with(&[SLASH, SLASH]) && !ascii_prefix(p, br"\\?\") {
        Some(2)
    } else {
        None
    }
}

pub(super) fn root(p: &[u16]) -> Option<usize> {
    if let Some(start) = unc(p) {
        let n = p[start..]
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == SLASH)
            .nth(1)
            .map_or(p.len(), |(i, _)| start + i + 1);
        return Some(n);
    }
    let start = if ascii_prefix(p, br"\\?\") { 4 } else { 0 };
    if drive(&p[start..]).is_some() {
        return Some(start + 2 + usize::from(p.get(start + 2) == Some(&SLASH)));
    }
    (start == 0 && p.first() == Some(&SLASH)).then_some(1)
}

pub(super) fn is_root(p: &[u16]) -> bool {
    if let Some(start) = unc(p) {
        let tail = &p[start..];
        return tail.is_empty()
            || tail.iter().filter(|c| **c == SLASH).count() <= 1 && tail.last() != Some(&SLASH);
    }
    root(p) == Some(p.len()) && p.last() == Some(&SLASH)
}

pub(super) fn extension(p: &[u16]) -> usize {
    let mut ext = p.len();
    for (i, &u) in p.iter().enumerate() {
        match u {
            DOT => ext = i,
            SPACE | SLASH => ext = p.len(),
            _ => {}
        }
    }
    ext
}

pub(super) fn filename(p: &[u16]) -> usize {
    let mut at = 0;
    for (i, pair) in p.windows(2).enumerate() {
        if [SLASH, COLON, b'/' as u16].contains(&pair[0])
            && ![SLASH, b'/' as u16].contains(&pair[1])
        {
            at = i + 1;
        }
    }
    at
}

pub(super) fn remove_spec(p: &[u16]) -> usize {
    if is_root(p) {
        return p.len();
    }
    let floor = root(p).unwrap_or(0);
    let floor = if unc(p).is_some() && floor > 2 && p.get(floor - 1) == Some(&SLASH) {
        floor - 1
    } else {
        floor
    };
    p.iter().rposition(|c| *c == SLASH).unwrap_or(0).max(floor)
}

pub(super) fn strip_prefix(p: &[u16]) -> Option<Vec<u16>> {
    if ascii_prefix(p, br"\\?\UNC\") {
        let mut result = vec![SLASH, SLASH];
        result.extend_from_slice(&p[8..]);
        Some(result)
    } else if ascii_prefix(p, br"\\?\") && drive(&p[4..]).is_some() {
        Some(p[4..].to_vec())
    } else {
        None
    }
}

pub(super) fn canonicalize(path: &[u16]) -> Vec<u16> {
    let unprefixed = strip_prefix(path);
    let path = unprefixed.as_deref().unwrap_or(path);
    let start = if unc(path).is_some() {
        2
    } else if drive(path).is_some() && path.get(2) == Some(&SLASH) {
        3
    } else if path.first() == Some(&SLASH) {
        1
    } else {
        0
    };
    let mut result = path[..start].to_vec();
    let mut parts = Vec::new();
    for part in path[start..].split(|c| *c == SLASH) {
        if part == [DOT] {
            continue;
        }
        if part == [DOT, DOT] {
            if parts.pop().is_some() && parts.is_empty() && result.is_empty() {
                result.push(SLASH);
            }
        } else {
            parts.push(part);
        }
    }
    for (i, part) in parts.iter().enumerate() {
        if i != 0 {
            result.push(SLASH);
        }
        result.extend_from_slice(part);
    }
    while result.last() == Some(&DOT) {
        if result.len() > 1 && result[result.len() - 2] == b'*' as u16 {
            break;
        }
        result.pop();
    }
    if result.is_empty() || (drive(&result).is_some() && result.len() == 2) {
        result.push(SLASH);
    }
    result
}

pub(super) fn combine(base: &[u16], more: &[u16], append: bool) -> Vec<u16> {
    let more = if append && more.starts_with(&[SLASH]) && !more.starts_with(&[SLASH, SLASH]) {
        &more[1..]
    } else {
        more
    };
    let mut joined = if more.starts_with(&[SLASH, SLASH]) || drive(more).is_some() {
        Vec::new()
    } else if more.starts_with(&[SLASH]) {
        let mut volume = base[..root(base).unwrap_or(0)].to_vec();
        if volume.last() == Some(&SLASH) {
            volume.pop();
        }
        volume
    } else {
        base.to_vec()
    };
    if !joined.is_empty()
        && !more.is_empty()
        && joined.last() != Some(&SLASH)
        && more.first() != Some(&SLASH)
    {
        joined.push(SLASH);
    }
    joined.extend_from_slice(more);
    canonicalize(&joined)
}
