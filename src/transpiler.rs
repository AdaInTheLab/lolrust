use crate::keywords::keyword_mappings;

/// Transpile LolRust (.meow) source code into valid Rust (.rs) source code.
///
/// Uses a single-pass character scanner that tracks context
/// (string literals, comments, normal code) and only performs
/// keyword replacement in normal code context.
pub fn transpile(source: &str) -> String {
    let mappings = keyword_mappings();
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut output = String::with_capacity(source.len());
    let mut i = 0;

    while i < len {
        // 1. Check for raw string literal: r#"..."# or r"..."
        if chars[i] == 'r' && i + 1 < len {
            let mut hash_count = 0;
            let mut j = i + 1;
            while j < len && chars[j] == '#' {
                hash_count += 1;
                j += 1;
            }
            if j < len && chars[j] == '"' && (hash_count > 0 || (j == i + 1)) {
                // Only treat as raw string if we have r#" or r" pattern
                if hash_count > 0 || j == i + 1 {
                    // Copy r, hashes, and opening quote
                    for k in i..=j {
                        output.push(chars[k]);
                    }
                    i = j + 1;
                    // Find closing: "###
                    'raw_outer: while i < len {
                        if chars[i] == '"' {
                            let mut closing_hashes = 0;
                            let mut k = i + 1;
                            while k < len && chars[k] == '#' && closing_hashes < hash_count {
                                closing_hashes += 1;
                                k += 1;
                            }
                            if closing_hashes == hash_count {
                                // Found the closing delimiter
                                for idx in i..k {
                                    output.push(chars[idx]);
                                }
                                i = k;
                                break 'raw_outer;
                            }
                        }
                        output.push(chars[i]);
                        i += 1;
                    }
                    continue;
                }
            }
        }

        // 2. Check for byte string b"..."
        if chars[i] == 'b' && i + 1 < len && chars[i + 1] == '"' {
            output.push(chars[i]); // b
            i += 1;
            // Fall through to string literal handling below
        }

        // 3. Check for string literal
        if chars[i] == '"' {
            output.push('"');
            i += 1;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < len {
                    output.push(chars[i]);
                    output.push(chars[i + 1]);
                    i += 2;
                } else {
                    output.push(chars[i]);
                    i += 1;
                }
            }
            if i < len {
                output.push('"'); // closing quote
                i += 1;
            }
            continue;
        }

        // 4. Check for line comment
        if chars[i] == '/' && i + 1 < len && chars[i + 1] == '/' {
            while i < len && chars[i] != '\n' {
                output.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // 5. Check for block comment (with nesting support)
        if chars[i] == '/' && i + 1 < len && chars[i + 1] == '*' {
            let mut depth = 1;
            output.push(chars[i]);
            output.push(chars[i + 1]);
            i += 2;
            while i < len && depth > 0 {
                if chars[i] == '/' && i + 1 < len && chars[i + 1] == '*' {
                    depth += 1;
                    output.push(chars[i]);
                    output.push(chars[i + 1]);
                    i += 2;
                } else if chars[i] == '*' && i + 1 < len && chars[i + 1] == '/' {
                    depth -= 1;
                    output.push(chars[i]);
                    output.push(chars[i + 1]);
                    i += 2;
                } else {
                    output.push(chars[i]);
                    i += 1;
                }
            }
            continue;
        }

        // 6. Check for char literal (not lifetime)
        if chars[i] == '\'' && is_char_literal(&chars, i) {
            // Copy the entire char literal
            output.push(chars[i]); // opening '
            i += 1;
            if i < len && chars[i] == '\\' {
                // Escape sequence
                output.push(chars[i]);
                i += 1;
                if i < len {
                    output.push(chars[i]);
                    i += 1;
                }
            } else if i < len {
                output.push(chars[i]);
                i += 1;
            }
            if i < len && chars[i] == '\'' {
                output.push(chars[i]); // closing '
                i += 1;
            }
            continue;
        }

        // 7. Try keyword matching
        let mut matched = false;
        for &(meow, rust) in &mappings {
            let meow_len = meow.len();
            if i + meow_len > len {
                continue;
            }

            // Check if source at position i matches the meow keyword
            let slice: String = chars[i..i + meow_len].iter().collect();
            if slice == meow && is_word_boundary(&chars, i, meow_len, meow) {
                output.push_str(rust);
                i += meow_len;
                matched = true;
                break;
            }
        }

        if !matched {
            output.push(chars[i]);
            i += 1;
        }
    }

    output
}

