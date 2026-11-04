use std::io::{self, Write, stdin, stdout};

pub fn brands() -> io::Result<(String, String, String)> {
    let mut bmwstr = String::new();
    let mut mercedesstr = String::new();
    let mut toyotastr = String::new();

    print!("Please Input BMW Brand: ");
    stdout().flush()?;
    stdin().read_line(&mut bmwstr)?;

    print!("Please Input Mercedes Brand: ");
    stdout().flush()?;
    stdin().read_line(&mut mercedesstr)?;

    print!("Please Input Toyota Brand: ");
    stdout().flush()?;
    stdin().read_line(&mut toyotastr)?;

    Ok((bmwstr, mercedesstr, toyotastr))
}   