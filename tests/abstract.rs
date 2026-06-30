mod support;

use sapphire::vm::{VmError, VmValue};
use support::{eval, eval_err, parse_err_msg};

#[test]
fn abstract_class_new_is_error() {
    let err = eval_err("abstract class Shape {\n  def area() -> Float\n}\nShape.new()");
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("cannot instantiate abstract class Shape"));
        }
        other => panic!("expected TypeError, got {:?}", other),
    }
}

#[test]
fn concrete_subclass_missing_abstract_method_errors_at_new() {
    let err = eval_err(
        "abstract class Shape {\n  def area() -> Float\n  def perimeter() -> Float\n}\nclass Broken < Shape {\n  def area() -> Float { 0.0 }\n}\nBroken.new()",
    );
    match err {
        VmError::TypeError { message, .. } => {
            assert!(message.contains("Broken"));
            assert!(message.contains("perimeter"));
        }
        other => panic!("expected TypeError, got {:?}", other),
    }
}

#[test]
fn concrete_subclass_implements_abstract_methods() {
    let src = "abstract class Shape {\n  def area() -> Float\n}\nclass Square < Shape {\n  def area() -> Float { 4.0 }\n}\nSquare.new().area()";
    assert_eq!(eval(src), VmValue::Float(4.0));
}

#[test]
fn abstract_partial_parent_concrete_grandchild() {
    let src = "abstract class A {\n  def foo() -> Int\n  def bar() -> Int\n}\nabstract class B < A {\n  def foo() -> Int { 1 }\n}\nclass C < B {\n  def bar() -> Int { 2 }\n}\nC.new().foo() + C.new().bar()";
    assert_eq!(eval(src), VmValue::Int(3));
}

#[test]
fn concrete_method_on_abstract_class_callable_on_subclass() {
    let src = "abstract class Base {\n  def m() -> Int\n  def helper() -> Int { 10 }\n}\nclass Child < Base {\n  def m() -> Int { self.helper() + 1 }\n}\nChild.new().m()";
    assert_eq!(eval(src), VmValue::Int(11));
}

#[test]
fn abstract_keyword_before_def_is_parse_error() {
    let msg = parse_err_msg("class C {\n  abstract def m() -> Int\n}");
    assert!(
        msg.contains("expected"),
        "expected parse error message, got: {msg}"
    );
}

#[test]
fn concrete_class_must_implement_inherited_abstract() {
    assert_typecheck_error!(
        "abstract class A {\n  def m() -> Int\n}\nclass B < A {\n}",
        "must implement abstract method: m",
    );
}
