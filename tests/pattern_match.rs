mod support;

use sapphire::vm::VmValue;
use support::{eval, parse_err_msg};

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
