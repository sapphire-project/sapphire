use super::{VmValue, eval};

#[test]
fn size() {
    assert_eq!(eval("a = [1,2,3]\na.size()"), VmValue::Int(3));
    assert_eq!(eval("[].size()"), VmValue::Int(0));
}

#[test]
fn first_last() {
    assert_eq!(eval("a = [1,2,3]\na.first()"), VmValue::Int(1));
    assert_eq!(eval("a = [1,2,3]\na.last()"), VmValue::Int(3));
}

#[test]
fn empty() {
    assert_eq!(eval("[].empty?()"), VmValue::Bool(true));
    assert_eq!(eval("[1,2].empty?()"), VmValue::Bool(false));
}

#[test]
fn include() {
    assert_eq!(eval("a = [1,2,3]\na.include?(2)"), VmValue::Bool(true));
    assert_eq!(eval("a = [1,2,3]\na.include?(5)"), VmValue::Bool(false));
}

#[test]
fn sort() {
    assert_eq!(eval("a = [3,1,2]\na.sort().first()"), VmValue::Int(1));
    assert_eq!(
        eval("result = [3, 1, 4, 1, 5, 9, 2].sort()\nresult[0]"),
        VmValue::Int(1)
    );
    assert_eq!(
        eval("result = [3, 1, 4, 1, 5, 9, 2].sort()\nresult[6]"),
        VmValue::Int(9)
    );
}

