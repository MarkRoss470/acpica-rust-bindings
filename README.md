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

## Changing Version of ACPICA
The source code of ACPICA is included as a tarball in the crate root. To update the version, download the new code from the [downloads page], and replace the tarball with this file. Then run `cargo clean` to remove the object files from the old version.

This library's build script automatically makes some changes to the source code to configure it for in-kernel use. Updates to the ACPICA code without updates to the rust code may make the library stop compiling, or stop working even if it compiles. 

[downloads page]: https://www.intel.com/content/www/us/en/download/776307/acpi-component-architecture-downloads-unix-format-source-code-and-build-environment-with-a-dual-license.html