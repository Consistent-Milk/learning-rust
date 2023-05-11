pub fn largest(num_list: Vec<i32>) -> i32 {
    let mut largest: &i32 = &num_list[0];
    for number in &num_list {
        if number > largest {
            largest = number;
        }
    }
    return *largest;
}

pub fn largest_proper(num_list: &[i32]) -> &i32 {
    let mut largest: &i32 = &num_list[0];

    for item in num_list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

// This generic type implements PartialOrd
pub fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        return &self.x;
    }

    pub fn y(&self) -> &U {
        return &self.y;
    }

    // Complete this later
    // pub fn distance_origin_generic(&self) -> f64 {

    // }
}

impl Point<f64, f64> {
    pub fn distance_from_origin(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}
