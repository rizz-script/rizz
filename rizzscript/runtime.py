from __future__ import annotations

import asyncio
import json
import os
import re
from dataclasses import dataclass
from typing import Any, Awaitable, Callable

try:
    import aiohttp  # type: ignore
except Exception:  # pragma: no cover
    aiohttp = None

from rizzscript import ast


class RizzRuntimeError(Exception):
    pass


@dataclass
class _Frame:
    return_value: Any = None


class Env:
    def __init__(self, parent: "Env | None" = None) -> None:
        self.parent = parent
        self.values: dict[str, Any] = {}
        self.consts: set[str] = set()

    def define(self, name: str, value: Any, *, is_const: bool = False) -> None:
        self.values[name] = value
        if is_const:
            self.consts.add(name)

    def get(self, name: str) -> Any:
        if name in self.values:
            return self.values[name]
        if self.parent is not None:
            return self.parent.get(name)
        raise RizzRuntimeError(f"Undefined variable: {name}")

    def set(self, name: str, value: Any) -> None:
        if name in self.values:
            if name in self.consts:
                raise RizzRuntimeError(f"Cannot assign to constant: {name}")
            self.values[name] = value
            return
        if self.parent is not None:
            self.parent.set(name, value)
            return
        raise RizzRuntimeError(f"Undefined variable: {name}")


@dataclass
class UserFunction:
    name: str
    params: list[str]
    body: ast.Block
    closure: Env
    is_async: bool

    async def __call__(self, rt: "Runtime", args: list[Any]) -> Any:
        if len(args) != len(self.params):
            raise RizzRuntimeError(f"{self.name}() expected {len(self.params)} args, got {len(args)}")
        env = Env(parent=self.closure)
        for k, v in zip(self.params, args, strict=True):
            env.define(k, v)
        frame = _Frame(return_value=None)
        await rt.exec_block(self.body, env, frame)
        return frame.return_value


