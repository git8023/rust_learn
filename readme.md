# 逻辑结构

- if...
    - if...else...
    - if...else...if...
    - let x = if {x} else {y};
- loop/while/for
- loop...break[value] 可返回计算结果
- 多重循环使用循环标签
  ```rust
  fn main() {
      let val = 'loop_label:loop {
          loop {
              // 跳出外层循环并返回结果: 10~~~~
              break 'loop_label 10;    
          }
      };
  }
  ```

# 所有权

- 所有权转移
  ```rust
  fn main(){
    let s = String::from("hello");
    let s1 = s; // s所有权转移到s1, 
                // s不可用, s离开作用域不调用 Drop trait
  }
  ```
- 不可变借用
  ```rust
  fn main() {
    let mut s = String::from("hello");
    let s2 = &s;        // s2拥有s不可变借用
                        // s任然可用
  }
  ```
- 可变借用
  ```rust
  fn main() {
    let mut s = String::from("hello");
    let s2 = &mut s;    // s2拥有s可变借用
                        // 可通过s2修改s中内容
                        // s任然可用
  }
  ```
- 相同作用域
    - 可以有多个不可变借用
    - 不可同时拥有多个可变借用
    - 不可同时拥有可变/不可变借用
  ```rust
  fn main() {
    let mut s = String::from("hello world");
    let first_word = &s[..5];                    // hello:&str 不可变借用
    // s.clear();                               // error! hello有效期内不可调用可变借用
    println!("first_word, {}", first_word);     // first_word 最后一次使用
    s.clear();                                  // 不可变借用已销毁, 可以调用可变借用 
  }
  ```

# 结构体

- 普通结构体
  ```rust
  struct User{
    uname: String,
    age: u32,
  }
  ```
- 元组结构体
  ```rust
  struct Color(i32,i32,i32);
  ```
- 结构体方法
  ```rust
  struct User {
    name:String,
  }
  
  /// impl Struct_Name
  impl User {
  
    /// 方法
    /// self:&mut Self
    fn name(&mut self, name:String) {
        self.name = name;
    }
  
    /// 方法
    /// self:&Self
    fn print(&self)->String {
        format!("User name:{}",self.name)  
    }
  
    /// 静态函数
    /// 不以 self 作为第一个参数
    fn new(name:String) -> Self {
        User { name }
    } 
  
  }
  
  /// 一个结构体可以有多个实现
  impl User {
    fn say_hi(&self)->String {
        format!("{}, Say hi!", self.name)
    }
  }
  
  fn main() {
    // 调用静态函数 
    let user = User::new("Mari");
    
    // 调用方法
    user.name("Abbo");
    user.print(); 
  }
  ```

# 枚举

- 枚举定义
  ```rust
  enum IpAddr{
    /// 多个数据
    V4(u8, u8, u8, u8),
    /// 带指定类型数据 
    /// 数据类型可以是任意类型
    /// - 基本数据类型
    /// - 结构体
    /// - 其他枚举等等
    V6(String, IpAddr),
    /// 无数据枚举
    None,
  }
  ```
- 定义方法
  ```rust
  enum Message{
    Short(String),
    Long(String),
    None,
  }
  
  impl Message {
    /// 静态方法
    fn write(msg:Message) {}
  
    /// 实例方法(枚举值)
    fn send(&self){}
  }
  ```
- 模式匹配(`match`, 模式匹配必须覆盖所有枚举值)
  ```rust
  #[derive(Debug)]
  enum UsState {
    Alabama,
    Alaska,
  }

  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Other,
  }
  
  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      // 大括号中只有一条表达式可简写
      // Coin::Quarter => 25
      // 获取枚举中携带的数据
      Coin::Quarter(usState) => {
        println!("State quarter from {:?}!", state);
        25
      }
  
      // other匹配甚于情况, 并接受值
      // other => 1,
      // 下划线开头表示匹配剩余情况, 并丢弃该值
      // _ => 2,
    }
  }
  ```
