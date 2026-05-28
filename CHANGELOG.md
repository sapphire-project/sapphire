# Changelog

## v0.9.0

**Language**

- Instance variables now use `@name` syntax — no `attr` declaration needed for internal state:

```ruby
# Before
class Counter {
  attr n
  def inc { self.n = self.n + 1 }
  def value { self.n }
}
Counter.new(n: 0)

# After
class Counter {
  def inc { @n = (@n || 0) + 1 }
  def value { @n || 0 }
}
Counter.new()
```

- Optional parameters with literal default values — omit a trailing argument and its default is used:

```ruby
def greet(name, prefix = "Hello") { prefix + ", " + name }
greet("World")         # "Hello, World"
greet("World", "Hi")   # "Hi, World"
```

- Private class methods — `defp` now works inside `self { }` blocks:

```ruby
class Calculator {
  self {
    def double(n) { self.scale(n) }
    defp scale(n) { n * 2 }
  }
}
```

- Redesigned exception syntax — `try`/`rescue` now use braces; `else` runs when no exception is raised, `ensure` always runs:

```ruby
# Before
begin
  risky_operation()
rescue e
  handle(e)
end

# After
try {
  risky_operation()
} rescue e {
  handle(e)
} else {
  on_success()
} ensure {
  cleanup()
}
```

- Abstract methods no longer use the `abstract` keyword — a bare `def` signature inside an `abstract class` declares the method as abstract:

```ruby
# Before
abstract class Shape {
  abstract def area -> Float
}

# After
abstract class Shape {
  def area -> Float
}
```

- Type annotations are now enforced at runtime; `Int` values are automatically promoted to `Float` when passed to a `Float`-annotated parameter

**Standard library**

- `Enumerable` module — include it in any class that defines `each` to get `all?`, `any?`, `none?`, `find`, `include?`, `count`, `partition`, `sort_by`, and `reduce`:

```ruby
[1, 2, 3].any? { |n| n > 2 }                       # true
[1, 2, 3, 4].reduce(0) { |acc, n| acc + n }         # 10
[1, 2, 3, 4].partition { |n| n.even? }              # [[2, 4], [1, 3]]
```

- `CLI.parse` — parse command-line flags and positional arguments from an argv array:

```ruby
r = CLI.parse(["--verbose", "--out=log.txt", "file.txt"])
r.options   # { "verbose" => true, "out" => "log.txt" }
r.rest      # ["file.txt"]
```

- `File` — filesystem helpers: `File.read`, `File.write`, `File.join`, `File.basename`, `File.dirname`, `File.directory?`, and more

- `IO` module — `IO.puts`, `IO.print`, and `IO.gets`

---

## v0.8.0

**Language**

- `match` expression — pattern match on literals, ranges, and multiple values per arm; use `_` as a wildcard and `if` guards for conditional arms:

```ruby
grade = match score {
  90..100      => { "A" }
  80..89       => { "B" }
  "Sat", "Sun" => { "Weekend" }
  n if n > 50  => { "Pass" }
  _            => { "Fail" }
}
```

- Structural interfaces — declare an interface with required method signatures; any class implementing those methods satisfies the interface, no explicit declaration needed:

```ruby
interface Drawable {
  def draw -> String
}

class Badge {
  def draw -> String { "drawing badge" }
}

def render(item: Drawable) -> String { item.draw() }
render(Badge.new())
```

- Abstract classes — mark a class `abstract` to prevent direct instantiation; use `abstract def` to require subclasses to implement a method:

```ruby
abstract class Shape {
  abstract def area -> Float
}

class Square < Shape {
  def area -> Float { 4.0 }
}
```

- Modules and mixins — define reusable method groups with `module` and mix them into classes with `include`:

```ruby
module Greetable {
  def greet -> String { "hello from " + self.name() }
}

class Person {
  include(Greetable)
  def name -> String { "Alice" }
}

Person.new().greet()  # "hello from Alice"
```

- `super` now works Ruby-style — bare `super` forwards all arguments; `super(args)` passes explicit arguments; `super.method` is no longer valid

**Error messages**

- Runtime errors now include source context and column position
- No-method errors show the value's type: `30 (Int) has no method 'push'`
- Typos in method names now show a did-you-mean suggestion
- Parse errors at or past end of file now show source context

**REPL**

- `quit` and `exit` commands now work in `sapphire console`

---

## v0.7.0

**Language**

- Union types — annotate a value as one of several types using `|`:

```ruby
def stringify(x: Int | Float): String
  x.to_s
end
```

- Type aliases — give a name to any type expression with `type`:

```ruby
type Numeric = Int | Float
```

- Generics — parameterize classes and methods with type variables (erased at runtime):

```ruby
class Box[T]
  attr value: T

  def get -> T { self.value }
end

def identity[T](x: T) -> T { x }

items: List[Int] = [1, 2, 3]
```

