#!/usr/bin/env python3
"""Check API, syscall, and kernel sources, plus Windows API module style."""
import argparse
import hashlib
import json
import re
from functools import lru_cache
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SOURCE_MODULES = ("api", "kernel", "syscall")


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


def orphan_sources(source_roots, compiled):
    for source_root in source_roots:
        if (source_root / "mod.rs").resolve() not in compiled:
            raise ValueError(f"dependency files do not contain the root module: {source_root}")
    return sorted(path.resolve() for source_root in source_roots
                  for path in source_root.rglob("*.rs")
                  if path.resolve() not in compiled)


def ignored_sources(root, sources):
    """Check ignore rules even for files already present in the Git index."""
    if not (root / ".git").exists():
        return None
    names = [path.relative_to(root).as_posix() for path in sources]
    if not names:
        return []
    result = subprocess.run(
        ["git", "check-ignore", "--no-index", "-z", "--stdin"],
        cwd=root, input="\0".join(names).encode("utf-8") + b"\0",
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    )
    if result.returncode not in (0, 1):
        raise ValueError(result.stderr.decode("utf-8", errors="replace").strip())
    return [root / name.decode("utf-8")
            for name in result.stdout.split(b"\0") if name]


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


@lru_cache(maxsize=None)
def crate_module_aliases(source):
    root = source / "lib.rs"
    if not root.is_file():
        return {}
    pattern = r"(?m)^(?:pub(?:\([^)]*\))?\s+)?use\s+(?:crate::)?([\w:]+)\s+as\s+(\w+)\s*;"
    return {alias: target.split("::") for target, alias in
            re.findall(pattern, root.read_text(encoding="utf-8"))}


def aggregating_modules(path, text):
    directory = path.parent if path.name == "mod.rs" else path.with_suffix("")
    declarations = r'(?m)^(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;'
    candidates = {name: directory / name for name in re.findall(declarations, text)}
    source = next((parent for parent in path.parents if parent.name == "src"), None)
    if source is not None:
        for target in re.findall(r'(?m)^pub use (crate::[\w:]+)::\*;', text):
            first, *rest = target.split("::")[1:]
            parts = crate_module_aliases(source).get(first, [first])
            candidates[target] = source.joinpath(*parts, *rest)
    aggregators = set()
    for name, stem in candidates.items():
        child = stem.with_suffix(".rs")
        if not child.is_file():
            child = stem / "mod.rs"
        if not child.is_file():
            continue
        content = child.read_text(encoding="utf-8")
        # A facade can re-export handlers from sibling modules without declaring children.
        public_definition = r"(?m)^pub\s+(?!(?:use|mod)\b)\w+"
        if re.search(r"(?m)^pub use ", content) and (
            re.search(declarations, content) or not re.search(public_definition, content)
        ):
            aggregators.add(name)
    return aggregators


