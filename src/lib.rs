macro_rules! offset_of {
    ($T:ty, $($field:ident).+) => {
        unsafe { &((*(0 as *const $T)).$($field).+) as *const _ as usize }
    }
}
