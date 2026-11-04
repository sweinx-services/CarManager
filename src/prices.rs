use std::io::{self, Write, stdin, stdout};

pub fn prices() -> io::Result<(i32, i32, i32)> {
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
    Ok((bmw, mercedes, toyota))
}