class Runtime:
    def __init__(self, filename: str = "<input>") -> None:
        self.filename = filename
        self.globals = Env()
        self._tasks: list[asyncio.Task[Any]] = []
        self._session: "aiohttp.ClientSession | None" = None
        self._install_builtins()

    async def _get_session(self) -> "aiohttp.ClientSession":
        if aiohttp is None:
            raise RizzRuntimeError("HTTP requires aiohttp. Install with: python -m pip install -e .")
        if self._session is None or self._session.closed:
            self._session = aiohttp.ClientSession()
        return self._session

    def _install_builtins(self) -> None:
        self.globals.define("Chill", _Builtin("Chill", self._b_chill))
        self.globals.define("Spit", _Builtin("Spit", self._b_spit))
        self.globals.define("Yeet", _Builtin("Yeet", self._b_yeet))
        self.globals.define("Flex", _Builtin("Flex", self._b_flex))
        self.globals.define("Ghost", _Builtin("Ghost", self._b_ghost))
        self.globals.define("Decode", _Builtin("Decode", self._b_decode))
        self.globals.define("Encode", _Builtin("Encode", self._b_encode))
        self.globals.define("Hunt", _Builtin("Hunt", self._b_hunt))
        self.globals.define("Swap", _Builtin("Swap", self._b_swap))
        self.globals.define("Matches", _Builtin("Matches", self._b_matches))
        self.globals.define("Split", _Builtin("Split", self._b_split))
        self.globals.define("Snag", _Builtin("Snag", self._b_snag))
        self.globals.define("Stash", _Builtin("Stash", self._b_stash))
        self.globals.define("KeepAdding", _Builtin("KeepAdding", self._b_keepadding))
        self.globals.define("Trash", _Builtin("Trash", self._b_trash))
        self.globals.define("FileExists", _Builtin("FileExists", self._b_fileexists))
        self.globals.define("Listen", _Builtin("Listen", self._b_listen))
        self.globals.define("Holla", _Builtin("Holla", self._b_holla))
        self.globals.define("Peek", _Builtin("Peek", self._b_peek))
        self.globals.define("Whisper", _Builtin("Whisper", self._b_whisper))
        self.globals.define("Dip", _Builtin("Dip", self._b_dip))

    async def exec_program(self, program: ast.Program) -> None:
        frame = _Frame(return_value=None)
        try:
            for s in program.statements:
                await self.exec_stmt(s, self.globals, frame)
        finally:
            if self._tasks:
                await asyncio.gather(*self._tasks, return_exceptions=False)
            if self._session is not None and not self._session.closed:
                await self._session.close()

    async def exec_block(self, block: ast.Block, env: Env, frame: _Frame) -> None:
        for s in block.statements:
            await self.exec_stmt(s, env, frame)

    async def exec_stmt(self, s: ast.Stmt, env: Env, frame: _Frame) -> None:
        if isinstance(s, ast.VarDecl):
            env.define(s.name, await self.eval_expr(s.value, env, frame))
            return
        if isinstance(s, ast.ConstDecl):
            env.define(s.name, await self.eval_expr(s.value, env, frame), is_const=True)
            return
        if isinstance(s, ast.Assign):
            env.set(s.name, await self.eval_expr(s.value, env, frame))
            return
        if isinstance(s, ast.FuncDef):
            fn = UserFunction(
                name=s.name,
                params=s.params,
                body=s.body,
                closure=env,
                is_async=s.is_async,
            )
            env.define(s.name, fn, is_const=True)
            return
        if isinstance(s, ast.RizzStmt):
            val = await self.eval_expr(s.value, env, frame)
            frame.return_value = val
            print(_stringify(val))
            return
        if isinstance(s, ast.CringeStmt):
            val = await self.eval_expr(s.value, env, frame)
            raise RizzRuntimeError(str(val))
        if isinstance(s, ast.ExprStmt):
            await self.eval_expr(s.expr, env, frame)
            return
        if isinstance(s, ast.IfChain):
            if _truthy(await self.eval_expr(s.cond, env, frame)):
                await self.exec_block(s.then_block, Env(parent=env), frame)
                return
            for cond, blk in s.elifs:
                if _truthy(await self.eval_expr(cond, env, frame)):
                    await self.exec_block(blk, Env(parent=env), frame)
                    return
            if s.else_block is not None:
                await self.exec_block(s.else_block, Env(parent=env), frame)
            return
        if isinstance(s, ast.ForIn):
            it = await self.eval_expr(s.iterable, env, frame)
            if not isinstance(it, (list, tuple, range)):
                raise RizzRuntimeError("Crazy ... in ... expects an array/range")
            for v in list(it):
                loop_env = Env(parent=env)
                loop_env.define(s.var, v)
                await self.exec_block(s.body, loop_env, frame)
            return
        if isinstance(s, ast.WhileLoop):
            while _truthy(await self.eval_expr(s.cond, env, frame)):
                await self.exec_block(s.body, Env(parent=env), frame)
            return

        raise RizzRuntimeError(f"Unhandled statement: {type(s).__name__}")

    async def eval_expr(self, e: ast.Expr, env: Env, frame: _Frame) -> Any:
        if isinstance(e, ast.Literal):
            return e.value
        if isinstance(e, ast.Ident):
            return env.get(e.name)
        if isinstance(e, ast.ArrayLit):
            return [await self.eval_expr(x, env, frame) for x in e.items]
        if isinstance(e, ast.ObjectLit):
            out: dict[str, Any] = {}
            for k, v in e.items:
                if isinstance(k, str):
                    key = k
                else:
                    key = await self.eval_expr(k, env, frame)
                out[str(key)] = await self.eval_expr(v, env, frame)
            return out
        if isinstance(e, ast.Unary):
            v = await self.eval_expr(e.expr, env, frame)
            if e.op == "!":
                return not _truthy(v)
            if e.op == "-":
                return -v
            raise RizzRuntimeError(f"Unknown unary op: {e.op}")
        if isinstance(e, ast.Binary):
            l = await self.eval_expr(e.left, env, frame)
            r = await self.eval_expr(e.right, env, frame)
            return _binop(e.op, l, r)
        if isinstance(e, ast.Ternary):
            if _truthy(await self.eval_expr(e.cond, env, frame)):
                return await self.eval_expr(e.if_true, env, frame)
            return await self.eval_expr(e.if_false, env, frame)
        if isinstance(e, ast.Member):
            obj = await self.eval_expr(e.obj, env, frame)
            if isinstance(obj, dict):
                return obj.get(e.name)
            return getattr(obj, e.name)
        if isinstance(e, ast.Index):
            obj = await self.eval_expr(e.obj, env, frame)
            idx = await self.eval_expr(e.index, env, frame)
            return obj[idx]
        if isinstance(e, ast.Call):
            callee = await self.eval_expr(e.callee, env, frame)
            args = [await self.eval_expr(a, env, frame) for a in e.args]
            res = _call(self, callee, args)
            if asyncio.iscoroutine(res) or isinstance(res, Awaitable):
                return await res  # implicit await
            return res
        if isinstance(e, ast.Vibe):
            task = asyncio.create_task(self.eval_expr(e.expr, env, frame))
            self._tasks.append(task)
            return task
        if isinstance(e, ast.Attempt):
            try:
                await self.exec_block(e.try_block, Env(parent=env), frame)
                return frame.return_value
            except Exception as ex:
                catch_env = Env(parent=env)
                catch_env.define(e.err_name, str(ex))
                await self.exec_block(e.catch_block, catch_env, frame)
                return frame.return_value

        raise RizzRuntimeError(f"Unhandled expression: {type(e).__name__}")

    # --- builtins ---
    async def _b_chill(self, args: list[Any]) -> Any:
        if len(args) != 1:
            raise RizzRuntimeError("Chill(task) expects 1 arg")
        x = args[0]
        if isinstance(x, _ServerHandle):
            await x.server.serve_forever()
            return None
        if asyncio.isfuture(x) or asyncio.iscoroutine(x):
            return await x
        return x

    async def _http(self, method: str, url: str, data: Any = None, headers: Any = None) -> Any:
        sess = await self._get_session()
        hdrs = None
        if isinstance(headers, dict):
            hdrs = {str(k): str(v) for k, v in headers.items()}

        json_data = None
        body_data = None
        if data is not None:
            if isinstance(data, (dict, list)):
                json_data = data
            else:
                body_data = str(data).encode("utf-8")

        async with sess.request(method, url, json=json_data, data=body_data, headers=hdrs) as resp:
            text = await resp.text()
            return {"status": resp.status, "body": text, "headers": dict(resp.headers)}

    async def _b_spit(self, args: list[Any]) -> Any:
        if not (1 <= len(args) <= 2):
            raise RizzRuntimeError("Spit(url, headers?)")
        url = str(args[0])
        headers = args[1] if len(args) == 2 else None
        return await self._http("GET", url, headers=headers)

    async def _b_yeet(self, args: list[Any]) -> Any:
        if not (2 <= len(args) <= 3):
            raise RizzRuntimeError("Yeet(url, data, headers?)")
        url = str(args[0])
        data = args[1]
        headers = args[2] if len(args) == 3 else None
        return await self._http("POST", url, data=data, headers=headers)

    async def _b_flex(self, args: list[Any]) -> Any:
        if not (2 <= len(args) <= 3):
            raise RizzRuntimeError("Flex(url, data, headers?)")
        url = str(args[0])
        data = args[1]
        headers = args[2] if len(args) == 3 else None
        return await self._http("PUT", url, data=data, headers=headers)

    async def _b_ghost(self, args: list[Any]) -> Any:
        if not (1 <= len(args) <= 2):
            raise RizzRuntimeError("Ghost(url, headers?)")
        url = str(args[0])
        headers = args[1] if len(args) == 2 else None
        return await self._http("DELETE", url, headers=headers)

    async def _b_decode(self, args: list[Any]) -> Any:
        if len(args) != 1:
            raise RizzRuntimeError("Decode(jsonString)")
        return json.loads(str(args[0]))

    async def _b_encode(self, args: list[Any]) -> Any:
        if len(args) != 1:
            raise RizzRuntimeError("Encode(obj)")
        return json.dumps(args[0], separators=(",", ":"), ensure_ascii=False)

    async def _b_hunt(self, args: list[Any]) -> Any:
        if len(args) != 2:
            raise RizzRuntimeError("Hunt(text, pattern)")
        text = str(args[0])
        pat = _regex_pattern(args[1])
        return re.findall(pat, text)

    async def _b_swap(self, args: list[Any]) -> Any:
        if len(args) != 3:
            raise RizzRuntimeError("Swap(text, pattern, replacement)")
        text = str(args[0])
        pat = _regex_pattern(args[1])
        repl = str(args[2])
        return re.sub(pat, repl, text)

    async def _b_matches(self, args: list[Any]) -> Any:
        if len(args) != 2:
            raise RizzRuntimeError("Matches(text, pattern)")
        text = str(args[0])
        pat = _regex_pattern(args[1])
        return re.search(pat, text) is not None

    async def _b_split(self, args: list[Any]) -> Any:
        if len(args) != 2:
            raise RizzRuntimeError("Split(text, pattern)")
        text = str(args[0])
        pat = _regex_pattern(args[1])
        return re.split(pat, text)

    async def _b_snag(self, args: list[Any]) -> Any:
        if len(args) != 1:
            raise RizzRuntimeError("Snag(path)")
        path = str(args[0])
        return await asyncio.to_thread(lambda: open(path, "r", encoding="utf-8").read())

    async def _b_stash(self, args: list[Any]) -> Any:
        if len(args) != 2:
            raise RizzRuntimeError("Stash(path, text)")
        path = str(args[0])
        text = str(args[1])
        await asyncio.to_thread(lambda: open(path, "w", encoding="utf-8").write(text))
        return None

    async def _b_keepadding(self, args: list[Any]) -> Any:
        if len(args) != 2:
            raise RizzRuntimeError("KeepAdding(path, text)")
        path = str(args[0])
        text = str(args[1])
        await asyncio.to_thread(lambda: open(path, "a", encoding="utf-8").write(text))
        return None

    async def _b_trash(self, args: list[Any]) -> Any:
        if len(args) != 1:
            raise RizzRuntimeError("Trash(path)")
        path = str(args[0])
        await asyncio.to_thread(lambda: os.remove(path))
        return None

    async def _b_fileexists(self, args: list[Any]) -> Any:
        if len(args) != 1:
            raise RizzRuntimeError("FileExists(path)")
        return os.path.exists(str(args[0]))

    async def _b_listen(self, args: list[Any]) -> Any:
        if len(args) != 2:
            raise RizzRuntimeError("Listen(port, handler)")
        port = int(args[0])
        handler = args[1]

        async def client_connected(reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None:
            sock = {"reader": reader, "writer": writer}
            res = _call(self, handler, [sock])
            if asyncio.iscoroutine(res):
                await res

        server = await asyncio.start_server(client_connected, host="0.0.0.0", port=port)
        return _ServerHandle(server=server)

    async def _b_holla(self, args: list[Any]) -> Any:
        if len(args) != 2:
            raise RizzRuntimeError("Holla(host, port)")
        host = str(args[0])
        port = int(args[1])
        reader, writer = await asyncio.open_connection(host, port)
        return {"reader": reader, "writer": writer}

    async def _b_peek(self, args: list[Any]) -> Any:
        if len(args) != 1:
            raise RizzRuntimeError("Peek(socket)")
        sock = args[0]
        reader: asyncio.StreamReader = sock["reader"]
        data = await reader.read(4096)
        return data.decode("utf-8", errors="replace")

    async def _b_whisper(self, args: list[Any]) -> Any:
        if len(args) != 2:
            raise RizzRuntimeError("Whisper(socket, text)")
        sock = args[0]
        writer: asyncio.StreamWriter = sock["writer"]
        msg = str(args[1]).encode("utf-8")
        writer.write(msg)
        await writer.drain()
        return None

    async def _b_dip(self, args: list[Any]) -> Any:
        if len(args) != 1:
            raise RizzRuntimeError("Dip(socket)")
        sock = args[0]
        writer: asyncio.StreamWriter = sock["writer"]
        writer.close()
        try:
            await writer.wait_closed()
        except Exception:
            pass
        return None


@dataclass
class _ServerHandle:
    server: asyncio.AbstractServer


@dataclass
class _Builtin:
    name: str
    fn: Callable[[list[Any]], Awaitable[Any]]

    def __call__(self, rt: Runtime, args: list[Any]) -> Awaitable[Any]:
        return self.fn(args)


def _call(rt: Runtime, callee: Any, args: list[Any]) -> Any:
    if isinstance(callee, _Builtin):
        return callee(rt, args)
    if isinstance(callee, UserFunction):
        return callee(rt, args)
    if callable(callee):
        return callee(*args)
    raise RizzRuntimeError(f"Cannot call: {callee!r}")


def _truthy(v: Any) -> bool:
    return bool(v)


def _regex_pattern(p: Any) -> str:
    if isinstance(p, tuple) and len(p) == 2 and p[0] == "regex":
        return str(p[1])
    return str(p)


def _binop(op: str, l: Any, r: Any) -> Any:
    if op == "&&":
        return _truthy(l) and _truthy(r)
    if op == "||":
        return _truthy(l) or _truthy(r)
    if op == "==":
        return l == r
    if op == "!=":
        return l != r
    if op == ">":
        return l > r
    if op == "<":
        return l < r
    if op == ">=":
        return l >= r
    if op == "<=":
        return l <= r

    if op == "..":
        a = int(l)
        b = int(r)
        if a <= b:
            return list(range(a, b + 1))
        return list(range(a, b - 1, -1))

    if op == "+":
        if isinstance(l, str) or isinstance(r, str):
            return str(l) + str(r)
        if isinstance(l, list) and isinstance(r, list):
            return l + r
        return l + r
    if op == "-":
        return l - r
    if op == "*":
        return l * r
    if op == "/":
        return l / r
    if op == "%":
        return l % r
    raise RizzRuntimeError(f"Unknown operator: {op}")


def _stringify(v: Any) -> str:
    if isinstance(v, (dict, list)):
        try:
            return json.dumps(v, ensure_ascii=False)
        except Exception:
            return str(v)
    return str(v)

