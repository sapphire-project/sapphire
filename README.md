# Sapphire

A Ruby-inspired, gradually typed, object-oriented language — everything is an object, types are optional, and the syntax stays out of your way.

**[Website](https://sapphire-lang.dev/)** · **[Try it online](https://sapphire-lang.dev/try/)** · **[Tutorial](https://sapphire-lang.dev/tutorial/)**

Sapphire is designed around three ideas: elegant syntax that reads like prose, a gradual type system you can lean on as much or as little as you want, and tooling that gets out of your way. Types are enforced at runtime when present — start untyped and add annotations where they help, without a separate build step or type checker.

## Install

Sapphire is installed and managed via [Facet](https://github.com/sapphire-project/facet), the official toolchain manager (requires Rust):

```
cargo install --git https://github.com/sapphire-project/facet
facet sapphire install latest
```

Then verify:

```
sapphire version
```

Pre-built binaries are also available on the [releases page](https://github.com/sapphire-project/sapphire/releases).

## A first look

```ruby
class Shape {
  attr color = "red"

  def area() { 0 }

  def describe() {
    "A #{self.color} shape with area #{self.area()}"
  }
}

class Circle < Shape {
  attr radius: Float

  def area() {
    Math::PI * self.radius * self.radius
  }
}

c = Circle.new(color: "blue", radius: 3.0)
print c.describe()    #=> "A blue shape with area 28.274..."
print c.is_a?(Shape)  #=> true
```

`radius: Float` is enforced at runtime — `color` is untyped and takes any value.

## Collections

Blocks make iteration expressive without special syntax:

```ruby
numbers = [3, 1, 4, 1, 5, 9]

doubled = numbers.map    { |n| n * 2 }
evens   = numbers.select { |n| n % 2 == 0 }
total   = numbers.sum

print doubled  #=> [6, 2, 8, 2, 10, 18]
print evens    #=> [4]
print total    #=> 23
```

The same block syntax works for your own methods — `yield` calls the block passed to the current function.

## CLI

```
sapphire run file.spr       # run a file
sapphire test [path]        # run *_test.spr files recursively
sapphire typecheck file.spr # type-check without running
sapphire console            # interactive REPL
sapphire version
```

## Learn more

The [tutorial](https://sapphire-lang.dev/tutorial/) covers the full language: control flow, functions, classes, error handling, and imports.

- [Try it in the browser](https://sapphire-lang.dev/try/)
- [Standard library](https://sapphire-lang.dev/stdlib/)
- [Changelog](CHANGELOG.md)

## Contributing

Contributions are welcome. Open an issue to discuss a bug or feature before sending a pull request.

## License

MIT — see [LICENSE](LICENSE).
