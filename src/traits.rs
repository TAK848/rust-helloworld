trait Fruits {
    fn price(&self) -> u32;
}
struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        100
    }
}
struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        200
    }
}
trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    //     fn summarize(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
}
impl Message for NewsArticle {}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }
}
trait Message {
    fn message(&self) -> String {
        String::from("Hello")
    }
}
pub fn run() {
    let apple = Apple;
    let banana = Banana;
    get_price(apple);
    get_price(banana);
    let tweet = Tweet {
        username: String::from("user1"),
        content: String::from("Hello"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Hello"),
        location: String::from("Tokyo"),
        author: String::from("user1"),
        content: String::from("Hello, World!"),
    };
    println!("{}", article.summarize());
    notify(&tweet);
    notify_another(&article);
    // notify_another(&tweet);
}
fn get_price<T: Fruits>(fruit: T) {
    println!("price is {}", fruit.price());
    // fruit.price()
}
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message! {}", item.message());
}
