#!/usr/bin/env python3
"""Prepare mwemu test inputs and Windows system files using the Python standard library."""

import argparse
import gzip
import hashlib
import json
import os
import platform
import re
import shutil
import struct
import sys
from pathlib import Path
from tempfile import TemporaryDirectory
from urllib.request import urlopen
from zipfile import BadZipFile, ZipFile

from pe_dependencies import DLL_NAME, MACHINES, pe_imports


ROOT = Path(__file__).resolve().parents[1]
FIXTURE_URL = "https://github.com/mwemuorg/mwemu/releases/download/maps/test.zip"
SYMBOL_URL = "https://msdl.microsoft.com/download/symbols"
PASSWORD = b"mwemuTestSystem"
CACHE_BUILD = "26100.7920"
NLS = ("locale.nls", "C_1252.NLS", "C_437.NLS")


def ready(path, digest=None):
    try:
        with path.open("rb") as f:
            if digest is None:
                return bool(f.read(1))
            checksum = hashlib.sha256()
            for block in iter(lambda: f.read(1024 * 1024), b""):
                checksum.update(block)
            return checksum.hexdigest() == digest
    except FileNotFoundError:
        return False


def install(stream, dest, validate=None):
    dest.parent.mkdir(parents=True, exist_ok=True)
    with TemporaryDirectory(dir=dest.parent) as scratch:
        tmp = Path(scratch) / "download"
        with tmp.open("wb") as out:
            shutil.copyfileobj(stream, out)
        if validate:
            validate(tmp)
        tmp.replace(dest)


def members(bundle, root):
    files = []
    for entry in bundle.infolist():
        parts = entry.filename.rstrip("/").split("/")
        if (parts[0] != "test" or "\\" in entry.filename or ":" in entry.filename
                or any(part in ("", ".", "..") for part in parts[1:])):
            raise ValueError(f"Unexpected sample path: {entry.filename}")
        if entry.is_dir():
            continue
        dest = root.joinpath(*parts[1:])
        if len(parts) == 1 or not dest.resolve().is_relative_to(root):
            raise ValueError(f"Sample path escapes the test directory: {entry.filename}")
        files.append((entry, dest))
    return files


def validate_bundle(path, root):
    with ZipFile(path) as bundle:
        members(bundle, root)
        bundle.setpassword(PASSWORD)
        corrupt = bundle.testzip()
        if corrupt is not None:
            raise BadZipFile(f"Corrupt archive member: {corrupt}")


def test_fixtures(root, offline):
    root = root.resolve()
    cache = root / ".samples.zip"
    if not cache.exists():
        if offline:
            print(f"No cached fixture archive at {cache}. Tests can still use files in {root}.")
            return None
        with urlopen(FIXTURE_URL, timeout=90) as stream:
            install(stream, cache, lambda path: validate_bundle(path, root))
    else:
        validate_bundle(cache, root)

    with ZipFile(cache) as bundle:
        files = members(bundle, root)
        for entry, dest in files:
            if not ready(dest):
                with bundle.open(entry, pwd=PASSWORD) as stream:
                    install(stream, dest)
    return [], len(files)


def install_dll(dest, local, digest, key, offline):
    # An indexed digest applies to existing files, cache hits and new downloads.
    if ready(dest, digest):
        return True

    def verify(path):
        if not ready(path, digest):
            raise ValueError(f"SHA-256 mismatch for {dest.name} ({key})")

    if ready(local, digest):
        with local.open("rb") as stream:
            install(stream, dest, verify if digest else None)
        return True
    if digest is None or offline:
        return False
    with urlopen(f"{SYMBOL_URL}/{dest.name}/{key}/{dest.name}", timeout=60) as stream:
        install(stream, dest, verify)
    return True


