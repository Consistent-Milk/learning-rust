use c1::_1_2_2_display::*;
use c1::_1_2_3_formatting::*;
use rust_by_example::c1;

fn main() {
    let minmax: MinMax = MinMax(0, 14);

    let point: Point2D = Point2D { x: 2.0, y: 3.0 };

    let complex: Complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    let l: List = List(vec![1, 2, 3]);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    println!("Compare structures:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("Compare structures:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    println!("{}", l);

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

    for city in cities.iter() {
        println!("{}", *city);
    }

    for color in colors.iter() {
        println!("{:?}", *color);
    }
}
