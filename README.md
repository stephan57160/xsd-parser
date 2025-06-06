A comprehensive library for parsing XML schemas and generating code based on them.

This project originated as a fork of [`xsd-parser-rs`](https://github.com/lumeohq/xsd-parser-rs) but has since evolved into a complete rewrite.

If you enjoy the project and would like to support my work, you can [buy me a coffee](https://ko-fi.com/bergmann89) or [send a tip via PayPal](https://paypal.me/bergmann891/5EUR). Thanks a lot! 😊

<a href="https://github.com/Bergmann89/xsd-parser/blob/master/LICENSE"><img src="https://img.shields.io/crates/l/xsd-parser" alt="Crates.io License"></a> <a href="https://crates.io/crates/xsd-parser"><img src="https://img.shields.io/crates/v/xsd-parser" alt="Crates.io Version"></a> <a href="https://crates.io/crates/xsd-parser"><img src="https://img.shields.io/crates/d/xsd-parser" alt="Crates.io Total Downloads"></a> <a href="https://docs.rs/xsd-parser"><img src="https://img.shields.io/docsrs/xsd-parser" alt="docs.rs"></a> <a href="https://github.com/Bergmann89/xsd-parser/actions/workflows/main.yml"><img src="https://github.com/Bergmann89/xsd-parser/actions/workflows/main.yml/badge.svg" alt="Github CI"></a> <a href="https://deps.rs/repo/github/Bergmann89/xsd-parser"><img src="https://deps.rs/repo/github/Bergmann89/xsd-parser/status.svg" alt="Dependency Status"></a>


# Overview

The core idea of this library is to break down the code generation process into distinct steps, enabling better control and extensibility for users. The workflow is structured as follows:

1. **Parsing XML Schemas:**
   The `Parser` resolves XML schemas from various sources (e.g., local files, web links) and stores the processed information in the `Schemas` object. Resolving is done by using so called `Resolver`s, that can be implemented by the user.

2. **Interpreting Schemas:**
   The `Interpreter` uses the `Schemas` object to extract type information by applying extensions and restrictions defined in the schema. This results in a simplified `Types` structure that represents language-agnostic types.

3. **Optimizing Types:**
   The `Optimizer` refines the `Types` structure by applying various optimizations. This step is optional but may be necessary to handle specific features (e.g., `serde` support) when generating valid code.

4. **Generating Code:**
   Finally, the `Generator` converts the optimized `Types` information into Rust code by using so called `Renderer`s. As the resolvers, renderers can be implemented by the user to extend code generator.

![overview](doc/overview.svg "Overview")


This layered approach allows users to customize or extend the process at multiple stages. For example:
- Add custom logic to modify the type information.
- Replace the default generator with a custom implementation.
- Implement a custom interpreter to generate `Types` and use the default Rust code generator.

The possibilities for customization and extension are nearly limitless.


# Example

To quickly generate Rust code from an XML schema, you can use the `generate` function. This function acts as a simple wrapper for the entire process described above and is controlled via a `Config` structure to adjust various parameters.

```rust,ignore
use xsd_parser::{generate, Config, Error, config::{Schema, InterpreterFlags, OptimizerFlags, GeneratorFlags}};

fn main() -> Result<(), Error> {
    let mut config = Config::default();
    config.parser.schemas = vec![Schema::File("my-schema.xsd".into())];
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all();
    config.generator.flags = GeneratorFlags::all();

    let code = generate(config)?;
    println!("{code}");

    Ok(())
}
```

For more advanced examples, please refer to the [`examples`](./examples/README.md) directory.


# Features

This library provides the following features:

- **Rust Code Generation:** Convert any XML schema into Rust code.
- **Layered Architecture:** Add user-defined code to manipulate type information or generated code.
- **User-Defined Types:** Inject existing types into the generated code to reuse predefined structures.
- **`serde` Support:** Generate code for serialization and deserialization using [`serde`](https://docs.rs/serde) with [`serde_xml`](https://docs.rs/serde-xml-rs) or [`quick_xml`](https://docs.rs/quick-xml) as serializer/deserializer.
- **`quick_xml` Support:** Direct serialization/deserialization support using [`quick_xml`](https://docs.rs/quick-xml), avoiding `serde` limitations and leveraging asynchronous features.


# Changelog

Below you can find a short list of the most important changes for each released version.

## Version 1.1

- Implemented feature to generated boxed `quick_xml` deserializers to reduce stack usage during deserialization
- Improved naming of the generated types
- Implemented feature to split generated code into multiple module files
- Improved and implemented advanced examples
- General bug fixes and improvements

## Version 1.0

- First official release of `xsd-parser`

# Planned Features

- **Schema-Based Validation:** Generate validators directly from schemas to validate XML data during reading or writing.


# License

This crate is licensed under the MIT License.
