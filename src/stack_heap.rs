enum List {
    Node(i32, Box<List>),
    Nil,
}
pub fn run() {
    // let a1: [u8; 9000000] = [1; 9000000];
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);
    println!("Stack address of v3 is: {:p}", &v3);
    println!("Heap address of v1 is: {:?}", v1.as_ptr());
    println!("Heap address of v2 is: {:?}", v2.as_ptr());
    println!("Heap address of v3 is: {:?}", v3.as_ptr());
    println!("Len of v1 is: {}", v1.len());
    println!("Len of v2 is: {}", v2.len());
    println!("Len of v3 is: {}", v3.len());
    println!("Capacity of v1 is: {}", v1.capacity());
    println!("Capacity of v2 is: {}", v2.capacity());
    println!("Capacity of v3 is: {}", v3.capacity());
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);

    let t1: (i64, String) = (10, String::from("Hello"));
    println!("Stack address of t1 is: {:p}", &t1);
    println!("Heap address of t1.1 is: {:?}", t1.1.as_ptr());
    println!("Len of t1.1 is: {}", t1.1.len());
    println!("Capacity of t1.1 is: {}", t1.1.capacity());
    let mut b1 = Box::new(t1);
    (*b1).1 += " world!";
    println!("{} {}", b1.0, b1.1);
    println!("Stack address of b1 is: {:p}", &b1);
    println!("Heap address of b1 is: {:p}", b1);
}
