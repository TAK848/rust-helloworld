pub fn run() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("s2 is: {}", s2);

    let i1 = 10;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);

    let s3 = String::from("Hello");
    let s4 = s3.clone();
    println!("s3 is: {}", s3);
    println!("s4 is: {}", s4);
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap address of s3 is: {:?}", s3.as_ptr());
    println!("Heap address of s4 is: {:?}", s4.as_ptr());

    let s5 = String::from("Hello");
    println!("Stack address of s5 is: {:p}", &s5);
    println!("Heap address of s5 is: {:?}", s5.as_ptr());
    println!("Len is: {}", s5.len());
    println!("Capacity is: {}", s5.capacity());
    take_ownership(s5);
    // println!("s5 is: {}", s5); // error[E0382]: borrow of moved value: `s5`

    let s6 = String::from("Hello");
    println!("Stack address of s6 is: {:p}", &s6);
    println!("Heap address of s6 is: {:?}", s6.as_ptr());
    println!("Len is: {}", s6.len());
    println!("Capacity is: {}", s6.capacity());
    let s7 = take_giveback_ownship(s6);
    println!("Stack address of s7 is: {:p}", &s7);
    println!("Heap address of s7 is: {:?}", s7.as_ptr());
    println!("Len is: {}", s7.len());
    println!("Capacity is: {}", s7.capacity());

    let s8 = String::from("Hello");
    println!("Stack address of s8 is: {:p}", &s8);
    println!("Heap address of s8 is: {:?}", s8.as_ptr());
    println!("Len is: {}", s8.len());
    println!("Capacity is: {}", s8.capacity());
    let len = calculate_length(&s8);
    println!("The length of '{}' is {}.", s8, len);

    let mut s9 = String::from("Hello");
    change(&mut s9);
    println!("s9 is: {}", s9);

    let s10 = String::from("Hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // let mut s10 = String::from("Hello");
    // let r1 = &s10;
    // let r2 = &mut s10;
    // println!("{} {} {}", s10, r1, r2);

    let mut s11 = String::from("Hello");
    let r1 = &mut s11;
    println!("r1 is: {}", r1);
    println!("s11 is: {}", s11);

    let mut s12 = String::from("Hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} {} {}", s12, r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("World");
    println!("{}", s12);
}
fn take_ownership(s: String) {
    println!("Stack address of s is: {:p}", &s);
    println!("Heap address of s is: {:?}", s.as_ptr());
    println!("Len is: {}", s.len());
    println!("Capacity is: {}", s.capacity());
    println!("s is: {}", s);
}
fn take_giveback_ownship(s: String) -> String {
    s
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(s: &mut String) {
    s.push_str(", world");
}
