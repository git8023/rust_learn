use std::ops::Add;

pub fn test() {
    println!("\n\n========String");

    creation();
    update();
    index();
}

///
/// 字符串索引
/// 
fn index() {
    println!("\n---> index");
    // let _s = String::from("hello नमसत");
    // String不支持直接索引
    // let ch = s[0];
    // println!("ch: {}", ch);

    let s = String::from("jeck");
    for el in s.chars() {
      println!("ch: {}", el);
    }
    for b in s.bytes() {
      println!("byte: {}", b);
    }
}

///
/// 更新
///
fn update() {
    println!("\n---> update");
    let mut s = String::from("foo");
    println!("before: {}", s);
    s.push('a');
    s.push_str("bar");
    println!("after: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let _ = s1 + &s2;
    let _s3 = s1.add(&s2);
    let _: &str = &s2;

    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
}

///
/// 创建
///
fn creation() {
    println!("\n ---> creation");

    // 静态函数创建字符串
    // String
    let mut _s = String::new();
    let mut _s = String::from("initial content");

    // 字面量创建字符串
    // &str
    let s = "initial string";

    // &str -> String
    let s = s.to_string();
    let _ = s.to_string();
}
