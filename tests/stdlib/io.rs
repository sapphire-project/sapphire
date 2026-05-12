use super::{VmValue, eval, eval_err};
use sapphire::vm::VmError;

#[test]
fn puts_returns_nil() {
    assert_eq!(eval(r#"IO.puts("hello")"#), VmValue::Nil);
}

#[test]
fn print_returns_nil() {
    assert_eq!(eval(r#"IO.print("hello")"#), VmValue::Nil);
}

#[test]
fn puts_type_error_on_non_string() {
    assert!(matches!(eval_err("IO.puts(42)"), VmError::TypeError { .. }));
}

#[test]
fn print_type_error_on_non_string() {
    assert!(matches!(eval_err("IO.print(42)"), VmError::TypeError { .. }));
}

#[test]
fn gets_type_error_on_wrong_arity() {
    assert!(matches!(eval_err(r#"IO.gets("extra")"#), VmError::TypeError { .. }));
}
