mod models;
mod brands;

use std::io::{self, Write, stdin, stdout};
use std::num::ParseIntError;

fn main() -> io::Result<()> {
    let mut brands: Vec<String> = Vec::new();
    let mut models: Vec<String> = Vec::new();

    loop {
        let _ = clearscreen::clear();

        println!("--- Car Manager ---\n");
        println!("1. Add Car Brand\n2. Add Car Model\n3. List All Car Brands\n4. List All Car Models\n5. Exit\n");

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

                println!("--- Car Model Entry Menu ---\n");

                let new_model = models::get_models()?;
                models.extend(new_model);                
            }
            Ok(3) => {
                let _ = clearscreen::clear();

                println!("--- Brand Listing Menu ---\n");

                brands::list_brands(&brands)?;
                
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                stdin().read_line(&mut input)?;
            }
            Ok(4) => {
                let _ = clearscreen::clear();

                println!("--- Model Listing Menu ---\n");

                models::list_models(&models)?;
                
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                stdin().read_line(&mut input)?;
            }
            Ok(5) => {
                println!(">> Exiting the program. Goodbye!");
                break;
            }

            Ok(invalid) => {
                println!(
                    ">> Error: {} is not a valid menu option!",
                    invalid
                );
            }

            Err(_) => {
                println!(">> Error: Please enter only a number!");
            }
        }
    }

    Ok(())
}
