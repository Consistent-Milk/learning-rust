#![allow(unused)]
// fn main() {
//     let t: ([i32; 2], [i32; 4]) = ([1; 2], [3; 4]);
//     let (a, _) = t;
//     println!("{}", a[0] + t.1[0]);
// }

/////////////////////////

// fn main() {
//     let y: i32 = {
//         let x: i32 = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

/////////////////////////

// fn f(x: i32) -> i32 { x + 1 }
// fn main() {
//   println!("{}", f({
//     let y = 1;
//     y + 1
//   }));
// }

/////////////////////////

// fn main() {
//     let cond: bool = true;
//     let x: i32 = if cond { 1 } else { 0 };

//     println!("{}", x);

//     let y: i32;
//     if cond {
//         y = 1;
//     } else {
//         y = 0;
//     }

//     println!("{}", y);
// }

//////////////////////////////

// fn main() {
//     let mut x = 0;
//     'a: loop {
//         x += 1;
//         'b: loop {
//             if x > 10 {
//                 continue 'a;
//             } else {
//                 println!("Breaking 'b with x = {}", x);
//                 break 'b;
//             }
//         }
//         println!("Breaking 'a with x = {}" , x);
//         break;
//     }
// }

////////////////////////////////

// fn main() {
//     let a = [5; 10];
//     let mut sum = 0;
//     for x in a {
//         sum += x;
//     }
//     println!("{sum}");
// }

///////////////////////////////

// fn main() {
//     let s: String = String::from("hello");
//     let s2: *const u8 = s.as_ptr();
//     let b: bool = false;

//     println!("{:?}", s2);
//     println!("{}", s);
// }

/////////////////////////////////
// fn main() {
//     let n = 5;
//     let y = plus_one(&n);
//     println!("The value of y is: {y}");
//     println!("{}", n);

//     let s = String::from("Hello");
//     push_ref(&s);
//     println!("{}", s);

//     push_val(s);
//     // Commenting out the line below will
//     // cause a compilation error as s goes into the scope
//     // of push_val function and is dropped when push_val's scope ends
//     // println!("{}", s);
// }

// fn plus_one(x: &i32) -> i32 {
//     x + 1
// }

// fn push_ref(mut str: &String) -> () {}

// fn push_val(str: String) -> () {}

// fn main() {
//     let mut s = String::from("hello");
//     let s2 = &s;
//     let s3 = &mut s;
//     s3.push_str(" world");
//     println!("{s2}");
// }

use rust_book_quizzes::guessing_game;

fn main() {
    guessing_game();
}
