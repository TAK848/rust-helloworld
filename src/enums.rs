enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}
pub fn run() {
    let linux = OS::Linux(1991, String::from("Linus Torvalds"));
    print_os_info(linux);
    let windows = OS::Windows(1985, String::from("Bill Gates"));
    let mac = OS::Mac(2001, String::from("Steve Jobs"));
}
fn print_os_info(os: OS) {
    match os {
        OS::Windows(year, who) => {
            println!("Windows was created in {} by {}", year, who);
        }
        OS::Mac(year, who) => {
            println!("Mac was created in {} by {}", year, who);
        }
        OS::Linux(year, who) => {
            println!("Linux was created in {} by {}", year, who);
        }
    }
}
