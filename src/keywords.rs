/// Returns all keyword mappings from LolRust (.meow) to Rust (.rs),
/// sorted by descending length of the meow keyword.
/// This ordering is critical: longer matches must be tried first
/// to prevent partial replacements (e.g., "i can haz wiggly" before "i can haz").
pub fn keyword_mappings() -> Vec<(&'static str, &'static str)> {
    let mut mappings = vec![
        // Multi-word keywords
        ("or if ceiling cat sez", "else if"),
        ("if ceiling cat sez", "if"),
        ("or basement cat sez", "else"),
        ("i can haz wiggly", "let mut"),
        ("keep goin while", "while"),
        ("i can haz", "let"),
        ("but only if", "where"),
        ("pretend iz", "as"),

        // Macro replacements (include the !)
        ("hisss!", "eprintln!"),
        ("ohno!", "panic!"),
        ("meow!", "println!"),

        // Type aliases
        ("MaybeCheezburgr", "Option"),
        ("Cardboard", "Box"),
        ("EmptyBowl", "None"),
        ("Purrfect", "Ok"),
        ("Hairball", "Err"),

        // Keywords
        ("stickycat", "static"),
        ("waitforit", "await"),
        ("everycat", "pub"),
        ("cough up", "return"),
        ("copycat", "clone"),
        ("forever", "const"),
        ("flavurz", "enum"),
        ("zoomies", "loop"),
        ("wiggly", "mut"),
        ("around", "in"),
        ("skillz", "trait"),
        ("yoink", "move"),
        ("lazee", "async"),
        ("kinda", "type"),
        ("teech", "impl"),
        ("sniff", "match"),
        ("chase", "for"),
        ("again", "continue"),
        ("gimme", "use"),
        ("Tryz", "Result"),
        ("Yarn", "String"),
        ("Pile", "Vec"),
        ("loaf", "struct"),
        ("flop", "break"),
        ("yolo", "unsafe"),
        ("nope", "false"),
        ("Has", "Some"),
        ("Dis", "Self"),
        ("dis", "self"),
        ("yus", "true"),
        ("box", "mod"),
        ("iz", "fn"),
    ];

    // Sort by descending meow keyword length for longest-match-first
    mappings.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    mappings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mappings_sorted_by_length_descending() {
        let m = keyword_mappings();
        for w in m.windows(2) {
            assert!(
                w[0].0.len() >= w[1].0.len(),
                "\"{}\" (len {}) should come before \"{}\" (len {})",
                w[0].0, w[0].0.len(), w[1].0, w[1].0.len()
            );
        }
    }
}
