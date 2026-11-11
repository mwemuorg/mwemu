#!/usr/bin/env python3
"""Reject Windows API sources absent from the current Rust compilation."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys


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


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target")
    parser.add_argument("--offline", action="store_true")
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
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