def module_style_issues(text, aggregators=()):
    """Check module declarations, not ordinary wildcard imports of constants."""
    issues = []
    for match in re.finditer(r'(?m)^\s*#\[path\s*=', text):
        issues.append((text.count("\n", 0, match.start()) + 1,
                       "place the source beside its owning module instead of using #[path]"))
    groups = {}
    for match in re.finditer(r'(?m)^[ \t]*(pub(?:\([^)]*\))?)\s+use\s+([^;]+);', text):
        visibility, tree = match.groups()
        line = text.count("\n", 0, match.start()) + 1
        if "*" in tree:
            target = re.fullmatch(r'\s*([\w:]+)\s*::\s*\*\s*', tree)
            if not target or target[1] not in aggregators:
                issues.append((line, "list handler names explicitly; wildcard re-exports require a subsystem module"))
        prefix = tree.split("{", 1)[0].rsplit("::", 1)[0].strip()
        if "::" not in tree or tree.lstrip().startswith("{"):
            continue
        key = (visibility, re.sub(r'\s+', '', prefix))
        if key in groups:
            issues.append((line, f"combine re-exports from {prefix} with line {groups[key]}"))
        groups[key] = line
    return issues


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
            self.assertEqual(orphan_sources([root], {active.resolve()}), [orphan.resolve()])
            self.assertEqual(orphan_sources([root], {active.resolve(), orphan.resolve()}), [])

    def test_each_entry_tree_reports_unconnected_sources(self):
        with tempfile.TemporaryDirectory() as folder:
            roots = [Path(folder) / name for name in SOURCE_MODULES]
            compiled, orphans = set(), []
            for root in roots:
                root.mkdir()
                entry, orphan = root / "mod.rs", root / "unconnected.rs"
                entry.touch()
                orphan.touch()
                compiled.add(entry.resolve())
                orphans.append(orphan.resolve())
            self.assertEqual(orphan_sources(roots, compiled), sorted(orphans))
            self.assertEqual(orphan_sources(roots, compiled | set(orphans)), [])
            for root in roots:
                with self.subTest(missing=root.name), self.assertRaises(ValueError):
                    orphan_sources(roots, compiled - {(root / "mod.rs").resolve()})

    def test_git_ignored_sources_are_checked_before_and_after_staging(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            source = root / "src" / "api" / "autoplay prompt.rs"
            source.parent.mkdir(parents=True)
            source.touch()
            self.assertIsNone(ignored_sources(root, [source]))
            subprocess.run(["git", "init", "--quiet"], cwd=root, check=True)
            ignore = root / ".gitignore"
            ignore.write_text("*prompt*\n", encoding="utf-8")
            self.assertEqual(ignored_sources(root, [source]), [source])
            subprocess.run(
                ["git", "add", "--force", "--", source.relative_to(root).as_posix()],
                cwd=root, check=True,
            )
            self.assertEqual(ignored_sources(root, [source]), [source])
            ignore.write_text("*prompt*\n!src/**/*.rs\n", encoding="utf-8")
            self.assertEqual(ignored_sources(root, [source]), [])

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


    def test_module_style_requires_explicit_grouped_reexports_and_normal_paths(self):
        good = 'mod text;\npub use text::{ApiA, ApiW};\nuse constants::*;\n'
        self.assertEqual(module_style_issues(good), [])
        self.assertEqual(module_style_issues("pub use strings_nls::*;", {"strings_nls"}), [])
        self.assertTrue(module_style_issues("pub use text::*;", {"strings_nls"}))
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            (root / "strings_nls").mkdir()
            (root / "strings_nls/mod.rs").write_text("mod text;\npub use text::{ApiA, ApiW};\n", encoding="utf-8")
            (root / "text.rs").write_text("pub fn ApiA() {}\n", encoding="utf-8")
            declarations = "mod strings_nls;\nmod text;\n"
            self.assertEqual(aggregating_modules(root / "mod.rs", declarations), {"strings_nls"})
        with tempfile.TemporaryDirectory() as folder:
            source = Path(folder) / "src"
            shared = source / "api" / "shared"
            shared.mkdir(parents=True)
            (shared / "mod.rs").write_text("mod text;\npub use text::ApiA;\n", encoding="utf-8")
            (shared / "text.rs").write_text("pub fn ApiA() {}\n", encoding="utf-8")
            caller = source / "api" / "windows" / "mod.rs"
            public = "pub use crate::api::shared::*;"
            self.assertEqual(module_style_issues(public, aggregating_modules(caller, public)), [])
            leaf = "pub use crate::api::shared::text::*;"
            self.assertTrue(module_style_issues(leaf, aggregating_modules(caller, leaf)))
        with tempfile.TemporaryDirectory() as folder:
            source = Path(folder) / "src"
            shared = source / "api" / "windows" / "shared"
            shared.mkdir(parents=True)
            (source / "lib.rs").write_text("pub(crate) use api::windows as winapi;\n", encoding="utf-8")
            (shared / "exports.rs").write_text("pub use super::text::{ApiA, ApiW};\n", encoding="utf-8")
            (shared / "text.rs").write_text("pub use super::other::ApiW;\npub fn ApiA() {}\n", encoding="utf-8")
            caller = source / "caller.rs"
            for module, issues in [("exports", False), ("text", True), ("missing", True)]:
                statement = f"pub use crate::winapi::shared::{module}::*;"
                with self.subTest(module=module):
                    self.assertEqual(bool(module_style_issues(statement, aggregating_modules(caller, statement))), issues)
        for bad in ['pub use text::*;\n',
                    'pub use text::ApiA;\npub use text::ApiW;\n',
                    '#[path = "../text.rs"]\nmod text;\n']:
            with self.subTest(source=bad):
                self.assertTrue(module_style_issues(bad))


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
        source_base = root / "crates/libmwemu/src"
        source_roots = [source_base / name for name in SOURCE_MODULES]
        sources = sorted(path for source_root in source_roots
                         for path in source_root.rglob("*.rs"))
        style_root = source_base / "api/windows"
        ignored = ignored_sources(root, sources)
        if ignored:
            names = "\n".join(path.relative_to(root).as_posix() for path in ignored)
            raise ValueError(f"Rust sources are ignored by Git:\n{names}")
        dep_files = current_compilation(root, args.target, args.offline)
        compiled = set().union(*(dependencies(path, root) for path in dep_files))
        orphans = orphan_sources(source_roots, compiled)
        style_issues = [
            {"path": path.relative_to(root).as_posix(), "line": line, "message": message}
            for path in sorted(style_root.rglob("*.rs"))
            for text in [path.read_text(encoding="utf-8")]
            for line, message in module_style_issues(text, aggregating_modules(path, text))
        ]
    except (OSError, ValueError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2
    report = {
        "dependency_files": {str(path): hashlib.sha256(path.read_bytes()).hexdigest()
                             for path in sorted(dep_files)},
        "source_roots": [path.relative_to(root).as_posix() for path in source_roots],
        "style_root": style_root.relative_to(root).as_posix(),
        "api_sources": len(sources),
        "ignored_sources": None if ignored is None else [],
        "orphans": [path.relative_to(root).as_posix() for path in orphans],
        "module_style": style_issues,
    }
    if args.report:
        args.report.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    if ignored is None:
        print("SKIP: Git ignore rules are not checked outside a Git checkout")
    for path in report["orphans"]:
        print(f"UNCONNECTED: {path}")
    for issue in style_issues:
        print(f"MODULE: {issue['path']}:{issue['line']}: {issue['message']}")
    if style_issues:
        print(f"FAIL: {len(style_issues)} Windows API module declarations need cleanup")
        return 1
    if orphans:
        print(f"FAIL: {len(orphans)} API, syscall, or kernel source files are not compiled")
        return 1
    print(f"OK: all {report['api_sources']} API, syscall, and kernel source files are compiled")
    return 0


if __name__ == "__main__":
    sys.exit(main())
