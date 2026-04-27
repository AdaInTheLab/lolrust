use rand::Rng;

/// Translates scary rustc errors into friendly (but still chaotic) cat speak
pub fn explain_errors(rustc_output: &str) -> String {
    let mut output = rustc_output.to_string();

    let translations = [
        // === No Touchie Checker (Borrow Checker) ===
        ("cannot borrow", "NO TOUCHIE! cannot borrow"),
        ("cannot move out of", "HEY! U CANT YOINK DAT FROM THERE"),
        ("does not live long enough", "dis kitteh ran out of lives too soon :("),
        ("already borrowed", "someone else is already petting this!!"),
        ("moved here", "it got yoinked right here"),
        ("value used here after move", "u tried to use it AFTER it was yoinked!!"),
        ("borrow of moved value", "u cant pet something that already got yoinked"),

        // === Beginner Friendly Messages ===
        ("expected `;`", "you forgot the ; at the end of the line kitteh!"),
        ("missing semicolon", "put a ; at the end silly goose"),
        ("cannot find value", "WHERE IZ IT?? i don't know what that is... did u make it earlier?"),
        ("not found in this scope", "WHERE IZ IT?? i can't see that from here"),
        ("no method named", "dis kitteh doesn't know that trick yet"),
        ("mismatched types", "WRONG FLAVOR!! types don't match"),
        ("expected type", "i expected dis flavor"),
        ("found type", "but u gave me dis other flavor"),

        // === General Help ===
        ("consider", "maybe try dis instead:"),
        ("help:", "HALP TIP:"),
        ("error[", "OH NOES["),
        ("error:", "OH NOES:"),
        ("warning:", "CAREFUL KITTEH:"),
        ("note:", "btw:"),
        ("unused variable", "u made a toy but never played with it :("),
        ("unused import", "u asked for a toy but never opened the box"),
        ("dead code", "dis code is napping forever"),

        // === Function / Argument stuff ===
        ("this function takes", "dis iz needs"),
        ("arguments were supplied", "but u gave"),
        ("too many arguments", "u gave too many treats!!"),
        ("not enough arguments", "u forgot some treats..."),
    ];

    for (rust, cat) in translations {
        output = output.replace(rust, cat);
    }

    // Add random cat reaction at the top
    let reactions = [
        "\n/\\_/\\   THE NO TOUCHIE CHECKER HAS SPOKEN\n",
        "\n(>_<)   u did a little whoopsie kitteh...\n",
        "\n> ^ <   here's what went wrong hooman:\n",
        "\n/|   |\\  try again brave kitteh!!\n",
        "\n🐾   don't give up! even big chonks make mistakes\n",
    ];

    let random_reaction = reactions[rand::thread_rng().gen_range(0..reactions.len())];

    format!("{}{}", random_reaction, output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semicolon_help() {
        let input = "error: expected `;`, found `}`";
        let result = explain_errors(input);
        assert!(result.contains("you forgot the ;"));
    }

    #[test]
    fn test_variable_not_found() {
        let input = "error[E0425]: cannot find value `fluffy` in this scope";
        let result = explain_errors(input);
        assert!(result.contains("WHERE IZ IT"));
    }

    #[test]
    fn test_no_touchie() {
        let input = "error: cannot borrow `x` as mutable";
        let result = explain_errors(input);
        assert!(result.contains("NO TOUCHIE!"));
    }
}