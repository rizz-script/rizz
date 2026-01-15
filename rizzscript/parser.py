from __future__ import annotations

from dataclasses import dataclass

from rizzscript import ast
from rizzscript.lexer import Token, lex


class ParseError(Exception):
    pass


def parse_program(src: str, filename: str = "<input>") -> ast.Program:
    tokens = lex(src)
    p = Parser(tokens=tokens, filename=filename)
    program = p.parse_program()
    return program


@dataclass
class Parser:
    tokens: list[Token]
    filename: str
    i: int = 0

    def cur(self) -> Token:
        return self.tokens[self.i]

    def at(self, kind: str) -> bool:
        return self.cur().kind == kind

    def advance(self) -> Token:
        t = self.cur()
        self.i += 1
        return t

    def span(self, t: Token) -> ast.Span:
        return ast.Span(filename=self.filename, line=t.line, col=t.col)

    def error(self, msg: str) -> ParseError:
        t = self.cur()
        return ParseError(f"{self.filename}:{t.line}:{t.col}: {msg} (got {t.kind} {t.value!r})")

    def expect(self, kind: str) -> Token:
        if not self.at(kind):
            raise self.error(f"Expected {kind}")
        return self.advance()

    def match(self, kind: str) -> Token | None:
        if self.at(kind):
            return self.advance()
        return None

    def skip_newlines(self) -> None:
        while self.at("NEWLINE"):
            self.advance()

    def parse_program(self) -> ast.Program:
        self.skip_newlines()
        start = self.cur()
        stmts: list[ast.Stmt] = []
        while not self.at("EOF"):
            stmts.append(self.parse_stmt())
            self.skip_newlines()
        return ast.Program(span=self.span(start), statements=stmts)

    def parse_block(self) -> ast.Block:
        lb = self.expect("LBRACE")
        self.skip_newlines()
        stmts: list[ast.Stmt] = []
        while not self.at("RBRACE"):
            if self.at("EOF"):
                raise self.error("Unterminated block")
            stmts.append(self.parse_stmt())
            self.skip_newlines()
        self.expect("RBRACE")
        return ast.Block(span=self.span(lb), statements=stmts)

    def parse_stmt(self) -> ast.Stmt:
        self.skip_newlines()
        t = self.cur()

        if self.at("AYO"):
            self.advance()
            name = self.expect("IDENT")
            self.expect("EQ")
            value = self.parse_expr()
            return ast.VarDecl(span=self.span(t), name=name.value, value=value)

        if self.at("YOO"):
            self.advance()
            name = self.expect("IDENT")
            self.expect("EQ")
            value = self.parse_expr()
            return ast.ConstDecl(span=self.span(t), name=name.value, value=value)

        if self.at("BRUH"):
            self.advance()
            name = self.expect("IDENT")
            self.expect("LPAREN")
            params: list[str] = []
            if not self.at("RPAREN"):
                while True:
                    ident = self.expect("IDENT")
                    params.append(ident.value)
                    if self.match("COMMA") is None:
                        break
            self.expect("RPAREN")
            is_async = self.match("HAWKTUAH") is not None
            body = self.parse_block()
            return ast.FuncDef(span=self.span(t), name=name.value, params=params, is_async=is_async, body=body)

        if self.at("MAYBE"):
            # if-statement: Maybe <cond> { ... } (Unless Maybe <cond> { ... })* (Unless { ... })?
            self.advance()
            cond = self.parse_expr(stop_kinds={"LBRACE"})
            then_block = self.parse_block()
            elifs: list[tuple[ast.Expr, ast.Block]] = []
            else_block: ast.Block | None = None
            while True:
                self.skip_newlines()
                if self.match("UNLESS") is None:
                    break
                self.skip_newlines()
                if self.match("MAYBE") is not None:
                    econd = self.parse_expr(stop_kinds={"LBRACE"})
                    eblock = self.parse_block()
                    elifs.append((econd, eblock))
                    continue
                else_block = self.parse_block()
                break
            return ast.IfChain(span=self.span(t), cond=cond, then_block=then_block, elifs=elifs, else_block=else_block)

        if self.at("CRAZY"):
            self.advance()
            # Crazy <ident> in <expr> { ... } OR Crazy <cond-expr> { ... }
            if self._peek_is_in():
                var = self.expect("IDENT").value
                self.advance()  # 'in' (lexed as IDENT)
                iterable = self.parse_expr(stop_kinds={"LBRACE"})
                body = self.parse_block()
                return ast.ForIn(span=self.span(t), var=var, iterable=iterable, body=body)

            cond = self.parse_expr(stop_kinds={"LBRACE"})
            body = self.parse_block()
            return ast.WhileLoop(span=self.span(t), cond=cond, body=body)

        if self.at("RIZZ"):
            self.advance()
            self.expect("LPAREN")
            val = self.parse_expr(stop_kinds={"RPAREN"})
            self.expect("RPAREN")
            return ast.RizzStmt(span=self.span(t), value=val)

        if self.at("CRINGE"):
            self.advance()
            self.expect("LPAREN")
            val = self.parse_expr(stop_kinds={"RPAREN"})
            self.expect("RPAREN")
            return ast.CringeStmt(span=self.span(t), value=val)

        # assignment: <ident> = <expr>
        if self.at("IDENT") and self.tokens[self.i + 1].kind == "EQ":
            name = self.advance()
            self.advance()  # EQ
            value = self.parse_expr()
            return ast.Assign(span=self.span(t), name=name.value, value=value)

        # otherwise expression statement
        expr = self.parse_expr()
        return ast.ExprStmt(span=self.span(t), expr=expr)

    def _peek_is_in(self) -> bool:
        if not self.at("IDENT"):
            return False
        if self.i + 1 >= len(self.tokens):
            return False
        nxt = self.tokens[self.i + 1]
        return nxt.kind == "IDENT" and nxt.value == "in"

    # --- expression parsing (Pratt) ---
    def parse_expr(self, min_bp: int = 0, stop_kinds: set[str] | None = None) -> ast.Expr:
        if stop_kinds is None:
            stop_kinds = set()

        t = self.cur()
        if t.kind in stop_kinds:
            raise self.error("Unexpected token in expression")

        # prefix
        if self.at("MAYBE"):
            kw = self.advance()
            cond = self.parse_expr(stop_kinds={"QMARK"})
            self.expect("QMARK")
            if_true = self.parse_expr(stop_kinds={"COLON"})
            self.expect("COLON")
            if_false = self.parse_expr(min_bp=min_bp, stop_kinds=stop_kinds)
            left: ast.Expr = ast.Ternary(span=self.span(kw), cond=cond, if_true=if_true, if_false=if_false)
        elif self.at("VIBE"):
            kw = self.advance()
            expr = self.parse_expr(min_bp=80, stop_kinds=stop_kinds)
            left = ast.Vibe(span=self.span(kw), expr=expr)
        elif self.at("ATTEMPT"):
            kw = self.advance()
            try_block = self.parse_block()
            self.skip_newlines()
            self.expect("EWW")
            self.expect("LPAREN")
            err = self.expect("IDENT")
            self.expect("RPAREN")
            catch_block = self.parse_block()
            left = ast.Attempt(span=self.span(kw), try_block=try_block, err_name=err.value, catch_block=catch_block)
        elif self.at("BANG") or self.at("MINUS"):
            op = self.advance()
            expr = self.parse_expr(min_bp=70, stop_kinds=stop_kinds)
            left = ast.Unary(span=self.span(op), op=op.value, expr=expr)
        else:
            left = self.parse_primary(stop_kinds=stop_kinds)

        # postfix + infix loop
        while True:
            if self.cur().kind in stop_kinds:
                break

            # postfix: member, index, call
            if self.at("DOT"):
                dot = self.advance()
                name = self.expect("IDENT")
                left = ast.Member(span=self.span(dot), obj=left, name=name.value)
                continue
            if self.at("LBRACKET"):
                lb = self.advance()
                idx = self.parse_expr(stop_kinds={"RBRACKET"})
                self.expect("RBRACKET")
                left = ast.Index(span=self.span(lb), obj=left, index=idx)
                continue
            if self.at("LPAREN"):
                lp = self.advance()
                args: list[ast.Expr] = []
                if not self.at("RPAREN"):
                    while True:
                        args.append(self.parse_expr(stop_kinds={"COMMA", "RPAREN"}))
                        if self.match("COMMA") is None:
                            break
                self.expect("RPAREN")
                left = ast.Call(span=self.span(lp), callee=left, args=args)
                continue

            # infix binding powers
            op_tok = self.cur()
            op = None
            if op_tok.kind in {"OROR", "ANDAND", "EQEQ", "NEQ", "GT", "LT", "GTE", "LTE", "DOTDOT", "PLUS", "MINUS", "STAR", "SLASH", "PERCENT"}:
                op = op_tok.value
            if op is None:
                break

            lbp, rbp = _infix_binding_power(op_tok.kind)
            if lbp < min_bp:
                break
            self.advance()
            right = self.parse_expr(min_bp=rbp, stop_kinds=stop_kinds)
            left = ast.Binary(span=self.span(op_tok), op=op, left=left, right=right)

        return left

    def parse_primary(self, stop_kinds: set[str]) -> ast.Expr:
        t = self.cur()

        if self.at("INT"):
            self.advance()
            return ast.Literal(span=self.span(t), value=int(t.value))
        if self.at("FLOAT"):
            self.advance()
            return ast.Literal(span=self.span(t), value=float(t.value))
        if self.at("STRING"):
            self.advance()
            return ast.Literal(span=self.span(t), value=t.value)
        if self.at("REGEX"):
            self.advance()
            return ast.Literal(span=self.span(t), value=("regex", t.value))
        if self.at("CHAR"):
            self.advance()
            return ast.Literal(span=self.span(t), value=t.value)

        if self.at("IDENT"):
            self.advance()
            if t.value == "null":
                return ast.Literal(span=self.span(t), value=None)
            if t.value == "true":
                return ast.Literal(span=self.span(t), value=True)
            if t.value == "false":
                return ast.Literal(span=self.span(t), value=False)
            return ast.Ident(span=self.span(t), name=t.value)

        if self.at("LPAREN"):
            self.advance()
            expr = self.parse_expr(stop_kinds={"RPAREN"})
            self.expect("RPAREN")
            return expr

        if self.at("LBRACKET"):
            lb = self.advance()
            items: list[ast.Expr] = []
            if not self.at("RBRACKET"):
                while True:
                    items.append(self.parse_expr(stop_kinds={"COMMA", "RBRACKET"}))
                    if self.match("COMMA") is None:
                        break
            self.expect("RBRACKET")
            return ast.ArrayLit(span=self.span(lb), items=items)

        if self.at("LBRACE"):
            lb = self.advance()
            items: list[tuple[str | ast.Expr, ast.Expr]] = []
            if not self.at("RBRACE"):
                while True:
                    # key
                    if self.at("IDENT"):
                        key_tok = self.advance()
                        key: str | ast.Expr = key_tok.value
                    else:
                        key = self.parse_expr(stop_kinds={"COLON"})
                    self.expect("COLON")
                    val = self.parse_expr(stop_kinds={"COMMA", "RBRACE"})
                    items.append((key, val))
                    if self.match("COMMA") is None:
                        break
            self.expect("RBRACE")
            return ast.ObjectLit(span=self.span(lb), items=items)

        raise self.error("Expected expression")


def _infix_binding_power(kind: str) -> tuple[int, int]:
    # higher = tighter binding
    if kind == "OROR":
        return (10, 11)
    if kind == "ANDAND":
        return (20, 21)
    if kind in {"EQEQ", "NEQ"}:
        return (30, 31)
    if kind in {"GT", "LT", "GTE", "LTE"}:
        return (40, 41)
    if kind == "DOTDOT":
        return (45, 46)
    if kind in {"PLUS", "MINUS"}:
        return (50, 51)
    if kind in {"STAR", "SLASH", "PERCENT"}:
        return (60, 61)
    return (0, 0)