- 使用`if let`简化 `match` 匹配
  ```rust
  fn main(){
    let opt = Option::Some(1);
  
    // 1. match
    let match_result = match opt {
      Some(i) => i*10,
      // 返回空元组表示不做任何处理
      _ => (),
    };
  
    // 2. if let
    let il_result:i32 = -1;
    if let Some(i) = opt {
      il_result = i * 10;
    } else {
      // 处理其他情况
      // else分支可选
    }
    
  }
  ```

# 模块

- 模块声明
  ```rust
  /// 本地声明
  /// 语法: mod mod_name {}
  mod mod_in_local {}
  
  /// 引用声明
  /// 语法: mod mod_name;
  /// 从当前文件所在目录开始查找:
  ///   1. 查找 ./mod_in_file.rs
  ///   2. 查找 ./mod_in_file/mod.rs
  /// 匹配到任意一项即可
  mod mod_in_file;
  
  /// mod_in_file.rs 或 ./mod_in_file/mod.rs
  /// 文件内部无需再次写上mod名称
  fn fn_name(){}
  ```
- 模块文件系统规则
    - 如果一个叫做 `foo` 的模块没有子模块，应该将 foo 的声明放入叫做 `foo.rs` 的文件中。
    - 如果一个叫做 `foo` 的模块有子模块，应该将 foo 的声明放入叫做 `foo/mod.rs` 的文件中。
- bin crate引入lib crate
  ```rust
  /// 引入外部crate到当前作用域
  extern crate mod_name;
  ```

- 私有性规则
    - 如果一个项是共有的, 它能被任何父模块访问
    - 如果一个项是私有的, 他能被其直接夫模块及其任何子模块访问
    - 相同父模块下, 远亲模块可见性与外部模块一致

- 使用 use 导入模块, 缩短访问路径/模块别名  
  `use`是相对于父模块开始
  ```rust
  pub mod a {
    pub mod series {
      pub mod of{
        pub fn nested_modules(){} 
      }
    }
  }
  
  enum TrafficLight {
    Red,
    Yellow,
    Green,
  }
  
  /// 引入模块
  use a::series::of;
  /// 引入枚举值
  use TrafficLight::{Red, Yellow};
  // 使用 global 引入所有 pub 模块/方法
  // 注意: 可能导致命名冲突
  // use TrafficLight::*;
  
  fn test() {
    /// 全路径访问
    a::series::of::nested_modules();
    /// use缩短访问路径 
    of::nested_modules();
  
    let red = Red;
    let green = TrafficLight::Green;
  }
  ```



# 集合
- `Vec<T>`  
  `vector`集合会获取所有权, 即: 集合声明周期结束时所有元素都会被释放.
  ```rust
  // 创建
  let mut v:Vec<i32> = Vec::new();
  let mut v = vec![1,2,3];

  // 获取元素
  // &i32
  let first = &v[0];
  // Option<&i32>
  let first = v.get(0);

  // 迭代
  for el in v {
    println!("el: {}",el);
  }

  // 迭代修改
  let mut v = vec![1,2,3];
  for el in &mut v { // 要使元素值可变, 集合本身必须是可变的
    *v += 10;
  }
  ```
  - 使用枚举泛型使集合保存不同数据类型
    ```rust
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
    ```

