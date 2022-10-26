pub fn run() {
    let st1 = String::from("Hello");
    let st2 = String::from("World");
    let res1 = get_longest(&st1, &st2);
    println!("The longest string is: {}", res1);

    let st3 = String::from("Hello");
    let res2;
    {
        let st4 = String::from("World");
        res2 = get_longest(&st3, &st4);
        println!("The longest string is: {}", res2);
    }
}

fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// fn dummy1<'a>() -> &'a str {
//     let s = String::from("Hello");
//     &s
// }
// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x
// }
fn dummy3() -> String {
    let s = String::from("Hello");
    s
}
