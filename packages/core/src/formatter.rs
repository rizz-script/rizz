pub fn format_source(src: &str) -> String {
    let mut out = String::new();
    let mut indent: i32 = 0;

    for raw in src.lines() {
        let line = raw.trim_end();
        let trimmed = line.trim();

        // preserve totally empty lines
        if trimmed.is_empty() {
            out.push('\n');
            continue;
        }

        // outdent on leading }
        if trimmed.starts_with('}') {
            indent = (indent - 1).max(0);
        }

        for _ in 0..indent {
            out.push_str("  ");
        }
        out.push_str(trimmed);
        out.push('\n');

        // indent on trailing {
        if trimmed.ends_with('{') {
            indent += 1;
        }
    }

    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}