- 字符串 `String`
  - 字符串类型  
    Rust核心语言中只有一种字符串类型: `str`字符串slice  
    `String`类型是由标注库提供的, 可增长\可变\有所有权\UTF-8编码
  - 其他字符串类型  
    OsString, OsStr, CString, CStr, ..
  - 创建
    ```rust
    let mut s = String::new();      // 创建空字符串
    let s = "initial contents";     // 字面量
    let mut s = String::from(s);    // 从其他字符串创建
    let mut s = s.to_string();      // 字符串克隆
    ```
  - 更新  
    ```rust
    let mut s = "foo".to_string();
    s.push(' ');
    s.push_str("bar");    // foobar

    // arith.rs
    // - Add Trait
    let s1 = String::from("Foo");
    let s2 = String::from("Bar");
    let s3 = s1 + &s2;        // s1所有权会移交到s3
    // let s3 = s1.add(&s2);  // +运算符底层调用

    // ❌s1不可用
    // println!("s1:{}", s1);

    // Add trait 声明参考
    pub trait Add<Rhs = Self> {
      type Output;
      // +运算符会获取 self 所有权
      fn add(self, rhs:Rhs) -> Self::Output;
    }

    // 使用format! macro
    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");
    let s4 = format!("{}-{}-{}",s1, s2, s3);
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
    ```
    1. `push()`/`push_str()`
    2. `+`(`Add Trait`), `&String`可以强转为`&str`
    3. `format!`不获取所有权
  - 索引  
    rust不支持字符串直接索引, 可以使用 `charts()`/`bytes()`分别获取字符和字节
    ```rust
    let s = String::from("jeck");
    for el in s.charts() {
      println!("ch: {}", el);
    }
    for b in s.bytes() {
      println!("byte: {}", b);
    }
    ```
- `HashMap`键值对集合  
  `HashMap`键值使用都会获取所有权
  ```rust
  // 导入集合
  use std::collections::HashMap;

  // 创建空集合
  // 逐条加入
  let mut score = HashMap::new();
  score.insert(String::from("Blue"), 10);
  score.insert(String::from("Red"), 50);

  // // 从两个vector中收集
  // let v1 = vec![String::from("Blue"), String::from("Red")];
  // let v2 = vec![10, 50];
  // // collect() 方法需要明确返回值类型
  // let map:HashMap<_,_> = v1.iter().zip(v2.iter()).collect();

  // 获取数据
  let blue = String::from("Blue");
  let opt_val:Option<i32> = score.get(&blue);

  // 更新
  score.insert(String::from("Blue"), 20);   // 覆盖更新
  let yellow_score = score.entry(String::from("Yellow"))
      .or_insert(30);                       // 仅首次插入
  *yellow_score += 100;                     // 解引用后更新

  ```

# 错误与恢复
- `Result<T,E>`  
  执行逻辑可能导致 `panic` 时, 优先考虑 `Result<T,E>` 将错误告知调用者
- 匹配不同的错误
  ```rust
  let f = File::open("hello.txt");
  let f = match f {
      Ok(f) => f,
      // 模式守卫者
      // Element(ref data) if condition => {}
      // condition 为true匹配成功, 否则继续下面的匹配
      Err(ref error) if error.kind() == ErrorKind::NotFound => {
          match File::create("hello.txt") {
              Ok(f) => f,
              Err(error) => {
                  panic!("Tried to crated file but there wath a problem:{:?}", error)
              },
          }
      },
      Err(e) => {
          panic!("There was a problem opening the file: {:?}", e);
      }
  };
  ```
- 使用`?`代替 `match Result` 传播错误
  ```rust
  pub fn read_file()->Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string()?;
    Ok(s)
  }
  ```
- 错误处理指导原则
  - 有害状态并不包含 预期 会偶尔发生的错误
  - 之后的代码的运行依赖于处于这种有害状态
  - 当没有可行的手段来将有害状态信息编码进所使用的类型中的情况

