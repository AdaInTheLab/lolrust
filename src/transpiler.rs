use crate::keywords::keyword_mappings;

/// Transpile LolRust (.meow) → Rust (.rs)
pub fn transpile(source: &str) -> String {
    let mappings = keyword_mappings();
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut output = String::with_capacity(source.len() * 11 / 10);
    let mut i = 0;

    while i < len {
        // Raw strings, byte strings, normal strings, comments, char literals
        if let Some(new_i) = try_raw_string(&chars, i, &mut output) {
            i = new_i;
            continue;
        }
        if chars[i] == 'b' && i + 1 < len && chars[i + 1] == '"' {
            output.push('b');
            i += 1;
        }
        if chars[i] == '"' {
            i = copy_string_literal(&chars, i, &mut output);
            continue;
        }
        if chars[i] == '/' {
            if i + 1 < len && chars[i + 1] == '/' {
                i = copy_line_comment(&chars, i, &mut output);
                continue;
            }
            if i + 1 < len && chars[i + 1] == '*' {
                i = copy_block_comment(&chars, i, &mut output);
                continue;
            }
        }
        if chars[i] == '\'' && is_char_literal(&chars, i) {
            i = copy_char_literal(&chars, i, &mut output);
            continue;
        }

        // &wiggly → &mut
        if try_ampersand_wiggly(&chars, i, &mut output) {
            i += "&wiggly".len();
            continue;
        }

        // Multi-char / multi-word mappings (sorted longest-first in keywords.rs).
        // Run BEFORE the bare-name aliases so `meow!` beats `meow`.
        let mut matched = false;
        for &(meow, rust) in &mappings {
            let mlen = meow.len();
            if i + mlen > len { continue; }

            let slice: String = chars[i..i + mlen].iter().collect();
            if slice == meow && is_word_boundary(&chars, i, mlen) {
                output.push_str(rust);
                i += mlen;
                matched = true;
                break;
            }
        }
        if matched { continue; }

        // Beginner-friendly bare `meow` (no !) → `println!`. Skipped when
        // followed by `!` so `meow!(...)` falls through to the mappings above.
        if let Some(new_i) = try_bare_alias(&chars, i, "meow", "println!", &mut output) {
            i = new_i;
            continue;
        }

        output.push(chars[i]);
        i += 1;
    }

    output
}

// ==================== HELPERS ====================

/// Match a bare beginner alias like `meow` → `println!`.
/// Returns the new index on match. Refuses to match if followed by `!`
/// so the macro form (`meow!`) falls through to the keyword mappings.
fn try_bare_alias(chars: &[char], i: usize, name: &str, rust: &str, output: &mut String) -> Option<usize> {
    let nlen = name.len();
    if i + nlen > chars.len() { return None; }
    let slice: String = chars[i..i + nlen].iter().collect();
    if slice != name { return None; }
    if i > 0 && is_word_char(chars[i - 1]) { return None; }
    if i + nlen < chars.len() {
        let next = chars[i + nlen];
        if is_word_char(next) || next == '!' { return None; }
    }
    output.push_str(rust);
    Some(i + nlen)
}

fn try_ampersand_wiggly(chars: &[char], i: usize, output: &mut String) -> bool {
    let prefix = "&wiggly";
    if i + prefix.len() > chars.len() { return false; }
    let slice: String = chars[i..i + prefix.len()].iter().collect();
    if slice == prefix && (i + prefix.len() == chars.len() || !is_word_char(chars[i + prefix.len()])) {
        output.push_str("&mut");
        return true;
    }
    false
}

fn is_word_boundary(chars: &[char], pos: usize, meow_len: usize) -> bool {
    if pos > 0 && is_word_char(chars[pos - 1]) { return false; }
    let end = pos + meow_len;
    if end < chars.len() && is_word_char(chars[end]) { return false; }
    true
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Detect a raw string literal `r"..."` or `r#"..."#`. Copies it verbatim
/// (no keyword replacement inside). Returns the index after the closing
/// delimiter, or None if there's no raw string at `i`.
fn try_raw_string(chars: &[char], i: usize, output: &mut String) -> Option<usize> {
    let len = chars.len();
    if chars[i] != 'r' || i + 1 >= len { return None; }
    if i > 0 && is_word_char(chars[i - 1]) { return None; }

    let mut hash_count = 0;
    let mut j = i + 1;
    while j < len && chars[j] == '#' {
        hash_count += 1;
        j += 1;
    }
    if j >= len || chars[j] != '"' { return None; }

    for k in i..=j {
        output.push(chars[k]);
    }
    let mut k = j + 1;
    while k < len {
        if chars[k] == '"' {
            let mut closing = 0;
            let mut m = k + 1;
            while m < len && chars[m] == '#' && closing < hash_count {
                closing += 1;
                m += 1;
            }
            if closing == hash_count {
                for idx in k..m {
                    output.push(chars[idx]);
                }
                return Some(m);
            }
        }
        output.push(chars[k]);
        k += 1;
    }
    Some(k)
}

/// Copy a `"..."` string literal verbatim, honoring `\` escapes.
fn copy_string_literal(chars: &[char], i: usize, output: &mut String) -> usize {
    let len = chars.len();
    output.push('"');
    let mut k = i + 1;
    while k < len && chars[k] != '"' {
        if chars[k] == '\\' && k + 1 < len {
            output.push(chars[k]);
            output.push(chars[k + 1]);
            k += 2;
        } else {
            output.push(chars[k]);
            k += 1;
        }
    }
    if k < len {
        output.push('"');
        k += 1;
    }
    k
}

fn copy_line_comment(chars: &[char], i: usize, output: &mut String) -> usize {
    let len = chars.len();
    let mut k = i;
    while k < len && chars[k] != '\n' {
        output.push(chars[k]);
        k += 1;
    }
    k
}

/// Copy a `/* ... */` block comment, supporting nesting.
fn copy_block_comment(chars: &[char], i: usize, output: &mut String) -> usize {
    let len = chars.len();
    let mut depth = 1;
    output.push(chars[i]);
    output.push(chars[i + 1]);
    let mut k = i + 2;
    while k < len && depth > 0 {
        if chars[k] == '/' && k + 1 < len && chars[k + 1] == '*' {
            depth += 1;
            output.push(chars[k]);
            output.push(chars[k + 1]);
            k += 2;
        } else if chars[k] == '*' && k + 1 < len && chars[k + 1] == '/' {
            depth -= 1;
            output.push(chars[k]);
            output.push(chars[k + 1]);
            k += 2;
        } else {
            output.push(chars[k]);
            k += 1;
        }
    }
    k
}

fn copy_char_literal(chars: &[char], i: usize, output: &mut String) -> usize {
    let len = chars.len();
    output.push(chars[i]);
    let mut k = i + 1;
    if k < len && chars[k] == '\\' {
        output.push(chars[k]);
        k += 1;
        if k < len {
            output.push(chars[k]);
            k += 1;
        }
    } else if k < len {
        output.push(chars[k]);
        k += 1;
    }
    if k < len && chars[k] == '\'' {
        output.push(chars[k]);
        k += 1;
    }
    k
}

/// Distinguish char literal `'x'` / `'\n'` from a lifetime annotation `'a`.
fn is_char_literal(chars: &[char], pos: usize) -> bool {
    let len = chars.len();
    if pos + 2 >= len { return false; }
    if chars[pos + 1] == '\\' {
        pos + 3 < len && chars[pos + 3] == '\''
    } else {
        pos + 2 < len && chars[pos + 2] == '\''
    }
}
