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
fn puts_coerces_int() {
    assert_eq!(eval("IO.puts(42)"), VmValue::Nil);
}

#[test]
fn puts_coerces_bool() {
    assert_eq!(eval("IO.puts(true)"), VmValue::Nil);
}

#[test]
fn puts_coerces_nil_value() {
    assert_eq!(eval("IO.puts(nil)"), VmValue::Nil);
}

#[test]
fn gets_wrong_arity_is_type_error() {
    assert!(matches!(
        eval_err(r#"IO.gets("extra")"#),
        VmError::TypeError { .. }
    ));
}
