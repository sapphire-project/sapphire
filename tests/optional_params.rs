mod support;

use sapphire::vm::{VmError, VmValue};
use support::{eval, eval_err};

#[test]
fn optional_param_uses_default() {
    let src = "def greet(name, prefix = \"Hello\") { prefix + \", \" + name }\ngreet(\"World\")";
    assert_eq!(eval(src), VmValue::Str("Hello, World".into()));
}

#[test]
fn optional_param_overridden_by_caller() {
    let src =
        "def greet(name, prefix = \"Hello\") { prefix + \", \" + name }\ngreet(\"World\", \"Hi\")";
    assert_eq!(eval(src), VmValue::Str("Hi, World".into()));
}

#[test]
fn optional_param_int_default() {
    let src = "def add(a, b = 10) { a + b }\nadd(5)";
    assert_eq!(eval(src), VmValue::Int(15));
}

#[test]
fn optional_param_nil_default() {
    let src = "def f(x = nil) { x }\nf()";
    assert_eq!(eval(src), VmValue::Nil);
}

#[test]
fn optional_param_bool_default() {
    let src = "def f(x = true) { x }\nf()";
    assert_eq!(eval(src), VmValue::Bool(true));
}

#[test]
fn optional_method_param_uses_default() {
    let src = r#"
class C {
  def greet(name, greeting = "Hi") { greeting + ", " + name }
}
C.new().greet("Alice")"#;
    assert_eq!(eval(src), VmValue::Str("Hi, Alice".into()));
}

#[test]
fn optional_param_too_few_args_errors() {
    let err = eval_err("def f(a, b = 1) { a + b }\nf()");
    assert!(matches!(err, VmError::TypeError { .. }));
}

#[test]
fn optional_param_too_many_args_errors() {
    let err = eval_err("def f(a, b = 1) { a + b }\nf(1, 2, 3)");
    assert!(matches!(err, VmError::TypeError { .. }));
}