def component_versions(item):
    yield item.get("fileInfo", {}).get("version") or ""
    # Some resources use a product version such as 5.82. Their servicing
    # assembly records carry the OS build and distinguish System32 from WinSxS.
    for updates in item.get("windowsVersions", {}).values():
        for update in updates.values():
            for assembly in update.get("assemblies", {}).values():
                if any(attr.get("destinationPath", "").rstrip("\\/").lower() == "$(runtime.system32)"
                       for attr in assembly.get("attributes", [])):
                    yield assembly.get("assemblyIdentity", {}).get("version", "")


def symbol_info(index_dir, name, machine, build, offline):
    path = index_dir / (name + ".json")
    if not path.exists():
        if offline:
            return None
        url = f"https://winbindex.m417z.com/data/by_filename_compressed/{name}.json.gz"
        with urlopen(url, timeout=60) as stream, gzip.GzipFile(fileobj=stream) as decoded:
            install(decoded, path, read_index)
    index = read_index(path)
    family, revision = map(int, build.split("."))
    choices = []
    for digest, item in index.items():
        info = item.get("fileInfo", {})
        if info.get("machineType") != machine or not re.fullmatch("[0-9a-f]{64}", digest):
            continue
        stamp, size = info.get("timestamp"), info.get("virtualSize")
        if not isinstance(stamp, int) or not isinstance(size, int) or size <= 0:
            continue
        for version in component_versions(item):
            match = re.match(r"\d+\.\d+\.(\d+)\.(\d+)\b", version)
            if match and int(match[1]) == family and int(match[2]) <= revision:
                choices.append((int(match[2]), digest, f"{stamp:08X}{size:X}"))
    if not choices:
        raise ValueError(f"No indexed {name} for machine {machine:#x} at Windows build {build}")
    _, digest, key = max(choices)
    return digest, key


def read_index(path):
    with path.open(encoding="utf-8") as stream:
        index = json.load(stream)
    if not isinstance(index, dict):
        raise ValueError(f"Invalid Winbindex index: {path}")
    return index


def system_folder(arch):
    system = Path(os.environ["SystemRoot"])
    if arch == "x86":
        return system / "SysWOW64"
    native = system / "Sysnative"  # Used by a 32-bit Python on 64-bit Windows.
    return native if native.is_dir() else system / "System32"


def prepare_dll(maps, arch, name, source, build, offline):
    if not DLL_NAME.fullmatch(name):
        raise ValueError(f"Invalid DLL basename: {name!r}")
    dest = maps / "windows" / arch / name
    if source == "host":
        local = system_folder(arch) / name
        return dest if install_dll(dest, local, None, None, offline) else None
    if source == "auto" and ready(dest):
        return dest

    cache = maps / "winver"
    spec = symbol_info(cache / ".index", name, MACHINES[arch], build, offline)
    if spec and install_dll(dest, cache / build / arch / name, *spec, offline):
        return dest
    return None


def fixture_imports(root, test_dir):
    # Raw shellcode has no import table. Both Windows loaders need the core DLLs at startup.
    roots = {arch: {"ntdll.dll", "kernel32.dll", "kernelbase.dll"} for arch in MACHINES}
    bundled = {arch: set() for arch in MACHINES}
    for folder in (test_dir, root / "crates/libmwemu/src/tests/fixtures"):
        if not folder.is_dir():
            continue
        for path in sorted(folder.rglob("*")):
            if path.suffix.lower() not in (".exe", ".dll", ".bin") or not path.is_file():
                continue
            try:
                image = image_imports(path)
            except ValueError as error:
                # Malformed images can themselves be parser-test inputs.
                print(f"Skipping import discovery: {error}", file=sys.stderr)
                continue
            if image:
                machine, names = image
                for arch, value in MACHINES.items():
                    if value == machine:
                        roots[arch].update(names)
                        if path.suffix.lower() == ".dll":
                            bundled[arch].add(path.name.lower())
    return roots, bundled


