fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("number: {}", *r1);
        *r2 = 3;
        println!("number: {}", *r2);
    }
}
