use sapphire::compiler::compile;
use sapphire::lexer::Lexer;
use sapphire::parser::Parser;
use sapphire::vm::{Vm, VmError, VmValue};
use std::path::PathBuf;

fn eval(src: &str) -> VmValue {
    let tokens = Lexer::new(src).scan_tokens();
    let stmts = Parser::new(tokens).parse().expect("parse error");
    let func = compile(&stmts).expect("compile error");
    Vm::new(func, PathBuf::new())
        .run()
        .expect("vm error")
        .expect("empty stack")
}

fn eval_err(src: &str) -> VmError {
    let tokens = Lexer::new(src).scan_tokens();
    let stmts = Parser::new(tokens).parse().expect("parse error");
    let func = compile(&stmts).expect("compile error");
    Vm::new(func, PathBuf::new())
        .run()
        .expect_err("expected VM error")
}

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
