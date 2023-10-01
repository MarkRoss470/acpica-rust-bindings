# ACPICA Rust Bindings
Rust bindings to the ACPICA kernel subsystem for interacting with ACPI tables and AML code.

## Dependencies
This crate builds ACPICA from source, which means that the following system commands must be available:
* GCC for compiling C code
* AR for creating a static library

## Usage
The ACPICA kernel subsystem calls into OS code using various functions prefixed with `AcpiOs`. These are translated by this library into calls to methods on the `AcpiHandler` trait. An object implementing this trait must be passed to `register_handler` before any ACPI functionality can be used. Initializing the library could look like this:

```rust
let handler = HandlerStruct {};

let initialization = register_interface(interface)?;
let initialization = initialization.load_tables()?;
let initialization = initialization.enable_subsystem()?;
let initialization = initialization.initialize_objects()?;
```