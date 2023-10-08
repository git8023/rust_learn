use std::collections::HashMap;

pub fn test() {
    println!("\n\n====> HashMap");
    let mut scores = creation();
    index(&scores);
    update(&mut scores);
}

///
/// 更新
///
fn update(scores: &mut HashMap<String, i32>) {
    println!("\n---> update");
    println!("before: {:?}", scores);

    // 直接覆盖
    scores.insert(String::from("Blue"), 25);
    println!("insert: {:?}", scores);

    // 不存在则插入
    let yellow_val = scores.entry(String::from("Yellow")).or_insert(30);
    *yellow_val += 100;
    println!("entry or insert: {:?}", scores);
}

///
/// 获取值
///
fn index(scores: &HashMap<String, i32>) {
    println!("\n---> index");
    let blue = String::from("Blue");
    let score = scores.get(&blue);
    println!("score: {:?}", score);
    if let Some(val) = score {
        println!("real score: {}", val);
    }

    println!("\n---> iteration");
    for (k, v) in scores {
        println!("{}:{}", k, v);
    }
}

///
/// 创建
///
fn creation() -> HashMap<String, i32> {
    println!("\n--> creation");

    // 创建空集合
    let mut score_map = HashMap::new();
    score_map.insert(String::from("Blue"), 10);
    score_map.insert(String::from("Red"), 50);

    // 从两个vector中一一对应创建HashMap
    let names = vec![String::from("Blue"), String::from("Red")];
    let scores = vec![10, 50];
    let map: HashMap<_, _> = names.iter().zip(scores.iter()).collect();
    println!("map: {:?}", map);

    score_map
}
