/// Translates rustc error messages into lolcat speak.
/// Takes the raw stderr output from rustc and returns a cat-ified version.
pub fn explain_errors(rustc_output: &str) -> String {
    let mut output = rustc_output.to_string();

    // Error-level translations (order matters - longer first)
    let translations = [
        // Borrow checker errors - the no touchie checker
        ("cannot borrow", "NO TOUCHIE! cannot borrow"),
        ("cannot move out of", "HEY! dat iz not urs to yoink from"),
        ("does not live long enough", "died too soon :( not enuff lives"),
        ("already borrowed", "sumone else iz already sniffing"),
        ("moved here", "got yoinked here"),
        ("value used here after move", "u tried to use it after it got yoinked! D:"),
        ("borrow of moved value", "u tried to sniff sumthing dat got yoinked"),
        ("cannot assign twice to immutable variable", "NO! dat iz not wiggly! u cant change it"),
        ("cannot assign to", "NO TOUCHIE! dat iz not wiggly"),
        ("immutable", "not wiggly"),
        ("mutable", "wiggly"),

        // Type errors
        ("mismatched types", "WRONG FLAVOR! mismatched types"),
        ("expected type", "i wanted dis flavor"),
        ("found type", "but u gave me dis flavor"),
        ("type mismatch", "FLAVORS DONT MATCH"),
        ("the trait bound", "dis kitteh dont haz da skillz"),
        ("doesn't implement", "never learned how to"),
        ("is not implemented for", "iz not teached to"),
        ("no method named", "dis kitteh dont know how to"),
        ("not found in this scope", "i cant see dat from here! iz it hiding?"),
        ("not found", "WHERE IZ IT?? not found"),

        // Lifetime errors
        ("lifetime", "how many lives left"),
        ("'static", "forever-lives"),
        ("does not fulfill", "not enuff lives to do dat"),
        ("outlives", "has more lives than"),

        // Common errors
        ("unused variable", "u made dis but never played with it"),
        ("unused import", "u asked for dis toy but never played with it"),
        ("dead code", "dis code iz sleeping forever"),
        ("unreachable pattern", "dis pattern iz in a cardboard box no one can reach"),
        ("missing field", "u forgot to put sumthing in da loaf!"),
        ("no such field", "dat loaf dont haz dat!"),
        ("expected struct", "i wanted a loaf"),
        ("expected enum", "i wanted flavurz"),
        ("this function takes", "dis iz needs"),
        ("supplied", "but u gave"),
        ("arguments were supplied", "treats were given"),
        ("expected", "i wanted"),
        ("found", "but i got"),

        // Macro errors
        ("cannot find macro", "WHERE IZ DA MACRO?? lost it"),
        ("unexpected end of macro", "da macro ended too soon! D:"),

        // Module errors
        ("unresolved import", "i said gimme but it wasnt there"),
        ("could not find", "looked everywhere but couldnt find"),

        // Misc
        ("consider", "maybe try"),
        ("help:", "HALP:"),
        ("error[", "OH NOES["),
        ("error:", "OH NOES:"),
        ("warning:", "CAREFUL KITTEH:"),
        ("note:", "btw:"),
        (" --> ", " --> "),  // keep file locations as-is
    ];

    for (rust_msg, cat_msg) in translations {
        output = output.replace(rust_msg, cat_msg);
    }

    // Add a header
    let header = r#"
   /\_/\  ~ THE NO TOUCHIE CHECKER HAS SPOKEN ~
  ( >_< )
   > ^ <  here iz wat went wrong:
  /|   |\
 (_|   |_)
"#;

    format!("{}{}", header, output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_borrow_error() {
        let input = "error: cannot borrow `x` as mutable";
        let result = explain_errors(input);
        assert!(result.contains("NO TOUCHIE!"));
        assert!(result.contains("wiggly"));
    }

    #[test]
    fn translates_type_mismatch() {
        let input = "error: mismatched types";
        let result = explain_errors(input);
        assert!(result.contains("WRONG FLAVOR!"));
    }

    #[test]
    fn translates_not_found() {
        let input = "error: not found in this scope";
        let result = explain_errors(input);
        assert!(result.contains("hiding"));
    }

    #[test]
    fn translates_unused_variable() {
        let input = "warning: unused variable: `x`";
        let result = explain_errors(input);
        assert!(result.contains("never played with it"));
    }

    #[test]
    fn adds_cat_header() {
        let result = explain_errors("error: something");
        assert!(result.contains("NO TOUCHIE CHECKER"));
    }

    #[test]
    fn translates_move_error() {
        let input = "error: cannot move out of `x`";
        let result = explain_errors(input);
        assert!(result.contains("yoink"));
    }
}
