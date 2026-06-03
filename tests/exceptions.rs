mod support;

use sapphire::error::SapphireError;
use sapphire::vm::{VmError, VmValue};
use support::{eval, eval_err, parse_err};

#[test]
fn raise_unhandled() {
    let err = eval_err(r#"raise "oops""#);
    assert!(matches!(err, VmError::Raised(..)));
}

#[test]
fn try_rescue_else_ensure() {
    let src = r#"x = 0
result = try {
  x = x + 1
  10
} rescue e {
  0
} else {
  x = x + 10
  20
} ensure {
  x = x + 100
}
x + result"#;
    assert_eq!(eval(src), VmValue::Int(131));
}

#[test]
fn multiple_typed_rescue_handlers() {
    let src = r#"class IoError {}
class ParseError {}
def handle(kind) {
  try {
    if kind == 0 { raise IoError.new() }
    raise ParseError.new()
  } rescue e : IoError {
    1
  } rescue e {
    2
  }
}
handle(0) * 10 + handle(1)"#;
    assert_eq!(eval(src), VmValue::Int(12));
}

#[test]
fn inline_rescue_assigns_fallback() {
    let src = r#"x = 1 / 0 rescue 7
x"#;
    assert_eq!(eval(src), VmValue::Int(7));
}

#[test]
fn method_suffix_rescue() {
    let src = r#"def risky(x: Int): Int {
  if x < 0 { raise "bad" }
  x * 2
} rescue e {
  0
}
risky(5) + risky(-1)"#;
    assert_eq!(eval(src), VmValue::Int(10));
}

#[test]
fn try_inside_while_rescue_and_ensure() {
    let src = r#"i = 0
while i < 3 {
  try {
    i = i + 1
    if i == 1 { raise "bad" }
  } rescue e {
    i = 10
  } ensure {
    i = i + 1
  }
}
i"#;
    assert_eq!(eval(src), VmValue::Int(11));
}

#[test]
fn call_with_block_suffix_rescue() {
    let src = r#"def with_block() {
  yield()
}
with_block { raise "bad" } rescue { 7 }"#;
    assert_eq!(eval(src), VmValue::Int(7));
}

#[test]
fn if_while_suffix_rescue_parse_error() {
    for src in [
        r#"if true { 1 } rescue { 2 }"#,
        r#"while false { 1 } rescue { 2 }"#,
    ] {
        let err = parse_err(src);
        assert!(matches!(
            err,
            SapphireError::ParseError { ref message, .. }
                if message == "use 'try { ... }' to rescue if/while bodies"
        ));
    }
}

#[test]
fn inline_rescue_in_function() {
    let src = r#"def risky(x) {
  if x < 0 { raise "bad" }
  x * 2
rescue e
  0
}
risky(5)"#;
    assert_eq!(eval(src), VmValue::Int(10));
    let src2 = r#"def risky(x) {
  if x < 0 { raise "bad" }
  x * 2
rescue e
  0
}
risky(-1)"#;
    assert_eq!(eval(src2), VmValue::Int(0));
}

#[test]
fn inline_rescue_binds_error() {
    let src = r#"def boom() {
  raise "oops"
  1
rescue e
  e
}
boom()"#;
    assert_eq!(eval(src), VmValue::Str("oops".into()));
}

#[test]
fn inline_rescue_in_method() {
    let src = r#"class Safe {
  def try_div(x) {
    10 / x
  rescue e
    -1
  }
}
Safe.new().try_div(2)"#;
    assert_eq!(eval(src), VmValue::Int(5));
    let src2 = r#"class Safe {
  def try_div(x) {
    10 / x
  rescue e
    -1
  }
}
Safe.new().try_div(0)"#;
    assert_eq!(eval(src2), VmValue::Int(-1));
}

#[test]
fn raise_instance() {
    let src = r#"class Err { attr msg }
result = try {
  raise Err.new(msg: "bad")
} rescue e {
  e.msg
}
result"#;
    assert_eq!(eval(src), VmValue::Str("bad".into()));
}