- Typechecking now runs automatically before `run`, so `sapphire run` will report type errors before executing

**Typechecker**

- Return types are now inferred for unannotated functions and methods, and propagated back to their signatures
- Type inference now handles the following expression forms:
  - List and map literals
  - `if` expressions where both branches agree on a type
  - `begin` expressions with no `rescue` clause
  - Unary operators
  - `String + String` → `String`
  - Variable assignments and property assignments (inferred from the RHS)
  - `while` loops → `Nil`
  - Class-level constants
  - Multi-assignment
  - Safe-navigation method calls → `Nil | T`
- Inferred types propagate across mutually recursive functions

**Infrastructure**

- Sapphire is now published to crates.io as `sapphire-lang`
- MIT license added

---

## v0.6.0

**Language**

- Regular expressions — `Regex.new` creates a regex from a pattern string; supports `match?`, `match`, `scan`, `replace`, and `replace_all`. Case-insensitive matching via `ignore_case: true`. Matches return a `Regex.Match` with `full`, `captures`, `start`, and `end_pos` fields:

```ruby
r = Regex.new("[0-9]+")
r.match?("foo123")      # true
m = r.match("foo123")
m.full                  # "123"

ci = Regex.new("hello", ignore_case: true)
ci.match?("Hello World")  # true
```

- Constants defined in an outer class are now visible inside nested class bodies and their methods without qualification

**Standard library**

- `Object#class` — returns the class of any object
- `Class#superclass` — returns the parent class of a class

**Bug fixes**

- `is_a?` now works correctly when passed a class object argument such as `List`
- Fixed a bug where local variables declared inside a `while` loop body were incorrectly hoisted when nested control flow (`if`, inner `while`) was present

---

## v0.5.2

**CI**

- Reverted parallel WASM build — the native and WASM release jobs now run sequentially again

---

## v0.5.1

This is a version bump to verify the release pipeline.

**CI**

- WASM build now runs in parallel with native builds, reducing total release time

---

## v0.5.0

**Language**

- Heredoc strings — triple-quoted multi-line string literals with automatic indent stripping:

```ruby
message = """
  Hello,
  world!
  """
```

- `return` now works correctly inside blocks passed to native methods such as `each`

**Standard library**

- `DateTime` module — `Instant`, `Date`, `Time`, `DateTime`, `ZonedDateTime`, and `Duration` types for date and time handling
- `Math` — trigonometric methods: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`
- `Set` — unordered collection with set-membership semantics
- `Socket` — minimal TCP client support via `Socket.new` and `connect`, `send`, `receive`, `close`
- `Env` — read and write environment variables with `Env.get`, `Env.set`, and `Env.all`
- `Process` — run subprocesses with `Process.run`; result is a `Process.Result` with `stdout`, `stderr`, and `exit_code`
- `Class#instance_method_names` — returns a list of method names defined on a class
- All collection types now consistently use `size` instead of `length`

**CLI**

- `sapphire test` now reports the total test run time

**Bug fixes**

- `Map#all?` now handles entries with `nil` values correctly
- `Map#none?` no longer recurses infinitely

---

## v0.4.0

**Language**

- Class-level constants — define named constants directly inside a class body:

```ruby
class Circle {
  PI = 3.14159

  def area(r) { PI * r * r }
}
```

- Bitwise operators — `&`, `|`, `^`, `~`, `<<`, `>>` are now supported on integers
- Numeric literal improvements — underscore separators and hexadecimal literals are now valid:

```ruby
population = 8_000_000_000
color      = 0xFF5733
```

- Parentheses are optional for zero-argument method definitions and calls:

```ruby
def greet { "hello" }   # same as def greet() { "hello" }

greet                    # same as greet()
"hello".upcase           # same as "hello".upcase()
```

**Standard library**

- `Math` class with `Math.PI` and `Math.E` constants
- `File` class — `File.read(path)`, `File.write(path, content)`, and `File.exist?(path)` for basic file I/O

**CLI**

- `sapphire test` — built-in test runner for `.spr` test files

**Editor support**

