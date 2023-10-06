pub fn test() {
    println!("\n\n ==== Vector");
    creation();
    get_element();
    iteration();

    use_enum_save_more_types();
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn use_enum_save_more_types() {
    println!("\n---> use_enum_save_more_types");
    let v = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("vec: {:?}", v);
}

fn iteration() {
    println!("\n---> iteration");
    let v = vec![2, 3, 5, 8, 13];
    for ele in v {
        println!("element {}", ele);
    }

    println!("mutable element:");
    let mut v = vec![2, 4, 6, 10, 16];
    println!("before: {:?}", v);
    for el in &mut v {
        *el += 100;
    }
    println!("after: {:?}", v);
}

fn get_element() {
    println!("\n---> get_element:");
    let v = vec![1, 2, 3];
    let first = &v[0];
    println!("first: {}", first);

    let first = v.get(0);
    let first = first.unwrap();
    println!("get first: {}", first);

    let mut v = vec![1,2,3];
    println!("all elements: {:?}", v);
    let last = v.pop();
    if let Some(val) = last {
        println!("pop last element value: {}", val);
    }
}

fn creation() {
    println!("\n---> creation");

    // 使用静态函数创建实例
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("vec: {:?}", v);

    // 使用宏创建
    let v = vec![1, 2, 3];
    println!("macro: {:?}", v);
}
