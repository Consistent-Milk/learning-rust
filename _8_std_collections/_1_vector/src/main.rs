fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; -> Panics
    // let does_not_exist = v.get(100); -> Returns None

    let v_new: Vec<i32> = vec![100, 52, 48];

    for i in &v_new {
        println!("{i}");
    }

    let mut v_new_mut: Vec<i32> = vec![100, 52, 48];

    for i in &mut v_new_mut {
        *i += 10;
        println!("{i}");
    }

    // Storing values of multiple types in a vector using enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
