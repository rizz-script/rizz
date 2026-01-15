from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class Span:
    filename: str
    line: int
    col: int


class Node:
    span: Span


@dataclass
class Program(Node):
    span: Span
    statements: list["Stmt"]


class Stmt(Node):
    pass


@dataclass
class Block(Node):
    span: Span
    statements: list[Stmt]


@dataclass
class VarDecl(Stmt):
    span: Span
    name: str
    value: "Expr"


@dataclass
class ConstDecl(Stmt):
    span: Span
    name: str
    value: "Expr"


@dataclass
class Assign(Stmt):
    span: Span
    name: str
    value: "Expr"


@dataclass
class FuncDef(Stmt):
    span: Span
    name: str
    params: list[str]
    is_async: bool
    body: Block


@dataclass
class RizzStmt(Stmt):
    span: Span
    value: "Expr"


@dataclass
class CringeStmt(Stmt):
    span: Span
    value: "Expr"


@dataclass
class ExprStmt(Stmt):
    span: Span
    expr: "Expr"


@dataclass
class IfChain(Stmt):
    span: Span
    cond: "Expr"
    then_block: Block
    elifs: list[tuple["Expr", Block]]
    else_block: Block | None


@dataclass
class ForIn(Stmt):
    span: Span
    var: str
    iterable: "Expr"
    body: Block


@dataclass
class WhileLoop(Stmt):
    span: Span
    cond: "Expr"
    body: Block


class Expr(Node):
    pass


@dataclass
class Ident(Expr):
    span: Span
    name: str


@dataclass
class Literal(Expr):
    span: Span
    value: Any


@dataclass
class ArrayLit(Expr):
    span: Span
    items: list[Expr]


@dataclass
class ObjectLit(Expr):
    span: Span
    items: list[tuple[str | Expr, Expr]]  # key can be identifier-string or expression (e.g. string literal)


@dataclass
class Unary(Expr):
    span: Span
    op: str
    expr: Expr


@dataclass
class Binary(Expr):
    span: Span
    op: str
    left: Expr
    right: Expr


@dataclass
class Ternary(Expr):
    span: Span
    cond: Expr
    if_true: Expr
    if_false: Expr


@dataclass
class Call(Expr):
    span: Span
    callee: Expr
    args: list[Expr]


@dataclass
class Member(Expr):
    span: Span
    obj: Expr
    name: str


@dataclass
class Index(Expr):
    span: Span
    obj: Expr
    index: Expr


@dataclass
class Vibe(Expr):
    span: Span
    expr: Expr


@dataclass
class Attempt(Expr):
    span: Span
    try_block: Block
    err_name: str
    catch_block: Block

