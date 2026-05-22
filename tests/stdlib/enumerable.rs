use super::{VmValue, eval};

#[test]
fn all_when_every_element_matches() {
    assert_eq!(
        eval("[1, 2, 3].all?() { |x| x > 0 }"),
        VmValue::Bool(true)
    );
}

#[test]
fn all_when_one_element_fails() {
    assert_eq!(
        eval("[1, 2, 3].all?() { |x| x < 3 }"),
        VmValue::Bool(false)
    );
}

#[test]
fn all_on_empty_list() {
    assert_eq!(eval("[].all?() { |x| false }"), VmValue::Bool(true));
}

#[test]
fn any_when_one_element_matches() {
    assert_eq!(
        eval("[1, 2, 3].any?() { |x| x > 2 }"),
        VmValue::Bool(true)
    );
}

#[test]
fn any_when_none_match() {
    assert_eq!(
        eval("[1, 2, 3].any?() { |x| x > 9 }"),
        VmValue::Bool(false)
    );
}

#[test]
fn any_on_empty_list() {
    assert_eq!(eval("[].any?() { |x| true }"), VmValue::Bool(false));
}

#[test]
fn none_when_no_element_matches() {
    assert_eq!(
        eval("[1, 2, 3].none?() { |x| x > 9 }"),
        VmValue::Bool(true)
    );
}

#[test]
fn none_when_one_matches() {
    assert_eq!(
        eval("[1, 2, 3].none?() { |x| x == 2 }"),
        VmValue::Bool(false)
    );
}

#[test]
fn find_returns_first_match() {
    assert_eq!(
        eval("[1, 2, 3, 4].find() { |x| x > 2 }"),
        VmValue::Int(3)
    );
}

#[test]
fn find_returns_nil_when_missing() {
    assert_eq!(
        eval("[1, 2, 3].find() { |x| x > 9 }"),
        VmValue::Nil
    );
}

#[test]
fn enumerable_include_on_list() {
    assert_eq!(eval("[1, 2, 3].include?(2)"), VmValue::Bool(true));
    assert_eq!(eval("[1, 2, 3].include?(9)"), VmValue::Bool(false));
}