# 泛型/Trait/lifecircle
- `Generic Type`: 泛型  
  泛型声明位于具体申明之后  
  - 结构体  
    `struct Type<T> {}`
  - 实现  
    `impl<T> Type<T> {}`
  - 方法:  
    `fn new<T>(value:T) -> Type<T> {}`
  ```rust
  // 泛型结构体
  struct Point<T, U> {
    x: T,
    y: U,
  }

  // 泛型结构体方法
  // impl<T> Point<T> {}
  // 泛型类型需要定义再 impl 之后: impl<T>
  impl<T, U> Point<T, U> {

    // 实例泛型使用
    pub fn x(&self) -> &T {
      &self.x
    }

    // 方法泛型
    pub fn mixup<V, W>(self, other: Point<V,W>) -> Point<T, W> {
      Point {
        x: self.x,
        y: other.y
      }
    }

  }
  ```
  - 泛型编译时会展开为`单态模式`
  ```rust
  let integer = Some(5);
  let float = Some(5.0);

  // 展开为
  enum Option_i32 {
    Some(i32),
    None
  }
  enum Option_f64 {
    Some(f64),
    None
  }
  ```
- `Trait`定义共享行为
  - 定义
  ```rust
  ///
  /// 可摘要的
  ///
  pub trait Summarizable {
    ///
    /// 获取摘要
    ///
    fn summary(&self) -> String;

    ///
    /// trait默认实现
    /// 
    fn summary_default(&self) -> String {
      format!("(Read more...)")
    }
  }
  ```
  - 为`struct`实现`trait`
  ```rust
  pub struct NewsArticle {
      pub headline: String,
      pub location: String,
      pub author: String,
      pub content: String,
  }

  ///
  /// 实现Summarizable trait
  /// 
  ///
  impl Summarizable for NewsArticle {

    ///
    /// summary签名来自 Summarizable
    /// 
    fn summary(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }
  }
  ```
  - `Trait Bounds`特性边界限定  
    使用`+`限定多个特性
  ```rust
  /// 一个trait
  fn notify<T: Summarizable>(item: T) {}

  /// item 必须同时实现 Summarizable 和 Disable 两个特性
  fn fmt_news<T: Summarizable + Disable>(item: T) {}
  ```

  - 对现有`struct`扩展新`trait`  
    对已有`Tweet`扩展`DisplayPretty`
  ```rust
  // tweet_new.rs
  use super::tweet::Tweet;
  pub trait DispayPretty {
    fn summary(&self) -> String {
      format!("default pretty summary...")
    }
  }

  impl DisplayPretty for Tweet {}
  ```
  使用
  ```rust
  use crate::tweet::Tweet;
  use crate::tweet_new::DisplayPretty;

  fn test() {
    let tweet = Tweet {};

    // 限定使用 tweet_new.rs 中实现的方法
    DisplayPretty::summary(&tweet);
  }
  ```
  - 针对实现了特定`trait`的类型实现特定方法
  ```rust
  trait StringLength {
      fn str_len(&self) -> usize;
  }

  impl<T: Display> StringLength for T {
      fn str_len(&self) -> usize {
          let s = self.to_string();
          s.len()
      }
  }

  fn test() {
    let str_len = 3.str_len();
    println!("3 str_len: {}", str_len);
  }
  ```
- `Life Circle`生命周期  
  - 函数参数/返回值
  ```rust
  ///
  /// 泛型 'a 为生命周期标记
  /// 
  /// 参数a类型: &str
  /// 参数b类型: &str
  /// 返回值: &str
  /// 
  /// 参数a/b/返回值具有相同生命周期
  /// 
  fn largest<'a>(a:&'a str, b:&'b str) -> &'a str;

  ///
  /// 泛型+生命周期
  /// 
  /// 参数a: &str
  /// 参数b: &str
  /// 参数c: T
  /// 
  /// 参数a/b/返回值具有相同生命周期
  /// 
  /// 参数c必须实现Display trait
  /// 
  fn longest<'a, T>(a:&'a str, b:&'a str, c:T) -> &'a str
  where
    T: Display;
  ```
  - 结构体  
    结构体中如果有引用字段, 必须注明字段生命周期注解.  
    声明: `struct StructName<'a, 'b> {}`.  
    实现: `impl<'a'> StructName<'a'> {}`.
  ```rust
  #[derive(Debug)]
  struct ImporantExcerpt<'a> {
    ///
    /// part引用不能小于ImportantExcerpt示例
    /// 
    part: &'a str,
  }

  impl<'a> ImportantExcerpt<'a> {
    pub fn part(&mut self, part:&'a &str) -> &mut Self {
      self.part = part;
      self
    }
  }

  fn test() {
    let s = String::from("Call me Ishmeal. Some years ago...");
    let part = s.split(".").next().expect("Could not find a '.'");
    let ie = ImportantExcerpt {
      part
    };
    println!("ie. {:?}", ie);
  }
  ```
  - 默认生命周期规则  
    1. 每一个是引用的参数都有它自己的生命周期参数
    2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
    3. 如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为`&self`或`&mut self`, 那么`self`的生命周期被赋给所有输出生命周期参数

