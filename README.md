`bytes_of`
==========

This is a tiny crate that adds a macro for calculating byte offset of given
field within a struct. It also defines similar macros for calculating size and
alignment.

Usage
-----

```rust
struct Foo {
    bar: i16,
    baz: i32,
}

struct Bar {
    quux: i32,
    norf: i64,
}

let baz_offset = offset_of!(Foo, baz);
let norf_offset = offset_of!(Foo, bar.norf);
```

Why?
----

Rust has reserved the `offsetof` keyword but has no use for it right now. Until
it gets added to the language we have to deal with it in some other way. And
because macros were invented to overcome shortcomings of the built-in syntax,
the `offset_of!` macro has to do for the time being.

Similarly, keywords `sizeof` and `alignof` are also reserved but are not part of
the language even though functions `mem::size_of` and `mem::align_of` exist in
the standard library. Because of that macros `size_of!` and `align_of!` are
completely unnecessary. However, they were added to this crate for consistency:
`offset_of!` is usually used in places where `size_of!` is needed as well and
both of them being used as macros next to each other just feels nicer.

Alternatives
------------

There also exists a [`field-offset`] crate that has a bit richer API and might
be a better fit for your needs. I, however, wanted something simpler and with
interface identical to the one found in other languages.

[`field-offset`]: https://crates.io/crates/field-offset

Acknowledgements
----------------

See issue [#1144][gh-issue] for discussion about `sizeof`, `alignof` and
`offsetof`. The `offset_of!` macro implementation is based on [this][so-thread]
thread.

[gh-issue]: https://github.com/rust-lang/rfcs/issues/1144
[so-thread]: https://stackoverflow.com/questions/40310483/how-to-get-pointer-offset-in-bytes/40310851#40310851
