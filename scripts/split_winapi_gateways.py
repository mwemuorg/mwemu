#!/usr/bin/env python3
"""Split giant WinAPI dispatchers into one file per API (the kernel32/ layout).

For each `pub fn gateway|gateway_by_name(...) { ... match api { ... } ... }`:
  - `foo.rs` becomes `foo/mod.rs` (a `mod.rs` stays where it is);
  - every block-bodied arm `"Name" [| "Alias"] => { body }` becomes
    `"Name" [| "Alias"] => Name(emu[, api][, addr]),` and the body moves verbatim
    into `foo/<snake_name>.rs` as `pub fn Name(emu: &mut emu::Emu, ...) { body }`;
  - each new file imports exactly the names its body uses (from the parent's
    `use` lines, the parent's own items via `super::`, glob imports resolved);
  - `mod <snake_name>;` + `pub use <snake_name>::Name;` are added to mod.rs.

Arms stay inline when moving could change meaning: bodies with the `?`
operator, references to dispatcher locals other than `emu`, `api`, `addr`, or
`return` when the dispatcher does more than return `String::new()` after the
match. `return String::new();` becomes `return;`; any other `return <string>`
makes the handler return String and the arm `return Name(...)`.

usage: split_gateways.py <crate_src_dir> <file.rs>...
"""
import os
import re
import subprocess
import sys

IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
KEYWORDS = set("""as break const continue crate else enum extern false fn for if impl in
let loop match mod move mut pub ref return self Self static struct super trait true type
unsafe use where while async await dyn abstract become box do final macro override priv
typeof unsized virtual yield try gen union""".split())


def tokens(src, start, end):
    """(kind, text, pos) for code tokens, skipping comments; strings/chars are 'str'."""
    i = start
    raw = re.compile(r'b?r(#*)"')
    char = re.compile(r"'(\\u\{[0-9a-fA-F]+\}|\\.|[^'\\\n])'")
    while i < end:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i); i = end if j < 0 else j; continue
        if src.startswith("/*", i):
            depth = 1; i += 2
            while depth and i < end:
                if src.startswith("/*", i): depth += 1; i += 2
                elif src.startswith("*/", i): depth -= 1; i += 2
                else: i += 1
            continue
        if c in "br" and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = raw.match(src, i)
            if m:
                close = '"' + m.group(1)
                j = src.find(close, m.end())
                yield ("str", src[i:j + len(close)], i); i = j + len(close); continue
        if c == '"' or src.startswith('b"', i) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            j = i + (2 if c == "b" else 1)
            while src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            yield ("str", src[i:j + 1], i); i = j + 1; continue
        if c == "'" or src.startswith("b'", i):
            k = i + (1 if c == "b" else 0)
            m = char.match(src, k)
            if m:
                yield ("str", src[i:m.end()], i); i = m.end(); continue
            if c == "'":
                yield ("punct", "'", i); i += 1; continue
        if c.isalpha() or c == "_":
            m = IDENT.match(src, i)
            yield ("ident", m.group(0), i); i = m.end(); continue
        if c.isspace():
            i += 1; continue
        yield ("punct", c, i); i += 1


def match_brace(src, open_pos):
    depth = 0
    for kind, text, pos in tokens(src, open_pos, len(src)):
        if kind == "punct":
            if text == "{": depth += 1
            elif text == "}":
                depth -= 1
                if depth == 0: return pos
    raise ValueError("unbalanced braces")


def body_names(src, start, end):
    """Free-standing identifiers (not after `.`/`::`, not macro names) and an
    unsafe flag for control flow that cannot move into a function: the `?`
    operator. `return` is handled by the caller; `break`/`continue` can only
    target loops inside the arm, since the dispatcher match is not in a loop."""
    names, unsafe = set(), False
    prev = prev2 = None
    toks = list(tokens(src, start, end))
    for n, (kind, text, pos) in enumerate(toks):
        if kind == "ident":
            after_dot = prev == "."
            after_path = prev == ":" and prev2 == ":"
            is_macro = n + 1 < len(toks) and toks[n + 1][1] == "!" and toks[n + 1][0] == "punct"
            if not (after_dot or after_path or is_macro):
                names.add(text)
        elif kind == "punct" and text == "?":
            unsafe = True
        prev2, prev = prev, (text if kind == "punct" else None)
    return names, unsafe


