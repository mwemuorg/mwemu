#!/usr/bin/env python3
"""Cyclomatic complexity estimator for Rust source files.

Counts branch points (if, else if, match =>, while, for, loop, &&, ||, ?)
inside each function body. CC = 1 + branch_points.

Usage:
    python3 scripts/complexity.py [--threshold N] [--top N] [path ...]

Defaults to scanning crates/libmwemu/src/.
Exit code 1 if any function exceeds the threshold (default: 40).
"""

import argparse
import os
import re
import sys

BRANCH_KEYWORDS = re.compile(
    r'\b(if|else\s+if|while|for|loop)\b'
    r'|=>'
    r'|\?\s*[^?]'
    r'|&&'
    r'|\|\|'
)

FN_START = re.compile(
    r'^\s*(?:pub(?:\s*\(.*?\))?\s+)?(?:unsafe\s+)?(?:async\s+)?fn\s+(\w+)'
)


def measure_file(path):
    results = []
    try:
        with open(path, 'r', errors='replace') as f:
            lines = f.readlines()
    except (OSError, UnicodeDecodeError):
        return results

    i = 0
    while i < len(lines):
        m = FN_START.match(lines[i])
        if not m:
            i += 1
            continue

        fn_name = m.group(1)
        fn_line = i + 1
        depth = 0
        started = False
        branch_count = 0
        fn_lines = 0

        for j in range(i, len(lines)):
            line = lines[j]
            for ch in line:
                if ch == '{':
                    if not started:
                        started = True
                    depth += 1
                elif ch == '}':
                    depth -= 1

            if started:
                fn_lines += 1
                stripped = line.strip()
                if not stripped.startswith('//'):
                    branch_count += len(BRANCH_KEYWORDS.findall(line))

            if started and depth <= 0:
                i = j + 1
                break
        else:
            i = len(lines)

        cc = 1 + branch_count
        results.append((cc, fn_lines, fn_name, path, fn_line))

    return results


def main():
    parser = argparse.ArgumentParser(description='Rust cyclomatic complexity')
    parser.add_argument('paths', nargs='*', default=['crates/libmwemu/src'])
    parser.add_argument('--threshold', type=int, default=40,
                        help='max allowed CC (default: 40)')
    parser.add_argument('--top', type=int, default=50,
                        help='show top N functions (default: 50)')
    parser.add_argument('--all', action='store_true',
                        help='show all functions, not just top N')
    parser.add_argument('--max-cc', type=int, default=900,
                        help='hard ceiling for CC (default: 900)')
    parser.add_argument('--max-lines', type=int, default=1500,
                        help='hard ceiling for function lines (default: 1500)')
    args = parser.parse_args()

    all_results = []
    for root in args.paths:
        if os.path.isfile(root):
            all_results.extend(measure_file(root))
        else:
            for dirpath, _, filenames in os.walk(root):
                for fname in filenames:
                    if fname.endswith('.rs'):
                        fpath = os.path.join(dirpath, fname)
                        all_results.extend(measure_file(fpath))

    all_results.sort(key=lambda x: x[0], reverse=True)

    limit = len(all_results) if args.all else args.top
    shown = all_results[:limit]

    if not shown:
        print("No functions found.")
        return 0

    max_cc_width = max(len(str(r[0])) for r in shown)
    max_lines_width = max(len(str(r[1])) for r in shown)

    print(f"{'CC':>{max_cc_width}}  {'Lines':>{max_lines_width}}  Function")
    print(f"{'--':>{max_cc_width}}  {'-----':>{max_lines_width}}  --------")

    violations = 0
    for cc, fn_lines, fn_name, path, line_no in shown:
        marker = " !!!" if cc > args.threshold else ""
        rel = os.path.relpath(path)
        print(f"{cc:>{max_cc_width}}  {fn_lines:>{max_lines_width}}  {rel}:{line_no} {fn_name}{marker}")
        if cc > args.threshold:
            violations += 1

    total = len(all_results)
    over = sum(1 for r in all_results if r[0] > args.threshold)
    avg = sum(r[0] for r in all_results) / total if total else 0

    print(f"\nTotal functions: {total}")
    print(f"Average CC: {avg:.1f}")
    print(f"Over threshold ({args.threshold}): {over}")

    monsters = [(cc, ln, name, path, lno) for cc, ln, name, path, lno in all_results
                if cc > args.max_cc or ln > args.max_lines]
    if monsters:
        print(f"\nHARD CEILING VIOLATED (CC > {args.max_cc} or lines > {args.max_lines}):")
        for cc, ln, name, path, lno in monsters:
            rel = os.path.relpath(path)
            print(f"  {cc} CC  {ln} lines  {rel}:{lno} {name}")
        return 2

    if over > 0:
        print(f"\nFAIL: {over} function(s) exceed CC threshold of {args.threshold}")
        return 1

    print("\nOK: all functions within threshold")
    return 0


if __name__ == '__main__':
    sys.exit(main())
