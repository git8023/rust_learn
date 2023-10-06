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

