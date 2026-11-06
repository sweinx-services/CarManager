use std::{io::{self, Write, stdin, stdout} };

pub fn get_brand() -> io::Result<Vec<String>> {
    let mut brand: Vec<String> = Vec::new();

    print!("Please enter car brand: "); stdout().flush()?;

    let mut brandstr = String::new();
    stdin().read_line(&mut brandstr)?;

    let brandstr = brandstr.trim().to_string();
    brand.push(brandstr);

    Ok(brand)
}

pub fn list_brands(brand: &Vec<String>) -> io::Result<()> {
    for i in brand {
        println!("- {}", i); 
    }

    Ok(())
}