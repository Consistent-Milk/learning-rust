fn main() {
    let t: u64 = 2_134_835_688;

    let earth_year_second: u64 = 31557600;

    let mercury_orbital: f64 = 0.2408467;

    let mercury_years: f64 = (t as f64) / (mercury_orbital * (earth_year_second as f64));

    println!(
        "{} seconds on Mercury is = {:.2} years on Mercury",
        t, mercury_years
    );
}
