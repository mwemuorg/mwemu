#!/usr/bin/env python3
"""Reject Windows API sources absent from the current Rust compilation."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


def dependencies(dep_info, root):
    """Rust emits a phony make target for each input, including #[path] modules."""
    result = set()
    for line in dep_info.read_text(encoding="utf-8").splitlines():
        if not line.endswith(".rs:"):
            continue
        path = line[:-1].replace(r"\ ", " ").replace(r"\#", "#")
        path = path.replace(r"\:", ":").replace("$$", "$").replace("\\", "/")
        result.add((root / path).resolve())
    if not result:
        raise ValueError(f"no Rust source dependencies in {dep_info}")
    return result


def orphan_sources(source_root, compiled):
    return sorted(path.resolve() for path in source_root.rglob("*.rs")
                  if path.resolve() not in compiled)


def library_dep_info(artifact):
    for filename in artifact["filenames"]:
        path = Path(filename)
        if path.suffix not in (".rmeta", ".rlib"):
            continue
        stem = path.stem
        if stem.startswith("liblibmwemu-"):
            stem = stem[3:]
        dep_info = path.with_name(stem + ".d")
        if dep_info.is_file():
            return dep_info
    raise ValueError("libmwemu artifact has no dependency file")


def current_compilation(root, target=None, offline=False):
    command = ["cargo", "check", "--locked", "--package", "libmwemu", "--all-targets",
               "--message-format=json"]
    if target:
        command += ["--target", target]
    if offline:
        command.append("--offline")
    result = subprocess.run(command, cwd=root, text=True, stdout=subprocess.PIPE)
    artifacts = []
    for line in result.stdout.splitlines():
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        if message.get("reason") == "compiler-message":
            print(message["message"].get("rendered", ""), end="", file=sys.stderr)
        if (message.get("reason") == "compiler-artifact"
                and message["target"]["name"] == "libmwemu"):
            artifacts.append(message)
    if result.returncode:
        raise ValueError("cargo check failed; source inventory was not checked")
    if not artifacts:
        raise ValueError("cargo did not report a libmwemu library artifact")
    return {library_dep_info(artifact) for artifact in artifacts}


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


def run_self_tests():
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(ApiModulesTests)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    return 0 if result.wasSuccessful() else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true",
                        help="test the checker without running Cargo")
    parser.add_argument("--target")
    parser.add_argument("--offline", action="store_true")
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
    if args.self_test:
        if args.target or args.offline or args.report:
            parser.error("--self-test cannot be combined with --target, --offline or --report")
        return run_self_tests()
    root = Path(__file__).resolve().parents[1]
    try:
        dep_files = current_compilation(root, args.target, args.offline)
        compiled = set().union(*(dependencies(path, root) for path in dep_files))
        source_root = root / "crates/libmwemu/src/api/windows"
        if (source_root / "mod.rs").resolve() not in compiled:
            raise ValueError("dependency file does not contain the Windows API root module")
        orphans = orphan_sources(source_root, compiled)
    except (OSError, ValueError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2
    report = {
        "dependency_files": {str(path): hashlib.sha256(path.read_bytes()).hexdigest()
                             for path in sorted(dep_files)},
        "api_sources": len(list(source_root.rglob("*.rs"))),
        "orphans": [path.relative_to(root).as_posix() for path in orphans],
    }
    if args.report:
        args.report.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    for path in report["orphans"]:
        print(f"UNCONNECTED: {path}")
    if orphans:
        print(f"FAIL: {len(orphans)} Windows API source files are not compiled")
        return 1
    print(f"OK: all {report['api_sources']} Windows API source files are compiled")
    return 0


if __name__ == "__main__":
    sys.exit(main())
