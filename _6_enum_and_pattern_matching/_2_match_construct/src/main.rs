enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let penny: Coin = Coin::Penny;
    let nickel: Coin = Coin::Nickel;
    let dime: Coin = Coin::Dime;
    let quarter: Coin = Coin::Quarter;

    let c_1: u8 = value_in_cents(penny);
    let c_2: u8 = value_in_cents(nickel);
    let c_3: u8 = value_in_cents(dime);
    let c_4: u8 = value_in_cents(quarter);

    println!("{} {} {} {}", c_1, c_2, c_3, c_4);

    // if let allows us to check for a specific match case
    // and what to do when the case matches
    let config_max: Option<u8> = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let x: Option<i32> = Some(3);

    println!(
        "Adding 1 to Option<i32> with value 3 returns: {}",
        plus_one(x).unwrap()
    );
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
