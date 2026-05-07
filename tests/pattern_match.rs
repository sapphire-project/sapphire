use sapphire::compiler::compile;
use sapphire::lexer::Lexer;
use sapphire::parser::Parser;
use sapphire::vm::{Vm, VmValue};

fn eval(src: &str) -> VmValue {
    let tokens = Lexer::new(src).scan_tokens();
    let stmts = Parser::new(tokens).parse().expect("parse error");
    let func = compile(&stmts).expect("compile error");
    Vm::new(func, std::path::PathBuf::new())
        .run()
        .expect("vm error")
        .expect("empty stack")
}

fn parse_err_msg(src: &str) -> String {
    let tokens = sapphire::lexer::Lexer::new(src).scan_tokens();
    let err = sapphire::parser::Parser::new(tokens)
        .parse()
        .expect_err("expected parse error");
    format!("{}", err)
}

#[test]
fn match_literal_hit_first_arm() {
    let result = eval(r#"match 200 { 200 => { "OK" } 404 => { "Not Found" } _ => { "Other" } }"#);
    assert_eq!(result, VmValue::Str("OK".into()));
}

#[test]
fn match_literal_fall_through_to_wildcard() {
    let result = eval(r#"match 500 { 200 => { "OK" } 404 => { "Not Found" } _ => { "Other" } }"#);
    assert_eq!(result, VmValue::Str("Other".into()));
}

#[test]
fn match_multiple_values_per_arm() {
    let sat = eval(r#"match "Sat" { "Sat", "Sun" => { "Weekend" } _ => { "Weekday" } }"#);
    assert_eq!(sat, VmValue::Str("Weekend".into()));
    let sun = eval(r#"match "Sun" { "Sat", "Sun" => { "Weekend" } _ => { "Weekday" } }"#);
    assert_eq!(sun, VmValue::Str("Weekend".into()));
    let mon = eval(r#"match "Mon" { "Sat", "Sun" => { "Weekend" } _ => { "Weekday" } }"#);
    assert_eq!(mon, VmValue::Str("Weekday".into()));
}

#[test]
fn match_range_pattern() {
    let a = eval("match 95 { 90..100 => { 1 } 80..89 => { 2 } _ => { 3 } }");
    assert_eq!(a, VmValue::Int(1));
    let b = eval("match 85 { 90..100 => { 1 } 80..89 => { 2 } _ => { 3 } }");
    assert_eq!(b, VmValue::Int(2));
    let f = eval("match 70 { 90..100 => { 1 } 80..89 => { 2 } _ => { 3 } }");
    assert_eq!(f, VmValue::Int(3));
}

#[test]
fn match_guard_clause() {
    let huge = eval("match 150 { n if n > 100 => { \"huge\" } n if n > 10 => { \"big\" } _ => { \"small\" } }");
    assert_eq!(huge, VmValue::Str("huge".into()));
    let big = eval("match 42 { n if n > 100 => { \"huge\" } n if n > 10 => { \"big\" } _ => { \"small\" } }");
    assert_eq!(big, VmValue::Str("big".into()));
    let small = eval("match 5 { n if n > 100 => { \"huge\" } n if n > 10 => { \"big\" } _ => { \"small\" } }");
    assert_eq!(small, VmValue::Str("small".into()));
}

#[test]
fn match_bare_binding() {
    let result = eval(r#"match 7 { nil => { "nothing" } v => { v + 1 } }"#);
    assert_eq!(result, VmValue::Int(8));
}

#[test]
fn match_binding_nil() {
    let result = eval(r#"match nil { nil => { "nothing" } v => { "got it" } }"#);
    assert_eq!(result, VmValue::Str("nothing".into()));
}

#[test]
fn match_type_pattern() {
    let int_arm = eval(r#"match 5 { Int => { "int" } String => { "str" } _ => { "other" } }"#);
    assert_eq!(int_arm, VmValue::Str("int".into()));
    let str_arm = eval(r#"match "hi" { Int => { "int" } String => { "str" } _ => { "other" } }"#);
    assert_eq!(str_arm, VmValue::Str("str".into()));
    let other_arm = eval(r#"match 3.14 { Int => { "int" } String => { "str" } _ => { "other" } }"#);
    assert_eq!(other_arm, VmValue::Str("other".into()));
}

#[test]
fn match_list_destructuring() {
    let two_d = eval(r#"match [1, 2] { [x, y] => { "2D" } [x, y, z] => { "3D" } _ => { "nope" } }"#);
    assert_eq!(two_d, VmValue::Str("2D".into()));
    let three_d = eval(r#"match [1, 2, 3] { [x, y] => { "2D" } [x, y, z] => { "3D" } _ => { "nope" } }"#);
    assert_eq!(three_d, VmValue::Str("3D".into()));
    let nope = eval(r#"match [1] { [x, y] => { "2D" } [x, y, z] => { "3D" } _ => { "nope" } }"#);
    assert_eq!(nope, VmValue::Str("nope".into()));
}

#[test]
fn match_list_literal_element() {
    let result = eval(r#"match [42] { [42] => { "yes" } _ => { "no" } }"#);
    assert_eq!(result, VmValue::Str("yes".into()));
    let no = eval(r#"match [99] { [42] => { "yes" } _ => { "no" } }"#);
    assert_eq!(no, VmValue::Str("no".into()));
}

#[test]
fn match_list_range_element() {
    let b = eval(r#"match [85] { [90..100] => { "A" } [80..89] => { "B" } _ => { "F" } }"#);
    assert_eq!(b, VmValue::Str("B".into()));
}

#[test]
fn match_as_expression_assigned() {
    let result = eval(r#"x = match 42 { 42 => { "yes" } _ => { "no" } }
x"#);
    assert_eq!(result, VmValue::Str("yes".into()));
}

#[test]
fn match_nested() {
    let src = r#"
outer = match 1 {
  1 => {
    match "a" {
      "a" => { "one-a" }
      _   => { "one-other" }
    }
  }
  _ => { "other" }
}
outer
"#;
    assert_eq!(eval(src), VmValue::Str("one-a".into()));
}

#[test]
fn match_missing_wildcard_parse_error() {
    let msg = parse_err_msg("match 1 { 1 => { \"one\" } }");
    assert!(
        msg.contains("exhaustive") || msg.contains("wildcard") || msg.contains("_"),
        "expected exhaustiveness error, got: {msg}"
    );
}
