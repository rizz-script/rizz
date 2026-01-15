from __future__ import annotations

import argparse
import asyncio
import pathlib
import sys

from rizzscript.parser import parse_program
from rizzscript.runtime import Runtime


def _build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="rizz", description="RizzScript interpreter (v0.1)")
    sub = p.add_subparsers(dest="cmd", required=True)

    run = sub.add_parser("run", help="Run a .rizz file")
    run.add_argument("file", type=str, help="Path to .rizz file")

    return p


async def _run_file(path: pathlib.Path) -> int:
    src = path.read_text(encoding="utf-8")
    program = parse_program(src, filename=str(path))
    rt = Runtime(filename=str(path))
    await rt.exec_program(program)
    return 0


def main(argv: list[str] | None = None) -> None:
    args = _build_parser().parse_args(argv)

    if args.cmd == "run":
        path = pathlib.Path(args.file)
        if not path.exists():
            print(f"rizz: file not found: {path}", file=sys.stderr)
            raise SystemExit(2)

        raise SystemExit(asyncio.run(_run_file(path)))

    raise SystemExit(2)


if __name__ == "__main__":
    main()

