use std::io::{self, Write, stdin, stdout};

struct brands {
    bmw: String,
    mercedes: String,
    toyota: String,
}

struct prices {
    bmw_price: i32,
    mercedes_price: i32,
    toyota_price: i32,
}

fn main() -> io::Result<()> {

    let mut bmwstr = String::new();
    let mut mercedesstr = String::new();
    let mut toyotastr = String::new();
    
    // BMW Fiyat isteme
    print!("Please input BMW price: ");
    stdout().flush()?;
    stdin().read_line(&mut bmwstr)?;

    // Mercedes Fiyat isteme
    print!("Please input Mercedes price: ");
    stdout().flush()?;
    stdin().read_line(&mut mercedesstr)?;

    // Toyota Fiyat isteme
    print!("Please input Toyota price: ");
    stdout().flush()?;
    stdin().read_line(&mut toyotastr)?;
    
    // BMW i32 Parse
    let bmw: i32 = match bmwstr.trim().parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("BMW Parse Error: {}", e);
            0
        }
    };

    // Mercedes i32 Parse
    let mercedes: i32 = match mercedesstr.trim().parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Mercedes Parse Error: {}", e);
            0
        }
    };

    // Toyota i32 Parse
    let toyota: i32 = match toyotastr.trim().parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Toyota Parse Error: {}", e);
            0
        }
    };


    
    let mut car_brands = brands {
        bmw: String::from("M5 E60"),
        mercedes: String::from("Benz F16"),
        toyota: String::from("Supra"),
    };

    let mut car_prices = prices {
        bmw_price: bmw,
        mercedes_price: mercedes,
        toyota_price: toyota,
    };

    Ok(())
}
