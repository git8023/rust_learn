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
  