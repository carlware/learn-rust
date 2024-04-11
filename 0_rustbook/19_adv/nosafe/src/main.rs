use std::slice;

fn main() {
    // f1()
    // f2()
    // f4()
    f5()
}

fn f1(){
    let mut num = 5;

    // raw pointers references mut and immutable
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;
}

fn f2() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // a deref to a raw pointer requires unsafe
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn f4() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn f5() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}