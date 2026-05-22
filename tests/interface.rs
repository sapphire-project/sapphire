mod support;

use sapphire::vm::VmValue;
use support::{eval, typecheck_ok};

#[test]
fn class_structurally_satisfies_interface_without_declaration() {
    typecheck_ok(
        r#"
interface Drawable {
  def draw -> String
}

class Circle {
  def draw -> String { "circle" }
}

def render(item: Drawable) -> String {
  item.draw()
}

render(Circle.new())
"#,
    );
}

#[test]
fn interface_annotation_is_static_only_at_runtime() {
    let value = eval(
        r#"
interface Drawable {
  def draw -> String
}

class Circle {
  def draw -> String { "circle" }
}

def render(item: Drawable) -> String {
  item.draw()
}

render(Circle.new())
"#,
    );

    assert_eq!(value, VmValue::Str("circle".into()));
}

#[test]
fn missing_interface_method_is_type_error() {
    assert_typecheck_error!(
        r#"
interface Drawable {
  def draw -> String
}

class Circle {
}

def render(item: Drawable) -> String {
  item.draw()
}

render(Circle.new())
"#,
        "Circle",
        "Drawable",
        "draw",
    );
}

#[test]
fn interface_method_return_must_match() {
    assert_typecheck_error!(
        r#"
interface Drawable {
  def draw -> String
}

class Circle {
  def draw -> Int { 1 }
}

def render(item: Drawable) -> String {
  item.draw()
}

render(Circle.new())
"#,
        "Circle",
        "Drawable",
        "draw",
        "expected String",
        "got Int",
    );
}

#[test]
fn interface_typed_value_only_exposes_interface_methods() {
    assert_typecheck_error!(
        r#"
interface Drawable {
  def draw -> String
}

class Circle {
  def draw -> String { "circle" }
  def radius -> Int { 5 }
}

def render(item: Drawable) -> Int {
  item.radius()
}

render(Circle.new())
"#,
        "method 'radius' is not defined by interface Drawable",
    );
}

#[test]
fn generic_interface_substitutes_type_arguments() {
    typecheck_ok(
        r#"
interface Sink[T] {
  def push(value: T) -> Nil
}

class StringSink {
  def push(value: String) -> Nil { nil }
}

def write(sink: Sink[String]) -> Nil {
  sink.push("hello")
}

write(StringSink.new())
"#,
    );
}

#[test]
fn generic_interface_rejects_wrong_type_argument() {
    assert_typecheck_error!(
        r#"
interface Sink[T] {
  def push(value: T) -> Nil
}

class IntSink {
  def push(value: Int) -> Nil { nil }
}

def write(sink: Sink[String]) -> Nil {
  sink.push("hello")
}

write(IntSink.new())
"#,
        "IntSink",
        "Sink[String]",
        "push",
        "expected String",
        "got Int",
    );
}

#[test]
fn included_module_methods_count_for_structural_interface() {
    typecheck_ok(
        r#"
interface Greeter {
  def greet -> String
}

module Greeting {
  def greet -> String { "hi" }
}

class Person {
  include(Greeting)
}

def greet(person: Greeter) -> String {
  person.greet()
}

greet(Person.new())
"#,
    );
}