BINDING_PATTERNS = [
    r"\blet\s+(?:mut\s+)?(\w+)",
    r"\b(?:Some|Ok|Err)\(\s*(?:mut\s+)?(\w+)\s*\)",
    r"\bfor\s+(?:mut\s+)?(\w+)\s+in\b",
]
TUPLE_BINDINGS = [r"\blet\s+(?:mut\s+)?\(([^)]*)\)", r"\bfor\s+\(([^)]*)\)\s+in\b", r"(?:[(,=]|\bmove)\s*\|([^|\n]*)\|"]


def local_bindings(text):
    """Names a body binds itself (let / for / if-let / closure params): these
    shadow same-named parent items and must not be imported."""
    names = set()
    for pat in BINDING_PATTERNS:
        names |= set(re.findall(pat, text))
    for pat in TUPLE_BINDINGS:
        for group in re.findall(pat, text):
            names |= set(re.findall(r"\b(?:mut\s+)?([a-z_]\w*)\b(?!\s*::)", group))
    return names - {"mut", "ref"}


def parse_uses(header):
    """name -> full `use` path for explicit imports; list of glob paths."""
    explicit, globs = {}, []
    for m in re.finditer(r"(?m)^\s*(?:pub(?:\([^)]*\))? )?use ([^;]+);", header):
        path = re.sub(r"\s+", " ", m.group(1)).strip()
        if "{" in path:
            base, inner = path.split("{", 1)
            for part in inner.rstrip("}").split(","):
                part = part.strip()
                if not part: continue
                if part == "self":
                    explicit[base.rstrip(":").split("::")[-1]] = base.rstrip(":"); continue
                if " as " in part:
                    orig, alias = [x.strip() for x in part.split(" as ")]
                    explicit[alias] = f"{base}{orig} as {alias}"
                else:
                    explicit[part.split("::")[-1]] = base + part
        elif path.endswith("::*"):
            globs.append(path[:-3])
        elif " as " in path:
            orig, alias = [x.strip() for x in path.split(" as ")]
            explicit[alias] = f"{orig} as {alias}"
        else:
            explicit[path.split("::")[-1]] = path
    return explicit, globs


def module_file(crate_src, path):
    rel = path.split("::")
    assert rel[0] == "crate", path
    base = os.path.join(crate_src, *rel[1:])
    for cand in (base + ".rs", os.path.join(base, "mod.rs")):
        if os.path.exists(cand): return cand
    raise FileNotFoundError(path)


def pub_items(src):
    return set(re.findall(r"(?m)^pub (?:const|static|fn|struct|enum|type|trait) (\w+)", src))


def pub_names(file, seen=None):
    """Public names a module exports, following `pub use child::*` / `pub use child::{..}`."""
    seen = seen if seen is not None else set()
    if file in seen: return set()
    seen.add(file)
    src = open(file).read()
    names = pub_items(src)
    d = os.path.dirname(file) if os.path.basename(file) == "mod.rs" else file[:-3]
    for m in re.finditer(r"(?m)^pub use (?:self::)?(\w+)::(\*|\{[^}]*\}|\w+);", src):
        child, what = m.group(1), m.group(2)
        if what == "*":
            for cand in (os.path.join(d, child + ".rs"), os.path.join(d, child, "mod.rs")):
                if os.path.exists(cand):
                    names |= pub_names(cand, seen); break
        else:
            names |= {w.strip().split(" as ")[-1] for w in what.strip("{}").split(",") if w.strip()}
    return names


def top_level_items(src):
    items = set(re.findall(r"(?m)^(?:pub(?:\([^)]*\))? )?(?:const|static(?: mut)?|fn|struct|enum|type|trait|mod) (\w+)", src))
    for block in re.finditer(r"(?m)^(?:lazy_static|thread_local)! \{", src):
        end = match_brace(src, block.end() - 1)
        items |= set(re.findall(r"static (?:ref )?(\w+)", src[block.end():end]))
    return items


