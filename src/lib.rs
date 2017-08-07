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
/// assert_eq!(offset_of!(Foo, baz), std::mem::size_of::<Bar>());
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

macro_rules! size_of {
    ($T:ty) => {
        ::std::mem::size_of::<$T>()
    }
}
