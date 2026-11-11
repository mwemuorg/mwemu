import tempfile
import unittest
from pathlib import Path

from check_api_modules import dependencies, library_dep_info, orphan_sources


class ApiModulesTests(unittest.TestCase):
    def test_make_paths_and_path_attributes(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            dep = root / "libmwemu.d"
            dep.write_text("output: ignored\ncrates/api/mod.rs:\n"
                           "crates/api/group/../name\\ with\\ space.rs:\n"
                           "crates\\api\\other.rs:\n", encoding="utf-8")
            self.assertEqual(dependencies(dep, root), {
                (root / "crates/api/mod.rs").resolve(),
                (root / "crates/api/name with space.rs").resolve(),
                (root / "crates/api/other.rs").resolve(),
            })

    def test_orphan_is_reported_and_non_rust_files_are_ignored(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            active, orphan = root / "mod.rs", root / "orphan.rs"
            for path in (active, orphan, root / "reference.h"):
                path.touch()
            self.assertEqual(orphan_sources(root, {active.resolve()}), [orphan.resolve()])
            self.assertEqual(orphan_sources(root, {active.resolve(), orphan.resolve()}), [])

    def test_invalid_dep_info_fails_closed(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            dep = root / "invalid.d"
            dep.write_text("output: no Rust inputs\n", encoding="utf-8")
            with self.assertRaises(ValueError):
                dependencies(dep, root)

    def test_only_the_reported_artifact_dependency_file_is_used(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            stale = root / "libmwemu-old.d"
            stale.touch()
            artifact = {"filenames": [str(root / "liblibmwemu-new.rmeta")]}
            with self.assertRaises(ValueError):
                library_dep_info(artifact)
            current = root / "libmwemu-new.d"
            current.touch()
            self.assertEqual(library_dep_info(artifact), current)


if __name__ == "__main__":
    unittest.main()