def snake(name):
    s = re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", name)
    s = re.sub(r"(?<=[A-Z])(?=[A-Z][a-z])", "_", s)
    s = re.sub(r"\W", "_", s).lower()
    return s + "_api" if s in KEYWORDS or s in CRATE_NAMES or not s or s[0].isdigit() else s


CRATE_NAMES = {"log", "std", "core", "alloc", "serde", "lazy_static", "iced_x86", "crate"}


def relocate(path):
    """A parent's `use` path as seen from a child module."""
    if path.startswith("self::"):
        return "super::" + path.split("::", 1)[1]
    if path.startswith("super::"):
        return "super::" + path
    return path


CALL_ONLY = re.compile(r"\s*(?:return\s+)?[\w:]+\(\s*\w+(?:\s*,\s*\w+)*\s*\)\s*;?\s*")
ARM = re.compile(r'(?m)^(?P<ind>[ \t]+)(?P<pats>"[^"\n]*"(?:\s*\|\s*"[^"\n]*")*)\s*=>\s*\{')
GATEWAY = re.compile(r"(?m)^pub fn (gateway|gateway_by_name)\((?P<params>[^)]*)\)[^{]*\{")


def split_file(crate_src, path):
    src = open(path).read()
    if os.path.basename(path) == "mod.rs":
        folder, mod_path = os.path.dirname(path), path
    else:
        folder, mod_path = path[:-3], os.path.join(path[:-3], "mod.rs")
    first_gw = GATEWAY.search(src)
    if not first_gw:
        return 0, 0
    explicit, globs = parse_uses(src[:first_gw.start()])
    glob_names = {}
    local_globs = False
    for g in globs:
        if not g.startswith("crate::"):
            local_globs = True      # `pub use appcompat::*;` -> reach through super::
            continue
        for name in pub_names(module_file(crate_src, g)):
            glob_names[name] = g
    if local_globs and os.path.isdir(folder):
        for root, _, fs in os.walk(folder):
            for f in fs:
                if f.endswith(".rs") and os.path.join(root, f) != path:
                    for name in pub_items(open(os.path.join(root, f)).read()):
                        glob_names.setdefault(name, "super")
    parent_items = top_level_items(src)
    taken = set(parent_items) | set(explicit)
    existing_files = set(os.listdir(folder)) if os.path.isdir(folder) else set()
    # names already defined by files in the folder (kernel32's category re-exports)
    if os.path.isdir(folder):
        for root, _, fs in os.walk(folder):
            for f in fs:
                if f.endswith(".rs") and os.path.join(root, f) != path:
                    taken |= pub_items(open(os.path.join(root, f)).read())

    new_files, mod_lines = {}, []
    pieces, cursor, moved, kept = [], 0, 0, 0
    for g in list(GATEWAY.finditer(src)):
        body_open = g.end() - 1
        body_close = match_brace(src, body_open)
        params = {p.split(":")[0].strip(): p.split(":", 1)[1].strip()
                  for p in g.group("params").split(",") if ":" in p}
        m = re.compile(r"\bmatch\s+(\w+)\s*\{").search(src, body_open, body_close)
        if not m or "emu" not in params:
            continue
        scrutinee = m.group(1)
        locals_ = set(re.findall(r"\blet\s+(?:mut\s+)?(\w+)", src[body_open:m.start()])) | set(params)
        types = dict(params); types.setdefault(scrutinee, "&str")
        match_open = m.end() - 1
        match_close = match_brace(src, match_open)
        # `return X` in an arm leaves the dispatcher; we can only reproduce that when
        # the dispatcher returns `String::new()` right after the match and the match
        # is not inside a loop (for break/continue)
        tail_is_empty_string = src[match_close + 1:body_close].strip() == "String::new()"
        in_loop = any(k == "ident" and t in ("loop", "while", "for")
                      for k, t, _ in tokens(src, body_open + 1, m.start()))
        pos, arm_cursor, out = match_open + 1, match_open + 1, []
        while True:
            a = ARM.search(src, pos, match_close)
            if not a: break
            open_pos = a.end() - 1
            close_pos = match_brace(src, open_pos)
            names, unsafe = body_names(src, open_pos + 1, close_pos)
            body = src[open_pos + 1:close_pos]
            if CALL_ONLY.fullmatch(body):
                # already a call (rustfmt wraps long `"Name" => Name(emu),` in a block)
                pos = close_pos + 1; continue
            refs = sorted((names & locals_) - {"emu"})
            returns = [r.strip() for r in re.findall(r"\breturn\b([^;]*);", body)] if "return" in names else []
            bad_flow = in_loop and ({"break", "continue"} & names)
            if unsafe or bad_flow or any(r not in types for r in refs) or (returns and not tail_is_empty_string):
                kept += 1; pos = close_pos + 1; continue
            returns_string = False
            if returns:
                if all(r == "String::new()" for r in returns):
                    # leaving the dispatcher early == the handler returning
                    body = re.sub(r"\breturn\s+String::new\(\)\s*;", "return;", body)
                else:
                    # the arm hands a value back to the dispatcher's caller
                    returns_string = True
                    body = body.rstrip() + "\n    String::new()\n"
            api = re.findall(r'"([^"\n]*)"', a.group("pats"))[0]
            fname = api if IDENT.fullmatch(api) and api not in KEYWORDS else re.sub(r"\W", "_", api)
            base, n = fname, 2
            if fname in taken:
                fname = base = base + "_api"
            while fname in taken:
                fname = f"{base}_{n}"; n += 1
            taken.add(fname)
            fstem, n = snake(fname), 2
            while f"{fstem}.rs" in existing_files or f"{fstem}.rs" in new_files:
                fstem = f"{snake(fname)}_{n}"; n += 1
            # imports this body needs
            uses = ["use crate::emu;"]
            for name in sorted(names - {"emu"} - set(refs) - local_bindings(body)):
                if name in explicit and explicit[name] != "crate::emu":
                    uses.append(f"use {relocate(explicit[name])};")
                elif name in parent_items:
                    uses.append(f"use super::{name};")
                elif name in glob_names:
                    uses.append(f"use {glob_names[name]}::{name};")
            args = ["emu"] + refs
            sig = ", ".join([f"emu: {params['emu']}"] + [f"{r}: {types[r]}" for r in refs])
            ret_ty = " -> String" if returns_string else ""
            new_files[f"{fstem}.rs"] = "\n".join(sorted(set(uses), key=uses.index)) + \
                f"\n\npub fn {fname}({sig}){ret_ty} {{{body}}}\n"
            mod_lines.append(f"mod {fstem};\npub use {fstem}::{fname};\n")
            end = close_pos + 1
            if src[end:end + 1] == ",": end += 1
            call = f"{fname}({', '.join(args)})"
            out.append(src[arm_cursor:open_pos] + (f"return {call}," if returns_string else f"{call},"))
            arm_cursor, pos = end, end
            moved += 1
        out.append(src[arm_cursor:match_close])
        pieces.append(src[cursor:match_open + 1] + "".join(out))
        cursor = match_close
    pieces.append(src[cursor:])
    new_src = "".join(pieces)

    # step 2: handler functions the dispatcher calls that still live in this file
    called = set()
    for g in GATEWAY.finditer(new_src):
        gb = match_brace(new_src, g.end() - 1)
        called |= set(re.findall(r'"\s*=>\s*(\w+)\(', new_src[g.end():gb]))
    for fname in sorted(called):
        fm = re.search(rf"(?m)^(?P<vis>pub(?:\([^)]*\))? )?fn {re.escape(fname)}\b", new_src)
        if not fm or fname in ("gateway", "gateway_by_name"):
            continue
        brace = next(p for k, t, p in tokens(new_src, fm.end(), len(new_src)) if k == "punct" and t == "{")
        fend = match_brace(new_src, brace) + 1
        # doc comments / attributes directly above
        start = fm.start()
        while True:
            prev_line_start = new_src.rfind("\n", 0, start - 1) + 1
            line = new_src[prev_line_start:start - 1].strip()
            if start > 0 and (line.startswith("///") or line.startswith("#[")):
                start = prev_line_start
            else:
                break
        item = new_src[start:fend]
        names, _ = body_names(new_src, fm.end(), fend)
        fstem, n = snake(fname), 2
        while f"{fstem}.rs" in existing_files or f"{fstem}.rs" in new_files:
            fstem = f"{snake(fname)}_{n}"; n += 1
        uses = ["use crate::emu;"]
        for name in sorted(names - {"emu", fname} - local_bindings(item)):
            if name in explicit and explicit[name] != "crate::emu":
                uses.append(f"use {relocate(explicit[name])};")
            elif name in parent_items:
                uses.append(f"use super::{name};")
            elif name in glob_names:
                uses.append(f"use {glob_names[name]}::{name};")
        public = (fm.group("vis") or "").strip() == "pub"
        if not public:
            item = item.replace(f"fn {fname}", f"pub(super) fn {fname}", 1) if not fm.group("vis") \
                else item
        new_files[f"{fstem}.rs"] = "\n".join(uses) + "\n\n" + item.strip("\n") + "\n"
        mod_lines.append(f"mod {fstem};\n{'pub ' if public else ''}use {fstem}::{fname};\n")
        new_src = new_src[:start] + new_src[fend:].lstrip("\n")
        moved += 1

    if not moved:
        return 0, kept
    # mod declarations go after the leading `use` block
    uses_end = 0
    for m in re.finditer(r"(?m)^#!\[[^\n]*\]\n|^//[^\n]*\n|^(?:pub(?:\([^)]*\))? )?use [^;]+;\n|^mod \w+;\n|^pub use [^;]+;\n|^\n", new_src):
        if m.start() != uses_end: break
        uses_end = m.end()
    new_src = new_src[:uses_end] + "\n" + "".join(mod_lines) + "\n" + new_src[uses_end:]
    new_src = prune_unused_uses(new_src, new_files, glob_names)
    if mod_path != path:
        os.makedirs(folder, exist_ok=True)
        subprocess.run(["git", "mv", path, mod_path], check=True)
    open(mod_path, "w").write(new_src)
    for name, text in new_files.items():
        open(os.path.join(folder, name), "w").write(text)
    return moved, kept