# 测试
- 测试模块
  ```rust
  ///
  /// 测试模块需要添加测试标记
  /// 
  /// #[cfg(test)]
  /// 
  #[cfg(test)]
  mod tests {

    ///
    /// 测试入口方法
    /// 
    /// #[test]
    /// 
    #[test]
    fn it_works() {

      // 断言宏
      assert!(2+2,4);
    }

  }
  ```
  - 常用断言宏
  ```rust
  ///
  /// 接受bool值
  /// 
  /// tipsFmt:
  ///   - 字符串/格式字符串, 交由 format! 处理
  ///   - 可选
  ///   - 测试失败时打印的提示消息
  ///  
  /// fmtVals:
  ///   - traits: Display
  ///   - 可选
  ///   - tipsFmt占位符实际值
  /// 
  assert!(bool, tipsFmt?, fmtVals?);

  ///
  /// 接受两个参数 a,b
  /// 所有参数都必须实现 PartialEq, Debug 两个trait
  /// 
  /// tipsFmt:
  ///   - 字符串/格式字符串, 交由 format! 处理
  ///   - 可选
  ///   - 测试失败时打印的提示消息
  ///  
  /// fmtVals:
  ///   - traits: Display
  ///   - 可选
  ///   - tipsFmt占位符实际值
  /// 
  assert_eq!(a, b, tipsFmt?, fmtVals);
  ```

  | 接口 | 类型 | 参数 | 说明 | 示例 |
  |--|--|--|--|--|
  | test | `Attribute` | - | 标记为测试模块<br>标记为测试方法入口 | `#[cfg(test)]`<br>`#[test]` |
  | should_panic | `Attribute` | - 可选, 消息全量匹配 <br>- expected: 可选, 消息包含匹配 | 标记为目标测试方法必须`panic`否则失败, 需与`#[test]`联合使用 | `#[should_panic]`<br>`#[should_panic="full_err_msg"]`<br> `#[should_panic(expected="container_err_msg")]`|
  | ignore | `Attribute` | - | 排除指定测试方法 | `#[ignore]` |
  | assert | `Macro` | - c: `bool`测试条件 <br>- sof: 可选, 消息或格式化消息 <br>- fv: 可选, 格式消息替换值 | 断言条件是否为`true` | `assert!(false, "失败了, {}", "测试条件");` |
  | assert_eq | `Macro` | - lv: `expr`左值表达式 <br>- rv: `expr`右值表达式 <br>- sof: 可选, 消息或格式化消息 <br>- fv: 可选, 格式消息替换值 | 断言两个值是否相等<br>所有参数必须实现`PartialEq`和`Debug` | `assert!(false, "失败了, {}", "测试条件");` |
  | assert_ne | `Macro` | - lv: `expr`左值表达式 <br>- rv: `expr`右值表达式 <br>- sof: 可选, 消息或格式化消息 <br>- fv: 可选, 格式消息替换值 | 断言两个值是否不相等<br>所有参数必须实现`PartialEq`和`Debug` | `assert!(false, "失败了, {}", "测试条件");` |
  
