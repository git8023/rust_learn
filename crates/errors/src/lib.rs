use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

pub fn test() {
    // panic_macro();
    // be_panic();
    // match_handle_result();
    let s = read_username_from_file();
    println!("s = {:?}", s);
}

///
/// 错误传递
///
pub fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

///
/// match处理Result
///
pub fn match_handle_result() {
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(f) => f,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(f) => f,
            Err(error) => {
                panic!("Tried to crated file but there wath a problem:{:?}", error)
            }
        },
        Err(e) => {
            panic!("There was a problem opening the file: {:?}", e);
        }
    };
}

///
/// 被动panic
///
pub fn be_panic() {
    let v = vec![1, 2, 3];
    println!("{}", v[99]);
}

///
/// 调用 panic 宏
///
pub fn panic_macro() {
    panic!("crash and burn");
}