def windows_maps(root, source, offline, test_dir=None, extra_dlls=(), build=CACHE_BUILD):
    maps = root / "maps"
    roots, bundled = fixture_imports(root, test_dir or root / "test")
    missing = []
    total = 0
    for arch, pending in roots.items():
        pending.update(extra_dlls)
        visited = set()
        while pending:
            name = min(pending)
            pending.remove(name)
            if name.startswith("api-ms-win-crt-"):
                name = "ucrtbase.dll"
            elif name.startswith(("api-ms-", "ext-ms-")):
                continue  # Virtual API sets do not have separate DLLs to install.
            if name in visited or name in bundled[arch]:
                continue
            visited.add(name)
            total += 1
            path = prepare_dll(maps, arch, name, source, build, offline)
            if path is None:
                missing.append(maps / "windows" / arch / name)
                continue
            image = image_imports(path)
            if not image or image[0] != MACHINES[arch]:
                raise ValueError(f"Expected a {arch} DLL: {path}")
            pending.update(image[1] - visited)

    # The Windows loader reads these NLS tables as data. They have no PE imports.
    sources = [maps / "winver" / build / "x86_64", maps / "iso/x86_64", maps / "iso/x86"]
    if source == "host" or (os.name == "nt" and "SystemRoot" in os.environ):
        sources.insert(0, system_folder("x86_64"))
    for name in NLS:
        dest = maps / "windows/x86_64" / name
        if not ready(dest):
            for directory in sources:
                if ready(directory / name):
                    with (directory / name).open("rb") as stream:
                        install(stream, dest)
                    break
            else:
                if source != "host" and not offline:
                    print(f"Optional NLS table unavailable: {dest}")
                    continue
                missing.append(dest)
        total += 1
    return sorted(missing), total


def image_imports(path):
    with path.open("rb") as stream:
        if stream.read(2) != b"MZ":
            return None
        stream.seek(0)
        try:
            return pe_imports(stream.read())
        except (struct.error, ValueError) as error:
            raise ValueError(f"Invalid PE metadata in {path}: {error}") from error


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("group", nargs="?", choices=("maps", "test-fixtures"))
    parser.add_argument("--offline", action="store_true", help="use local files and report missing assets")
    parser.add_argument("--test-dir", type=Path, default=ROOT / os.environ.get("MWEMU_TEST_DIR", "test"),
                        help="fixture directory for extraction and DLL discovery (default: MWEMU_TEST_DIR or test/ in the checkout)")
    parser.add_argument("--source", choices=("host", "symbol-server"), help="source of Windows DLLs")
    parser.add_argument("--dll", action="append", default=[], type=str.lower, help="also prepare a dynamically loaded DLL")
    parser.add_argument("--build", help=f"Windows build for symbol lookup (default: {CACHE_BUILD})")
    args = parser.parse_args()
    if args.source == "host" and args.build:
        parser.error("--build selects symbol-server DLLs")
    if not re.fullmatch(r"[0-9]+\.[0-9]+", args.build or CACHE_BUILD) or any(not DLL_NAME.fullmatch(n) for n in args.dll):
        parser.error("expected a Windows build such as 26100.7920 and plain DLL basenames")
    if (args.source or args.dll or args.build) and args.group == "test-fixtures":
        parser.error("--source, --dll and --build apply to maps")
    host = os.name == "nt" and platform.machine().lower() in ("amd64", "x86_64")
    source = args.source or ("host" if host else "auto")
    if args.build:
        source = "symbol-server"
    if source == "host" and not host:
        parser.error("--source host requires x64 Windows")

    groups = (args.group,) if args.group else ("test-fixtures", "maps")
    complete = True
    for group in groups:
        if group == "test-fixtures":
            result = test_fixtures(args.test_dir, args.offline)
            if result is None:
                continue
            missing, total = result
        else:
            missing, total = windows_maps(ROOT, source, args.offline, args.test_dir, args.dll, args.build or CACHE_BUILD)
        print(f"{group}: {total - len(missing)} ready, {len(missing)} missing")
        for path in missing:
            print(f"  missing {path}")
        complete = complete and not missing
    return 0 if complete or args.offline else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, BadZipFile, RuntimeError) as error:
        sys.exit(f"sync-assets: {error}")
