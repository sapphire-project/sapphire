mod support;

use sapphire::vm::{VmError, VmValue};
use support::{eval, eval_err};

// ── Parameter type checking ────────────────────────────────────────────────────

#[test]
fn param_type_ok_plain_function() {
    assert_eq!(
        eval("def greet(name: String) { name }\ngreet(\"Alice\")"),
        VmValue::Str("Alice".into())
    );
}

#[test]
fn param_type_wrong_plain_function() {
    let err = eval_err("def greet(name: String) { name }\ngreet(42)");
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("argument 1"), "missing arg position: {message}");
            assert!(message.contains("greet"), "missing fn name: {message}");
            assert!(message.contains("String"), "missing expected type: {message}");
            assert!(message.contains("Int"), "missing actual type: {message}");
        }
        other => panic!("expected TypeError, got {other:?}"),
    }
}

#[test]
fn param_type_multiple_params_checks_each() {
    let err = eval_err("def add(a: Int, b: Int) { a + b }\nadd(1, \"two\")");
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("argument 2"), "should report second arg: {message}");
        }
        other => panic!("expected TypeError, got {other:?}"),
    }
}

#[test]
fn param_type_unannotated_accepts_anything() {
    assert_eq!(eval("def f(x) { x }\nf(42)"), VmValue::Int(42));
    assert_eq!(eval("def f(x) { x }\nf(\"hi\")"), VmValue::Str("hi".into()));
}

#[test]
fn param_type_union_accepts_either_arm() {
    assert_eq!(eval("def f(x: Int | String) { x }\nf(1)"), VmValue::Int(1));
    assert_eq!(
        eval("def f(x: Int | String) { x }\nf(\"hi\")"),
        VmValue::Str("hi".into())
    );
}

#[test]
fn param_type_union_rejects_third_type() {
    let err = eval_err("def f(x: Int | String) { x }\nf(true)");
    assert!(matches!(err, VmError::TypeError { .. }));
}

#[test]
fn param_type_literal_union_accepts_matching_value() {
    let src = "def pick(mode: \"dev\" | \"prod\") { mode }\npick(\"dev\")";
    assert_eq!(eval(src), VmValue::Str("dev".into()));
}

#[test]
fn param_type_instance_method() {
    let src = "class Greeter {\n  def greet(name: String) { \"Hello, \" + name }\n}\ng = Greeter.new()\ng.greet(\"World\")";
    assert_eq!(eval(src), VmValue::Str("Hello, World".into()));
}

#[test]
fn param_type_instance_method_wrong_type() {
    let src = "class Greeter {\n  def greet(name: String) { \"Hello, \" + name }\n}\ng = Greeter.new()\ng.greet(99)";
    let err = eval_err(src);
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("argument 1"), "missing arg position: {message}");
            assert!(message.contains("String"), "missing expected type: {message}");
        }
        other => panic!("expected TypeError, got {other:?}"),
    }
}

#[test]
fn param_type_generic_erases_to_any() {
    assert_eq!(
        eval("def identity[T](x: T) -> T { x }\nidentity(42)"),
        VmValue::Int(42)
    );
    assert_eq!(
        eval("def identity[T](x: T) -> T { x }\nidentity(\"hi\")"),
        VmValue::Str("hi".into())
    );
}

#[test]
fn param_type_nullable_accepts_nil() {
    assert_eq!(eval("def f(x: Int?) { x }\nf(nil)"), VmValue::Nil);
    assert_eq!(eval("def f(x: Int?) { x }\nf(5)"), VmValue::Int(5));
}

// ── Int → Float promotion ──────────────────────────────────────────────────────

#[test]
fn param_type_int_promotes_to_float() {
    assert_eq!(
        eval("def foo(n: Float) { n }\nfoo(4)"),
        VmValue::Float(4.0)
    );
}

#[test]
fn param_type_int_to_float_enables_float_arithmetic() {
    assert_eq!(
        eval("def foo(n: Float) { n + 0.5 }\nfoo(4)"),
        VmValue::Float(4.5)
    );
}

#[test]
fn param_type_int_to_float_method() {
    let src = "class Scaler {\n  def scale(n: Float) { n * 2.0 }\n}\nScaler.new().scale(3)";
    assert_eq!(eval(src), VmValue::Float(6.0));
}

#[test]
fn param_type_int_to_float_multiple_params() {
    assert_eq!(
        eval("def add(a: Float, b: Float) { a + b }\nadd(1, 2)"),
        VmValue::Float(3.0)
    );
}

#[test]
fn param_type_float_still_accepted_directly() {
    assert_eq!(
        eval("def foo(n: Float) { n }\nfoo(4.0)"),
        VmValue::Float(4.0)
    );
}

#[test]
fn param_type_int_does_not_promote_to_string() {
    let err = eval_err("def f(x: String) { x }\nf(42)");
    assert!(matches!(err, VmError::TypeError { .. }));
}
