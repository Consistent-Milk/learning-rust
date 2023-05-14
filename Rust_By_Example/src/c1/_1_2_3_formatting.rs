use std::fmt::{self, Display, Formatter};

pub struct City {
    pub name: &'static str,
    // Latitude
    pub lat: f32,
    // Longitude
    pub lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return write!(
            f,
            "RGB ({}, {}, {}) 0x{:0>2X}{:0>2X}{:0>2X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue
        );
    }
}

impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let val = self.red;

        return fmt::UpperHex::fmt(&val, f);
    }
}