USE_LINE = re.compile(r"(?m)^use ([^;{]*?)(\{[^}]*\}|\*|\w+(?: as \w+)?);\n")


def prune_unused_uses(src, new_files, glob_names):
    """Drop the dispatcher's private `use` imports that only moved bodies needed.
    A name stays if the rest of mod.rs uses it or a child reaches it via `super::`."""
    rest = USE_LINE.sub("", src)
    used, _ = body_names(rest, 0, len(rest))
    used |= set(re.findall(r"\b(\w+)!", rest))   # macros imported with `use`
    for text in new_files.values():
        used |= set(re.findall(r"^use super::(\w+);", text, re.M))

    def keep(m):
        base, what = m.group(1), m.group(2)
        if what == "*":
            module = base.rstrip(":")
            names = {n for n, g in glob_names.items() if g == module}
            return m.group(0) if not names or names & used else ""
        parts = [p.strip() for p in what.strip("{}").split(",") if p.strip()]
        alive = [p for p in parts if (p.split(" as ")[-1] if " as " in p else p.split("::")[-1]) in used
                 or (p == "self" and base.rstrip(":").split("::")[-1] in used)]
        if not alive:
            return ""
        if not what.startswith("{"):
            return m.group(0)
        return f"use {base}{alive[0]};\n" if len(alive) == 1 else f"use {base}{{{', '.join(alive)}}};\n"

    return USE_LINE.sub(keep, src)


if __name__ == "__main__":
    crate_src = sys.argv[1]
    tm = tk = 0
    for path in sys.argv[2:]:
        m, k = split_file(crate_src, path)
        tm += m; tk += k
        print(f"{m:6} moved {k:4} kept  {path}")
    print(f"{tm:6} moved {tk:4} kept  TOTAL")
