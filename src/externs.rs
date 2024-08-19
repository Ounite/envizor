unsafe fn copy_forward(dest: *mut u8, src: *const u8, len: usize) {
    for i in 0..len {
        dest.add(i).write(src.add(i).read());
    };
}


unsafe fn copy_backward(dest: *mut u8, src: *const u8, len: usize) {
    for i in (0..len).rev() {
        dest.add(i).write(src.add(i).read());
    };
}


#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    copy_forward(dest, src, len);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    let delta = (dest as usize).wrapping_sub(src as usize);

    if delta >= len {
        copy_forward(dest, src, len);
    } else {
        copy_backward(dest, src, len);
    };

    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, value: i32, len: usize) -> *mut u8 {
    for i in 0..len {
        dest.add(i).write(value as u8);
    };
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, len: usize) -> i32 {
    for i in 0..len {
        let a_v = a.add(i).read();
        let b_v = b.add(i).read();

        if a_v != b_v {
            return a_v as i32 - b_v as i32;
        };
    };

    0
}
