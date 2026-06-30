use sapphire::typechecker::{CheckedTypes, TypeChecker};

use super::parse_stmts;

pub fn typecheck_ok(src: &str) {
    let errors = TypeChecker::check(&parse_stmts(src));
    assert!(errors.is_empty(), "unexpected type errors: {:?}", errors);
}

pub fn typecheck_err_msg(src: &str) -> String {
    let errors = TypeChecker::check(&parse_stmts(src));
    assert!(!errors.is_empty(), "expected type errors for:\n{src}");
    errors[0].message.clone()
}

pub fn check_types_ok(src: &str) -> CheckedTypes {
    let info = TypeChecker::check_info(&parse_stmts(src));
    assert!(
        info.errors.is_empty(),
        "unexpected type errors: {:?}",
        info.errors
    );
    info.types
}
