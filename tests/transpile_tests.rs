use lolrust::transpiler::transpile;

#[test]
fn test_hello_world() {
    let input = r#"iz main() {
    meow!("hello world");
}"#;
    let expected = r#"fn main() {
    println!("hello world");
}"#;
    assert_eq!(transpile(input), expected);
}

#[test]
fn test_let_and_let_mut() {
    assert_eq!(transpile("i can haz x = 5;"), "let x = 5;");
    assert_eq!(transpile("i can haz wiggly x = 5;"), "let mut x = 5;");
}

#[test]
fn test_string_literals_not_replaced() {
    assert_eq!(
        transpile(r#"meow!("i can haz cheezburger iz yus");"#),
        r#"println!("i can haz cheezburger iz yus");"#
    );
}

#[test]
fn test_line_comments_not_replaced() {
    let input = "// iz a comment with meow! and zoomies\niz main() {}";
    let expected = "// iz a comment with meow! and zoomies\nfn main() {}";
    assert_eq!(transpile(input), expected);
}

#[test]
fn test_block_comments_not_replaced() {
    assert_eq!(
        transpile("/* iz not replaced */ iz main() {}"),
        "/* iz not replaced */ fn main() {}"
    );
}

#[test]
fn test_nested_block_comments() {
    assert_eq!(
        transpile("/* outer /* iz inner */ iz still comment */ iz main() {}"),
        "/* outer /* iz inner */ iz still comment */ fn main() {}"
    );
}

#[test]
fn test_word_boundary_prevents_partial_replacement() {
    // "wizard" contains "iz" but should NOT be replaced
    assert_eq!(transpile("i can haz wizard = 1;"), "let wizard = 1;");
    // "sniffing" contains "sniff" but should NOT be replaced
    assert_eq!(transpile("i can haz sniffing = yus;"), "let sniffing = true;");
    // "dismiss" contains "dis" but should NOT be replaced
    assert_eq!(transpile("dismiss();"), "dismiss();");
}

#[test]
fn test_else_if_matched_before_if() {
    let input = "if ceiling cat sez x > 0 {\n} or if ceiling cat sez x < 0 {\n} or basement cat sez {\n}";
    let expected = "if x > 0 {\n} else if x < 0 {\n} else {\n}";
    assert_eq!(transpile(input), expected);
}

#[test]
fn test_for_in_loop() {
    assert_eq!(
        transpile("chase i around 0..10 {}"),
        "for i in 0..10 {}"
    );
}

#[test]
fn test_match_expression() {
    let input = "sniff x {\n    Has(v) => v,\n    EmptyBowl => 0,\n}";
    let expected = "match x {\n    Some(v) => v,\n    None => 0,\n}";
    assert_eq!(transpile(input), expected);
}

#[test]
fn test_result_types() {
    assert_eq!(transpile("Purrfect(42)"), "Ok(42)");
    assert_eq!(transpile("Hairball(e)"), "Err(e)");
    assert_eq!(transpile("Tryz<i32, Yarn>"), "Result<i32, String>");
}

#[test]
fn test_struct_and_impl() {
    assert_eq!(transpile("loaf Cat {}"), "struct Cat {}");
    assert_eq!(transpile("teech Cat {}"), "impl Cat {}");
}

#[test]
fn test_escape_sequences_in_strings() {
    assert_eq!(
        transpile(r#"meow!("say \"iz\" loudly");"#),
        r#"println!("say \"iz\" loudly");"#
    );
}

#[test]
fn test_ampersand_mut() {
    assert_eq!(transpile("&wiggly x"), "&mut x");
}

#[test]
fn test_pretend_iz_vs_iz() {
    assert_eq!(transpile("x pretend iz i32"), "x as i32");
}

#[test]
fn test_full_program() {
    let input = r#"gimme std::io;

iz main() {
    i can haz wiggly count: i32 = 0;

    zoomies {
        if ceiling cat sez count >= 3 {
            meow!("done! count iz {}", count);
            flop;
        }
        count = count + 1;
    }
}"#;
    let expected = r#"use std::io;

fn main() {
    let mut count: i32 = 0;

    loop {
        if count >= 3 {
            println!("done! count iz {}", count);
            break;
        }
        count = count + 1;
    }
}"#;
    assert_eq!(transpile(input), expected);
}
