# Trash
[![Github Actions][img_gh-actions]][gh-actions]

Trash is simple and extensible programming language. It is something between Haskell, Tcl and Forth porgramming languages.

## Example

Hello world:
```trash
$puts "Hello, world!"
```

## State

Now, trash interpreter is in alpha-testing version, all for core features and some base type methonds are implemented, 
but all of standart features (`@while` function, functions, which generate sructures and enums) haven't
been implemented yet.

### Which featires are working now?

- `$set` operator (with multiple assignment)
- `$puts` operator (also with several arguments)
- unquoted strings (may contain only ASCII alphanumeric symbols)
- quoted strings (without escape sequences)
- string variables (also quoted strings (without any escape characters))
- calling string variables and strings (any string returns itself)
- assigning closures to variables and calling them
- calling raw closures
- creating tuples, assigning and calling (any tuple returns itself) them
- `len` `split` `push` and `eq` methods for strings
- `push` `pop` `is_empty` and `{any number}` methods for tuples
- `@if` function, which calls condition closure and then call them closure or else closure

[gh-actions]: https://github.com/timcryt/trash/actions?query=workflow%3ARust
[img_gh-actions]: https://github.com/timcryt/trash/workflows/Rust/badge.svg