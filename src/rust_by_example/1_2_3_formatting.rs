// Formatting
// https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html



/*
We've seen that formatting is specified via a format string:

    format!("{}", foo) -> "3735928559"
    format!("0x{:X}", foo) -> "0xDEADBEEF"
    format!("0o{:o}", foo) -> "0o33653337357"

The same variable (foo) can be formatted differently depending on which argument type is used: X vs o vs unspecified.

This formatting functionality is implemented via traits, and there is one trait for each argument type. The most common formatting trait is Display, which handles cases where the argument type is left unspecified: {} for instance.
*/



use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
    }
}




/*
You can view a full list of formatting traits and their argument types in the std::fmt documentation.
*/





