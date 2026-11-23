"""Offline regression tests for the asset installer."""

import gzip
import hashlib
import json
import io
import os
import tempfile
import unittest
from pathlib import Path
from struct import pack_into
from unittest.mock import patch
from zipfile import BadZipFile, ZipFile

import sync_assets as sync


class SyncAssetsTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.fixtures = self.root / "test inputs"

    def archive(self, names):
        data = io.BytesIO()
        with ZipFile(data, "w") as bundle:
            for name in names:
                bundle.writestr(name, b"release")
        data.seek(0)
        return data

    def test_fixture_download_cache_offline_and_local_edits(self):
        missing = [self.fixtures / n for n in ("first", "second")]
        with patch.object(sync, "urlopen", side_effect=AssertionError("unexpected network access")):
            self.assertIsNone(sync.test_fixtures(self.fixtures, True))
        with patch.object(sync, "urlopen", return_value=self.archive(["test/first", "test/second"])) as fetch:
            self.assertEqual(sync.test_fixtures(self.fixtures, False), ([], 2))
            fetch.assert_called_once()
        missing[0].write_bytes(b"local build")
        missing[1].write_bytes(b"")
        with patch.object(sync, "urlopen", side_effect=AssertionError("unexpected network access")):
            self.assertEqual(sync.test_fixtures(self.fixtures, True), ([], 2))
            self.assertEqual(missing[0].read_bytes(), b"local build")
            self.assertEqual(missing[1].read_bytes(), b"release")
            (self.fixtures / ".samples.zip").unlink()
            self.assertIsNone(sync.test_fixtures(self.fixtures, True))
        with patch.object(sync, "urlopen", return_value=self.archive(["test/first", "test/second", "test/new/third"])):
            self.assertEqual(sync.test_fixtures(self.fixtures, False), ([], 3))
        self.assertEqual((self.fixtures / "new/third").read_bytes(), b"release")
        self.assertEqual(missing[0].read_bytes(), b"local build")

    def test_invalid_archive_never_installs_partial_files(self):
        for data, expected in [(self.archive(["test/first", "test/../outside"]), ValueError),
                               (io.BytesIO(b"corrupt"), BadZipFile)]:
            with self.subTest(expected=expected), patch.object(sync, "urlopen", return_value=data):
                with self.assertRaises(expected):
                    sync.test_fixtures(self.fixtures, False)
            self.assertFalse((self.fixtures / "first").exists())
            self.assertFalse((self.fixtures / ".samples.zip").exists())
        (self.fixtures / ".samples.zip").write_bytes(b"corrupt")
        with self.assertRaises(BadZipFile):
            sync.test_fixtures(self.fixtures, True)

    @staticmethod
    def pe(arch, *imports, marker=b""):
        # One section contains the import descriptors followed by their DLL names.
        image = bytearray(1024)
        image[:2] = b"MZ"
        pack_into("<I", image, 60, 64)
        image[64:68] = b"PE\0\0"
        machine = sync.MACHINES[arch]
        magic, directory = (0x10B, 96) if arch == "x86" else (0x20B, 112)
        optional = 88
        optional_size = directory + 16
        pack_into("<HH", image, 68, machine, 1)
        pack_into("<H", image, 84, optional_size)
        pack_into("<H", image, optional, magic)
        pack_into("<I", image, optional + directory - 4, 2)
        rva, raw = 4096, 512
        pack_into("<III", image, optional + optional_size + 12, rva, 512, raw)
        table_size = 20 * (len(imports) + 1)
        pack_into("<II", image, optional + directory + 8, rva, table_size)
        name_pos = table_size
        for i, name in enumerate(imports):
            text = name.encode("ascii") + b"\0"
            pack_into("<I", image, raw + i * 20 + 12, rva + name_pos)
            image[raw + name_pos:raw + name_pos + len(text)] = text
            name_pos += len(text)
        return bytes(image) + marker

    def test_host_discovers_imports_by_architecture(self):
        windows = self.root / "Windows"
        for arch, subdir in (("x86", "SysWOW64"), ("x86_64", "System32")):
            folder = windows / subdir
            folder.mkdir(parents=True)
            for name in ("ntdll.dll", "kernel32.dll", "kernelbase.dll", "child.dll"):
                (folder / name).write_bytes(self.pe(arch))
        native = windows / "System32"
        for name in sync.NLS:
            (native / name).write_bytes(b"nls")
        (native / "parent.dll").write_bytes(self.pe("x86_64", "CHILD.dll"))
        self.fixtures.mkdir()
        (self.fixtures / "guest.bin").write_bytes(self.pe("x86_64", "parent.dll", "private.dll"))
        (self.fixtures / "private.dll").write_bytes(self.pe("x86_64", "child.dll"))
        maps = self.root / "maps/windows"
        with patch.dict(os.environ, SystemRoot=str(windows)), patch.object(sync, "urlopen", side_effect=AssertionError):
            self.assertEqual(sync.windows_maps(self.root, "host", True, self.fixtures), ([], 11))
            self.assertEqual((maps / "x86_64/child.dll").read_bytes(), self.pe("x86_64"))
            self.assertFalse((maps / "x86/child.dll").exists())
            self.assertFalse((maps / "x86_64/private.dll").exists())
            (native / "parent.dll").unlink()
            self.assertEqual(sync.windows_maps(self.root, "auto", True, self.fixtures), ([], 11))

    def test_symbol_selection_cache_and_integrity(self):
        ordinary = self.pe("x86")
        pinned = self.pe("x86", marker=b"component")
        digest = hashlib.sha256(pinned).hexdigest()
        cache = self.root / ".index"
        dest = self.root / "maps/component.dll"
        dest.parent.mkdir()
        dest.write_bytes(ordinary)
        local = self.root / "cached.dll"
        target = {"machineType": sync.MACHINES["x86"], "timestamp": 16, "virtualSize": 4096, "version": "5.82"}
        component = {"assemblyIdentity": {"version": "10.0.26100.10"},
                     "attributes": [{"destinationPath": "$(runtime.system32)\\"}]}
        index = {
            digest: {"fileInfo": target, "windowsVersions": {"11-24H2": {"KB": {"assemblies": {"cmp": component}}}}},
            "f" * 64: {"fileInfo": dict(target, version="10.0.26100.99")},
            "e" * 64: {"fileInfo": dict(target, machineType=sync.MACHINES["x86_64"], version="10.0.26100.12")},
        }
        with patch.object(sync, "urlopen", return_value=io.BytesIO(gzip.compress(json.dumps(index).encode()))) as fetch:
            spec = sync.symbol_info(cache, dest.name, sync.MACHINES["x86"], "26100.12", False)
            self.assertEqual(spec, (digest, "000000101000"))
            fetch.assert_called_once()
        with patch.object(sync, "urlopen", side_effect=AssertionError):
            self.assertEqual(sync.symbol_info(cache, dest.name, sync.MACHINES["x86"], "26100.12", True), spec)
            self.assertFalse(sync.install_dll(dest, local, *spec, True))
        with patch.object(sync, "urlopen", return_value=io.BytesIO(b"bad download")):
            with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
                sync.install_dll(dest, local, *spec, False)
            self.assertEqual(dest.read_bytes(), ordinary)
        with patch.object(sync, "urlopen", return_value=io.BytesIO(pinned)) as fetch:
            self.assertTrue(sync.install_dll(dest, local, *spec, False))
            self.assertIn("/component.dll/000000101000/component.dll", fetch.call_args.args[0])
        dest.unlink()
        local.write_bytes(pinned)
        with patch.object(sync, "urlopen", side_effect=AssertionError):
            self.assertTrue(sync.install_dll(dest, local, *spec, True))
            self.assertEqual(dest.read_bytes(), pinned)


if __name__ == "__main__":
    unittest.main()