- Vim plugin — syntax highlighting for `.spr` files is available at [sapphire-project/vim-sapphire](https://github.com/sapphire-project/vim-sapphire)

**REPL**

- Command history and multiline input support in `sapphire console`

**VM**

- Mark-and-sweep garbage collector to break reference cycles in object graphs

**Bug fixes**

- Class namespace constants defined inside nested classes are now correctly preserved when loading the standard library

---

## v0.3.0

**Language**

- Nested class definitions — classes can now be defined inside other classes and accessed with dot notation (`Geometry.Point`), including as superclasses:

```ruby
class Geometry {
  class Point {
    attr x: 0
    attr y: 0
  }
}

p = Geometry.Point.new(x: 1, y: 2)
```

- Relative file imports — use `import "./path"` to load a `.spr` file relative to the current file; imported classes and functions become available in the importing file; duplicate imports are silently skipped

**VM**

- Return type annotations are now enforced at runtime — functions declared with `-> TypeName` raise a type error if the return value doesn't match; the `Num` supertype accepts both `Int` and `Float`

**Bug fixes**

- `break` inside blocks passed to native methods (e.g. `each`, `map`, `select`) now works correctly — previously it would silently stop execution past the native call
- `break` and `next` inside `while` loops now work correctly

---

## v0.2.1

**Bug fixes**
- `Float#to_s` now preserves the trailing `.0` for whole-number floats (`1.0.to_s()` returns `"1.0"` instead of `"1"`)
- `Float#zero?` now returns `true` for `0.0` (previously always returned `false` due to an integer comparison)

---

## v0.2.0

**VM**
- The bytecode VM is now the sole runtime — the tree-walk interpreter has been removed
- The REPL (`sapphire console`) now runs on the VM

**Parser fixes**

Method chaining after a block now works, both on one line and across lines:

```ruby
# Now works — previously: parse error: unexpected token 'Dot'
[1, 2, 3].map { |n| n * 2 }.each { |n| print n }

# Also works now
[1, 2, 3]
  .map { |n| n * 2 }
  .each { |n| print n }
```

`elsif` and `else` can now appear on the next line after the closing `}`:

```ruby
# Now works — previously: parse error: unexpected token 'Elsif'
if x == 1 { "one" }
elsif x == 2 { "two" }
else { "other" }
```

---

## v0.1.1

**Classes**
- Class methods via `self { ... }` blocks — methods callable on the class itself, inherited by subclasses

**CLI**
- `sapphire version` command — prints the language name and version (e.g. `Sapphire 0.1.1`)
- More detailed usage output

---

## v0.1.0

First public preview of the Sapphire language.

### Language features

**Primitives & literals**
- `Int`, `Float`, `Bool`, `Nil` literals
- String literals with interpolation (`"hello #{name}"`) and escape sequences (`\n`, `\t`, `\\`, `\"`)
- Range literals (`1..5`)
- List literals (`[1, 2, 3]`) with index access and mutation
- Map literals (`{x: 1, y: 2}`) with string key access and mutation

**Operators**
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Boolean: `&&`, `||`, `!`
- String concatenation with `+`
- Safe navigation: `obj&.method`
- Modulo with division-by-zero error handling

**Variables & assignment**
- Variable assignment and reassignment
- Multiple assignment (`a, b = 1, 2`)
- Swap (`a, b = b, a`)

**Control flow**
- `if` / `elsif` / `else` — as a statement or expression
- `while` loops
- Postfix / trailing `if` (`raise "msg" if condition`)
- `return` for early exit

**Functions & closures**
- Named functions with `def`
- Closures that capture variables from enclosing scopes
- First-class anonymous lambdas (`f = def(x) { x * 2 }; f.call(5)`)
- Top-level `def` desugars into `Object` methods (Ruby-style)

**Blocks**
- Block syntax: `list.each { |x| print x }`
- `yield` to call a passed block
- `next` to return a value from the current block iteration
- `break` to exit the block's caller early

**Classes**
- Class definitions with `attr` fields and default values
- Keyword constructor: `Point.new(x: 1, y: 2)`
- Instance methods with implicit `self`
- Private methods with `defp`
- Single inheritance (`class Dog < Animal`)
- `super` for calling parent methods
- `is_a?` with full inheritance chain check

**Error handling**
- `raise` with a string message
- `begin` / `rescue` / `else` / `end` blocks
- Inline rescue inside `def` bodies
- `begin`/`rescue` as an expression

**Standard library**
- `Int`: `to_s`, `to_f`, `abs`, `even?`, `odd?`, `zero?`, `times`
- `Float`: `round`, `floor`, `ceil`, `to_i`, `abs`
- `String`: `size`, `upcase`, `downcase`, `reverse`, `strip`, `to_i`, `to_f`, `empty?`, `include?`, `starts_with?`, `ends_with?`, `split`
- `Bool` / `Nil`: `nil?`, `to_s`
- `List`: `size`, `first`, `last`, `empty?`, `include?`, `sort`, `join`, `push`, `pop`, `each`, `map`, `select`, `any?`, `all?`, `none?`, `reduce`, `flatten`
- `Map`: `size`, `has_key?`, `delete`, `merge`, `each`, `select`, `any?`, `all?`, `none?`
- `Range`: `each`, `to_a`, `include?`
- `Object`: `is_a?`, `nil?`, `class`

**Type system**
- Optional type annotations on parameters and return types
- Runtime enforcement when annotations are present
- Static type checker (`sapphire typecheck`)

**Execution**
- Tree-walk interpreter (`sapphire run`)
- Bytecode compiler + stack-based VM (`sapphire vm`)
- Interactive REPL (`sapphire console`)
