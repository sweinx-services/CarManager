mod test;
mod brands;
mod prices;
use std::io;
use crate::{brands::brands, prices::prices};

struct Brands {
    bmw_brand: String,
    mercedes_brand: String,
    toyota_brand: String,
}

struct Prices {
    bmw_price: i32,
    mercedes_price: i32,
    toyota_price: i32,
}

fn main() -> io::Result<()> {
    let (bmwstr, mercedesstr, toyotastr) = brands()?;
    let car_brands = Brands {
        bmw_brand: bmwstr,
        mercedes_brand: mercedesstr,
        toyota_brand: toyotastr,
    };

    let (bmw, mercedes, toyota) = prices()?;
    let car_prices = Prices {
        bmw_price: bmw,
        mercedes_price: mercedes,
        toyota_price: toyota,
    };

    println!("Car Brands: {}, {}, {}", car_brands.bmw_brand, car_brands.mercedes_brand, car_brands.toyota_brand);
    println!("Car Prices: {}, {}, {}", car_prices.bmw_price, car_prices.mercedes_price, car_prices.toyota_price);

    Ok(())
}
