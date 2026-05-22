mod support;

use sapphire::vm::VmValue;
use support::eval;

#[test]
fn ivar_read_write_basic() {
    let src = r#"class C {
  def set(v) { @x = v }
  def get { @x }
}
c = C.new()
c.set(42)
c.get"#;
    assert_eq!(eval(src), VmValue::Int(42));
}

#[test]
fn ivar_nil_before_first_write() {
    let src = r#"class C {
  def get { @x }
}
C.new().get"#;
    assert_eq!(eval(src), VmValue::Nil);
}

#[test]
fn ivar_memoization_pattern() {
    let src = r#"class C {
  attr n
  def result {
    @cached = @cached || (self.n * 2)
    @cached
  }
}
c = C.new(n: 21)
c.result
c.result"#;
    assert_eq!(eval(src), VmValue::Int(42));
}

#[test]
fn ivar_counter_no_attr() {
    let src = r#"class Counter {
  def inc { @n = (@n || 0) + 1 }
  def value { @n || 0 }
}
c = Counter.new()
c.inc
c.inc
c.value"#;
    assert_eq!(eval(src), VmValue::Int(2));
}
