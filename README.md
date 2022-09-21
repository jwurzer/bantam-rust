bantam-rust
===========

This is a tiny little Rust app to demonstrate Pratt parsing. For a full
explanation, see Bob Nystrom's blog post [**Pratt Parsers: Expression Parsing Made Easy**][blog].

[blog]: http://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/

This is a Rust port of Bob Nystrom's [bantam][java].
The original bantam is implemented in Java (see: https://github.com/munificent/bantam).

[java]: https://github.com/munificent/bantam

## About this Rust port

This Rust port with recoverable error handling can be found on [github][rust] at
https://github.com/jwurzer/bantam-rust/tree/with-error-handling.

[rust]: https://github.com/jwurzer/bantam-rust/tree/with-error-handling

This Rust version is similar in structure and design to the original Java version.
The error handling is done with the type `Result<T, E>`. If an error happend then
the variant `Err` with the type `ParseError` is used. `ParseError` includes the
error message. For unrecoverable error handing with `panic!` checkout the branch `master`
(https://github.com/jwurzer/bantam-rust) instead of this `with-error-handling` branch.

## Implementations & Ports

Original [bantam](https://github.com/munificent/bantam) in Java by Bob Nystrom.

This [Rust port](https://github.com/jwurzer/bantam-rust) by Jürgen Wurzer.

[C# port](https://github.com/jfcardinal/BantamCs) by John Cardinal.

[C++11 port](https://github.com/jwurzer/bantam-cpp) by Jürgen Wurzer

[C++20 port](https://github.com/stefanboca/bantam-cpp) by Stefan Boca

## License

bantam-rust is licensed like the original bantam under the very permissive [MIT license](LICENSE).
