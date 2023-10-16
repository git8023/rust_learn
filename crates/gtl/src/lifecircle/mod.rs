use std::fmt::Display;

pub fn test() {
    test_one();

    println!("结构体生命周期");
    let s = String::from("Call me Isheel. Some years ago...");
    let first_sentence = s.split(".").next().expect("Could not find a '.'");
    let mut ie = ImportantExcerpt { part: "" };
    ie.part(first_sentence);
    println!("ie. {:?}", ie);

    // let mut ie = ImportantExcerpt { part: "" };
    // {
    //     let s = String::from("hello, world");
    //     // let first = s.split(",").next().unwrap();
    //     // ie.part(first);
    //     ie.part(&s);
    // }
    // println!("ie. {:?}", ie);

    println!("\n--> 泛型+生命周期");
    let a = String::from("abcd");
    let b = "xyz";
    let ann = "Begin..";
    let longest = longest_with_an_announcement(&a, b, ann);
    println!("longest: {}", longest);
}

fn longest_with_an_announcement<'a, T>(a: &'a str, b: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

///
/// 测试
///
fn test_one() {
    let s = String::from("abcd");
    let s2 = "xyz";
    let r = longest(&s, s2);
    println!("logest: {}", r);
}

///
/// 明确声明周期
///
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        // &a[0..1]
        "123"
    } else {
        &b[0..1]
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn part(&mut self, part: &'a str) -> &mut Self {
        self.part = part;
        self
    }
}
