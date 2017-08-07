/// Returns the offset, in bytes, from the beginning of an object of a given
/// type to a specific field.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate bytes_of;
/// struct Foo {
///     bar: Bar,
///     baz: i32,
/// }
///
/// struct Bar {
///     quux: i64,
///     norf: i8,
/// }
///
/// # fn main() {
/// assert_eq!(offset_of!(Foo, bar), 0);
/// assert_eq!(offset_of!(Foo, baz), size_of!(Bar));
/// assert_eq!(offset_of!(Foo, bar.quux), 0);
/// assert_eq!(offset_of!(Foo, bar.norf), 8);
/// # }
/// ```
#[macro_export]
macro_rules! offset_of {
    ($T:ty, $($field:ident).+) => {
        unsafe { &((*(0 as *const $T)).$($field).+) as *const _ as usize }
    }
}

/// Returns the size, in bytes, of a given type.
///
/// It is equivalent to `std::mem::size_of<T>::()`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate bytes_of;
/// # fn main() {
/// assert_eq!(size_of!(i32), 4);
/// assert_eq!(size_of!(i64), 8);
/// # }
/// ```
#[macro_export]
macro_rules! size_of {
    ($T:ty) => {
        ::std::mem::size_of::<$T>()
    }
}

/// Returns the [alignment], in bytes, of a given type.
///
/// It is equivalent to `std::mem::align_of<T>::()`.
///
/// [alignment]: https://en.wikipedia.org/wiki/Data_structure_alignment
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate bytes_of;
/// # fn main() {
/// assert_eq!(align_of!(i32), 4);
/// # }
/// ```
#[macro_export]
macro_rules! align_of {
    ($T:ty) => {
        ::std::mem::align_of::<$T>()
    }
}
