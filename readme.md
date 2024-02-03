`ezlog` - a simple, easy-to-configure, lightweight, action-based logger.

The `ezlog` crate provides a simple logger. It _does not_ provide any integration with the `log` API.

The log request consists of an _action_, _input_ data and _output_ data. Nothing more.

A lot of things are hardcoded, so using this library as-is is not advised. Save it as your own crate.

# Install

`cargo add easy-log`

# Usage

The library has a `Logger` type and a `map![]` macro.

To build a logger, use the provided methods like this:

```rust
use ezlog::{Logger, map, Map};

let default = Logger::new(); // empty
let default2 = Logger::default(); // same as the above

let default = default.ok(); // nothing set => does nothing
let default = default.action("test").ok(); // prints TEST: in green
let default = default.ok();

assert_eq!(
    map![test1: 234],
    Map(&[("test1".to_string(), "234".to_string())]) // auto conversion to String
);

let test2 = "This is a test.";
assert_eq!(
    map![test2],
    Map(&[("test2".to_string(), "This is a test.".to_string())]) // auto expansion
);

// you can chain methods
default
    .action("test2")
    .input(map![test2])
    .output(36) // not only maps
    .warn(); // prints the action in yellow

default2
    .action("test")
    .action("third") // you can override the preferences set before
    .err();

let really = true;
Logger::new()
    .action("final")
    .input(map![what: "test"])
    .output(map![of_what: "of this lib", really])
    .print("purple"); // set a custom color
```
