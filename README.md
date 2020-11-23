# Trash
[![Github Actions][img_gh-actions]][gh-actions]

Trash is simple and extensible programming language. It is something between Haskell, Tcl and Forth porgramming languages.

## Example

Hello world:
```trash
$puts "Hello, world!"
```

## State

Now, trash interpreter is in alpha-testing version, most of core features (such as `$set` statement, quoted strings 
parsing and clousre calling) are implemented, but all of standart features (`@if` function, `@while` function, functions,
which generate strctures and enums) haven't been implemented yet.

### Which featires are working now?

- `$set` operator (withot multiple assignment)
- `$puts` operator (also with several arguments)
- unquoted strings (may contain only ASCII alphanumeric symbols)
- string variables (also quoted strings (without any escape characters))
- calling string variables and strings (any string returns itself)
- assigning closures to variables and calling them
- calling raw closures


[gh-actions]: https://github.com/timcryt/trash/actions?query=workflow%3ARust
[img_gh-actions]: https://github.com/timcryt/trash/workflows/Rust/badge.svg