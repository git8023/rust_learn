pub struct Guess {
    val: u32,
}

impl Guess {
    pub fn new(val: i32) -> Guess {
        if !(0 <= val && val < 100) {
            panic!("期望范围: [0~100)")
        }

        Guess { val: val as u32 }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new() {
//         Guess::new(50);
//     }

//     #[test]
//     #[should_panic(expected="container_msg")]
//     fn test_new_panic() {
//         Guess::new(-1);
//     }
// }
