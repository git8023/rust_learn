pub fn test() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let largest = largest_i32(&number_list);
    let largest_el = largest(&number_list);
    // assert_eq!(100, *largest_el);
    println!("i32 largest: {}", largest_el);

    let char_list = vec!['a', 'b', 'd', 'c'];
    // let lagest = largest_char(&char_list);
    let largest_el = largest(&char_list);
    // assert_eq!('d', *largest_el);
    println!("char largest: {}", largest_el);
}

///
/// 获取最大值引用
///
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for el in list.iter() {
        if el > largest {
            largest = el;
        }
    }

    largest
}

///
/// 排序 + 拷贝(栈内, 堆拷贝:Clone)
///
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &el in list.iter() {
//         if largest < el {
//             largest = el;
//         }
//     }
//     largest
// }

// fn largest_char(slice: &[char]) -> char {
//     let mut largest = slice[0];
//     for &el in slice.iter() {
//         if largest < el {
//             largest = el;
//         }
//     }
//     largest
// }

// fn largest_i32(slice: &[i32]) -> i32 {
//     let mut largest = slice[0];
//     for &el in slice.iter() {
//         if largest < el {
//             largest = el;
//         }
//     }
//     largest
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn minup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
