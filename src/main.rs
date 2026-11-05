
mod brands;

use std::io::{self, Write, stdin, stdout};
use std::num::ParseIntError;

fn main() -> io::Result<()> {
    let mut brands: Vec<String> = Vec::new();

    loop {
        let _ = clearscreen::clear();

        println!("--- Car Manager ---\n");
        println!("1. Add Car Brand and Model");
        println!("2. List All Cars");
        println!("3. Exit\n");

        print!(">> Please enter choice: ");
        stdout().flush()?;

        let mut choicestr = String::new();
        stdin().read_line(&mut choicestr)?;

        let choice: Result<u8, ParseIntError> = choicestr.trim().parse();

        match choice {
            Ok(1) => {
                let _ = clearscreen::clear();

                println!("--- Car Brand Entry Menu ---\n");

                let new_brand = brands::get_brand()?;
                brands.extend(new_brand);
            }

            Ok(2) => {
                let _ = clearscreen::clear();

                println!("--- Brand Listing Menu ---\n");

                brands::list_brands(&brands)?;
                
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                stdin().read_line(&mut input)?;
            }

            Ok(3) => {
                println!(">> Programdan çıkılıyor. Hoşça kalın!");
                break;
            }

            Ok(invalid) => {
                println!(
                    ">> Hata: {} geçerli bir menü seçeneği değil!",
                    invalid
                );
            }

            Err(_) => {
                println!(">> Hata: Lütfen sadece bir sayı giriniz!");
            }
        }
    }

    Ok(())
}
