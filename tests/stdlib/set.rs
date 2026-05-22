use super::{VmValue, eval};

#[test]
fn new_empty() {
    assert_eq!(eval("Set.new().size()"), VmValue::Int(0));
    assert_eq!(eval("Set.new().empty?()"), VmValue::Bool(true));
}

#[test]
fn new_from_list_dedupes() {
    assert_eq!(eval("Set.new([1, 2, 2, 3]).size()"), VmValue::Int(3));
    assert_eq!(eval("Set.new([1, 2, 2, 3]).include?(2)"), VmValue::Bool(true));
}

#[test]
fn add_and_delete() {
    assert_eq!(
        eval("s = Set.new()\ns.add(1).add(2).add(1).size()"),
        VmValue::Int(2)
    );
    let src = "s = Set.new([1, 2, 3])\ns.delete(2)\ns.size()";
    assert_eq!(eval(src), VmValue::Int(2));
}

#[test]
fn include() {
    assert_eq!(eval("Set.new([1, 2]).include?(2)"), VmValue::Bool(true));
    assert_eq!(eval("Set.new([1, 2]).include?(9)"), VmValue::Bool(false));
}

#[test]
fn union_intersection_difference() {
    let src = "a = Set.new([1, 2])\nb = Set.new([2, 3])\na.union(b).size()";
    assert_eq!(eval(src), VmValue::Int(3));
    let src = "a = Set.new([1, 2, 3])\nb = Set.new([2, 3, 4])\na.intersection(b).size()";
    assert_eq!(eval(src), VmValue::Int(2));
    let src = "a = Set.new([1, 2, 3])\nb = Set.new([2, 3])\na.difference(b).size()";
    assert_eq!(eval(src), VmValue::Int(1));
}

#[test]
fn subset_superset_disjoint() {
    assert_eq!(
        eval("Set.new([1, 2]).subset?(Set.new([1, 2, 3]))"),
        VmValue::Bool(true)
    );
    assert_eq!(
        eval("Set.new([1, 2, 3]).superset?(Set.new([1, 2]))"),
        VmValue::Bool(true)
    );
    assert_eq!(
        eval("Set.new([1, 2]).disjoint?(Set.new([3, 4]))"),
        VmValue::Bool(true)
    );
    assert_eq!(
        eval("Set.new([1, 2]).disjoint?(Set.new([2, 3]))"),
        VmValue::Bool(false)
    );
}

#[test]
fn to_a() {
    assert_eq!(eval("Set.new([3, 1, 2]).to_a().size()"), VmValue::Int(3));
}

#[test]
fn select() {
    let src = "Set.new([1, 2, 3, 4]).select() { |x| x > 2 }.size()";
    assert_eq!(eval(src), VmValue::Int(2));
}

#[test]
fn reject() {
    let src = "Set.new([1, 2, 3]).reject() { |x| x == 2 }.size()";
    assert_eq!(eval(src), VmValue::Int(2));
}
