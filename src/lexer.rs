use anyhow::bail;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    // keywords
    Ayo,
    Yoo,
    Bruh,
    HawkTuah,
    Rizz,
    Maybe,
    Unless,
    Crazy,
    Vibe,
    Attempt,
    Eww,
    Cringe,
    Let,
    Const,
    Function,
    Return,
    Throw,
    Try,
    Catch,
    Finally,

    Ident,
    Int,
    Float,
    Str,
    Char,
    Regex,

    Newline,
    Eof,

    // punctuation/operators
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Qmark,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    Bang,
    Gt,
    Lt,
    AndAnd,
    OrOr,
    EqEq,
    Neq,
    Gte,
    Lte,
    DotDot,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub line: usize,
    pub col: usize,
}

pub fn lex(src: &str) -> anyhow::Result<Vec<Token>> {
    let mut out = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0usize;
    let mut line = 1usize;
    let mut col = 1usize;

    let mut push = |kind: Kind, value: String, l: usize, c: usize| {
        out.push(Token {
            kind,
            value,
            line: l,
            col: c,
        })
    };

    while i < bytes.len() {
        let ch = bytes[i] as char;

        // whitespace
        if ch == ' ' || ch == '\t' || ch == '\r' {
            i += 1;
            col += 1;
            continue;
        }
        if ch == '\n' {
            push(Kind::Newline, "\n".to_string(), line, col);
            i += 1;
            line += 1;
            col = 1;
            continue;
        }

        // comments
        if ch == '/' && i + 1 < bytes.len() && bytes[i + 1] as char == '/' {
            i += 2;
            col += 2;
            while i < bytes.len() && bytes[i] as char != '\n' {
                i += 1;
                col += 1;
            }
            continue;
        }
        if ch == '/' && i + 1 < bytes.len() && bytes[i + 1] as char == '*' {
            i += 2;
            col += 2;
            while i < bytes.len() {
                let c = bytes[i] as char;
                if c == '\n' {
                    i += 1;
                    line += 1;
                    col = 1;
                    continue;
                }
                if c == '*' && i + 1 < bytes.len() && bytes[i + 1] as char == '/' {
                    i += 2;
                    col += 2;
                    break;
                }
                i += 1;
                col += 1;
            }
            if i >= bytes.len() {
                bail!("Unterminated block comment at {line}:{col}");
            }
            continue;
        }

        // two-char ops
        if i + 1 < bytes.len() {
            let two = [bytes[i] as char, bytes[i + 1] as char];
            let (k, s) = match two {
                ['&', '&'] => (Some(Kind::AndAnd), "&&"),
                ['|', '|'] => (Some(Kind::OrOr), "||"),
                ['=', '='] => (Some(Kind::EqEq), "=="),
                ['!', '='] => (Some(Kind::Neq), "!="),
                ['>', '='] => (Some(Kind::Gte), ">="),
                ['<', '='] => (Some(Kind::Lte), "<="),
                ['.', '.'] => (Some(Kind::DotDot), ".."),
                _ => (None, ""),
            };
            if let Some(kind) = k {
                push(kind, s.to_string(), line, col);
                i += 2;
                col += 2;
                continue;
            }
        }

        // regex literal r"..."
        if ch == 'r' && i + 1 < bytes.len() && bytes[i + 1] as char == '"' {
            let start_line = line;
            let start_col = col;
            i += 2;
            col += 2;
            let mut s = String::new();
            while i < bytes.len() {
                let c = bytes[i] as char;
                if c == '"' {
                    i += 1;
                    col += 1;
                    break;
                }
                if c == '\\' {
                    if i + 1 >= bytes.len() {
                        bail!("Bad escape at {line}:{col}");
                    }
                    s.push('\\');
                    s.push(bytes[i + 1] as char);
                    i += 2;
                    col += 2;
                    continue;
                }
                if c == '\n' {
                    bail!("Unterminated regex at {start_line}:{start_col}");
                }
                s.push(c);
                i += 1;
                col += 1;
            }
            push(Kind::Regex, s, start_line, start_col);
            continue;
        }

        // string literal
        if ch == '"' {
            let start_line = line;
            let start_col = col;
            i += 1;
            col += 1;
            let mut s = String::new();
            while i < bytes.len() {
                let c = bytes[i] as char;
                if c == '"' {
                    i += 1;
                    col += 1;
                    break;
                }
                if c == '\\' {
                    if i + 1 >= bytes.len() {
                        bail!("Bad escape at {line}:{col}");
                    }
                    let esc = bytes[i + 1] as char;
                    match esc {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        '"' => s.push('"'),
                        '\\' => s.push('\\'),
                        'u' => {
                            // \uXXXX
                            if i + 5 >= bytes.len() {
                                bail!("Bad \\u escape at {line}:{col}");
                            }
                            let hex = std::str::from_utf8(&bytes[i + 2..i + 6])?;
                            let cp = u32::from_str_radix(hex, 16)
                                .map_err(|_| anyhow::anyhow!("Bad \\u escape at {line}:{col}"))?;
                            let ch = char::from_u32(cp)
                                .ok_or_else(|| anyhow::anyhow!("Bad unicode codepoint at {line}:{col}"))?;
                            s.push(ch);
                            i += 6;
                            col += 6;
                            continue;
                        }
                        'x' => {
                            // \xNN
                            if i + 3 >= bytes.len() {
                                bail!("Bad \\x escape at {line}:{col}");
                            }
                            let hex = std::str::from_utf8(&bytes[i + 2..i + 4])?;
                            let b = u8::from_str_radix(hex, 16)
                                .map_err(|_| anyhow::anyhow!("Bad \\x escape at {line}:{col}"))?;
                            s.push(b as char);
                            i += 4;
                            col += 4;
                            continue;
                        }
                        other => s.push(other),
                    }
                    i += 2;
                    col += 2;
                    continue;
                }
                if c == '\n' {
                    bail!("Unterminated string at {start_line}:{start_col}");
                }
                s.push(c);
                i += 1;
                col += 1;
            }
            push(Kind::Str, s, start_line, start_col);
            continue;
        }

        // char literal
        if ch == '\'' {
            let start_line = line;
            let start_col = col;
            i += 1;
            col += 1;
            if i >= bytes.len() {
                bail!("Unterminated char at {start_line}:{start_col}");
            }
            let mut val = bytes[i] as char;
            if val == '\\' {
                if i + 1 >= bytes.len() {
                    bail!("Bad escape at {start_line}:{start_col}");
                }
                let esc = bytes[i + 1] as char;
                val = match esc {
                    'n' => '\n',
                    't' => '\t',
                    other => other,
                };
                i += 2;
                col += 2;
            } else {
                i += 1;
                col += 1;
            }
            if i >= bytes.len() || bytes[i] as char != '\'' {
                bail!("Unterminated char at {start_line}:{start_col}");
            }
            i += 1;
            col += 1;
            push(Kind::Char, val.to_string(), start_line, start_col);
            continue;
        }

        // number
        if ch.is_ascii_digit() || (ch == '-' && i + 1 < bytes.len() && (bytes[i + 1] as char).is_ascii_digit())
        {
            let start_line = line;
            let start_col = col;
            let mut j = i;
            if bytes[j] as char == '-' {
                j += 1;
            }
            while j < bytes.len() && (bytes[j] as char).is_ascii_digit() {
                j += 1;
            }
            let mut is_float = false;
            if j + 1 < bytes.len()
                && bytes[j] as char == '.'
                && (bytes[j + 1] as char).is_ascii_digit()
            {
                is_float = true;
                j += 1;
                while j < bytes.len() && (bytes[j] as char).is_ascii_digit() {
                    j += 1;
                }
            }
            let s = std::str::from_utf8(&bytes[i..j])?.to_string();
            push(if is_float { Kind::Float } else { Kind::Int }, s, start_line, start_col);
            col += j - i;
            i = j;
            continue;
        }

        // identifier / keyword / literals
        if ch.is_ascii_alphabetic() || ch == '_' {
            let start_line = line;
            let start_col = col;
            let mut j = i + 1;
            while j < bytes.len() {
                let c = bytes[j] as char;
                if c.is_ascii_alphanumeric() || c == '_' {
                    j += 1;
                } else {
                    break;
                }
            }
            let word = std::str::from_utf8(&bytes[i..j])?.to_string();
            let kind = match word.as_str() {
                "Ayo" => Kind::Ayo,
                "Yoo" => Kind::Yoo,
                "Bruh" => Kind::Bruh,
                "HawkTuah" => Kind::HawkTuah,
                "Rizz" => Kind::Rizz,
                "Maybe" => Kind::Maybe,
                "Unless" => Kind::Unless,
                "Crazy" => Kind::Crazy,
                "Vibe" => Kind::Vibe,
                "Attempt" => Kind::Attempt,
                "Eww" => Kind::Eww,
                "Cringe" => Kind::Cringe,
                "let" => Kind::Let,
                "const" => Kind::Const,
                "function" => Kind::Function,
                "return" => Kind::Return,
                "throw" => Kind::Throw,
                "try" => Kind::Try,
                "catch" => Kind::Catch,
                "finally" => Kind::Finally,
                _ => Kind::Ident,
            };
            push(kind, word, start_line, start_col);
            col += j - i;
            i = j;
            continue;
        }

        // single-char tokens
        let start_line = line;
        let start_col = col;
        let kind = match ch {
            '(' => Some(Kind::LParen),
            ')' => Some(Kind::RParen),
            '{' => Some(Kind::LBrace),
            '}' => Some(Kind::RBrace),
            '[' => Some(Kind::LBracket),
            ']' => Some(Kind::RBracket),
            ',' => Some(Kind::Comma),
            ':' => Some(Kind::Colon),
            '?' => Some(Kind::Qmark),
            '.' => Some(Kind::Dot),
            '+' => Some(Kind::Plus),
            '-' => Some(Kind::Minus),
            '*' => Some(Kind::Star),
            '/' => Some(Kind::Slash),
            '%' => Some(Kind::Percent),
            '=' => Some(Kind::Eq),
            '!' => Some(Kind::Bang),
            '>' => Some(Kind::Gt),
            '<' => Some(Kind::Lt),
            _ => None,
        };
        if let Some(k) = kind {
            push(k, ch.to_string(), start_line, start_col);
            i += 1;
            col += 1;
            continue;
        }

        bail!("Unexpected character {ch:?} at {line}:{col}");
    }

    push(Kind::Eof, "".to_string(), line, col);
    Ok(out)
}

