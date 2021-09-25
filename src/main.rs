unsafe fn dangerous() {}

fn main() {
    println!("Hello, world!");
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);

        *r2 += 1;

        println!("r2: {}", *r2);
        println!("r1: {}", *r1);
    }

    unsafe {
        dangerous();

        let i = -3;

        println!("abs of {} is {}", i, abs(i));
    }

    unsafe {
        println!("{}", HELLO);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("you just called me from rust");
}

static mut HELLO: &'static str = "hello world";

unsafe trait Foo {}

unsafe impl Foo for i32 {}

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     let ptr = slice.as_mut_ptr();
//
//     assert!(mid <= len);
//
//     // (&mut slice[..mid], &mut slice[mid..])
//
//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_parts_mut(ptr.offset(mid as isize), len - mid),
//         )
//     }
// }
