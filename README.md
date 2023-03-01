# Strict types AST and typelib implementation

#### Protobufs for functional programming

This is a set of libraries for working with abstract syntax trees and libraries
of [strict types] &ndash; type system made with category theory which ensures
provable properties and bounds for the in-memory and serialized type
representation.

Strict types is a formal notation for defining and serializing
[generalized algebraic data types (GADT)][gadt] in a deterministic
and confined way. It is developed with [type theory] in mind.

Strict Types are:
* __schema-based__ (with the schema being strict encoding notation),
* __semantic__, i.e. defines types not just as they are layed out in memory,
  but also depending on their meaning,
* __deterministic__, i.e. produces the same result for a given type,
* __portabile__, i.e. can run on ahy hardware architecture and OS, including
  low-performant embedded systems,
* __confined__, i.e. provides guarantees and static analysis on a maximum size
  of the typed data,
* __formally verifiabile__.

To learn more about strict encoding [read the spec](https://strict-types.org).

Strict types works with type definitions. It allows:
- static analysis of data types, like
  * defining semantic type ids;
  * specifying exact memory layout;
  * type equivalence in terms of semantics and memory layout;
  * size of serialized data
- composing types into type libraries;
- versioning type libraries basing on the semantic types;

The library allows to generate & compile strict type libraries (STL) from rust 
types implementing `StrictEncoding` trait -- and ensures that the 
deserialization with `StrictDecode` follows the same memory and semantic layout.


## Strict Types Library

The library is able to reflect on itself, producing replica of its rust data
types in strict types system.

Strict types library id:
`stl:DYTdc4m4C6HLfM9hQCoavFHbdnwVuFQkWStTHDeHfDZ1#final-olivia-segment`

Import this lib by putting in the file header
`import final_olivia_segment_DYTdc4m4C6HLfM9hQCoavFHbdnwVuFQkWStTHDeHfDZ1 as StrictTypes`

Source code can be found in [`stl/StrictTypes.sty`] file.


## Contributing

[CONTRIBUTING.md](../CONTRIBUTING.md)

## License

The libraries are distributed on the terms of [Apache 2.0 license](LICENSE).

[strict types]: https://strict-types.org
[gadt]: https://en.wikipedia.org/wiki/Algebraic_data_type
[type theory]: https://en.wikipedia.org/wiki/Type_theory
