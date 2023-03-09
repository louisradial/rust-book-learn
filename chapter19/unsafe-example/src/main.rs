use std::slice;

// function uses unsafe block but is safe
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);

    let ptr = values.as_mut_ptr();

    // the raw pointers created are valid pointers, because of the assertion
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe fn dangerous() {}

// extern blocks are always unsafe
extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;
    // creating raw pointers is not unsafe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize;
    let _r = address as *const i32;
    // but deferencing them is
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
    unsafe {
        dangerous();
    }

    // creating a safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // function uses unsafe code but is safe
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("absolute value of -3 according to C: {}", abs(-3));
    }

    println!("{}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        // mutable static variables are thread unsafe
        println!("COUNTER: {}", COUNTER);
    }
}