/// Check if the match at position `pos` with length `meow_len` is at a word boundary.
/// For keywords ending with `!` (macros), we only check the leading boundary.
/// For other keywords, we check both leading and trailing boundaries.
fn is_word_boundary(chars: &[char], pos: usize, meow_len: usize, meow: &str) -> bool {
    // Check leading boundary
    if pos > 0 && is_word_char(chars[pos - 1]) {
        return false;
    }

    // For macros ending with !, don't check trailing boundary
    // (the ! naturally separates from following content)
    if meow.ends_with('!') {
        return true;
    }

    // Check trailing boundary
    let end = pos + meow_len;
    if end < chars.len() && is_word_char(chars[end]) {
        return false;
    }

    true
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Heuristic to detect char literals vs lifetime annotations.
/// A char literal is: 'x' or '\x' where x is some char.
fn is_char_literal(chars: &[char], pos: usize) -> bool {
    let len = chars.len();
    if pos + 2 >= len {
        return false;
    }

    if chars[pos + 1] == '\\' {
        // Escape sequence: '\n', '\t', '\0', '\\', '\''
        pos + 3 < len && chars[pos + 3] == '\''
    } else {
        // Simple char: 'a'
        pos + 2 < len && chars[pos + 2] == '\''
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_fn() {
        assert_eq!(transpile("iz main() {}"), "fn main() {}");
    }

    #[test]
    fn let_mut() {
        assert_eq!(transpile("i can haz wiggly x = 5;"), "let mut x = 5;");
    }

    #[test]
    fn let_immutable() {
        assert_eq!(transpile("i can haz x = 5;"), "let x = 5;");
    }

    #[test]
    fn string_preserved() {
        assert_eq!(
            transpile(r#"meow!("i can haz cheezburger");"#),
            r#"println!("i can haz cheezburger");"#
        );
    }

    #[test]
    fn line_comment_preserved() {
        assert_eq!(
            transpile("// i can haz comment\niz main() {}"),
            "// i can haz comment\nfn main() {}"
        );
    }

    #[test]
    fn word_boundary_no_partial() {
        assert_eq!(transpile("i can haz wizard = 1;"), "let wizard = 1;");
    }

    #[test]
    fn else_if_before_if() {
        assert_eq!(
            transpile("} or if ceiling cat sez x > 0 {"),
            "} else if x > 0 {"
        );
    }

    #[test]
    fn block_comment_preserved() {
        assert_eq!(
            transpile("/* iz a comment */ iz main() {}"),
            "/* iz a comment */ fn main() {}"
        );
    }

    #[test]
    fn nested_block_comment() {
        assert_eq!(
            transpile("/* outer /* inner */ still */ iz main() {}"),
            "/* outer /* inner */ still */ fn main() {}"
        );
    }

    #[test]
    fn escape_in_string() {
        assert_eq!(
            transpile(r#"meow!("say \"iz\" loudly");"#),
            r#"println!("say \"iz\" loudly");"#
        );
    }

    #[test]
    fn all_keywords() {
        assert_eq!(transpile("everycat iz main() {}"), "pub fn main() {}");
        assert_eq!(transpile("loaf Foo {}"), "struct Foo {}");
        assert_eq!(transpile("teech Foo {}"), "impl Foo {}");
        assert_eq!(transpile("flavurz Color {}"), "enum Color {}");
        assert_eq!(transpile("skillz Bar {}"), "trait Bar {}");
        assert_eq!(transpile("gimme std::io;"), "use std::io;");
        assert_eq!(transpile("cough up 42;"), "return 42;");
        assert_eq!(transpile("zoomies {}"), "loop {}");
        assert_eq!(transpile("chase i around 0..10 {}"), "for i in 0..10 {}");
        assert_eq!(transpile("flop;"), "break;");
        assert_eq!(transpile("again;"), "continue;");
        assert_eq!(transpile("yolo {}"), "unsafe {}");
        assert_eq!(transpile("yus"), "true");
        assert_eq!(transpile("nope"), "false");
    }

    #[test]
    fn type_aliases() {
        assert_eq!(transpile("MaybeCheezburgr<i32>"), "Option<i32>");
        assert_eq!(transpile("Tryz<i32, Yarn>"), "Result<i32, String>");
        assert_eq!(transpile("Pile<i32>"), "Vec<i32>");
        assert_eq!(transpile("Yarn::new()"), "String::new()");
        assert_eq!(transpile("Has(42)"), "Some(42)");
        assert_eq!(transpile("EmptyBowl"), "None");
        assert_eq!(transpile("Purrfect(x)"), "Ok(x)");
        assert_eq!(transpile("Hairball(e)"), "Err(e)");
    }

    #[test]
    fn self_and_self_type() {
        assert_eq!(transpile("dis.foo()"), "self.foo()");
        assert_eq!(transpile("Dis::new()"), "Self::new()");
        // Should NOT replace inside "dismiss"
        assert_eq!(transpile("dismiss()"), "dismiss()");
    }

    #[test]
    fn ampersand_mut() {
        assert_eq!(transpile("&wiggly x"), "&mut x");
    }

    #[test]
    fn pretend_iz_vs_iz() {
        assert_eq!(transpile("x pretend iz i32"), "x as i32");
        assert_eq!(transpile("iz main() {}"), "fn main() {}");
    }

    #[test]
    fn macros() {
        assert_eq!(transpile("meow!(\"hi\");"), "println!(\"hi\");");
        assert_eq!(transpile("hisss!(\"err\");"), "eprintln!(\"err\");");
        assert_eq!(transpile("ohno!(\"boom\");"), "panic!(\"boom\");");
    }
}
