mod support;

use sapphire::vm::{VmError, VmValue};
use support::{eval, eval_err};

// ── Happy path ────────────────────────────────────────────────────────────────

#[test]
fn single_required_attr_provided() {
    let src = r#"class Foo { attr a }
Foo.new(a: 42).a"#;
    assert_eq!(eval(src), VmValue::Int(42));
}

#[test]
fn multiple_required_attrs_all_provided() {
    let src = r#"class Point { attr x
  attr y }
p = Point.new(x: 3, y: 4)
p.x + p.y"#;
    assert_eq!(eval(src), VmValue::Int(7));
}

#[test]
fn required_and_optional_attrs_all_provided() {
    let src = r#"class Foo { attr a
  attr b = 10 }
Foo.new(a: 1, b: 2).b"#;
    assert_eq!(eval(src), VmValue::Int(2));
}

#[test]
fn optional_attr_uses_default_when_omitted() {
    let src = r#"class Foo { attr a
  attr b = 99 }
Foo.new(a: 1).b"#;
    assert_eq!(eval(src), VmValue::Int(99));
}

#[test]
fn nil_default_attr_is_optional() {
    let src = r#"class Foo { attr a = nil }
Foo.new.a"#;
    assert_eq!(eval(src), VmValue::Nil);
}

#[test]
fn bool_default_attr_is_optional() {
    let src = r#"class Foo { attr flag = false }
Foo.new.flag"#;
    assert_eq!(eval(src), VmValue::Bool(false));
}

#[test]
fn no_attrs_class_new_with_no_args() {
    let src = r#"class Empty {}
e = Empty.new
42"#;
    assert_eq!(eval(src), VmValue::Int(42));
}

// ── Inherited required attrs ───────────────────────────────────────────────

#[test]
fn child_inherits_required_attr_from_parent() {
    let src = r#"class Animal { attr name }
class Dog < Animal {}
Dog.new(name: "Rex").name"#;
    assert_eq!(eval(src), VmValue::Str("Rex".to_string()));
}

#[test]
fn child_with_own_required_attr_and_inherited_required_attr() {
    let src = r#"class Animal { attr name }
class Dog < Animal { attr breed }
d = Dog.new(name: "Rex", breed: "Husky")
d.breed"#;
    assert_eq!(eval(src), VmValue::Str("Husky".to_string()));
}

// ── Error: missing required attr ──────────────────────────────────────────

#[test]
fn new_with_no_args_errors_when_attr_required() {
    let err = eval_err(
        r#"class Foo { attr a }
Foo.new"#,
    );
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("Foo.new"));
            assert!(message.contains("'a'"));
        }
        other => panic!("expected TypeError, got {:?}", other),
    }
}

#[test]
fn new_missing_one_of_two_required_attrs() {
    let err = eval_err(
        r#"class Foo { attr a
  attr b }
Foo.new(a: 1)"#,
    );
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("Foo.new"));
            assert!(message.contains("'b'"));
        }
        other => panic!("expected TypeError, got {:?}", other),
    }
}

#[test]
fn new_missing_required_attr_when_optional_attr_provided() {
    let err = eval_err(
        r#"class Foo { attr a
  attr b = 10 }
Foo.new(b: 20)"#,
    );
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("Foo.new"));
            assert!(message.contains("'a'"));
        }
        other => panic!("expected TypeError, got {:?}", other),
    }
}

#[test]
fn child_errors_when_inherited_required_attr_missing() {
    let err = eval_err(
        r#"class Animal { attr name }
class Dog < Animal {}
Dog.new"#,
    );
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("Dog.new"));
            assert!(message.contains("'name'"));
        }
        other => panic!("expected TypeError, got {:?}", other),
    }
}

// ── Error: positional args rejected ───────────────────────────────────────

#[test]
fn positional_arg_to_new_is_error() {
    let err = eval_err(
        r#"class Foo { attr a }
Foo.new(42)"#,
    );
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("Foo.new"));
            assert!(message.contains("keyword"));
        }
        other => panic!("expected TypeError, got {:?}", other),
    }
}

#[test]
fn positional_arg_on_no_attr_class_is_error() {
    let err = eval_err(
        r#"class Empty {}
Empty.new(1)"#,
    );
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("Empty.new"));
            assert!(message.contains("keyword"));
        }
        other => panic!("expected TypeError, got {:?}", other),
    }
}
