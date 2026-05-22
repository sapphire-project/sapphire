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
fn each_next() {
    let src = "sum = 0\n[1, 2, 3, 4, 5].each() { |x| if x == 3 { next nil }\nsum = sum + x }\nsum";
    assert_eq!(eval(src), VmValue::Int(12));
}

#[test]
fn break_in_each_stops_iteration() {
    let result = eval(
        r#"
sum = 0
[1, 2, 3, 4, 5].each { |n|
  break if n == 3
  sum = sum + n
}
sum"#,
    );
    assert_eq!(result, VmValue::Int(3)); // 1 + 2, stops before 3
}

#[test]
fn break_in_each_execution_continues_after() {
    let result = eval(
        r#"
x = 0
[1, 2, 3].each { |n| break if n == 2 }
x = 99
x"#,
    );
    assert_eq!(result, VmValue::Int(99));
}

#[test]
fn break_in_map_stops_early() {
    // map collects [10, 20] then hits break; result is a partial list
    let result = eval(
        r#"
[1, 2, 3, 4].map { |n|
  break if n == 3
  n * 10
}.size"#,
    );
    assert_eq!(result, VmValue::Int(3)); // [10, 20, nil] — 2 mapped + the break value
}

#[test]
fn next_in_each_skips_element() {
    let result = eval(
        r#"
sum = 0
[1, 2, 3, 4, 5].each { |n|
  next if n % 2 == 0
  sum = sum + n
}
sum"#,
    );
    assert_eq!(result, VmValue::Int(9)); // 1 + 3 + 5
}

#[test]
fn break_in_nested_each_exits_inner_only() {
    let result = eval(
        r#"
count = 0
[1, 2, 3].each { |i|
  [10, 20, 30].each { |j|
    break if j == 20
    count = count + 1
  }
}
count"#,
    );
    assert_eq!(result, VmValue::Int(3)); // inner each runs once per outer iteration
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

