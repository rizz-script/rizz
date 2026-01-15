from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class Token:
    kind: str
    value: str
    line: int
    col: int


KEYWORDS = {
    "Ayo": "AYO",
    "Yoo": "YOO",
    "Bruh": "BRUH",
    "HawkTuah": "HAWKTUAH",
    "Rizz": "RIZZ",
    "Maybe": "MAYBE",
    "Unless": "UNLESS",
    "Crazy": "CRAZY",
    "Vibe": "VIBE",
    "Attempt": "ATTEMPT",
    "Eww": "EWW",
    "Cringe": "CRINGE",
}


TWO_CHAR = {
    "&&": "ANDAND",
    "||": "OROR",
    "==": "EQEQ",
    "!=": "NEQ",
    ">=": "GTE",
    "<=": "LTE",
    "..": "DOTDOT",
}


SINGLE = {
    "(": "LPAREN",
    ")": "RPAREN",
    "{": "LBRACE",
    "}": "RBRACE",
    "[": "LBRACKET",
    "]": "RBRACKET",
    ",": "COMMA",
    ":": "COLON",
    "?": "QMARK",
    ".": "DOT",
    "+": "PLUS",
    "-": "MINUS",
    "*": "STAR",
    "/": "SLASH",
    "%": "PERCENT",
    ">": "GT",
    "<": "LT",
    "!": "BANG",
    "=": "EQ",
}


class LexError(Exception):
    pass


def lex(src: str) -> list[Token]:
    tokens: list[Token] = []
    i = 0
    line = 1
    col = 1
    n = len(src)

    def push(kind: str, value: str, l: int, c: int) -> None:
        tokens.append(Token(kind=kind, value=value, line=l, col=c))

    while i < n:
        ch = src[i]

        # whitespace
        if ch in " \t\r":
            i += 1
            col += 1
            continue
        if ch == "\n":
            push("NEWLINE", "\n", line, col)
            i += 1
            line += 1
            col = 1
            continue

        # comments
        if ch == "/" and i + 1 < n and src[i + 1] == "/":
            i += 2
            col += 2
            while i < n and src[i] != "\n":
                i += 1
                col += 1
            continue
        if ch == "/" and i + 1 < n and src[i + 1] == "*":
            i += 2
            col += 2
            while i < n:
                if src[i] == "\n":
                    i += 1
                    line += 1
                    col = 1
                    continue
                if src[i] == "*" and i + 1 < n and src[i + 1] == "/":
                    i += 2
                    col += 2
                    break
                i += 1
                col += 1
            else:
                raise LexError(f"Unterminated block comment at {line}:{col}")
            continue

        # two-char ops
        if i + 1 < n:
            two = src[i : i + 2]
            if two in TWO_CHAR:
                push(TWO_CHAR[two], two, line, col)
                i += 2
                col += 2
                continue

        # regex literal: r"..."
        if ch == "r" and i + 1 < n and src[i + 1] == '"':
            start_line, start_col = line, col
            i += 2
            col += 2
            s = []
            while i < n:
                c = src[i]
                if c == '"':
                    i += 1
                    col += 1
                    break
                if c == "\\":
                    if i + 1 >= n:
                        raise LexError(f"Bad escape at {line}:{col}")
                    s.append(c)
                    s.append(src[i + 1])
                    i += 2
                    col += 2
                    continue
                if c == "\n":
                    raise LexError(f"Unterminated string at {start_line}:{start_col}")
                s.append(c)
                i += 1
                col += 1
            else:
                raise LexError(f"Unterminated string at {start_line}:{start_col}")
            push("REGEX", "".join(s), start_line, start_col)
            continue

        # string literal
        if ch == '"':
            start_line, start_col = line, col
            i += 1
            col += 1
            s = []
            while i < n:
                c = src[i]
                if c == '"':
                    i += 1
                    col += 1
                    break
                if c == "\\":
                    if i + 1 >= n:
                        raise LexError(f"Bad escape at {line}:{col}")
                    esc = src[i + 1]
                    if esc == "n":
                        s.append("\n")
                    elif esc == "t":
                        s.append("\t")
                    elif esc == '"':
                        s.append('"')
                    elif esc == "\\":
                        s.append("\\")
                    else:
                        s.append(esc)
                    i += 2
                    col += 2
                    continue
                if c == "\n":
                    raise LexError(f"Unterminated string at {start_line}:{start_col}")
                s.append(c)
                i += 1
                col += 1
            else:
                raise LexError(f"Unterminated string at {start_line}:{start_col}")
            push("STRING", "".join(s), start_line, start_col)
            continue

        # char literal
        if ch == "'":
            start_line, start_col = line, col
            i += 1
            col += 1
            if i >= n:
                raise LexError(f"Unterminated char at {start_line}:{start_col}")
            c = src[i]
            if c == "\\":
                if i + 1 >= n:
                    raise LexError(f"Bad escape at {start_line}:{start_col}")
                esc = src[i + 1]
                if esc == "n":
                    val = "\n"
                elif esc == "t":
                    val = "\t"
                else:
                    val = esc
                i += 2
                col += 2
            else:
                val = c
                i += 1
                col += 1
            if i >= n or src[i] != "'":
                raise LexError(f"Unterminated char at {start_line}:{start_col}")
            i += 1
            col += 1
            push("CHAR", val, start_line, start_col)
            continue

        # number
        if ch.isdigit() or (ch == "-" and i + 1 < n and src[i + 1].isdigit()):
            start_line, start_col = line, col
            j = i
            if src[j] == "-":
                j += 1
            while j < n and src[j].isdigit():
                j += 1
            is_float = False
            if j < n and src[j] == "." and (j + 1 < n and src[j + 1].isdigit()):
                is_float = True
                j += 1
                while j < n and src[j].isdigit():
                    j += 1
            value = src[i:j]
            push("FLOAT" if is_float else "INT", value, start_line, start_col)
            col += j - i
            i = j
            continue

        # identifier / keyword / literals
        if ch.isalpha() or ch == "_":
            start_line, start_col = line, col
            j = i + 1
            while j < n and (src[j].isalnum() or src[j] == "_"):
                j += 1
            word = src[i:j]
            if word in KEYWORDS:
                push(KEYWORDS[word], word, start_line, start_col)
            else:
                push("IDENT", word, start_line, start_col)
            col += j - i
            i = j
            continue

        # single-char
        if ch in SINGLE:
            push(SINGLE[ch], ch, line, col)
            i += 1
            col += 1
            continue

        raise LexError(f"Unexpected character {ch!r} at {line}:{col}")

    push("EOF", "", line, col)
    return tokens

