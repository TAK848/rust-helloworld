struct Point<T> {
    x: T,
    y: T,
}
struct PointAnother<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}
pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);
    // println!("The largest number is {}", largest_i32(number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(char_list));
    println!("The largest number is {}", largest(number_list));
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 30.0, y: 40.0 };
    let p3 = PointAnother { x: 10, y: 20.0 };
    let p4 = PointAnother { x: "Rust", y: 40.0 };
    let p5 = p3.mixup(p4);
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // println!("p4.x = {}, p4.y = {}", p4.x, p4.y);
    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);
}
fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
