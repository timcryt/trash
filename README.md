# Trash

Trash is simple and extensible programming language. It is something between Haskell, Tcl and Forth porgramming languages.

## Example

Hello world:
```trash
$puts "Hello, world!"
```

### Why this hello world programm doesn't work

Now, trash interpreter is in pre-alpha version, so most of language features, such as quoted strings, aren't work.

### And which featires are working now?

- `$set` operator (withot multiple assignment)
- `$puts` operator (also with several arguments)
- unquoted strings (may contain only ASCII alphanumeric symbols)
- string variables
- calling string variables and strings (any string returns itself)
- assigning closures to variables and calling them
- calling raw closures