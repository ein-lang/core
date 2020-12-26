#[no_mangle]
extern "C" fn _ein_join_strings(one: ffi::EinString, other: ffi::EinString) -> ffi::EinString {
    one.join(&other)
}

#[no_mangle]
extern "C" fn _ein_slice_string(
    string: ffi::EinString,
    start: ffi::Number,
    end: ffi::Number,
) -> ffi::EinString {
    string.slice(start, end)
}
