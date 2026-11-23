"""Read the import names and machine type of Windows PE images."""

import re
from struct import unpack_from


MACHINES = {"x86": 0x014C, "x86_64": 0x8664}
DLL_NAME = re.compile(r"[a-z0-9_][a-z0-9_.-]*\.dll")


def pe_imports(data):
    """Read named imports from a PE32 or PE32+ image without loading it."""
    if data[:2] != b"MZ" or len(data) < 64:
        return None
    pe = unpack_from("<I", data, 60)[0]
    if data[pe:pe + 4] != b"PE\0\0":
        return None
    machine, sections = unpack_from("<HH", data, pe + 4)
    if machine not in MACHINES.values():
        return None
    optional_length = unpack_from("<H", data, pe + 20)[0]
    optional = pe + 24
    magic = unpack_from("<H", data, optional)[0]
    directory = {0x10B: 96, 0x20B: 112}.get(magic)
    if directory is None:
        return None
    if optional_length < directory + 16 or unpack_from("<I", data, optional + directory - 4)[0] < 2:
        return machine, set()
    imports, size = unpack_from("<II", data, optional + directory + 8)
    if not imports or not size:
        return machine, set()
    ranges = []
    for i in range(sections):
        section = optional + optional_length + i * 40
        rva, raw_size, raw = unpack_from("<III", data, section + 12)
        ranges.append((rva, raw, min(raw_size, max(0, len(data) - raw))))

    def offset(rva, length):
        for start, raw, available in ranges:
            if start <= rva and rva - start + length <= available:
                return raw + rva - start
        raise ValueError("import directory refers outside PE sections")

    names = set()
    for pos in range(0, size - 19, 20):
        descriptor = offset(imports + pos, 20)
        name_rva = unpack_from("<I", data, descriptor + 12)[0]
        if not name_rva:
            break
        start = offset(name_rva, 1)
        end = data.find(b"\0", start, start + 256)
        if end < 0:
            raise ValueError("unterminated PE import name")
        offset(name_rva, end - start + 1)
        name = data[start:end].decode("ascii").lower()
        if DLL_NAME.fullmatch(name):
            names.add(name)
    return machine, names