#[test]
fn sort_strings() {
    assert_eq!(
        eval(r#"result = ["banana", "apple", "cherry"].sort(); result[0]"#),
        VmValue::Str("apple".into())
    );
    assert_eq!(
        eval(r#"result = ["banana", "apple", "cherry"].sort(); result[2]"#),
        VmValue::Str("cherry".into())
    );
}

#[test]
fn join() {
    assert_eq!(eval(r#"[1,2,3].join(",")"#), VmValue::Str("1,2,3".into()));
}

#[test]
fn push_pop() {
    assert_eq!(eval("a = [1,2]\na.append(3)\na.size()"), VmValue::Int(3));
    assert_eq!(eval("a = [1,2,3]\na.pop()"), VmValue::Int(3));
    assert_eq!(eval("a = [1,2,3]\na.pop()\na.size()"), VmValue::Int(2));
}

#[test]
fn each() {
    let src = "a = [1,2,3]\nsum = 0\na.each() { |x| sum = sum + x }\nsum";
    assert_eq!(eval(src), VmValue::Int(6));
}

#[test]
fn reduce_with_initial() {
    let src = "[1, 2, 3, 4, 5].reduce(0) { |acc, n| acc + n }";
    assert_eq!(eval(src), VmValue::Int(15));
}

#[test]
fn reduce_without_initial() {
    let src = "[1, 2, 3, 4, 5].reduce() { |acc, n| acc * n }";
    assert_eq!(eval(src), VmValue::Int(120));
}

#[test]
fn flatten() {
    assert_eq!(
        eval("result = [[1, 2], [3, [4, 5]]].flatten()\nresult.size()"),
        VmValue::Int(5)
    );
    assert_eq!(
        eval("result = [[1, 2], [3, [4, 5]]].flatten()\nresult[3]"),
        VmValue::Int(4)
    );
}

#[test]
fn uniq() {
    let src = "result = [1, 2, 2, 3, 1].uniq()\nresult.size()";
    assert_eq!(eval(src), VmValue::Int(3));
}

#[test]
fn select() {
    let src = "result = [1, 2, 3, 4].select() { |x| x > 2 }\nresult.size()";
    assert_eq!(eval(src), VmValue::Int(2));
}

#[test]
fn each_with_index() {
    let src = r#"pairs = []
["a", "b", "c"].each_with_index() { |item, i| pairs.append(i) }
pairs[2]"#;
    assert_eq!(eval(src), VmValue::Int(2));
}

#[test]
fn zip() {
    let src = "result = [1, 2, 3].zip([4, 5, 6])\nresult.size()";
    assert_eq!(eval(src), VmValue::Int(3));
    let src2 = "result = [1, 2, 3].zip([4, 5, 6])\nresult[0][0]";
    assert_eq!(eval(src2), VmValue::Int(1));
    let src3 = "result = [1, 2, 3].zip([4, 5, 6])\nresult[0][1]";
    assert_eq!(eval(src3), VmValue::Int(4));
}

#[test]
fn implicit_it_in_each() {
    let src = "sum = 0\n[1, 2, 3].each() { |it| sum = sum + it }\nsum";
    assert_eq!(eval(src), VmValue::Int(6));
}

#[test]
fn implicit_it_in_map() {
    let src = "result = [1, 2, 3].map() { |it| it * 2 }\nresult[1]";
    assert_eq!(eval(src), VmValue::Int(4));
}

#[test]
fn each_next() {
    let src = "sum = 0\n[1, 2, 3, 4, 5].each() { |x| if x == 3 { next nil }\nsum = sum + x }\nsum";
    assert_eq!(eval(src), VmValue::Int(12));
}

#[test]
fn chain_map_then_size() {
    let src = "[1, 2, 3].map() { |x| x * 2 }.size()";
    assert_eq!(eval(src), VmValue::Int(3));
}

#[test]
fn chain_map_then_index() {
    let src = "[1, 2, 3].map() { |x| x * 10 }[1]";
    assert_eq!(eval(src), VmValue::Int(20));
}

#[test]
fn chain_map_then_map() {
    let src = "[1, 2, 3].map() { |x| x * 2 }.map() { |x| x + 1 }[0]";
    assert_eq!(eval(src), VmValue::Int(3));
}

#[test]
fn chain_multiline_map_then_size() {
    let src = "[1, 2, 3]\n  .map() { |x| x * 2 }\n  .size()";
    assert_eq!(eval(src), VmValue::Int(3));
}

#[test]
fn chain_multiline_map_then_map() {
    let src = "result = [1, 2, 3]\n  .map() { |x| x * 2 }\n  .map() { |x| x + 1 }\nresult[2]";
    assert_eq!(eval(src), VmValue::Int(7));
}

#[test]
fn chain_multiline_select_then_size() {
    let src = "[1, 2, 3, 4]\n  .select() { |x| x > 2 }\n  .size()";
    assert_eq!(eval(src), VmValue::Int(2));
}

#[test]
fn chain_multiline_map_then_select() {
    let src = "[1, 2, 3, 4]\n  .map() { |x| x * 2 }\n  .select() { |x| x > 4 }\n  .size()";
    assert_eq!(eval(src), VmValue::Int(2));
}

#[test]
fn break_in_each_stops_iteration() {
    let src = r#"
sum = 0
[1, 2, 3, 4, 5].each { |n|
  break if n == 3
  sum = sum + n
}
sum"#;
    assert_eq!(eval(src), VmValue::Int(3));
}

#[test]
fn break_in_each_execution_continues_after() {
    let src = r#"
x = 0
[1, 2, 3].each { |n| break if n == 2 }
x = 99
x"#;
    assert_eq!(eval(src), VmValue::Int(99));
}

#[test]
fn break_in_map_stops_early() {
    let src = r#"
[1, 2, 3, 4].map { |n|
  break if n == 3
  n * 10
}.size"#;
    assert_eq!(eval(src), VmValue::Int(3));
}

#[test]
fn next_in_each_skips_element() {
    let src = r#"
sum = 0
[1, 2, 3, 4, 5].each { |n|
  next if n % 2 == 0
  sum = sum + n
}
sum"#;
    assert_eq!(eval(src), VmValue::Int(9));
}

#[test]
fn break_in_nested_each_exits_inner_only() {
    let src = r#"
count = 0
[1, 2, 3].each { |i|
  [10, 20, 30].each { |j|
    break if j == 20
    count = count + 1
  }
}
count"#;
    assert_eq!(eval(src), VmValue::Int(3));
}

#[test]
fn while_condition_method_call_no_block_greed() {
    let src = "list = [1, 2, 3]\ni = 0\nsum = 0\nlen = list.size()\nwhile i < len { sum = sum + list[i]\ni = i + 1 }\nsum";
    assert_eq!(eval(src), VmValue::Int(6));
}

