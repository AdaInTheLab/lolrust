/// All LolRust → Rust keyword mappings
/// Sorted by length (longest first) — VERY IMPORTANT
pub fn keyword_mappings() -> Vec<(&'static str, &'static str)> {
    let mut mappings = vec![
        // === Meowademy Beginner Friendly ===
        ("make wiggly", "let mut"),
        ("make", "let"),
        ("when", "if"),
        ("otherwise", "else"),
        ("repeat while", "while"),
        ("do this", "loop"),
        ("say", "println!"),
        ("bigsay", "println!"),
        ("yell", "eprintln!"),

        // === Classic LolRust ===
        ("or if ceiling cat sez", "else if"),
        ("if ceiling cat sez", "if"),
        ("or basement cat sez", "else"),
        ("i can haz wiggly", "let mut"),
        ("i can haz", "let"),
        ("meow!", "println!"),
        ("hisss!", "eprintln!"),
        ("ohno!", "panic!"),
        ("skritch dat", "match"),
        ("zoomzoom", "loop"),
        ("yeet", "return"),
        ("nap", "break"),
        ("flop", "break"),
        ("pounce", "impl"),
        ("furrever", "for"),
        ("prowl", "while"),

        // === Legacy aliases (0.1.x compatibility) ===
        // These are the original keywords from 0.1.x, kept so existing user
        // code keeps compiling. They map to the same Rust output as the newer
        // beginner-friendly forms above. NOTE: `zoomies` was deliberately NOT
        // restored ~ it collides with `zoomies` used as a variable name in
        // lesson content (would transpile to `let mut loop = ...`). Use
        // `zoomzoom` for loops going forward.
        ("pretend iz", "as"),
        ("but only if", "where"),
        ("stickycat", "static"),
        ("forever", "const"),
        ("gimme", "use"),
        ("sniff", "match"),
        ("teech", "impl"),
        ("kinda", "type"),
        ("chonk", "super"),
        ("box", "mod"),
        ("EmptyBowl", "None"),
        ("Has", "Some"),

        // === Core Rust ===
        ("loaf", "struct"),
        ("flavurz", "enum"),
        ("iz", "fn"),
        ("skillz", "trait"),
        ("Tryz", "Result"),
        ("Purrfect", "Ok"),
        ("Hairball", "Err"),
        ("Pile", "Vec"),
        ("Yarn", "String"),
        ("Cardboard", "Box"),
        ("MaybeCheezburgr", "Option"),
        ("wiggly", "mut"),
        ("everycat", "pub"),
        ("lazee", "async"),
        ("waitforit", "await"),
        ("chase", "for"),
        ("around", "in"),
        ("copycat", "clone"),
        ("yoink", "move"),
        ("yus", "true"),
        ("nope", "false"),
        ("dis", "self"),
        ("Dis", "Self"),
        ("bigchonk", "crate"),
        ("purrive", "#[derive"),
    ];

    mappings.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    mappings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted() {
        let m = keyword_mappings();
        for w in m.windows(2) {
            assert!(w[0].0.len() >= w[1].0.len());
        }
    }
}