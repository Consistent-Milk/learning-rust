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

fn main() {
    let cond: bool = true;
    let x: i32 = if cond { 1 } else { 0 };

    println!("{}", x);

    let y: i32;
    if cond {
        y = 1;
    } else {
        y = 0;
    }

    println!("{}", y);
}