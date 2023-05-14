use c1::_1_2_2_display::*;
use c1::_1_2_3_formatting::*;
use c2::_2_all::*;
use rust_by_example::c1;
use rust_by_example::c2;
use std::mem;

fn main() {
    ///////////////////////////////////////////////////////////////////////////////////
    /////// Chapter 1 - Runtime Code

    println!("\n\nChapter 1 - Runtime Code Output");
    println!("------------------------------------");

    let minmax: MinMax = MinMax(0, 14);

    let point: Point2D = Point2D { x: 2.0, y: 3.0 };

    let complex: Complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    let l: List = List(vec![1, 2, 3]);

    println!("Compare structures(MinMax - Display vs Debug):");
    println!("Display: {}", minmax);
    println!("Debug: {:?}\n", minmax);

    println!("\nCompare structures:(Point - Display vs Debug)");
    println!("Display: {}", point);
    println!("Debug: {:?}\n", point);

    println!("\nCompare structures:(Complex - Display vs Debug)");
    println!("Display: {}", complex);
    println!("Debug: {:?}\n", complex);

    println!("\nPrinting a List using Display trait:");
    println!("{}\n", l);

    let cities: [City; 3] = [
        City {
            name: "Dublin",
            lat: 53.34778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ];

    let colors: [Color; 3] = [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ];

    println!("\nUsing Display trait of City:");
    for city in cities.iter() {
        println!("{}", *city);
    }

    println!("\nUsing Display trait of Color:");
    for color in colors.iter() {
        println!("{}", *color);
    }

    println!("\n\n\n");

    //////////////////////////////////////////////////////////////////////////////////////////// Chapter 2 - Runtime code

    println!("Chapter 2 - Runtime Code Output");
    println!("------------------------------------");

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reversed pair is {:?}", reverse_pair(pair));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("\n{:?}, {:?}, {:?}, {:?}\n", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);

    println!("Transposed Matrix:\n{}", matrix_transpose(matrix));

    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    println!("\n\n\n");
}
